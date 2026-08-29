//! A small local verifier for intentionally configured Git worktrees.
use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    fs,
    hash::{Hash, Hasher},
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    process::Command,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

const STATUS_REQUESTS_PER_SECOND: u32 = 60;

const DEFAULT_CONFIG: &str = r#"# Checks run only in the worktree listed under each entry.
# Keep checks short and avoid shared build caches when worktrees run together.
# Commands run with your normal user permissions. For a filesystem or network
# boundary, wrap the command with your OS sandbox (see README for a bwrap recipe).
[server]
address = "127.0.0.1:4318"
poll_seconds = 3

[[worktrees]]
name = "app"
path = "../app"
checks = ["npm test -- --runInBand"]
"#;

#[derive(Parser, Debug)]
#[command(
    name = "worktree-verifier",
    version,
    about = "Run local smoke checks in configured worktrees."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Write a commented configuration file.
    Init {
        #[arg(short, long, default_value = ".worktree-verifier.toml")]
        config: PathBuf,
        #[arg(long)]
        force: bool,
    },
    /// Run checks once, or keep watching and serve a local status board.
    Run {
        #[arg(short, long, default_value = ".worktree-verifier.toml")]
        config: PathBuf,
        /// Run each configured check once, then exit.
        #[arg(long)]
        once: bool,
        /// Print machine-readable result data. Implies --once.
        #[arg(long)]
        json: bool,
    },
    /// Create isolated sample worktrees and run the same verifier flow.
    Demo {
        /// Keep the sample directory instead of removing it after the run.
        #[arg(long)]
        keep: bool,
        /// Keep watching the isolated sample and serve its board.
        #[arg(long)]
        serve: bool,
    },
}

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(default)]
    server: ServerConfig,
    #[serde(default)]
    worktrees: Vec<WorktreeConfig>,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    #[serde(default = "default_address")]
    address: String,
    #[serde(default = "default_poll")]
    poll_seconds: u64,
}
impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            address: default_address(),
            poll_seconds: default_poll(),
        }
    }
}
fn default_address() -> String {
    "127.0.0.1:4318".into()
}
fn default_poll() -> u64 {
    3
}

#[derive(Debug, Deserialize)]
struct WorktreeConfig {
    name: String,
    path: PathBuf,
    #[serde(default)]
    checks: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
struct BoardRow {
    name: String,
    path: String,
    commit: String,
    last_pass_commit: Option<String>,
    changed_files: usize,
    status: Status,
    finished_at: u64,
    detail: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum Status {
    Pass,
    Fail,
    Idle,
    Error,
    Stale,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init { config, force } => init(&config, force),
        Commands::Run { config, once, json } => run_from_config(&config, once || json, json),
        Commands::Demo { keep, serve } => demo(keep, serve),
    }
}

fn init(path: &Path, force: bool) -> Result<()> {
    if path.exists() && !force {
        bail!(
            "{} already exists; pass --force to replace it",
            path.display()
        );
    }
    fs::write(path, DEFAULT_CONFIG).with_context(|| format!("writing {}", path.display()))?;
    println!(
        "Wrote {}. Edit its paths and smoke commands, then run worktree-verifier run.",
        path.display()
    );
    Ok(())
}

fn load(path: &Path) -> Result<Config> {
    let body =
        fs::read_to_string(path).with_context(|| format!("cannot read {}", path.display()))?;
    let cfg: Config =
        toml::from_str(&body).with_context(|| format!("invalid TOML in {}", path.display()))?;
    if cfg.worktrees.is_empty() {
        bail!(
            "{} has no [[worktrees]] entries; add one then run again",
            path.display()
        );
    }
    Ok(cfg)
}

fn run_from_config(path: &Path, once: bool, json: bool) -> Result<()> {
    let cfg = load(path)?;
    let rows = Arc::new(Mutex::new(Vec::new()));
    check_all(&cfg.worktrees, &rows);
    if json {
        println!("{}", serde_json::to_string_pretty(&*rows.lock().unwrap())?);
        if has_failed(&rows.lock().unwrap()) {
            bail!("one or more worktree checks failed")
        }
        return Ok(());
    }
    print_rows(&rows.lock().unwrap());
    if once && has_failed(&rows.lock().unwrap()) {
        bail!("one or more worktree checks failed")
    }
    if once {
        return Ok(());
    }
    let listener = TcpListener::bind(&cfg.server.address).with_context(|| {
        format!(
            "Status board could not bind {}. Choose a free loopback address in [server].address",
            cfg.server.address
        )
    })?;
    let board = rows.clone();
    thread::spawn(move || serve(listener, board));
    println!(
        "Watching every {}s. Board: http://{}",
        cfg.server.poll_seconds.max(1),
        cfg.server.address
    );
    let mut previous = signatures(&cfg.worktrees);
    loop {
        thread::sleep(Duration::from_secs(cfg.server.poll_seconds.max(1)));
        let next = signatures(&cfg.worktrees);
        let changed: Vec<usize> = next
            .iter()
            .zip(&previous)
            .enumerate()
            .filter_map(|(index, (current, prior))| (current != prior).then_some(index))
            .collect();
        if !changed.is_empty() {
            check_selected(&cfg.worktrees, &rows, &changed);
            print_rows(&rows.lock().unwrap());
            // A smoke check may create its own build output. Record the state after
            // that check so it does not cause an unrelated follow-up run.
            previous = signatures(&cfg.worktrees);
        }
    }
}

fn has_failed(rows: &[BoardRow]) -> bool {
    rows.iter()
        .any(|row| matches!(row.status, Status::Fail | Status::Error | Status::Stale))
}

fn check_all(configs: &[WorktreeConfig], rows: &Arc<Mutex<Vec<BoardRow>>>) {
    // Checks intentionally run serially: many test tools write shared caches.
    let prior = rows.lock().unwrap().clone();
    let next: Vec<BoardRow> = configs
        .iter()
        .map(|spec| check_until_stable(spec, matching_row(&prior, spec)))
        .collect();
    *rows.lock().unwrap() = next;
}

fn check_selected(configs: &[WorktreeConfig], rows: &Arc<Mutex<Vec<BoardRow>>>, indexes: &[usize]) {
    let mut updated = rows.lock().unwrap().clone();
    for &index in indexes {
        let spec = &configs[index];
        let prior = matching_row(&updated, spec).cloned();
        let row = check_until_stable(spec, prior.as_ref());
        if index < updated.len() {
            updated[index] = row;
        } else {
            updated.push(row);
        }
    }
    *rows.lock().unwrap() = updated;
}

fn matching_row<'a>(rows: &'a [BoardRow], spec: &WorktreeConfig) -> Option<&'a BoardRow> {
    rows.iter()
        .find(|row| row.name == spec.name && row.path == spec.path.to_string_lossy())
}

const STALE_RECHECKS: usize = 3;

fn check_until_stable(spec: &WorktreeConfig, prior: Option<&BoardRow>) -> BoardRow {
    let last_pass_commit = prior.and_then(|row| row.last_pass_commit.clone());
    let mut latest = check_worktree(spec, last_pass_commit.clone());
    for _ in 0..STALE_RECHECKS {
        if !matches!(latest.status, Status::Stale) {
            return latest;
        }
        latest = check_worktree(spec, last_pass_commit.clone());
    }
    latest
}

fn check_worktree(spec: &WorktreeConfig, last_pass_commit: Option<String>) -> BoardRow {
    let path = spec.path.to_string_lossy().into_owned();
    let now = now();
    if !spec.path.is_dir() {
        return BoardRow {
            name: spec.name.clone(),
            path,
            commit: "—".into(),
            last_pass_commit,
            changed_files: 0,
            status: Status::Error,
            finished_at: now,
            detail: "The configured path does not exist. Check path in the config.".into(),
        };
    }
    if !is_git_worktree(&spec.path) {
        return BoardRow {
            name: spec.name.clone(),
            path,
            commit: "—".into(),
            last_pass_commit,
            changed_files: 0,
            status: Status::Error,
            finished_at: now,
            detail: "The configured path is not a Git worktree. Point it at a Git checkout.".into(),
        };
    }
    let Some(before) = snapshot(&spec.path) else {
        return BoardRow {
            name: spec.name.clone(),
            path,
            commit: "—".into(),
            last_pass_commit,
            changed_files: 0,
            status: Status::Error,
            finished_at: now,
            detail: "Could not snapshot this Git worktree. Check that Git is available.".into(),
        };
    };
    if spec.checks.is_empty() {
        return BoardRow {
            name: spec.name.clone(),
            path,
            commit: before.commit,
            last_pass_commit,
            changed_files: before.changed_files,
            status: Status::Idle,
            finished_at: now,
            detail: "No checks declared. Add a smoke command to checks.".into(),
        };
    }
    let mut failure = None;
    for command in &spec.checks {
        match shell(&spec.path, command) {
            Ok(output) if output.status.success() => {}
            Ok(_output) => {
                failure = Some(format!("Failed: {}", command));
                break;
            }
            Err(_) => {
                failure = Some(format!("Could not start: {}", command));
                break;
            }
        }
    }
    let Some(after) = snapshot(&spec.path) else {
        return BoardRow {
            name: spec.name.clone(),
            path,
            commit: "—".into(),
            last_pass_commit,
            changed_files: 0,
            status: Status::Error,
            finished_at: now,
            detail: "Could not snapshot this Git worktree after its check.".into(),
        };
    };
    if before != after {
        return BoardRow {
            name: spec.name.clone(),
            path,
            commit: after.commit,
            last_pass_commit,
            changed_files: after.changed_files,
            status: Status::Stale,
            finished_at: now,
            detail: "Worktree changed while its checks ran. Rechecking the new snapshot.".into(),
        };
    }
    if let Some(detail) = failure {
        return BoardRow {
            name: spec.name.clone(),
            path,
            commit: before.commit,
            last_pass_commit,
            changed_files: before.changed_files,
            status: if detail.starts_with("Could not start") {
                Status::Error
            } else {
                Status::Fail
            },
            finished_at: now,
            detail,
        };
    }
    BoardRow {
        name: spec.name.clone(),
        path,
        commit: before.commit.clone(),
        last_pass_commit: Some(before.commit),
        changed_files: before.changed_files,
        status: Status::Pass,
        finished_at: now,
        detail: format!(
            "{} smoke check{} passed",
            spec.checks.len(),
            if spec.checks.len() == 1 { "" } else { "s" }
        ),
    }
}

fn shell(dir: &Path, command: &str) -> std::io::Result<std::process::Output> {
    #[cfg(windows)]
    let mut child = {
        let mut c = Command::new("cmd");
        c.args(["/C", command]);
        c
    };
    #[cfg(not(windows))]
    let mut child = {
        let mut c = Command::new("sh");
        c.args(["-lc", command]);
        c
    };
    child.current_dir(dir).output()
}
fn git(dir: &Path, args: &[&str]) -> Option<String> {
    Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
}
fn is_git_worktree(dir: &Path) -> bool {
    git(dir, &["rev-parse", "--is-inside-work-tree"]).as_deref() == Some("true")
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct WorktreeSnapshot {
    commit: String,
    porcelain: String,
    unstaged_diff: String,
    staged_diff: String,
    untracked_content: u64,
    changed_files: usize,
}

/// Capture the commit and visible working-tree state before and after a check.
/// The diffs catch repeated edits to an already-dirty tracked file; hashing
/// untracked file content catches the equivalent case without treating mtime as
/// a change (many tools only touch their own output files).
fn snapshot(dir: &Path) -> Option<WorktreeSnapshot> {
    let commit = git(dir, &["rev-parse", "--short", "HEAD"])?;
    let porcelain = git(dir, &["status", "--porcelain"])?;
    let unstaged_diff = git(dir, &["diff", "--no-ext-diff"])?;
    let staged_diff = git(dir, &["diff", "--cached", "--no-ext-diff"])?;
    Some(WorktreeSnapshot {
        commit,
        changed_files: porcelain.lines().count(),
        porcelain,
        unstaged_diff,
        staged_diff,
        untracked_content: untracked_content_hash(dir),
    })
}

fn untracked_content_hash(dir: &Path) -> u64 {
    let Some(paths) = git(dir, &["ls-files", "--others", "--exclude-standard", "-z"]) else {
        return 0;
    };
    let mut hasher = DefaultHasher::new();
    for relative in paths.split('\0').filter(|path| !path.is_empty()) {
        relative.hash(&mut hasher);
        match fs::read(dir.join(relative)) {
            Ok(contents) => contents.hash(&mut hasher),
            Err(error) => error.kind().hash(&mut hasher),
        }
    }
    hasher.finish()
}
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn signatures(configs: &[WorktreeConfig]) -> Vec<Option<WorktreeSnapshot>> {
    configs
        .iter()
        .map(|worktree| snapshot(&worktree.path))
        .collect()
}

fn print_rows(rows: &[BoardRow]) {
    for row in rows {
        println!(
            "{:<16} {:<5} {}  {} changed  last pass: {}  {}",
            row.name,
            match row.status {
                Status::Pass => "PASS",
                Status::Fail => "FAIL",
                Status::Idle => "IDLE",
                Status::Error => "ERROR",
                Status::Stale => "STALE",
            },
            row.commit,
            row.changed_files,
            row.last_pass_commit.as_deref().unwrap_or("none"),
            row.detail
        );
    }
}

#[derive(Debug)]
struct RateLimiter {
    window_started: Instant,
    requests: u32,
}
impl Default for RateLimiter {
    fn default() -> Self {
        Self {
            window_started: Instant::now(),
            requests: 0,
        }
    }
}
impl RateLimiter {
    fn allow(&mut self) -> bool {
        if self.window_started.elapsed() >= Duration::from_secs(1) {
            self.window_started = Instant::now();
            self.requests = 0;
        }
        if self.requests >= STATUS_REQUESTS_PER_SECOND {
            return false;
        }
        self.requests += 1;
        true
    }
}

fn serve(listener: TcpListener, rows: Arc<Mutex<Vec<BoardRow>>>) {
    let limiter = Arc::new(Mutex::new(RateLimiter::default()));
    for stream in listener.incoming().flatten() {
        let board = rows.clone();
        let request_limiter = limiter.clone();
        // A slow local client must not prevent another browser or script from
        // reading the board. Each connection is bounded by the read timeout.
        thread::spawn(move || {
            let _ = answer(stream, &board, &request_limiter);
        });
    }
}
fn answer(
    mut stream: TcpStream,
    rows: &Arc<Mutex<Vec<BoardRow>>>,
    limiter: &Arc<Mutex<RateLimiter>>,
) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(1)))?;
    stream.set_write_timeout(Some(Duration::from_secs(1)))?;
    let mut request = [0_u8; 1024];
    let _ = stream.read(&mut request)?;
    if !limiter.lock().unwrap().allow() {
        return write!(
            stream,
            "HTTP/1.1 429 Too Many Requests\r\nRetry-After: 1\r\n{}Content-Length: 0\r\n\r\n",
            board_headers()
        );
    }
    let first = String::from_utf8_lossy(&request);
    let json = first.starts_with("GET /status.json ");
    let body = if json {
        serde_json::to_string(&*rows.lock().unwrap()).unwrap_or_else(|_| "[]".into())
    } else {
        board_html(&rows.lock().unwrap())
    };
    let content = if json {
        "application/json"
    } else {
        "text/html; charset=utf-8"
    };
    write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: {content}\r\n{}Content-Length: {}\r\n\r\n{}",
        board_headers(),
        body.len(),
        body
    )
}
fn board_headers() -> &'static str {
    "Cache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nReferrer-Policy: strict-origin-when-cross-origin\r\nContent-Security-Policy: default-src 'none'; style-src 'unsafe-inline'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'\r\n"
}
fn board_html(rows: &[BoardRow]) -> String {
    let cards: String = rows.iter().map(|r| format!("<li><b>{}</b><span class=\"{}\">{}</span><small>Current: {} · Last pass: {} · {} changed files</small><p>{}</p></li>", esc(&r.name), match r.status { Status::Pass => "pass", Status::Fail => "fail", Status::Idle => "idle", Status::Error => "error", Status::Stale => "stale" }, match r.status { Status::Pass => "PASS", Status::Fail => "FAIL", Status::Idle => "IDLE", Status::Error => "ERROR", Status::Stale => "STALE" }, esc(&r.commit), esc(r.last_pass_commit.as_deref().unwrap_or("none")), r.changed_files, esc(&r.detail))).collect();
    format!("<!doctype html><html lang=en><meta charset=utf-8><meta name=viewport content=\"width=device-width,initial-scale=1\"><title>Worktree Verifier status</title><style>body{{background:#f5eedb;color:#17202b;font:16px system-ui;margin:0;padding:2rem}}main{{max-width:50rem;margin:auto}}li{{border-top:2px solid #b8ad94;padding:1rem 0;list-style:none}}span{{float:right;font-weight:700}}.pass{{color:#25674e}}.fail,.error{{color:#b84531}}.idle,.stale{{color:#a36313}}small{{display:block;color:#48515a;margin-top:.4rem}}p{{margin:.5rem 0 0}}</style><main><h1>Worktree status</h1><p>Only this computer can reach this board.</p><ul>{cards}</ul></main></html>")
}
fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn demo(keep: bool, serve: bool) -> Result<()> {
    let root = unique_temp_dir("worktree-verifier-demo");
    let source = root.join("source");
    let a = root.join("checkout-ui");
    let b = root.join("checkout-api");
    let c = root.join("checkout-docs");
    let result = (|| -> Result<()> {
        fs::create_dir_all(&source)?;
        run_git(&source, &["init"])?;
        run_git(
            &source,
            &["config", "user.email", "demo@worktree-verifier.local"],
        )?;
        run_git(&source, &["config", "user.name", "Worktree Verifier demo"])?;
        fs::write(source.join("README.md"), "Isolated demo source\n")?;
        run_git(&source, &["add", "README.md"])?;
        run_git(&source, &["commit", "-m", "Seed isolated demo"])?;

        for (name, dir, file) in [
            ("checkout-ui", &a, "button.ts"),
            ("checkout-api", &b, "health.rs"),
            ("checkout-docs", &c, "guide.md"),
        ] {
            let branch = format!("demo-{name}");
            run_git(
                &source,
                &[
                    "worktree",
                    "add",
                    "-b",
                    &branch,
                    dir.to_str().context("non-UTF-8 demo path")?,
                ],
            )?;
            fs::write(dir.join(file), "sample change\n")?;
            run_git(dir, &["add", file])?;
            run_git(dir, &["commit", "-m", &format!("Add {name} sample")])?;
        }

        let config = root.join("worktree-verifier.toml");
        fs::write(&config, format!("[server]\naddress = \"127.0.0.1:4319\"\npoll_seconds = 2\n\n[[worktrees]]\nname = \"checkout-ui\"\npath = \"{}\"\nchecks = [\"test -f button.ts\"]\n\n[[worktrees]]\nname = \"checkout-api\"\npath = \"{}\"\nchecks = [\"test -f health.rs\"]\n\n[[worktrees]]\nname = \"checkout-docs\"\npath = \"{}\"\nchecks = [\"test -f guide.md\"]\n", toml_path(&a), toml_path(&b), toml_path(&c)))?;
        println!("Sample worktrees: {}", root.display());
        println!(
            "The sample is isolated in a temporary directory. It does not touch your repository."
        );
        run_from_config(&config, !serve, false)
    })();
    if !keep && !serve {
        fs::remove_dir_all(&root).with_context(|| format!("removing {}", root.display()))?;
        println!("Removed sample worktrees.");
    } else {
        println!("Kept sample worktrees at {}", root.display());
    }
    result
}

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{}-{nanos}", std::process::id()))
}
fn run_git(dir: &Path, args: &[&str]) -> Result<()> {
    let output = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .with_context(|| format!("starting git {}", args.join(" ")))?;
    if !output.status.success() {
        bail!(
            "git {} failed in {}: {}",
            args.join(" "),
            dir.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(())
}
fn toml_path(path: &Path) -> String {
    path.to_string_lossy()
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_path_reports_a_next_step() {
        let row = check_worktree(
            &WorktreeConfig {
                name: "gone".into(),
                path: PathBuf::from("/definitely/not/here"),
                checks: vec!["true".into()],
            },
            None,
        );
        assert!(matches!(row.status, Status::Error));
        assert!(row.detail.contains("Check path"));
    }
    #[test]
    fn loopback_is_default() {
        assert_eq!(ServerConfig::default().address, "127.0.0.1:4318");
    }
    #[test]
    fn configured_command_runs_in_its_worktree() {
        let root = unique_temp_dir("wtv-command");
        fs::create_dir_all(&root).unwrap();
        run_git(&root, &["init"]).unwrap();
        run_git(
            &root,
            &["config", "user.email", "test@worktree-verifier.local"],
        )
        .unwrap();
        run_git(&root, &["config", "user.name", "Worktree Verifier test"]).unwrap();
        fs::write(root.join("seed"), "ok").unwrap();
        run_git(&root, &["add", "seed"]).unwrap();
        run_git(&root, &["commit", "-m", "Seed test worktree"]).unwrap();
        let marker = root.join("ran-here");
        let command = format!("printf yes > {}", marker.display());
        let row = check_until_stable(
            &WorktreeConfig {
                name: "sample".into(),
                path: root.clone(),
                checks: vec![command],
            },
            None,
        );
        assert!(matches!(row.status, Status::Pass));
        assert_eq!(fs::read_to_string(marker).unwrap(), "yes");
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn repeated_edit_changes_the_worktree_snapshot() {
        let root = unique_temp_dir("wtv-stamp");
        fs::create_dir_all(&root).unwrap();
        run_git(&root, &["init"]).unwrap();
        run_git(
            &root,
            &["config", "user.email", "test@worktree-verifier.local"],
        )
        .unwrap();
        run_git(&root, &["config", "user.name", "Worktree Verifier test"]).unwrap();
        let file = root.join("source.txt");
        fs::write(&file, "one").unwrap();
        run_git(&root, &["add", "source.txt"]).unwrap();
        run_git(&root, &["commit", "-m", "Seed snapshot test"]).unwrap();
        let first = snapshot(&root).unwrap();
        fs::write(&file, "two changes").unwrap();
        assert_ne!(first, snapshot(&root).unwrap());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn non_git_directory_is_rejected_before_its_command_runs() {
        let root = unique_temp_dir("wtv-non-git");
        fs::create_dir_all(&root).unwrap();
        let marker = root.join("must-not-exist");
        let row = check_worktree(
            &WorktreeConfig {
                name: "not-a-worktree".into(),
                path: root.clone(),
                checks: vec![format!("touch {}", marker.display())],
            },
            None,
        );
        assert!(matches!(row.status, Status::Error));
        assert!(row.detail.contains("not a Git worktree"));
        assert!(!marker.exists());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn rate_limit_returns_429_and_retry_after_after_sixty_requests() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let rows = Arc::new(Mutex::new(Vec::new()));
        let limiter = Arc::new(Mutex::new(RateLimiter::default()));
        let server_rows = rows.clone();
        let server_limiter = limiter.clone();
        let worker = thread::spawn(move || {
            for _ in 0..=STATUS_REQUESTS_PER_SECOND {
                let (stream, _) = listener.accept().unwrap();
                answer(stream, &server_rows, &server_limiter).unwrap();
            }
        });
        let mut last = String::new();
        for _ in 0..=STATUS_REQUESTS_PER_SECOND {
            let mut stream = TcpStream::connect(address).unwrap();
            stream
                .write_all(b"GET /status.json HTTP/1.1\r\nHost: localhost\r\n\r\n")
                .unwrap();
            let mut response = String::new();
            stream.read_to_string(&mut response).unwrap();
            last = response;
        }
        worker.join().unwrap();
        assert!(last.starts_with("HTTP/1.1 429"));
        assert!(last.contains("Retry-After: 1"));
        assert!(last.contains("X-Content-Type-Options: nosniff"));
        assert!(last.contains("Referrer-Policy: strict-origin-when-cross-origin"));
    }
}

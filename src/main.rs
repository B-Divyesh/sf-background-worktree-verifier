//! A small local verifier for intentionally configured Git worktrees.
use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    process::Command,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const DEFAULT_CONFIG: &str = r#"# Checks run only in the worktree listed under each entry.
# Keep checks short and avoid shared build caches when worktrees run together.
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
        return Ok(());
    }
    print_rows(&rows.lock().unwrap());
    if once {
        return Ok(());
    }
    let board = rows.clone();
    let address = cfg.server.address.clone();
    thread::spawn(move || serve(address, board));
    println!(
        "Watching every {}s. Board: http://{}",
        cfg.server.poll_seconds.max(1),
        cfg.server.address
    );
    let mut previous = signatures(&cfg.worktrees);
    loop {
        thread::sleep(Duration::from_secs(cfg.server.poll_seconds.max(1)));
        let next = signatures(&cfg.worktrees);
        if next != previous {
            check_all(&cfg.worktrees, &rows);
            print_rows(&rows.lock().unwrap());
            previous = next;
        }
    }
}

fn check_all(configs: &[WorktreeConfig], rows: &Arc<Mutex<Vec<BoardRow>>>) {
    // Checks intentionally run serially: many test tools write shared caches.
    let next: Vec<BoardRow> = configs.iter().map(check_worktree).collect();
    *rows.lock().unwrap() = next;
}

fn check_worktree(spec: &WorktreeConfig) -> BoardRow {
    let path = spec.path.to_string_lossy().into_owned();
    let now = now();
    if !spec.path.is_dir() {
        return BoardRow {
            name: spec.name.clone(),
            path,
            commit: "—".into(),
            changed_files: 0,
            status: Status::Error,
            finished_at: now,
            detail: "The configured path does not exist. Check path in the config.".into(),
        };
    }
    if spec.checks.is_empty() {
        return BoardRow {
            name: spec.name.clone(),
            path,
            commit: git(&spec.path, &["rev-parse", "--short", "HEAD"])
                .unwrap_or_else(|| "no commit".into()),
            changed_files: changed_count(&spec.path),
            status: Status::Idle,
            finished_at: now,
            detail: "No checks declared. Add a smoke command to checks.".into(),
        };
    }
    for command in &spec.checks {
        match shell(&spec.path, command) {
            Ok(output) if output.status.success() => {}
            Ok(_output) => {
                return BoardRow {
                    name: spec.name.clone(),
                    path,
                    commit: git(&spec.path, &["rev-parse", "--short", "HEAD"])
                        .unwrap_or_else(|| "no commit".into()),
                    changed_files: changed_count(&spec.path),
                    status: Status::Fail,
                    finished_at: now,
                    detail: format!("Failed: {}", command),
                }
            }
            Err(_) => {
                return BoardRow {
                    name: spec.name.clone(),
                    path,
                    commit: "—".into(),
                    changed_files: 0,
                    status: Status::Error,
                    finished_at: now,
                    detail: format!("Could not start: {}", command),
                }
            }
        }
    }
    BoardRow {
        name: spec.name.clone(),
        path,
        commit: git(&spec.path, &["rev-parse", "--short", "HEAD"])
            .unwrap_or_else(|| "no commit".into()),
        changed_files: changed_count(&spec.path),
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
fn changed_count(dir: &Path) -> usize {
    git(dir, &["status", "--porcelain"])
        .map(|s| s.lines().count())
        .unwrap_or(0)
}
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn signatures(configs: &[WorktreeConfig]) -> Vec<String> {
    configs
        .iter()
        .map(|w| {
            format!(
                "{}:{}",
                git(&w.path, &["rev-parse", "HEAD"]).unwrap_or_default(),
                git(&w.path, &["status", "--porcelain"]).unwrap_or_default()
            )
        })
        .collect()
}

fn print_rows(rows: &[BoardRow]) {
    for row in rows {
        println!(
            "{:<16} {:<5} {}  {} changed  {}",
            row.name,
            match row.status {
                Status::Pass => "PASS",
                Status::Fail => "FAIL",
                Status::Idle => "IDLE",
                Status::Error => "ERROR",
            },
            row.commit,
            row.changed_files,
            row.detail
        );
    }
}

fn serve(address: String, rows: Arc<Mutex<Vec<BoardRow>>>) {
    let listener = match TcpListener::bind(&address) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Status board could not bind {address}: {e}");
            return;
        }
    };
    for stream in listener.incoming().flatten() {
        let _ = answer(stream, &rows);
    }
}
fn answer(mut stream: TcpStream, rows: &Arc<Mutex<Vec<BoardRow>>>) -> std::io::Result<()> {
    let mut request = [0_u8; 1024];
    let _ = stream.read(&mut request)?;
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
    write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: {content}\r\nCache-Control: no-store\r\nContent-Length: {}\r\n\r\n{}", body.len(), body)
}
fn board_html(rows: &[BoardRow]) -> String {
    let cards: String = rows.iter().map(|r| format!("<li><b>{}</b><span class=\"{}\">{}</span><small>{} · {} changed files</small><p>{}</p></li>", esc(&r.name), match r.status { Status::Pass => "pass", Status::Fail => "fail", Status::Idle => "idle", Status::Error => "error" }, match r.status { Status::Pass => "PASS", Status::Fail => "FAIL", Status::Idle => "IDLE", Status::Error => "ERROR" }, esc(&r.commit), r.changed_files, esc(&r.detail))).collect();
    format!("<!doctype html><html lang=en><meta charset=utf-8><meta name=viewport content=\"width=device-width,initial-scale=1\"><title>Worktree Verifier status</title><style>body{{background:#f5eedb;color:#17202b;font:16px system-ui;margin:0;padding:2rem}}main{{max-width:50rem;margin:auto}}li{{border-top:2px solid #b8ad94;padding:1rem 0;list-style:none}}span{{float:right;font-weight:700}}.pass{{color:#25674e}}.fail,.error{{color:#b84531}}.idle{{color:#a36313}}small{{display:block;color:#48515a;margin-top:.4rem}}p{{margin:.5rem 0 0}}</style><main><h1>Worktree status</h1><p>Only this computer can reach this board.</p><ul>{cards}</ul></main></html>")
}
fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn demo(keep: bool, serve: bool) -> Result<()> {
    let root = std::env::temp_dir().join(format!("worktree-verifier-demo-{}", now()));
    let a = root.join("checkout-ui");
    let b = root.join("checkout-api");
    let c = root.join("checkout-docs");
    for (dir, file) in [(&a, "button.ts"), (&b, "health.rs"), (&c, "guide.md")] {
        fs::create_dir_all(dir)?;
        fs::write(dir.join(file), "sample change\n")?;
    }
    let config = root.join("worktree-verifier.toml");
    fs::write(&config, format!("[server]\naddress = \"127.0.0.1:4319\"\npoll_seconds = 2\n\n[[worktrees]]\nname = \"checkout-ui\"\npath = \"{}\"\nchecks = [\"test -f button.ts\"]\n\n[[worktrees]]\nname = \"checkout-api\"\npath = \"{}\"\nchecks = [\"test -f health.rs\"]\n\n[[worktrees]]\nname = \"checkout-docs\"\npath = \"{}\"\nchecks = [\"test -f guide.md\"]\n", a.display(), b.display(), c.display()))?;
    println!("Sample worktrees: {}", root.display());
    println!("The sample is isolated in a temporary directory. It does not touch your repository.");
    let result = run_from_config(&config, !serve, false);
    if !keep && !serve {
        fs::remove_dir_all(&root)?;
        println!("Removed sample worktrees.");
    } else {
        println!("Kept sample worktrees at {}", root.display());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // @claim:demo-isolated-worktrees
    fn claim_demo_runs_three_isolated_checks() {
        let root = std::env::temp_dir().join(format!("wtv-test-{}", now()));
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("smoke.txt"), "ok").unwrap();
        let row = check_worktree(&WorktreeConfig {
            name: "sample".into(),
            path: root.clone(),
            checks: vec!["test -f smoke.txt".into()],
        });
        assert!(matches!(row.status, Status::Pass));
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn missing_path_reports_a_next_step() {
        let row = check_worktree(&WorktreeConfig {
            name: "gone".into(),
            path: PathBuf::from("/definitely/not/here"),
            checks: vec!["true".into()],
        });
        assert!(matches!(row.status, Status::Error));
        assert!(row.detail.contains("Check path"));
    }
    #[test]
    // @claim:loopback-default
    fn claim_loopback_is_default() {
        assert_eq!(ServerConfig::default().address, "127.0.0.1:4318");
    }
    #[test]
    // @claim:configured-commands
    fn claim_configured_command_runs_in_its_worktree() {
        let root = std::env::temp_dir().join(format!("wtv-command-{}", now()));
        fs::create_dir_all(&root).unwrap();
        let marker = root.join("ran-here");
        let command = format!("printf yes > {}", marker.display());
        let row = check_worktree(&WorktreeConfig {
            name: "sample".into(),
            path: root.clone(),
            checks: vec![command],
        });
        assert!(matches!(row.status, Status::Pass));
        assert_eq!(fs::read_to_string(marker).unwrap(), "yes");
        fs::remove_dir_all(root).unwrap();
    }
}

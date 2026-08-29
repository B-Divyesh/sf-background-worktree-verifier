use serde_json::Value;
use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

fn temp_dir(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("{prefix}-{}-{nanos}", std::process::id()));
    fs::create_dir_all(&path).expect("create test directory");
    path
}

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_worktree-verifier"))
}

fn run_git(dir: &Path, args: &[&str]) {
    let output = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .expect("start git");
    assert!(
        output.status.success(),
        "git {} failed: {}",
        args.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn init_repo(path: &Path, files: &[(&str, &str)]) {
    run_git(path, &["init"]);
    run_git(
        path,
        &["config", "user.email", "test@worktree-verifier.local"],
    );
    run_git(path, &["config", "user.name", "Worktree Verifier test"]);
    for (name, contents) in files {
        fs::write(path.join(name), contents).expect("write seed file");
        run_git(path, &["add", name]);
    }
    run_git(path, &["commit", "-m", "Seed worktree"]);
}

fn short_commit(path: &Path) -> String {
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .current_dir(path)
        .output()
        .expect("read commit");
    assert!(output.status.success());
    String::from_utf8(output.stdout)
        .expect("commit text")
        .trim()
        .into()
}

fn free_address() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("reserve port");
    let address = listener.local_addr().expect("listener address").to_string();
    drop(listener);
    address
}

fn toml_string(value: &str) -> String {
    serde_json::to_string(value).expect("quote TOML string")
}

fn write_config(path: &Path, address: &str, worktrees: &[(&str, &Path, &[&str])]) {
    let mut body = format!(
        "[server]\naddress = {}\npoll_seconds = 1\n",
        toml_string(address)
    );
    for (name, directory, checks) in worktrees {
        body.push_str("\n[[worktrees]]\n");
        body.push_str(&format!("name = {}\n", toml_string(name)));
        body.push_str(&format!(
            "path = {}\n",
            toml_string(&directory.to_string_lossy())
        ));
        body.push_str("checks = [");
        for (index, check) in checks.iter().enumerate() {
            if index > 0 {
                body.push_str(", ");
            }
            body.push_str(&toml_string(check));
        }
        body.push_str("]\n");
    }
    fs::write(path, body).expect("write config");
}

fn get_response(address: &str) -> std::io::Result<String> {
    let mut stream = TcpStream::connect(address)?;
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    stream
        .write_all(b"GET /status.json HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    Ok(response)
}

fn wait_for_status(address: &str, predicate: impl Fn(&Value) -> bool) -> Value {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        if let Ok(response) = get_response(address) {
            if response.starts_with("HTTP/1.1 200 OK") {
                let (_, body) = response.split_once("\r\n\r\n").expect("HTTP body");
                let value: Value = serde_json::from_str(body).expect("status JSON");
                if predicate(&value) {
                    return value;
                }
            }
        }
        assert!(
            Instant::now() < deadline,
            "status board did not reach expected state"
        );
        thread::sleep(Duration::from_millis(100));
    }
}

fn start_watcher(config: &Path) -> Child {
    cli()
        .args(["run", "--config"])
        .arg(config)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("start watcher")
}

fn stop_watcher(child: &mut Child) {
    let _ = child.kill();
    let _ = child.wait();
}

#[test]
// @claim:demo-isolated-worktrees
fn claim_demo_runs_three_isolated_checks_and_cleans_up() {
    let output = cli()
        .arg("demo")
        .output()
        .expect("run shipped demo command");
    assert!(
        output.status.success(),
        "demo failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let root = stdout
        .lines()
        .find_map(|line| line.strip_prefix("Sample worktrees: "))
        .expect("demo reports its temporary root");
    let rows: Vec<_> = stdout
        .lines()
        .filter(|line| line.contains("checkout-") && line.contains("PASS"))
        .collect();
    assert_eq!(rows.len(), 3, "expected three passing rows:\n{stdout}");
    let commits: Vec<_> = rows
        .iter()
        .map(|row| row.split_whitespace().nth(2).expect("commit in row"))
        .collect();
    assert_eq!(commits.len(), 3);
    assert_ne!(commits[0], commits[1]);
    assert_ne!(commits[0], commits[2]);
    assert_ne!(commits[1], commits[2]);
    assert!(stdout.contains("Removed sample worktrees."));
    assert!(
        !Path::new(root).exists(),
        "demo root should be removed: {root}"
    );

    let kept = cli()
        .args(["demo", "--keep"])
        .output()
        .expect("run inspectable demo");
    assert!(kept.status.success());
    let kept_stdout = String::from_utf8_lossy(&kept.stdout);
    let kept_root = kept_stdout
        .lines()
        .find_map(|line| line.strip_prefix("Sample worktrees: "))
        .expect("kept demo root");
    for name in ["checkout-ui", "checkout-api", "checkout-docs"] {
        let checkout = Path::new(kept_root).join(name);
        let output = Command::new("git")
            .args(["rev-parse", "--is-inside-work-tree"])
            .current_dir(&checkout)
            .output()
            .expect("inspect demo checkout");
        assert!(output.status.success(), "{name} is not a Git worktree");
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "true");
    }
    fs::remove_dir_all(kept_root).expect("remove kept sample root");
}

#[test]
// @claim:loopback-default
fn claim_loopback_is_the_public_cli_default() {
    let root = temp_dir("wtv-loopback-claim");
    let config = root.join("worktree-verifier.toml");
    let output = cli()
        .args(["init", "--config"])
        .arg(&config)
        .output()
        .expect("run init");
    assert!(output.status.success());
    let config_text = fs::read_to_string(&config).expect("read generated config");
    assert!(config_text.contains("address = \"127.0.0.1:4318\""));
    fs::remove_dir_all(root).expect("remove claim fixture");
}

#[test]
// @claim:configured-commands
fn claim_public_cli_runs_only_declared_commands() {
    let root = temp_dir("wtv-configured-command-claim");
    let repo = root.join("checkout");
    fs::create_dir_all(&repo).expect("create repo");
    init_repo(&repo, &[("allowed.txt", "yes\n")]);
    let config = root.join("worktree-verifier.toml");
    write_config(
        &config,
        &free_address(),
        &[(
            "checkout",
            &repo,
            &["test -f allowed.txt && printf declared > declared-marker"],
        )],
    );
    let output = cli()
        .args(["run", "--config"])
        .arg(&config)
        .args(["--once", "--json"])
        .output()
        .expect("run configured smoke command");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read_to_string(repo.join("declared-marker")).unwrap(),
        "declared"
    );
    assert!(
        !repo.join("undeclared-marker").exists(),
        "the CLI must not discover or run an undeclared command"
    );
    fs::remove_dir_all(root).expect("remove claim fixture");
}

#[test]
// @claim:fresh-last-pass
fn claim_watcher_keeps_the_last_pass_and_never_promotes_an_unchecked_commit() {
    let root = temp_dir("wtv-fresh-last-pass-claim");
    let repo = root.join("checkout");
    fs::create_dir_all(&repo).expect("create repo");
    init_repo(
        &repo,
        &[("verdict.txt", "old\n"), (".gitignore", "started\ngo\n")],
    );
    fs::write(repo.join("go"), "initial\n").expect("prepare initial smoke check");
    let old_commit = short_commit(&repo);
    let address = free_address();
    let config = root.join("worktree-verifier.toml");
    write_config(
        &config,
        &address,
        &[(
            "freshness-case",
            &repo,
            &["date +%s%N > started; cat verdict.txt > tested.txt; while [ ! -f go ]; do sleep 0.05; done; grep -qx old tested.txt"],
        )],
    );
    let mut watcher = start_watcher(&config);
    let initial = wait_for_status(&address, |rows| {
        rows[0]["status"] == "pass" && rows[0]["last_pass_commit"] == old_commit
    });
    assert_eq!(initial[0]["commit"], old_commit);
    let started = repo.join("started");
    let before_start = fs::read_to_string(&started).expect("initial check start marker");

    fs::remove_file(repo.join("go")).expect("pause the next smoke check");
    fs::write(repo.join("trigger.txt"), "rerun\n").expect("trigger watcher");
    let start_deadline = Instant::now() + Duration::from_secs(8);
    loop {
        let observed = fs::read_to_string(&started).expect("rerun start marker");
        if observed != before_start {
            break;
        }
        assert!(
            Instant::now() < start_deadline,
            "watcher did not begin the rerun"
        );
        thread::sleep(Duration::from_millis(25));
    }
    fs::write(repo.join("verdict.txt"), "new\n").expect("change verdict during check");
    run_git(&repo, &["add", "verdict.txt"]);
    run_git(
        &repo,
        &["commit", "-m", "Change verdict during smoke check"],
    );
    fs::write(repo.join("go"), "continue\n").expect("finish paused smoke check");
    let new_commit = short_commit(&repo);

    let failed = wait_for_status(&address, |rows| {
        rows[0]["status"] == "fail" && rows[0]["commit"] == new_commit
    });
    assert_eq!(failed[0]["last_pass_commit"], old_commit);
    assert_eq!(
        fs::read_to_string(repo.join("tested.txt")).unwrap(),
        "new\n"
    );
    stop_watcher(&mut watcher);
    fs::remove_dir_all(root).expect("remove freshness fixture");
}

#[test]
// @claim:changed-worktree-only
fn claim_watcher_reruns_only_the_changed_worktree() {
    let root = temp_dir("wtv-scoped-watcher-claim");
    let ui = root.join("ui");
    let api = root.join("api");
    fs::create_dir_all(&ui).expect("create ui repo");
    fs::create_dir_all(&api).expect("create api repo");
    init_repo(&ui, &[("source.txt", "ui one\n")]);
    init_repo(&api, &[("source.txt", "api one\n")]);
    let address = free_address();
    let config = root.join("worktree-verifier.toml");
    write_config(
        &config,
        &address,
        &[
            ("ui", &ui, &["touch ui-check-ran"]),
            ("api", &api, &["touch api-check-ran"]),
        ],
    );
    let mut watcher = start_watcher(&config);
    wait_for_status(&address, |rows| {
        rows.as_array()
            .is_some_and(|rows| rows.len() == 2 && rows.iter().all(|row| row["status"] == "pass"))
    });
    let api_marker = api.join("api-check-ran");
    let api_before = fs::metadata(&api_marker)
        .expect("initial API check marker")
        .modified()
        .expect("initial API check marker time");
    thread::sleep(Duration::from_millis(30));
    fs::write(ui.join("source.txt"), "ui changed\n").expect("change only UI worktree");
    let ui_marker = ui.join("ui-check-ran");
    let deadline = Instant::now() + Duration::from_secs(8);
    loop {
        let ui_ran = fs::metadata(&ui_marker)
            .expect("UI marker")
            .modified()
            .expect("UI marker time")
            > api_before;
        if ui_ran {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "changed UI worktree was not rerun"
        );
        thread::sleep(Duration::from_millis(50));
    }
    let api_after = fs::metadata(&api_marker)
        .expect("API marker after UI change")
        .modified()
        .expect("API marker time after UI change");
    assert_eq!(api_after, api_before, "unchanged API worktree was rerun");
    stop_watcher(&mut watcher);
    fs::remove_dir_all(root).expect("remove watcher fixture");
}

#[test]
fn watcher_binds_before_reporting_ready_and_handles_slow_clients() {
    let root = temp_dir("wtv-server-regression");
    let repo = root.join("checkout");
    fs::create_dir_all(&repo).expect("create repo");
    init_repo(&repo, &[("source.txt", "ready\n")]);
    let occupied = TcpListener::bind("127.0.0.1:0").expect("occupy board port");
    let occupied_address = occupied.local_addr().expect("occupied address").to_string();
    let blocked_config = root.join("blocked.toml");
    write_config(
        &blocked_config,
        &occupied_address,
        &[("checkout", &repo, &[])],
    );
    let blocked = cli()
        .args(["run", "--config"])
        .arg(&blocked_config)
        .output()
        .expect("attempt watcher on occupied port");
    assert!(!blocked.status.success());
    assert!(!String::from_utf8_lossy(&blocked.stdout).contains("Watching every"));
    assert!(String::from_utf8_lossy(&blocked.stderr).contains("could not bind"));
    drop(occupied);

    let address = free_address();
    let config = root.join("server.toml");
    write_config(&config, &address, &[("checkout", &repo, &[])]);
    let mut watcher = start_watcher(&config);
    wait_for_status(&address, |_| true);
    let _slow_client = TcpStream::connect(&address).expect("open slow client");
    thread::sleep(Duration::from_millis(50));
    let started = Instant::now();
    let response = get_response(&address).expect("read board while slow client is open");
    assert!(
        started.elapsed() < Duration::from_millis(500),
        "slow client blocked board"
    );
    assert!(response.contains("X-Content-Type-Options: nosniff"));
    assert!(response.contains("Referrer-Policy: strict-origin-when-cross-origin"));
    assert!(response.contains("Content-Security-Policy:"));
    stop_watcher(&mut watcher);
    fs::remove_dir_all(root).expect("remove server fixture");
}

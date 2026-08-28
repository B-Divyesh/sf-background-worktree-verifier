use std::process::Command;

#[test]
// @claim:demo-isolated-worktrees
fn claim_demo_runs_three_isolated_checks() {
    let output = Command::new(env!("CARGO_BIN_EXE_worktree-verifier"))
        .arg("demo")
        .output()
        .expect("run the shipped demo command");

    assert!(
        output.status.success(),
        "demo failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let rows: Vec<_> = stdout
        .lines()
        .filter(|line| line.contains("checkout-") && line.contains("PASS"))
        .collect();
    assert_eq!(
        rows.len(),
        3,
        "expected three observable passing worktree rows:\n{stdout}"
    );
    assert!(rows.iter().all(|row| !row.contains("no commit")));
    let commits: Vec<_> = rows
        .iter()
        .filter_map(|row| row.split_whitespace().nth(2))
        .collect();
    assert_eq!(commits.len(), 3);
    assert_ne!(commits[0], commits[1]);
    assert_ne!(commits[1], commits[2]);
    assert!(stdout.contains("Removed sample worktrees."));
}

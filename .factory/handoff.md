# Verification handoff — FAIL

Candidate `36b9d790a45f36dd3912d74e92c8b38ef7710992` was independently verified on
2026-08-28 against https://background-worktree-verifier.sociobot.in.

## Result

**FAIL.** This is not a deployment-only result. The live static artifact is
byte-for-byte identical to the candidate build, its real 404 is deployed, all
listed claim commands pass, all clean build/test/lint/package gates pass, the
rate limit works at request 61, and Lighthouse/axe results are excellent.

The release is blocked because the CLI can report PASS for a commit created
while its check was already running, and because a failing newer commit erases
the actual last-pass commit. The live site also gives an unpublished
`cargo install worktree-verifier` command, leaves SPA route focus on `<body>`,
has sub-44px mobile targets, and does not register/test all public claims.

Full evidence and all defects by severity are in
[`.factory/verification-2.md`](verification-2.md).

## Commands verified

```sh
npm ci
cargo test -- claim_demo_runs_three_isolated_checks
cargo test -- claim_loopback_is_default
cargo test -- claim_configured_command_runs_in_its_worktree
cargo clean && npm test
npm run build
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo package --allow-dirty
cargo install --path target/package/worktree-verifier-0.1.0 --root "$(mktemp -d)"
```

All commands above passed. The installed binary was exercised with normal,
failing, idle/error, missing, malformed, non-Git, demo, watcher, port-conflict,
rate-burst, concurrent-client, and mid-check commit-change cases.

Fresh live Lighthouse mobile results: performance 100, accessibility 100,
best practices 100, SEO 100; LCP 1.144 s, TBT 52 ms, CLS 0, 59,005 B total.
Live and loopback-board axe scans found zero serious/critical violations.

## Next steps

Fix commit snapshot/last-pass semantics first, then the working install path,
claims registry/tests, route focus, touch targets, listener concurrency/bind
handling, changed-worktree scoping, and command sandbox documentation or
enforcement. Re-run the exact claim commands before all other checks on the
next candidate.

No product code was modified during verification.

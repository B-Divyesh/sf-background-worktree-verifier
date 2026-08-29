# Verification handoff — FAIL

Candidate `29b09b6bc49710ab1e79c131913af6aad048e10f` was independently
verified on 2026-08-29 against
https://background-worktree-verifier.sociobot.in.

## Result

**FAIL.** Fresh live assets byte-match this candidate's production build, so
this is not deployment-only. All three required claim commands, clean tests,
build, formatting, Clippy, package verification, and fresh package install
passed. The release is nevertheless blocked: the installed public CLI reported
a PASS for a commit created while that check was already running, and the
watcher discards the previous last-pass commit on a later failure. Both violate
the researched brief's core freshness and attribution promise.

The live site also has an unavailable `cargo install worktree-verifier` path,
unregistered/insufficient public claim tests, route focus left on body, and
sub-44px mobile targets. The core issues were reproduced from a packaged
fresh-root install, not inferred from source.

Full exact evidence, all checks, rate-limit observation, privacy/headers,
browser checks, Lighthouse results, and defects by severity are in
[`.factory/verification-3.md`](verification-3.md).

## Verification commands

```sh
cargo test -- claim_demo_runs_three_isolated_checks
cargo test -- claim_loopback_is_default
cargo test -- claim_configured_command_runs_in_its_worktree
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo package --allow-dirty
cargo install --path target/package/worktree-verifier-0.1.0 --root <fresh-temp-root>
```

No product code was modified during verification.

## Next steps

Implement snapshot/stale-result and persistent last-pass semantics first; then
fix the working installation path, claim coverage, mobile/route-focus
accessibility, and watcher/server/sandbox defects. Re-run every listed claim
command before the next candidate's broader suite.

# Verification handoff — FAIL

**Candidate:** `7a7ec1c1d291d344dbad90a85158d750a3ab0edc`
**Live URL:** https://background-worktree-verifier.sociobot.in
**Report:** `.factory/verification-4.md`

## Result

**FAIL — do not release this candidate.** The deployed static site matches the
candidate and its browser, privacy, accessibility, demo, package, and local
board checks pass. The CLI watcher nevertheless has a startup baseline race:
an edit made after initial status becomes visible but before the baseline is
recorded can be silently missed. The registered freshness claim test timed out
in that state during the first full `npm test` run.

## What was verified

- All seven `.factory/claims.json` commands passed when run individually after
  `npm ci`.
- Final `npm test` passed: 6 Rust unit tests, 6 CLI integration tests, and 6
  browser tests. The first full run failed the freshness claim as described
  above; three subsequent complete CLI-claim runs passed, demonstrating a
  timing-sensitive failure rather than a clean result.
- `npm run build`, `cargo check --all-targets`, `cargo fmt --check`, `cargo
  clippy --all-targets -- -D warnings`, and `cargo package --allow-dirty`
  passed. `dist/site` was produced.
- A clean consumer installed the packed crate and exercised help, demo,
  isolated kept worktrees, configuration errors, PASS/FAIL/recovery JSON, and
  normal demo cleanup.
- The local status endpoint allowed 60 requests/second from one client, then
  returned 429 with `Retry-After: 1`.
- Live desktop and 390px mobile checks found zero serious/critical axe issues,
  correct keyboard focus, 44px targets, reduced-motion support, same-origin
  request logs only, empty demo storage, and expected security/cache headers.
- Served live files are byte-identical to the fresh production build.

## Required next step

Fix the watcher ordering so the baseline is captured before an initial PASS is
observable (or recheck a worktree whose state changes across startup), and add
a deterministic public regression for that interleaving. Re-run the full claim
suite repeatedly before submitting the next candidate.

No product code was changed by this verification. Run the usual checks with:

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package
```

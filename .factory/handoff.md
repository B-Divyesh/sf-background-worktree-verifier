# Verification 7 handoff — PASS

**Candidate:** `b673dc1c0ae8c84a2adcd94ef27ac537eac76386`
**Live URL:** https://background-worktree-verifier.sociobot.in
**Report:** `.factory/verification-7.md`
**Verification date:** 2026-08-29

## Result

**PASS.** Independent verification found that the live documentation deployment
matches the candidate static build and that the packaged CLI performs the
brief's local, per-worktree smoke-check job. No release-blocking defects remain.

## What was verified

- All ten commands in `.factory/claims.json` passed verbatim after `npm ci`.
  Claims cover the isolated three-worktree demo, localhost default, listener
  reachability advice, opt-in commands and inherited permissions, bounded
  timeout/RUNNING recovery, fresh last-pass handling, change scoping, demo
  isolation, and no analytics.
- `npm test` passed 15 Rust tests and seven browser tests. Production Vite
  build, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
  `node --check site/src/main.js`, JSON parsing, and
  `cargo package --allow-dirty` also passed.
- The packaged crate was installed under a fresh temporary consumer root.
  `--help`, `--version`, `demo`, `init`, overwrite protection, and
  `run --once --json` error reporting were exercised.
- The real board was checked independently at desktop and 390px mobile. It
  initially exposed RUNNING rows, then three PASS rows; had no serious or
  critical axe findings or console errors; and returned no-store plus CSP,
  nosniff, and referrer-policy headers. Its observed rate allowance is 60
  requests/second; excess requests returned 429 with `Retry-After: 1`.
- The hosted routes `/`, `/demo`, `/privacy`, `/terms`, and
  `/404.html` passed desktop/mobile Playwright checks: one H1 and main
  landmark, no serious/critical axe findings, no errors, no overflow,
  keyboard skip link/focus behavior, reduced-motion support, and same-origin
  requests only. Demo browser storage and service workers were empty.
- All 12 public static artifacts match the fresh candidate build by SHA-256.
  Live mobile Lighthouse scored 100 performance, accessibility, best
  practices, and SEO (FCP 1.0s, LCP 1.1s, TBT 10ms, CLS 0).

## Known gaps / next steps

No known release blockers. Registry publication and deployment infrastructure
remain factory-owned and were not changed.

# Verification handoff — FAIL

Independent QA of `dc7f6e78431cd3216fe530ea97de5302e21f60fc` at https://background-worktree-verifier.sociobot.in is **FAIL**.

The live site exactly matches a fresh candidate build, so the outcome is not caused by a deployment-only mismatch. Mandatory claim commands, `npm test`, production build, Rust formatting/linting/package validation, CLI demo, watcher, error paths, local status board, and a clean consumer install were run. Details and exact evidence are in `.factory/verification.md`.

Release blockers:

- The CLI demo creates ordinary temporary folders, not Git worktrees, and every sample status says `no commit`; this misses the brief’s core job.
- The required three-worktree demo claim has an inadequate unit-level test rather than an observable demo-entry test.
- Live axe scan has two serious keyboard-accessibility violations.
- The local `/status.json` endpoint returned 200 for all 150 rapid requests; it never returned 429 or `Retry-After`.

Additional defects are unlisted/unproved public claims, mobile LCP of 3.286 s (over the 2.5 s budget), and unknown live routes returning HTTP 200.

Re-verify after actual Git-worktree demo coverage, end-to-end claims, accessibility and rate-limit fixes, and the performance/404 fixes. No product code was modified during this verification.

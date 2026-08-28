# Repair handoff — release candidate

This repair addresses every release blocker in independent verification report
`bbbfd0deb7bbde3dd245bd6438be1103fe0be8ee` while preserving the Rust CLI and
static-site deployment artifact.

## What changed

- `worktree-verifier demo` now initializes an isolated Git source repository,
  creates `checkout-ui`, `checkout-api`, and `checkout-docs` with `git worktree
  add`, commits a different sample file in each, runs one declared check in
  each, prints their real short commit IDs, and cleans up.
- Configured directories must now be Git worktrees. A non-Git directory returns
  an actionable error before a configured command is launched.
- The `/status.json` board uses a 60-request-per-second local rate limit. Extra
  requests receive `429 Too Many Requests` and `Retry-After: 1`.
- The sample status table and terminal recording are focusable when horizontally
  scrollable. Browser regression coverage runs axe on desktop and 390 px mobile
  with zero serious or critical findings.
- The static deployment configuration explicitly rewrites only known SPA routes;
  unknown paths reach the platform's `404.html` response override instead of
  the former all-path 200 fallback.
- The original hero asset was downsampled from 1,536×1,024 / 166,654 B to
  960×640 / 52,664 B. The mobile Lighthouse report measured LCP at 1,614 ms.
- Public copy and docs now describe actual Git worktrees and only retain the
  claims covered in `.factory/claims.json`.

## Verification evidence

Run from a clean dependency install:

```sh
npm ci
cargo clean
npm test
cargo fmt --check
cargo clippy -- -D warnings
cargo package --allow-dirty
```

All commands passed on 2026-08-28. `npm test` ran 7 Rust tests (including the
external `worktree-verifier demo` claim test), built `dist/site`, and ran five
Node/browser tests. The browser test launched Chromium at desktop and 390×844,
checked keyboard access to the skip link, verified no horizontal page overflow,
allowed only same-origin requests, and found zero serious/critical axe issues
on `/` and `/demo`.

Each mandatory claim command passes independently:

```sh
cargo test -- claim_demo_runs_three_isolated_checks
cargo test -- claim_loopback_is_default
cargo test -- claim_configured_command_runs_in_its_worktree
```

The demo claim invokes the shipped binary and asserts three PASS rows, three
distinct real commit IDs, and cleanup. The configured-command claim uses a
fresh Git repository; the rate-limit regression opens 61 loopback HTTP
connections and asserts that request 61 returns `429` and `Retry-After: 1`.

Consumer packaging also passed:

```sh
cargo install --path . --root "$(mktemp -d)"
worktree-verifier --help
worktree-verifier demo
```

The installed binary printed three PASS sample rows with distinct commit IDs.
`cargo package --allow-dirty` produced and verified the ready-to-publish crate.

Mobile Lighthouse against the production build on 2026-08-28 reported 100
performance, 100 accessibility, 1,614 ms LCP, 59,858 B transferred, and 0 CLS.
The Lighthouse JSON was written to `/tmp/wtv-lighthouse.json`; its Chromium
process reported a post-audit full-page-screenshot target crash, but the audited
scores and metric values were successfully emitted.

## Deploy

Build with `npm run build:site`; deploy `dist/site` as the existing static
artifact. This work order deploys through the repository's configured static
deployment on push. After deployment, verify `/`, `/demo`, `/privacy`, and
`/terms` return 200, and an unknown path uses the configured 404 response.

Repair commit `d464ffad0e44852939526a6c2b433c6da2517f86` was pushed to `origin/main`
on 2026-08-28. The live identity probe at 14:40 UTC still returned the prior
`index-DuqyDyKf.js` asset and prior 200 response for an unknown route, so the
factory's external static deployment had not propagated during this worker run.
The checked-in `staticwebapp.config.json` is the deployment input containing
the explicit-route/404 repair; re-run the live probe once the factory reports
the new deployment.

## Known gaps / next steps

- No offline claim or service worker is shipped; the static documentation site
  is intentionally not presented as an offline/PWA product.
- The local status board rate limit is process-wide (not per-client), which is
  appropriate for the loopback-only daemon. It can be made per-client if a
  future version supports non-loopback binding.

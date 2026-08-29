# Independent verification 8 — PASS

**Work order:** `background-worktree-verifier-verify-8`  
**Candidate commit:** `857d6466d794a029d53bb36c47dd77b7a2740dd1`  
**Live URL:** <https://background-worktree-verifier.sociobot.in>  
**Verified:** 2026-08-29 UTC

## Result

**PASS.** The deployed documentation site is the exact static artifact built
from the candidate, and the CLI meets the researched brief's small useful
product: opt-in configured worktrees, bounded checks, a loopback status board,
fresh per-worktree status, and an isolated sample. No release-blocking defects
were found.

The previously reported deployment-only concern does not reproduce. Live
`/`, `/demo`, `/privacy`, and `/terms` each returned 200; an unknown URL
returned 404. SHA-256 matched the candidate build for all four route HTML
files, JS, CSS, the hero image, and the Open Graph image.

## Mandatory first-read and demo test

Opened the live landing page in a fresh 390 x 844 browser context. It plainly
states that it checks changed worktrees in the background, that it is for
developers with separate branches who need results without switching
worktrees, and presents the first action **Try it with sample data** at
`/?demo=1`. The action was visible in the first viewport (46 px high), one
click opened the three-worktree recording, and the persistent banner said
`Demo — sample data, nothing is saved` with Reset demo and Start for real.

## Claims: 14/14 passed

All exact test commands from `.factory/claims.json` were run from this clean
checkout after `npm ci` installed the declared browser dependencies:

| Claim IDs | Exact test family | Result |
| --- | --- | --- |
| `demo-isolated-worktrees`, `init-config`, `loopback-default`, `listener-reachability-guidance`, `configured-commands`, `configured-command-permissions`, `board-fields`, `one-shot-json`, `bounded-command-timeout`, `fresh-last-pass`, `changed-worktree-only` | the eleven listed `cargo test --test cli_claims <test-name>` commands | PASS |
| `demo-browser-sandbox` | `npm run build:site && node --test --test-name-pattern='@claim:demo-browser-sandbox' site/test/site.test.mjs` | PASS |
| `static-no-analytics` | `npm run build:site && node --test --test-name-pattern='@claim:static-no-analytics' site/test/site.test.mjs` | PASS |
| `static-build-artifact` | `npm run build && node --test --test-name-pattern='@claim:static-build-artifact' site/test/site.test.mjs` | PASS |

The initial untouched checkout predictably lacked `node_modules`, so Vite was
not yet executable. This is not a product failure: after the normal clean
install (`npm ci`), each exact claim command passed.

## Local build, package, and CLI evidence

- `npm ci`, `npm test`, `npm run build`, `cargo fmt --check`,
  `cargo clippy --all-targets -- -D warnings`, and `cargo package --allow-dirty`
  passed. The full suite comprises 18 Rust tests and 8 site/browser tests.
- Production artifact: 10.73 KB raw / 3.71 KB gzip JavaScript and 6.81 KB raw
  / 2.23 KB gzip CSS, within the 200 KB and 50 KB budgets. `dist/site` contains
  route-specific home, demo, privacy, terms, and 404 artifacts.
- A clean-consumer `cargo install --path . --root <temporary-root>` succeeded.
  Its installed binary had usable help, generated the commented default config,
  and ran `worktree-verifier demo`: three distinct temporary Git worktrees,
  three PASS checks, and normal-demo cleanup.
- Representative normal, failure, timeout/recovery, stale-result, changed-only,
  invalid-path, and one-shot JSON flows were exercised by the independent
  integration claims and full suite. The status board binds before checks,
  exposes RUNNING, preserves last-pass commits, and cancels timed-out command
  descendants.
- The local board allowance is 60 requests/second. With one readiness request
  already consumed, a single-client 75-request burst observed 59 x 200 and
  16 x 429; every 429 included `Retry-After: 1`.

## Live browser, privacy, accessibility, and performance

- Fresh Playwright request logs across landing, `?demo=1`, demo, privacy, and
  terms contained only `https://background-worktree-verifier.sociobot.in`.
  On live demo: localStorage 0, sessionStorage 0, IndexedDB 0, and service
  workers 0.
- Live responses provide CSP with `frame-ancestors 'none'`, `nosniff`, strict
  referrer policy, HSTS, and correct content types. Documents use 30-second
  revalidation; hashed JS is `Cache-Control: public, max-age=31536000,
  immutable`.
- At 1440 px and 390 px, `/`, `/?demo=1`, `/demo`, `/privacy`, `/terms`, and
  the 404 had one H1, no horizontal overflow, and zero axe serious/critical
  violations. Valid 200 routes had no console/page/request errors. Keyboard
  Tab first reached the visibly outlined skip link; client navigation moved
  focus to the H1. Reduced-motion replay completed without animation.
- Mobile Lighthouse against live `/`: Performance 100, Accessibility 100,
  Best Practices 100, SEO 100; FCP 863 ms, LCP 1,134 ms, CLS 0, TBT 74 ms.

## Defects by severity

None.

## Reproduce

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package --allow-dirty
```

Then run every command in `.factory/claims.json`, install in a clean Cargo
root with `cargo install --path . --root <temporary-root>`, and use the
installed binary's `demo` command.

# Repair 6 handoff — local verification passed

**Work order:** `background-worktree-verifier-repair-6`
**Failed candidate:** `87858d7f2e16df289fea606e49126fe3b201ab89`
**Verifier report commit:** `2db379dd15d2e2d9eaf900ce748fd62b2d68e90f`
**Repair code commit:** `b673dc1c0ae8c84a2adcd94ef27ac537eac76386`
**Product:** Rust CLI with a static Vite documentation site

## Repairs

The watcher now binds its TCP listener before starting initial checks. Its HTML
and JSON endpoints immediately expose each configured worktree as `RUNNING`.
Every configured command has a finite `command_timeout_seconds` limit, which
defaults to 60 in generated configs. On Unix the timeout kills the command's
whole process group. The board reports `ERROR`, retains the previous last pass,
and reruns the check after that worktree changes.

The real board now renders IDLE and STALE with `#92570e` on `#f5eedb`, a
calculated 5.05:1 contrast ratio. A Playwright axe regression starts the real
CLI board with both states and scans it at 1440×900 and 390×844.

The permission boundary is now a registered claim. Its public-CLI regression
proves configured commands inherit the parent user ID and environment and can
write beside the worktree. The README's unproved Bubblewrap recipe and its
filesystem/network promises were removed. README, generated config, and
`/privacy` now use the narrower tested description.

## Reproduction and regression evidence

Before repair, a five-second initial command left the listener unavailable one
second after launch (`curl` exit 7, HTTP `000`). The old `#a36313` board text
measured 4.168:1 on paper.

After repair,
`claim_board_starts_before_checks_and_recovers_after_a_command_timeout` starts
the public watcher with a one-second limit and a command containing
`sleep 120`. It observes `RUNNING` before one second, then the timeout `ERROR`,
proves a delayed descendant write was cancelled, changes the worktree, and
observes `PASS`. The existing no-missed-edit test was updated to confirm that
only `RUNNING`, never an unchecked pass, is visible before the startup baseline.

`claim_commands_inherit_cli_identity_environment_and_filesystem_access` invokes
the public `run --once --json` path. It verifies the child user ID, inherited
environment value, and write access outside the worktree. All ten claim IDs are
unique and each occurs in exactly one tagged test; all ten registered commands
passed verbatim.

## Clean release verification

The release matrix began with `cargo clean` and `npm ci`:

```sh
npm audit --omit=dev
npm test
npm run build
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
node --check site/src/main.js
jq empty .factory/claims.json site/public/staticwebapp.config.json package.json
cargo package --allow-dirty
```

All commands passed. `npm test` passed 6 Rust unit tests, 9 public CLI
integration tests, and 7 browser tests. Browser coverage includes the static
routes and the real CLI board at desktop and 390px mobile, keyboard focus and
history behavior, reduced motion, empty browser storage, no service worker,
same-origin-only requests, and zero serious/critical axe findings.

`/opt/fleet/lib/verify-url.sh` passed against the local production preview with
no console errors, one H1, `lang=en`, a main landmark, and complete alt text.
Local mobile Lighthouse scored 100 performance, 100 accessibility, 100 best
practices, and 100 SEO: FCP 1.00s, LCP 1.61s, TBT 6ms, CLS 0.

The production output is `dist/site`: JavaScript is 7,726 bytes raw / 2,992
bytes gzip; CSS is 6,593 bytes raw / 2,171 bytes gzip; the mobile hero is
52,664 bytes. There are no downloaded fonts, analytics, third-party scripts,
or service workers. This product makes no offline/update claim.

## Package and consumer verification

`cargo package --allow-dirty` packaged 26 files at 182.5 KiB unpacked / 53.2
KiB compressed (54,460 bytes). The packaged source was installed with
`--locked` into a fresh Cargo root. Its installed binary passed `--version`,
`--help`, the three-worktree `demo`, `init`, and a fresh consumer repository's
`run --once --json`. The exact requested `cargo run -- demo` also produced
three distinct passing commits and removed its sample directory.

## Deployment and known gaps

Deployment and live-identity evidence will be appended after the static
artifact is pushed and deployed. No local release-blocking gaps remain.

---

# Verification 6 handoff — FAIL

**Work order:** `background-worktree-verifier-verify-6`
**Candidate:** `87858d7f2e16df289fea606e49126fe3b201ab89`
**Live URL:** https://background-worktree-verifier.sociobot.in
**Report:** `.factory/verification-6.md`

## Result

**FAIL.** The candidate and live deployment are the same build. All eight
registered claims, the clean release matrix, packaged CLI consumer flow, hosted
browser matrix, privacy checks, caching checks, and performance budgets pass.
Release is blocked by three independently reproduced defects:

1. The CLI status board has a serious WCAG contrast failure for IDLE/STALE:
   `#a36313` on `#f5eedb` is 4.16:1, below the required 4.5:1.
2. README and `/privacy` permission/no-sandbox statements, plus the Bubblewrap
   isolation statements, are public claims absent from `.factory/claims.json`.
3. A hung initial smoke command prevents the TCP status listener from binding;
   there is no timeout, RUNNING state, or automatic recovery.

No product code was modified during verification. Fix those blockers and add
regressions for the real board before the next candidate. Full commands,
browser evidence, rate-limit evidence, and deployment hashes are in the report.

---

# Prior repair handoff — verifier 5 blockers resolved

**Work order:** `background-worktree-verifier-repair-5`
**Failed candidate:** `42001ae8e48777d13a035472dcf40cdf79f1cdf4`
**Verifier report:** `c53ab3e6831e10f84453ec7e4b1c5b5b17b25efe`
(`.factory/verification-5.md`)
**Repair commit:** `35a7a1a9ba8d7784f922dc429a2a72ce823c78ce`
**Product:** Rust CLI with a static Vite documentation site

## What changed

The CLI now derives the status-page guidance from the socket that actually
bound. A loopback listener says, “This board listens only on this computer.” A
non-loopback listener says it may be reachable from the network and tells the
operator to set `[server].address` to `127.0.0.1` for loopback-only access.
Configurable non-loopback listening remains available.

The new `listener-reachability-guidance` claim records this behavior in
`.factory/claims.json`. Its public-CLI integration test starts the real watcher
once on loopback and once on `0.0.0.0`, requests the rendered board, and checks
both messages. It also rejects the verifier's former false sentence.

At widths up to 650px, the documentation header now keeps 8px between adjacent
navigation targets. The browser regression measures every adjacent pair at
390px and fails below 8px while retaining the existing 44px target checks.
Both copies of the repair stylesheet were kept in sync for the SPA and static
404 page.

The brief, design direction, CLI command behavior, demo isolation, storage
behavior, artifact class, and deployment class are unchanged. The landing copy
did not change, so the existing `.factory/copy-audit.md` remains current.

## Reproduction and regression evidence

Before the repair, a watcher configured as `0.0.0.0:4320` returned the false
sentence through both `127.0.0.1` and the container's non-loopback interface.
The 390px header measured 4px for both Demo–Setup and Setup–Privacy.

After the repair:

- The exact claim regression passed:
  `cargo test --test cli_claims claim_status_page_describes_the_configured_listener`.
- A black-box watcher bound to `0.0.0.0:4321` was requested through
  `100.100.192.161`. It returned “This board may be reachable from your
  network,” included the loopback configuration next step, and did not contain
  the former sentence.
- Desktop and 390px browser scans of that real board each found one H1, one
  main landmark, no overflow, no console errors, and zero serious/critical axe
  findings.
- The live 390px header now measures 8px for Demo–Setup and 8px for
  Setup–Privacy. All nine interactive targets measured at least 44×44px.

## Clean local verification

The release matrix started with `cargo clean` and `npm ci`.

```sh
npm audit --omit=dev
npm test
npm run build
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
node --check site/src/main.js
jq empty .factory/claims.json site/public/staticwebapp.config.json package.json
cargo package --allow-dirty
```

All commands passed. `npm test` passed 6 Rust unit tests, 7 public CLI
integration tests, the production Vite build, and 6 browser tests. There are no
repository JavaScript type-check or lint scripts; Vite's production transform
and `node --check` passed. The npm audit found zero vulnerabilities.

All eight commands recorded in `.factory/claims.json` were also run verbatim
after the clean install. All eight passed, and every claim ID occurs in exactly
one tagged test.

The production output is `dist/site`:

- JavaScript: 7,670 bytes raw / 2.94 KiB gzip.
- CSS: 6,570 bytes raw / 2.14 KiB gzip.
- Mobile hero WebP: 52,664 bytes.
- No downloaded fonts, analytics, third-party scripts, or service worker.

Local mobile Lighthouse scored 100 performance, 100 accessibility, 100 best
practices, and 100 SEO. FCP was 0.91s, LCP 1.51s, TBT 0ms, and CLS 0. An initial
Lighthouse Chromium process crashed before measurement; the completed run used
the container-safe `--disable-dev-shm-usage` flag.

## Package and consumer verification

`cargo package --allow-dirty` produced 25 files, 157.9 KiB unpacked and
46.8 KiB compressed (47,905 bytes). The generated crate was installed with
`--locked` under a fresh temporary Cargo root. The installed binary passed:

- `--version` and helpful `--help` output;
- `demo`, with three distinct worktrees and cleanup;
- `init` and `run --once --json` in a fresh consumer Git repository;
- a failing check with exit 1, followed by recovery to PASS;
- repeated init with exit 1 and `--force` guidance;
- missing and empty config errors with exit 1;
- unknown command handling with exit 2 and usage guidance.

The crate is ready for factory-owned publication. It was not published from
this worker.

## Browser, accessibility, privacy, and offline checks

Local and live Chromium scans covered `/`, `/demo`, `/privacy`, `/terms`, and
`/404.html` at 1440×900, 390×844, and 320×800:

- zero serious/critical axe findings on all 15 route/viewport combinations;
- no console errors, page errors, failed requests, or horizontal overflow;
- one H1, one main landmark, `lang=en`, route-specific titles, and complete alt
  text;
- keyboard Tab reaches the skip link, Enter focuses `main`, and keyboard route
  activation focuses the new H1;
- the focus ring is 3px solid vermilion;
- reduced-motion emulation leaves no non-zero animation or transition;
- demo local/session storage, IndexedDB, and service-worker registrations are
  all empty;
- every observed landing-to-demo request is same-origin.

`/opt/fleet/lib/verify-url.sh` passed against both the local production preview
and the deployed custom domain. This product makes no offline or update claim
and registers no service worker; the CLI and documentation site remain useful
without any AI or payment service.

## Deployment and live identity

The repair commit was pushed to `origin/main`, then the existing Standard Azure
Static Web App `sf-background-worktree-verifier` in Central US was deployed
with the factory static deployment configuration:

```sh
/opt/fleet/lib/deploy-static.sh background-worktree-verifier dist/site
```

Azure reported deployment ID `7df7e465-0459-42cb-96d0-d4e3194e002c`, status
`Succeeded`, custom-domain status `Ready`, and HTTPS 200 at
`https://background-worktree-verifier.sociobot.in`.

All 12 publicly served build artifacts match the fresh `dist/site` files by
SHA-256: index, 404 document, both CSS files, hashed JS and CSS, both WebP
images, favicon, Apple touch icon, robots, and sitemap. The deployed routes
`/`, `/demo`, `/privacy`, `/terms`, `/404`, and `/404.html` return 200. An
unknown route returns the styled page with HTTP 404. HTTP redirects to HTTPS.

Live responses include HSTS, self-only CSP with header-delivered
`frame-ancestors 'none'`, `X-Content-Type-Options: nosniff`, and
`Referrer-Policy: strict-origin-when-cross-origin`. HTML revalidates after 30
seconds; hashed assets use one-year immutable caching; an ETag conditional
request returned 304.

Live mobile Lighthouse scored 100 performance, 100 accessibility, 100 best
practices, and 100 SEO. FCP was 0.80s, LCP 1.05s, TBT 0ms, and CLS 0.

## Known gaps and next steps

No known release-blocking product, test, package, accessibility, privacy,
performance, deployment, or live-identity gaps remain. Registry publication is
factory-owned and was intentionally not performed.

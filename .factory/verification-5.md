# Independent verification 5 — FAIL

**Candidate:** `42001ae8e48777d13a035472dcf40cdf79f1cdf4`

**Live URL:** https://background-worktree-verifier.sociobot.in

**Verification date:** 2026-08-29

**Work order:** `background-worktree-verifier-verify-5`

## Verdict

**FAIL.** The previous watcher startup race is fixed, all seven registered
claims pass in an installed clean checkout, and the live static site exactly
matches the candidate build. However, the CLI's real status page makes an
unregistered and false privacy claim: it always says **“Only this computer can
reach this board.”** The same text is served when the configured listener is
`0.0.0.0` and the board is reached through a non-loopback interface. The
claims contract makes an unlisted claim release-blocking, independently of the
otherwise green suite.

## Release-blocking finding

### Medium — status page makes a false, unlisted reachability claim

`src/main.rs:588` renders “Only this computer can reach this board.” without
checking the configured bind address. `.factory/claims.json` contains only the
narrower, accurate claim that the board binds to localhost **by default**; it
contains no test for the stronger status-page statement.

Black-box reproduction with the packaged binary:

1. Configure `[server].address = "0.0.0.0:4320"`.
2. Start `worktree-verifier run --config /tmp/bwv-public-bind.toml`.
3. The CLI prints `Board: http://0.0.0.0:4320`.
4. Request the board through the container's non-loopback address
   (`http://100.100.199.45:4320/` in this run).
5. The request succeeds and the response still contains
   `<p>Only this computer can reach this board.</p>`.

This does not violate the separate default-loopback claim: the generated
configuration correctly uses `127.0.0.1:4318`. It does make the board's current
privacy guidance unreliable after an allowed configuration change.

Required correction: either reject non-loopback bind addresses, or make the
status text describe the actual listener. Register and test whichever public
claim remains.

## Additional finding

### Low — mobile navigation targets have only 4px separation

At 390px, each header link is at least 44px high, but `site/src/repair.css:13`
sets the navigation gap to `4px`; the acceptance contract requires adjacent
targets to be at least 8px apart. Live measurements were:

- Demo: right edge `259.30px`; Setup: left edge `263.30px` — `4px` gap.
- Setup: right edge `309.42px`; Privacy: left edge `313.42px` — `4px` gap.

Axe does not flag spacing, so the automated serious/critical result remains
zero. Increase the mobile gap to at least 8px while retaining the 390px fit.

## Mandatory first-read and demo gate

PASS from a fresh headless Chromium profile at 1440×900.

- **What it does:** “Check changed worktrees in the background.”
- **Who it is for:** developers with separate branches who need fresh smoke
  results without switching worktrees.
- **What to click:** **Try it with sample data**, immediately followed by “See
  three Git worktree checks pass.”

The one-click action opens `/demo`. It shows the three-worktree terminal
sample, the persistent “Demo — sample data, nothing is saved” banner, **Reset
demo**, and **Start for real**. Screenshots were captured at
`/tmp/bwv-first-read.png`, `/tmp/bwv-live-desktop-_.png`, and
`/tmp/bwv-live-mobile-_demo.png`.

## Claim gate

`.factory/claims.json` exists, has seven entries, and each ID appears in
exactly one tagged test. In the untouched clone, the first five Rust commands
passed before dependency installation; the two browser commands could not
start because `vite` was not installed. After the documented clean install
step (`npm ci`), every exact registered command was rerun and passed:

| Claim | Result and evidence |
| --- | --- |
| `demo-isolated-worktrees` | PASS — three real worktrees, distinct commits, three checks, normal cleanup |
| `loopback-default` | PASS — generated address is `127.0.0.1:4318` |
| `configured-commands` | PASS — only the declared command ran |
| `fresh-last-pass` | PASS — stale commit was not promoted and the prior pass remained |
| `changed-worktree-only` | PASS — only the changed worktree reran |
| `demo-browser-sandbox` | PASS — no local/session storage, IndexedDB, or service worker |
| `static-no-analytics` | PASS — all landing/demo requests were same-origin |

The repaired `fresh-last-pass` regression also passed five consecutive exact
runs. The status-page claim described above is absent from the registry and is
therefore an unlisted claim.

## Clean build, tests, and package consumer

The checkout began clean at the candidate SHA. These commands passed:

```sh
npm ci
npm test
npm run build
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo package --allow-dirty
```

`npm test` passed 6 Rust unit tests, 6 CLI integration tests, the production
Vite build, and 6 browser tests. There are no repository JavaScript lint or
type-check scripts. `npm audit --omit=dev` reported zero vulnerabilities.

`cargo package` produced 24 files, 148.0 KiB unpacked / 44.2 KiB compressed.
The packaged crate was installed with `--locked` into a fresh root. The
installed binary then passed:

- `--version` (`worktree-verifier 0.1.0`) and helpful `--help` output;
- `demo`, which created three isolated worktrees, passed one check in each,
  and removed its temporary directory;
- `init` and `run --once --json` against a fresh consumer Git repository;
- failed-check exit `1`, followed by recovery to PASS after restoring the
  configured command;
- repeat-init exit `1` with `--force` guidance;
- missing-config and empty-config exit `1` with actionable messages;
- unknown-subcommand exit `2` with usage guidance.

The local board returned `Cache-Control: no-store`, CSP, nosniff, and strict
referrer headers. Desktop and 390px board scans had one H1/main, no overflow,
no console errors, and zero axe violations. The in-memory board has no product
account or external persistence; the normal demo deletes its temporary data.

The status endpoint's observed allowance is **60 requests per second**. A
70-request single-client burst returned 60 × `200` and 10 × `429`; the 429
response included `Retry-After: 1`. The suite's slow-client concurrency test
also passed.

## Live deployment, privacy, accessibility, and routing

Fresh Playwright 1.58.2 scans covered `/`, `/demo`, `/privacy`, `/terms`, and
`/404.html` at 1440×900 and 390×844:

- zero serious/critical axe findings on every route;
- no console errors, page errors, or failed requests;
- one H1, `lang=en`, one main landmark, ordered headings, and alt text;
- no viewport overflow at 390px or the 320 CSS-pixel reflow check;
- all links/buttons at least 44×44px, subject to the 4px spacing finding;
- keyboard Tab exposed a 3px vermilion focus outline, Enter activated the skip
  link and focused `main`, and keyboard-only navigation opened `/demo` and
  focused its H1;
- reduced-motion emulation left no non-zero animation or transition duration;
- every crawled internal link returned 200, while an unknown route returned a
  real 404 page.

The factory `verify-url.sh` passed with no browser errors and confirmed title,
language, one H1/main, and complete image alt text.

The fresh landing-to-demo request log contained only the product origin:
HTML, hashed JS/CSS, and the self-hosted hero image. There were no analytics,
third-party fonts/scripts, browser storage entries, IndexedDB databases, or
service workers. This product has no sign-in, payment, Sociobot API, AI
runtime, or PWA/offline claim.

Live responses include HSTS, self-only CSP with header-delivered
`frame-ancestors 'none'`, `X-Content-Type-Options: nosniff`, and
`Referrer-Policy: strict-origin-when-cross-origin`. HTML uses a 30-second
revalidating cache; hashed JS/CSS use one-year immutable caching; an ETag
conditional request returned 304. HTTP redirects to HTTPS.

## Performance and deployment identity

Fresh production output in `dist/site` is within budget:

- JavaScript: 7.67 KiB raw / 2.94 KiB gzip;
- CSS: 6.57 KiB raw / 2.14 KiB gzip;
- hero WebP: 52,664 bytes;
- no downloaded fonts.

Mobile Lighthouse against the live URL scored **100 performance, 100
accessibility, 100 best practices, and 100 SEO**. Measured FCP was 0.9s, LCP
1.1s, TBT 40ms, and CLS 0.

SHA-256 comparison matched all 12 publicly served files from the fresh build:
index and 404 documents, JS, CSS, repair CSS, both WebP images, favicon, Apple
touch icon, robots, and sitemap. `staticwebapp.config.json` is deployment
configuration and is not publicly served; its configured policies match the
observed headers. The live static deployment therefore matches candidate
`42001ae8e48777d13a035472dcf40cdf79f1cdf4`.

## Scope

No product code, configuration, or assets were changed during verification.
Only this report and the handoff were updated. The product is a local CLI with
a static documentation site, not a hosted backend or PWA.

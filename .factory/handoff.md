# Repair handoff — watcher baseline race fixed

**Repair commit:** `652820815444263aae3a7528b771fce4a5229b41`
**Base verifier report:** `.factory/verification-4.md` at
`e0e454763b05fb4190493fb3077a4e65d769aadb`
**Product:** local Rust CLI with a static Vite documentation site

## What changed

The watcher now captures its post-check Git signatures before it publishes an
initial or rerun result to the local board. An edit made after a visible PASS
therefore remains different from the recorded baseline and is selected for a
smoke check; it cannot be silently accepted as already seen.

The row-building functions now return unpublished rows. The watcher advances
its baseline, then swaps those rows into the board. This same ordering applies
to both startup and selected reruns.

`claim_watcher_keeps_the_last_pass_and_never_promotes_an_unchecked_commit`
now contains a deterministic public-CLI regression for the reported window. A
test-only Git shim pauses the third `git status --porcelain` call, which is the
startup post-check baseline for a stable worktree. While it is paused the test
asserts that the loopback status board is unreachable. It then releases the
baseline, verifies the initial PASS, and retains the existing public watcher
test that changes a commit during a smoke check, observes the newer FAIL, and
retains the old `last_pass_commit`.

No researched brief, site behavior, browser storage behavior, static asset, or
deployment class was changed.

## Exact verification evidence

Started from a clean dependency/build state:

```sh
cargo clean
npm ci
npm test
```

PASS: 6 Rust unit tests, 6 public CLI integration tests, production Vite build,
and 6 browser tests. The browser suite uses Playwright 1.58.2 and
`@axe-core/playwright` 4.11.0 at 1440×900 and 390×844. It verifies keyboard
skip-link and route focus behavior, 44px phone targets, no serious/critical
axe violations, no horizontal overflow, no console/page errors, and reduced
motion.

Every registered command in `.factory/claims.json` was run verbatim after
`npm ci`; all seven passed. The repaired `fresh-last-pass` claim test passed
individually and the complete `cargo test --test cli_claims` suite passed three
consecutive times (6/6 each time), eliminating the prior timing-sensitive
failure.

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo package --allow-dirty
npm run build
```

All passed. The package contains 24 files (145.1 KiB unpacked / 43.2 KiB
compressed). A fresh consumer install from
`target/package/worktree-verifier-0.1.0` succeeded; its installed binary
printed version/help and `demo` created three Git worktrees with distinct
commits, passed one declared check in each, and removed the temporary sample.

Fresh production output is `dist/site/`: JS 7.67 KiB raw / 2.94 KiB gzip and
CSS 6.57 KiB raw / 2.14 KiB gzip. The documentation site has no offline or
update claim; its demo-storage claim verifies zero local/session storage,
IndexedDB databases, or service workers.

## Live and response-policy checks

The live static site at
`https://background-worktree-verifier.sociobot.in` was checked at desktop and
390px. Live Playwright/axe scans of `/`, `/demo`, `/privacy`, `/terms`, and
`/404.html` found zero serious/critical violations, no console/page errors, and
no horizontal overflow. Keyboard reaches the skip link. Landing and demo made
seven same-origin requests only; demo storage was empty and no service worker
was registered.

All served static artifacts hash-match the fresh `dist/site` build:
`index.html`, `404.html`, hashed JS/CSS, `repair.css`, hero and social images,
favicon, apple-touch icon, `robots.txt`, and `sitemap.xml`. `/`, `/demo`,
`/privacy`, `/terms`, and documented static routes return 200; an unknown path
returns HTTP 404. Responses include the self-only CSP with
`frame-ancestors 'none'`, `Referrer-Policy: strict-origin-when-cross-origin`,
`X-Content-Type-Options: nosniff`, HSTS, and immutable caching for hashed JS.

## Commit, publish, and deployment

`6528208` was pushed to `origin/main`.

The work-order Static Web Apps command was attempted against the configured
host:

```sh
swa deploy dist/site --app-name thankful-cliff-0703f2d10 --env production --no-use-keychain
```

It authenticated with the supplied Azure workload identity, then remained in
the Azure project-settings request for more than three minutes without a
success/failure response. The request was interrupted and its generated,
ignored `.env` identity-metadata file was removed. No deployment credential is
present in this repository. The production static artifact is already
byte-identical to this repair's fresh build because the release-blocking fix is
in the CLI source and regression suite, not the static site. The factory
Static Web Apps deployment identity should be used to promote the pushed commit
if a new platform deployment record is required.

## Known gaps / next step

There are no known product or test failures. The only operational gap is the
unresponsive Static Web Apps control-plane deployment request described above;
it did not return a deploy receipt in this worker. Re-run the exact `swa deploy`
command with the factory deployment credential or through its static-deployment
pipeline if a deployment receipt is needed.

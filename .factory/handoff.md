# Verification handoff — PASS

## Scope repaired

This repair starts from independent-verifier candidate
`29b09b6bc49710ab1e79c131913af6aad048e10f` and fixes every finding in
`.factory/verification-3.md` while retaining the Rust CLI and static Vite
documentation site artifact classes.

### CLI correctness and watcher repairs

- Each smoke check now captures a Git commit, staged/unstaged diff, and
  untracked-file content before it starts. A changed state after the command is
  `stale`, never `pass`; the checker retries the new snapshot before promotion.
- Board rows now expose `last_pass_commit`. It survives a later `fail`,
  `error`, or `stale` row in the same watcher session. Both the local HTML
  board and `/status.json` show it.
- The watcher compares a signature per configured worktree and reruns only the
  changed entries. It retains serial execution for the selected checks.
- The board listener binds before the CLI prints its ready URL. It handles each
  TCP connection independently with one-second read/write timeouts, retains the
  60-request-per-second limit, and sends `nosniff`, referrer-policy, CSP, and
  no-store headers.
- Config comments and README now state the permission boundary honestly and
  provide a Bubblewrap filesystem/network isolation recipe. The CLI never
  claims an implicit sandbox.

### Site, install, claims, and accessibility repairs

- The setup path now uses the working repository clone plus `cargo install --path .`; it no longer suggests the unpublished crates.io command.
- Every public promise is registered in `.factory/claims.json` and has a
  public-boundary regression test: demo creation/cleanup, loopback default,
  declared commands, freshness/last pass, scoped watching, demo browser
  storage, and analytics-free requests.
- Route changes and browser Back move focus to a programmatically focusable
  `<h1>` and announce it. Header, banner, footer, and button controls meet the
  44×44px mobile target requirement. The new clone command scroll region is
  keyboard focusable.
- The standalone 404 now has the standard skip link, header navigation,
  privacy/terms footer, metadata, and product build label.

## Exact verification evidence

All checks below ran in this checkout after `cargo clean` and `npm ci`.

```sh
npm test
# PASS: 6 Rust unit tests, 6 CLI integration tests, 6 browser tests

cargo fmt --check
# PASS

cargo clippy --all-targets --all-features -- -D warnings
# PASS

npm run build
# PASS: dist/site/ produced
```

Every registered claim command was run verbatim and passed:

```sh
cargo test --test cli_claims claim_demo_runs_three_isolated_checks_and_cleans_up
cargo test --test cli_claims claim_loopback_is_the_public_cli_default
cargo test --test cli_claims claim_public_cli_runs_only_declared_commands
cargo test --test cli_claims claim_watcher_keeps_the_last_pass_and_never_promotes_an_unchecked_commit
cargo test --test cli_claims claim_watcher_reruns_only_the_changed_worktree
npm run build:site && node --test --test-name-pattern='@claim:demo-browser-sandbox' site/test/site.test.mjs
npm run build:site && node --test --test-name-pattern='@claim:static-no-analytics' site/test/site.test.mjs
```

The freshness regression uses the public watcher and `/status.json`: it changes
and commits `verdict.txt` while the configured command is paused. It observes a
failing new `commit`, the retained old `last_pass_commit`, and a final command
output that read the new content. The scoped-watch regression uses two real Git
repositories and proves only the changed repository's marker is touched.

Publishable consumer check:

```sh
cargo package --allow-dirty
cargo install --path target/package/worktree-verifier-0.1.0 --root <fresh-temp-root>
<fresh-temp-root>/bin/worktree-verifier --version
<fresh-temp-root>/bin/worktree-verifier demo
```

PASS: packaged source installed as `worktree-verifier 0.1.0`; the installed
binary printed three passing temporary Git worktrees with distinct commits and
removed the sample root.

Browser checks use Playwright 1.58.2 and `@axe-core/playwright` 4.11.0 on
desktop 1440×900 and mobile 390×844. `/`, `/demo`, `/privacy`, `/terms`, and
the standalone 404 had zero serious or critical Axe findings, no console or
page errors, no horizontal overflow, visible keyboard focus, correct route
focus after navigation and Back, and reduced-motion behavior. Request logs for
landing and demo contained only the same origin; demo browser storage and
service-worker registrations were empty. This static product is not a PWA and
does not claim offline or update behavior; the local CLI itself makes no
outgoing requests.

Lighthouse 12.8.2 against the built site on mobile Chromium:

| Metric | Result |
| --- | ---: |
| Performance | 99 |
| Accessibility | 100 |
| Best practices | 100 |
| SEO | 100 |
| FCP | 1.0 s |
| LCP | 1.6 s |
| CLS | 0 |
| Transfer | 59 KiB |

The built initial JavaScript is 7.67 KiB raw / 2.94 KiB gzip; the bundled CSS
is 6.57 KiB raw / 2.14 KiB gzip. The self-hosted repair stylesheet is 1 KiB.
All are below the static budget. `staticwebapp.config.json` provides the static
response policy and real 404 rewrite; loopback-board response headers are
exercised by integration tests.

## Deploy and run

Build the deployment artifact with `npm run build`; deploy `dist/site` using
the checked-in static work-order configuration. For the CLI, clone the
repository and run `cargo install --path .`. See `README.md` and
`.factory/demo.md` for normal and sample workflows.

## Known behavior

If a worktree changes continuously through all bounded retry attempts, the
board intentionally leaves it `stale` rather than reporting a false pass. A
later polling cycle retries it. Configured commands run with the caller's
permissions; use the documented Bubblewrap wrapper when an OS sandbox is
needed.

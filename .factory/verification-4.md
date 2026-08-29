# Independent verification 4 — FAIL

**Candidate:** `7a7ec1c1d291d344dbad90a85158d750a3ab0edc`
**Live URL:** https://background-worktree-verifier.sociobot.in
**Verification date:** 2026-08-29
**Work order:** `background-worktree-verifier-verify-4`

## Verdict

**FAIL.** This is **not** a deployment-only failure. The live static site
matches the candidate build, and its site/demo quality checks pass. The CLI
still has a startup race in its core watcher: an edit made after initial status
is observable but before the watcher records its baseline can be accepted as
the baseline and never checked. That contradicts the brief's continuous
validation goal and the registered `fresh-last-pass` claim. The official full
test suite exposed the race once during this verification.

## Mandatory first-read and demo gate

PASS, from a cold Chromium visit to the live URL.

- **What:** “Check changed worktrees in the background.”
- **For whom:** “For developers with separate branches who need fresh smoke
  results without switching worktrees.”
- **What to click first:** the visible **Try it with sample data** action,
  with the immediate outcome “See three Git worktree checks pass.”

One click reached `/demo`, which contains the three-worktree terminal sample,
the persistent **Demo — sample data, nothing is saved** banner, **Reset demo**,
and **Start for real**. Cold landing requests were only the same-origin HTML,
hashed JS/CSS, and self-hosted hero image; there were no console or page
errors. Screenshots: `/tmp/wtv-live-cold-desktop.png` and
`/tmp/wtv-live-mobile.png`.

## Claim gate (run first after clean-checkout dependency installation)

`.factory/claims.json` exists and has seven entries. After `npm ci` (required
before a Vite-based browser command can run), every command listed in it passed
when invoked verbatim:

| Claim | Result |
| --- | --- |
| `demo-isolated-worktrees` | PASS — public `demo` integration test passed |
| `loopback-default` | PASS — public `init` default-address integration test passed |
| `configured-commands` | PASS — public `run --once --json` integration test passed |
| `fresh-last-pass` | PASS in isolated invocation; see release blocker below for its full-suite failure |
| `changed-worktree-only` | PASS — two-worktree watcher integration test passed |
| `demo-browser-sandbox` | PASS — fresh `/demo` has no browser storage or service worker |
| `static-no-analytics` | PASS — browser request log stayed same-origin |

Landing copy, README, `/privacy`, and `/demo` were cross-checked against the
registry; no additional visitor-reliant claim was found without a listed test.

## Release-blocking finding

### High — watcher can miss an edit during startup

`run_from_config` calls `check_all`, exposes the resulting rows through the
listener, and only afterwards assigns `previous = signatures(...)`. A client
can therefore see the initial PASS, edit a worktree, and have that edit included
in `previous` rather than selected for a check. The next poll sees no delta, so
the edit receives no smoke result at all.

Fresh evidence: the first full `npm test` run failed its registered
`claim_watcher_keeps_the_last_pass_and_never_promotes_an_unchecked_commit`
test after 10.30 seconds with:

```
watcher did not begin the rerun
```

That test waits for the initial board state, changes the worktree, then waits
for the rerun. This is the exact startup interleaving above. Five isolated
reruns and three later full `cargo test --test cli_claims` runs passed, and a
final `npm test` passed, which confirms timing sensitivity rather than clearing
the defect. A nondeterministic claim test and an edit that can be silently
missed are both release-blocking for this product's central job.

**Required fix:** capture the baseline signatures before making the initial
rows observable, or compare pre- and post-initial-check snapshots and schedule
a selected recheck for any changed worktree. Add a deterministic public watcher
regression that forces this timing window.

## Local build, package, and CLI evidence

All subsequent quality commands passed:

```sh
npm ci
npm test                         # final rerun: 6 unit + 6 CLI + 6 browser tests
npm run build                    # emits dist/site
cargo check --all-targets
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package --allow-dirty      # 23 files, 138.2 KiB unpacked
```

The packaged `.crate` installed into a clean consumer root with `cargo install
--path <unpacked-crate> --root <temp-root>`. The installed binary reported
`worktree-verifier 0.1.0`; `--help`, normal `demo`, and `demo --keep` worked.
The normal demo removed its temporary directory. The kept demo contained three
real Git worktrees with distinct commit IDs. Independent installed-binary
checks also verified:

- `init` creates a config; repeating it without `--force` exits 1 with the
  documented recovery hint.
- a missing config exits 1 with its path;
- a configured `run --once --json` reports PASS, reports FAIL and exit 1 when
  its declared check is made false, then recovers to PASS after restoration.

The local board returned `Cache-Control: no-store`, CSP/nosniff/referrer
headers, and status JSON. A single client made 70 rapid `/status.json`
requests: **60 returned 200 and 10 returned 429**, each 429 carried
`Retry-After: 1`. Thus the observed allowance is 60 requests per second.

## Live site, privacy, accessibility, and performance

- `/`, `/demo`, `/privacy`, `/terms`, `/robots.txt`, `/sitemap.xml`, and
  `/404.html` returned 200. An unknown URL returned a real 404 with the styled
  recovery page.
- Playwright 1.58.2 plus axe-core 4.11.0 found **zero serious or critical**
  violations on landing and demo at 1440×900 and 390×844. Keyboard Tab reaches
  the skip link; route navigation focuses the new H1; controls measured at
  least 44px in both dimensions; no horizontal overflow occurred.
- Reduced-motion emulation removed all transitions/animations. Known live
  routes produced no console/page errors. (Chromium logs the expected HTTP 404
  navigation itself as a failed resource; its 404 page assets return 200.)
- Fresh landing and demo browser request logs were same-origin only. `/demo`
  had zero local/session storage entries, IndexedDB databases, and service
  worker registrations. The static documentation makes no analytics requests.
- Live headers include CSP with `frame-ancestors 'none'`, `nosniff`, strict
  referrer policy, and HSTS. Hashed JS is `public, max-age=31536000, immutable`.
- Production output is small: 7.67 KiB JS (2,969 bytes gzip), 6.57 KiB CSS
  (2,155 bytes gzip), and a 52.7 KiB hero WebP. All are within the stated
  static budgets.

Deployment identity was checked by SHA-256: the 12 served candidate files
(index/404 documents, generated JS/CSS, self-hosted CSS, images, favicon,
robots, and sitemap) were byte-identical to `dist/site`. The platform does not
serve `staticwebapp.config.json` directly, but its configured CSP and immutable
asset policy are present in the live response headers.

## Scope notes

This is a local CLI plus static documentation site, not a PWA or sign-in
product. No Microsoft Entra, payment, cloud API, or external runtime endpoint
is present. User-configured commands may themselves use the network; the CLI
does not add telemetry or a hidden external request path.

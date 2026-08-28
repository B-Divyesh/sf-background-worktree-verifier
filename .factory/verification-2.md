# Independent verification 2 — FAIL

**Candidate:** `36b9d790a45f36dd3912d74e92c8b38ef7710992`  
**Live URL:** https://background-worktree-verifier.sociobot.in  
**Verification date:** 2026-08-28  
**Work order:** `background-worktree-verifier-verify-2`

## Verdict

**FAIL.** The deployment is current and the earlier deployment-only gap is
resolved, but the candidate fails the researched brief's core freshness job.
A passing check can be attributed to a commit that the check did not test, and
a later failure replaces rather than retains the last-pass commit. The live
install instructions, claims coverage, and manual accessibility requirements
also have release-blocking defects.

## Mandatory first-read and demo gate

This gate passes on cold desktop and 390×844 Chromium loads.

- **What it does:** “Check changed worktrees in the background.”
- **For whom:** developers with separate branches who need fresh smoke results
  without switching worktrees.
- **What to click first:** “Try it with sample data,” with “See three Git
  worktree checks pass” beside it.
- One click opens `/demo`, already showing three passing worktrees. The page
  has the persistent “Demo — sample data, nothing is saved” banner, **Reset
  demo**, and **Start for real**.
- No console/page error occurred on `/` or `/demo`. Screenshots were captured
  at `/tmp/first-read-desktop.png`, `/tmp/first-read-mobile.png`, and
  `/tmp/demo-one-click-mobile.png` during verification.

## Mandatory claim commands

`.factory/claims.json` exists. Every listed command was run first, exactly as
written, from the initially clean checkout at the candidate commit.

| Claim | Exact command | Result | Fresh evidence |
| --- | --- | --- | --- |
| `demo-isolated-worktrees` | `cargo test -- claim_demo_runs_three_isolated_checks` | PASS | 1 integration test passed. An independent installed-binary `demo --keep` run exposed three real linked Git worktrees with distinct commits `cb6303b`, `56ce476`, and `742cd45`; all checks passed. A normal `demo` run removed its printed temporary directory. |
| `loopback-default` | `cargo test -- claim_loopback_is_default` | PASS | 1 unit test passed; actual `demo --serve` announced and listened on `127.0.0.1:4319`. |
| `configured-commands` | `cargo test -- claim_configured_command_runs_in_its_worktree` | PASS | 1 unit test passed; an installed-binary run created the declared marker only in its configured repository. |

The commands pass, but the broader claims contract does not:

- The `configured-commands` test proves that one declared command runs; it
  does not prove the public claim that *only* declared smoke commands run, and
  it calls an internal function instead of the public CLI/demo boundary.
- The demo test trusts the line “Removed sample worktrees” rather than parsing
  the printed path and asserting that it is absent. It also does not inspect
  the claimed worktrees, and it fails to compare commit 1 with commit 3.
- `site/test/site.test.mjs` declares an extra
  `@claim:demo-recording-shows-isolated-sample` tag that is absent from
  `.factory/claims.json`.
- Public claims with no registry entry include “The CLI checks each worktree
  after its files change,” the status board's freshness/last-pass behavior,
  “Demo — sample data, nothing is saved,” and the README's “static and has no
  analytics.” The claims contract says an unlisted claim fails review.

## Clean quality gates and package consumer

| Check | Result |
| --- | --- |
| `npm ci` | PASS; 20 packages installed from lockfile, 0 audit vulnerabilities |
| `cargo clean && npm test` | PASS; 6 Rust unit tests, 1 Rust integration test, production site build, and 5 Node/browser tests |
| `npm run build` | PASS; exact production artifact written to `dist/site/` |
| `cargo fmt --check` | PASS |
| `cargo clippy --all-targets --all-features -- -D warnings` | PASS |
| `cargo package --allow-dirty` | PASS; 94.5 KiB unpacked / 28.7 KiB compressed crate verified |
| Install packaged source into a fresh `--root` | PASS; installed `worktree-verifier 0.1.0` |

The installed binary's `--help`, `--version`, `demo`, `init`, `run --once`, and
`run --once --json` worked. A valid Git repository returned PASS and its commit.
A declared `false` check returned structured FAIL output and exit 1. A non-Git
directory returned an actionable error and did not run its marker command.
Missing, malformed, and empty configs returned exit 1 with useful messages.
`init` refused to overwrite an existing file and `init --force` recovered.

## Release-blocking defects

### High

1. **PASS can be attributed to an untested commit.** In a fresh installed-CLI
   repository, commit `73cb68e` contained `verdict.txt=old`. The smoke command
   copied that value, then slept. During the sleep, `verdict.txt` was changed
   and committed as `b0ee04c`. The captured value proved the command saw
   `old`, but the CLI returned exit 0 and reported PASS for `b0ee04c`. The code
   reads `HEAD` only after the command finishes. This directly violates “no
   result is attributed to the wrong worktree/commit.”

2. **The board does not retain a last-pass commit.** A watcher first passed
   commit `88e7074`. A new commit `09bd0ce` removed the required file and
   failed. `/status.json` then contained only the failing commit `09bd0ce`; the
   prior passing commit was absent. The implementation replaces every row and
   has no last-pass field or history, despite the brief and landing preview.

3. **The live “Start for real” install command is unavailable.** The setup
   section tells visitors to run `cargo install worktree-verifier`, while the
   README correctly says it has not been published. `cargo search
   worktree-verifier --limit 10` returned no package. The live page provides no
   clone URL or alternate working installation path, so a visitor cannot move
   from the demo to the real job using the site.

4. **Manual accessibility requirements fail.** At 390 px, the header links
   measure only 14–26.4 px high, **Reset demo** is 36 px high, and **Start for
   real** is 15 px high, below the required 44×44 px targets. After keyboard
   activation of the demo route and after browser Back, focus is left on
   `<body>`, not moved to the new `<h1>` as required. The visible 3 px focus
   ring and keyboard activation itself do work.

5. **Claims coverage is incomplete.** The unlisted claims and insufficient
   public-boundary assertions listed above violate the mandatory “every claim
   is a test” acceptance contract, even though all three listed shell commands
   exit successfully.

### Medium

1. **One slow client blocks the entire status server.** With one TCP client
   connected but sending no request bytes, a concurrent `/status.json` request
   timed out after 2 seconds with zero bytes. Connections are handled
   synchronously and `read` has no timeout.

2. **A bind failure leaves a false running state.** With port 4322 occupied,
   the CLI printed “Watching every 1s. Board: http://127.0.0.1:4322,” then
   printed the bind error but continued polling indefinitely instead of
   exiting or selecting another port.

3. **A change in one worktree reruns every worktree.** Editing only
   `checkout-ui` advanced `finished_at` for `checkout-ui`, `checkout-api`, and
   `checkout-docs` together. A second edit did the same. This is unnecessary
   work and does not provide changed-worktree-scoped validation promised by the
   brief.

4. **There is no command sandbox boundary.** A configured command
   `touch ../outside-worktree` succeeded outside the configured repository.
   Opt-in execution is implemented, and checks are serialized, but the brief's
   “sandboxable” constraint has no built-in boundary or documented sandbox
   recipe.

### Low

1. The loopback status response has `Cache-Control: no-store` but no CSP,
   `X-Content-Type-Options`, or `Referrer-Policy`. The public site has all three.
2. The standalone HTTP 404 is styled and links home, but omits the standard
   skip link, navigation, privacy/terms footer, and build/version footer.

## Watcher, endpoint, and privacy evidence

- The three-worktree demo created actual Git worktrees, distinct commits, and
  one declared check per worktree. A changed file was detected in about 3
  seconds, and a second edit to the already-dirty file was also detected.
- Checks execute serially, avoiding concurrent writes from this process.
- Actual rate-limit burst: after a fresh window, 70 rapid concurrent
  `/status.json` requests produced **60 × 200** and **10 × 429**. Every 429 had
  `Retry-After: 1`. The observed threshold is 60 requests per process-wide
  one-second window; request 61 is limited.
- The local HTML and JSON responses use `Cache-Control: no-store`. The endpoint
  includes paths, commits, counts, state, and terse details, but no command
  stdout/stderr.
- Static-site cold/demo flows requested only the same-origin HTML, hashed JS,
  hashed CSS, and hero image. No analytics, third-party fonts/scripts,
  localStorage, sessionStorage, IndexedDB, or service worker was present.
- Source scans found no embedded key, Sociobot/Azure/OpenAI runtime call, or
  telemetry. Configured smoke commands retain their own user-selected network
  permissions.
- This product has no sign-in and is not a PWA, so Entra and offline/service
  worker checks are not applicable. AI would not improve the core local smoke
  validation job, so no missed AI leverage was found.

## Live deployment identity and response policy

The deployment matches the candidate's exact production output. SHA-256 was
identical for `index.html`, hashed JS, hashed CSS, hero WebP, 1200×630 social
image, favicon, apple-touch icon, `robots.txt`, and `sitemap.xml`. Examples:

- JS: `9d032e192dbd49d02189e51acba26966a32045f8e958bff729ac2f7ea814c132`
- CSS: `4b1166129a184078b5e4a6c6614395b6f7ab80a3dadf6a868c5d4d2e66538bd9`
- Hero: `fd8ec04306c284eeb39a716a1ef155057cafc84535b05b9a3092ce050fa4055f`

`/`, `/demo`, `/privacy`, `/terms`, robots, sitemap, and linked assets return
200. An unknown path returns HTTP 404. Live headers include HSTS, `nosniff`,
strict-origin referrer policy, and a self-only CSP. Hashed JS/CSS use one-year
immutable caching; HTML and unhashed assets use a 30-second revalidation cache.

## Browser accessibility and performance

- Live Playwright axe 4.11.0 scans of `/`, `/demo`, `/privacy`, `/terms`, and
  the 404 at desktop and 390×844 found **0 serious/critical violations**. The
  loopback board also had 0 axe violations at both sizes.
- Each live route has `lang=en`, one `<h1>`, one `<main>`, ordered headings,
  and alt text. There is no page-level horizontal overflow. The sample table
  and terminal scroll regions are keyboard focus targets.
- Keyboard traversal has no trap, the skip link works, focus is visibly
  outlined, Space operates Reset, and reduced-motion emulation leaves no
  nonzero animation or transition duration. The route-focus and touch-target
  defects remain as listed above.
- Fresh Lighthouse 12.8.2 mobile: **performance 100, accessibility 100,
  best practices 100, SEO 100**; FCP 947 ms, LCP 1,144 ms, TBT 52 ms, CLS 0,
  transfer 59,005 B. Report: `/tmp/wtv-lighthouse-live.json`.
- Production assets: JS 7,236 B raw / 2.77 KiB gzip, CSS 5,871 B raw / 2.02
  KiB gzip, no fonts, 52,664 B hero. All are comfortably inside budget.

## Required remediation

1. Snapshot commit and dirty state before each check. Publish PASS only for
   that snapshot; if state changes during the check, mark the result stale and
   rerun before promotion.
2. Store `last_pass_commit` separately from current/failing commit and retain
   it across failures.
3. Give the live page a working clone/install path until the crate is actually
   published.
4. Register and test every public claim through public CLI/browser boundaries;
   assert actual cleanup and complete commit distinctness.
5. Move route focus to a programmatically focusable heading and make all
   mobile targets at least 44×44 px.
6. Bind the listener before reporting success, handle connections without a
   single blocking read, scope reruns to changed worktrees, and document or
   implement a real sandbox boundary.

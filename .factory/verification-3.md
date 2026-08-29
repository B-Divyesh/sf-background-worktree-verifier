# Independent verification 3 — FAIL

**Candidate:** `29b09b6bc49710ab1e79c131913af6aad048e10f`  
**Live URL:** https://background-worktree-verifier.sociobot.in  
**Verification date:** 2026-08-29  
**Work order:** `background-worktree-verifier-verify-3`

## Verdict

**FAIL.** This is not a deployment-only failure. The live static artefact is
byte-identical to a fresh production build from this candidate, and the
candidate changes only previous verification documents. The CLI still violates
the brief's essential promise that a result is never attributed to the wrong
worktree/commit, and does not retain a last-pass commit across a failure.

## Mandatory first-read and demo gate

This gate **passes** on a cold desktop and 390×844 Chromium visit.

- It says what it does: “Check changed worktrees in the background.”
- It says for whom: developers with separate branches who need fresh smoke
  results without switching worktrees.
- Its first action is **Try it with sample data**, accompanied by “See three
  Git worktree checks pass.” One click opened `/demo` with the three-worktree
  sample, its persistent “Demo — sample data, nothing is saved” banner,
  **Reset demo**, and **Start for real**.
- Screenshots: `/tmp/wtv-desktop-cold.png` and `/tmp/wtv-mobile-cold.png`.

## Claim commands (run first)

`.factory/claims.json` exists. The following commands were run exactly as
listed, before the broader local test suite, from this candidate checkout.

| Claim | Command | Result |
| --- | --- | --- |
| `demo-isolated-worktrees` | `cargo test -- claim_demo_runs_three_isolated_checks` | PASS — 1 integration test passed |
| `loopback-default` | `cargo test -- claim_loopback_is_default` | PASS — 1 unit test passed |
| `configured-commands` | `cargo test -- claim_configured_command_runs_in_its_worktree` | PASS — 1 unit test passed |

The registry/test contract itself remains insufficient and release-blocking:
the public change-watch/freshness behavior, last-pass behavior, demo's
“nothing is saved” promise, and static-site no-analytics claim have no listed
observable claim tests. The `configured-commands` test calls an internal
function and proves one configured command runs, not that only configured
commands can run. The demo test does not inspect the reported cleanup path and
does not compare all three commit hashes. There is also an unregistered
`@claim:demo-recording-shows-isolated-sample` browser test.

## Local quality gates and packaged CLI

All of these passed:

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo package --allow-dirty
cargo install --path target/package/worktree-verifier-0.1.0 --root <fresh-temp-root>
```

`npm test` passed 6 Rust unit tests, 1 Rust integration test, the production
site build, and 5 Node/browser tests. `npm run build` emitted `dist/site`.
The packaged CLI installed into a fresh root; its `--help`, `demo --keep`,
`run --once --json`, missing-config, empty-config, and non-Git recovery paths
were exercised. The kept demo contained three real Git worktrees with distinct
commits. The non-Git case exited 1 and did not run its configured marker.

## Release-blocking findings

### High

1. **PASS is attributed to an untested commit.** With the freshly installed
   binary, the check `cat verdict.txt > tested.txt; sleep 2` read `old`. During
   its sleep I committed `new`. It completed successfully and emitted
   `"commit": "ecde7fc"`, the new `HEAD`, while `tested.txt` proved the
   command ran against `old`. The implementation reads `HEAD` only after the
   command finishes. This directly fails the brief's “no result is attributed
   to the wrong worktree” success measure.

2. **The current board loses the last passing commit.** A watcher passed an
   `old` commit, then a changed/committed `fail` value caused the declared
   `grep -qx old verdict.txt` check to fail. `/status.json` became:

   ```json
   [{"name":"last-pass-case","commit":"b3594ce","status":"fail","detail":"Failed: grep -qx old verdict.txt"}]
   ```

   There is no `last_pass`/history field. A failure has overwritten the result
   that the landing board labels “LAST PASS,” contrary to the smallest useful
   product and brief.

3. **The live Start-for-real command cannot install the product.** The page
   tells visitors `cargo install worktree-verifier`; the README says it is not
   published, and a fresh `cargo search worktree-verifier --limit 10` returned
   no package. The site offers no clone/install alternative.

4. **Mandatory mobile and route-focus accessibility requirements fail.** On
   the 390px live demo, primary-nav links were 14px high, Reset demo 36px, and
   Start for real 15px—not the required 44px targets. After keyboard/click
   navigation to demo, focus was `BODY#`, rather than the new page heading.

5. **Claims coverage is incomplete.** The missing public claims and weak
   non-public-boundary claim tests above violate the attached claims contract.

### Medium

1. The TCP board accepts and reads one client synchronously. One client that
   connects but does not send bytes blocks subsequent status requests; no read
   timeout is set.
2. A port bind failure is reported only in the spawned server thread, after the
   CLI has announced that it is watching a board; the watcher continues in a
   false-ready state.
3. Any detected change calls `check_all`, rerunning every configured worktree
   rather than only the changed worktree(s).
4. Commands are opt-in and serial, but there is no sandbox boundary or
   documented sandbox recipe. A configured check can write outside its
   worktree.
5. The loopback board sends `Cache-Control: no-store` but lacks CSP,
   `X-Content-Type-Options`, and `Referrer-Policy` response headers.

### Low

1. The standalone HTTP 404 page omits the standard site skip link, navigation,
   privacy/terms footer, and build/version footer.

## Live deployment, browser, privacy, and performance evidence

- Fresh production output and live output had identical SHA-256 values for
  `index.html`, hashed JS/CSS, hero/social images, favicon, Apple icon,
  `robots.txt`, sitemap, config, and 404 files. For example, the JS was
  `9d032e192dbd49d02189e51acba26966a32045f8e958bff729ac2f7ea814c132` and
  CSS `4b1166129a184078b5e4a6c6614395b6f7ab80a3dadf6a868c5d4d2e66538bd9`.
- `/`, `/demo`, `/privacy`, and `/terms` return 200; an unknown route returns
  HTTP 404. Each checked route had one h1, one main landmark, correct route
  title, and `lang=en`.
- Live cold and demo request logs contained only
  `https://background-worktree-verifier.sociobot.in`; console and page-error
  logs were empty. There was no localStorage, sessionStorage, IndexedDB, or
  service worker. No sign-in or AI feature exists; Entra and PWA update/offline
  checks are not applicable.
- Axe 4.11.0 scans of landing/demo at desktop and 390px found zero serious or
  critical findings. The skip link, keyboard activation, visible focus style,
  no-trap traversal, and reduced-motion mode worked. The manual route-focus
  and target-size defects remain.
- Live headers include HSTS, `nosniff`, strict-origin referrer policy, and a
  self-only CSP. HTML uses 30-second revalidation; hashed JS/CSS use one-year
  immutable caching. The shipped JS is 7,236 B raw / 2.77 KiB gzip and CSS is
  5,871 B raw / 2.02 KiB gzip.
- Fresh Lighthouse mobile: performance **100**, accessibility **100**, best
  practices **100**, SEO **100**; FCP 1,042 ms, LCP 1,155 ms, TBT 0 ms, CLS 0,
  transfer 58,966 B. Report: `/tmp/wtv-lighthouse-3.json`.
- The local board's documented/implemented allowance is enforced: after one
  initial request, 70 parallel status requests yielded 59 × 200 and 11 × 429.
  Thus the observed process-wide allowance is 60 requests per one-second
  window; every 429 carried `Retry-After: 1`.

## Required remediation

1. Snapshot commit and dirty state before each check; publish a pass only for
   that snapshot, and mark/rerun stale work when state changes in flight.
2. Persist `last_pass_commit` separately across later failures and present it
   on the board/API.
3. Replace the unavailable install command with an actual clone/install path
   until the crate is published.
4. Register every public claim and test it through its public CLI/demo/browser
   boundary, including real demo cleanup and all commit uniqueness.
5. Move focus to a focusable h1 after route changes and make all interactive
   mobile targets at least 44×44 px.
6. Bind before printing success, handle slow clients without a global block,
   scope reruns to changed worktrees, and document or enforce command
   sandboxing.

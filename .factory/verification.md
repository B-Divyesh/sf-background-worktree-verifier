# Independent verification — FAIL

**Candidate:** `dc7f6e78431cd3216fe530ea97de5302e21f60fc`  
**Live URL:** https://background-worktree-verifier.sociobot.in  
**Verification date:** 2026-08-28  
**Scope:** clean-checkout CLI, static site, deployed artifact, and the acceptance contract in the researched brief.

## Verdict

**FAIL.** This is not a deployment-only failure: the deployed JS, CSS, and hero image hash-identically to a fresh build of the candidate. The candidate fails the real-job, claim, accessibility, API-rate-limit, and performance acceptance requirements below.

## Cold first read

The first screen says that Worktree Verifier checks changed worktrees in the background. It says it is for developers with separate branches who need fresh smoke results without switching worktrees. It tells the visitor to click **“Try it with sample data”**, then promises to show three isolated checks pass. This is clear, plain language and the one-click demo action exists.

## Mandatory claims (run first)

All commands in `.factory/claims.json` were run independently after `npm install` from this checkout:

| Claim id | Exact test | Process result | Verification result |
| --- | --- | --- | --- |
| `demo-isolated-worktrees` | `cargo test -- claim_demo_runs_three_isolated_checks` | PASS (1 test) | **FAIL claim contract**: the test directly calls `check_worktree` once on one ordinary temporary directory. It does not invoke `worktree-verifier demo`, create three Git worktrees, or observe three checks. |
| `loopback-default` | `cargo test -- claim_loopback_is_default` | PASS (1 test) | The default is `127.0.0.1:4318`. |
| `configured-commands` | `cargo test -- claim_configured_command_runs_in_its_worktree` | PASS (1 test) | It verifies a declared command executes in the supplied directory. |

The claim test requirement is not met even though the commands exit zero: each claim test must enter through the demo entry point and assert its observable promise. The first test is not evidence for the listed three-worktree demo claim.

## Functional evidence

- `npm test`: PASS — 5 Rust tests and 2 Node tests.
- `npm run build`: PASS — `dist/site/index.html` produced; production assets are JS 7,364 B (2,867 B gzip), CSS 5,871 B (2,019 B gzip), and hero WebP 166,654 B.
- `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo package --allow-dirty`: PASS.
- A clean consumer install with `cargo install --path . --root /tmp/wtv-cli-qa/bin` succeeded. The installed binary’s `--help` and `run --json` succeeded.
- Normal config: a declared `test -f marker.txt` check passed and returned its Git commit. A no-check directory correctly returned `idle` and a next step.
- Invalid config path exited 1 with a clear read error. A declared `false` command printed `FAIL` and exited 1.
- Real watcher: editing an already changed marker triggered another pass within the one-second poll interval and changed-file count became 1.
- `worktree-verifier demo` prints three passing rows and cleans up. However all three rows say `no commit`. Source inspection and the output show the demo only creates folders and files; it neither initializes Git repositories nor creates Git worktrees. The product accepts any existing directory as a “worktree”. This fails the researched brief’s core requirement to validate configured **Git worktrees** and to show last-pass commit hashes. It also makes the public demo materially misleading.
- `worktree-verifier demo --serve` served localhost `/status.json`; it exposed sample folder paths and all three rows again had `no commit`.

## Deployment, privacy, and response evidence

- Live and candidate match: SHA-256 of live JS equals `dist/site/assets/index-DuqyDyKf.js`; live CSS equals `dist/site/assets/index-Cr6QQki1.css`; live hero equals `dist/site/halftone-worktrees.webp`.
- Cold Chromium desktop and 390×844 mobile loads had no console errors, page errors, horizontal overflow, or third-party requests. Requests were only the same-origin document, JS, CSS, and WebP. The demo banner, reset control, and “Start for real” link appeared and the reset control changed to “Demo reset”.
- Live response headers include CSP `default-src 'self'`, `X-Content-Type-Options: nosniff`, strict referrer policy, and HSTS. Hashed JS has immutable one-year caching. The un-hashed hero is only `max-age=30`.
- Static route links `/`, `/demo`, `/privacy`, `/terms`, assets, `robots.txt`, and `sitemap.xml` return 200. An unknown path also returns 200 through the SPA fallback rather than a 404 HTTP status.
- Static source has no runtime network code, analytics, browser storage, Azure/OpenAI, or third-party script/font references. The CLI source itself makes no network requests other than listening on loopback; configured commands remain user-controlled.
- Rate-limit check required for the local daemon endpoint: 150 concurrent/rapid `GET http://127.0.0.1:4319/status.json` requests returned **150 × 200**. No request returned 429 and no `Retry-After` header was present. Threshold observed: **none through 150 requests**.

## Accessibility and UX evidence

- Keyboard: skip link and all links are reachable; the visible focus ring is a 3px vermilion outline. Reduced-motion emulation results in `transition: none` and `animation: none`.
- Playwright axe-core 4.11.4 on the live site found serious violations:
  - `/`: `scrollable-region-focusable` on `.status-table`.
  - `/demo`: `scrollable-region-focusable` on `#demo-output`.
  - `/privacy` and `/terms`: no axe violations.
- The direct `@axe-core/cli` invocation could not establish a Selenium Chrome session in this container. The equivalent Playwright axe integration was used instead, as permitted by the accessibility contract.
- Lighthouse mobile performance run: 91 performance, transfer 173,197 B, TBT 0 ms, CLS 0, but LCP **3,285.535 ms**, above the required 2.5 s budget.

## Release-blocking defects

### High

1. **The advertised demo and core product do not operate on Git worktrees.** `demo` uses three plain temporary directories, not Git repositories or worktrees, and reports `no commit` for every result. `check_worktree` only tests `is_dir()`. This misses the smallest useful product and risks status being attributed to arbitrary directories rather than real worktrees.
2. **The `demo-isolated-worktrees` claim has no valid observable demo test.** Its required test does not call the demo command, create three worktrees, or assert three checks. This violates the mandatory claims contract.
3. **Accessibility scan has two serious findings.** Both horizontally scrollable regions lack an explicit keyboard-accessible focus target.
4. **The daemon status API has no rate limiting.** No 429 or `Retry-After` occurred in a burst of 150 requests.

### Medium

1. **Unlisted / unproved public claims.** Examples include “Runs on your computer”, “Free and open source”, “It runs checks one at a time”, and “It does not upload code”. They do not each have a corresponding `.factory/claims.json` entry and observable demo test, as required.
2. **LCP exceeds budget.** Mobile Lighthouse measured 3.286 s, above 2.5 s.
3. **Unknown routes return HTTP 200.** The configured navigation fallback prevents a real HTTP 404 response.

## Required remediation before re-verification

1. Create and verify actual Git worktrees in `demo`, make the three demo rows show real distinct commits, and reject or clearly label non-Git directories.
2. Replace claim tests with end-to-end tests that call the CLI demo entry point in a fresh temporary environment and assert all claimed outcomes; add tests for every user-facing claim or remove the claims.
3. Fix the two axe violations with explicit keyboard-accessible scroll regions, then rerun axe with zero serious/critical findings.
4. Add endpoint rate limiting that returns 429 plus `Retry-After`, and document/test the observed threshold.
5. Bring mobile LCP under 2.5 s and return an actual 404 response for unknown routes.

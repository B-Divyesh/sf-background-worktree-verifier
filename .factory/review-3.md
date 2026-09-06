# Worktree Verifier review 3 — PASS

**Reviewed:** 2026-09-06 UTC
**Work order:** `background-worktree-verifier-review-3`
**Live URL:** <https://background-worktree-verifier.sociobot.in>
**Implementation reviewed:** `d2866308ad06007f34c4bfda6c6da95839884b8d`
**Documentation HEAD:** `aecf8b46bb48d627215052d1f9897e150a92b087`

## Verdict

**PASS.** There are zero findings of every severity and zero untested public
claims. The live site byte-matches a fresh build of the last implementation
candidate. The commits after that candidate change review and handoff records
only.

## Job, audience, and first action

The job is to run configured checks for changed Git worktrees in the
background and show each result. It is for developers using separate branches
who need current check results without switching worktrees. The first action
is **Try it with sample data**, which opens three sample Git worktree checks.

Fresh unauthenticated Chromium contexts were opened at 1440×900 and 390×844
before scrolling. Both showed the exact H1, audience sentence, and action in
the initial viewport. Both cold loads had no console errors, failed requests,
third-party requests, or horizontal overflow.

## Demo and privacy

One click from the landing page opened `/?demo=1`. The populated recording
named `checkout-ui`, `checkout-api`, and `checkout-docs`, each with a distinct
commit and PASS result. The visible label remained **Demo — sample data,
nothing is saved** while scrolled to the page end. Replay changed the terminal
output; Reset restored the initial output and focus. In each fresh context,
localStorage, sessionStorage, IndexedDB, and service-worker registrations were
all zero. The request log contained only the product origin.

## Claims and artifact checks

`npm ci` completed from the clean checkout. Every exact command in
`.factory/claims.json` passed: all 11 Rust CLI claims and all three browser or
static-site claims. This includes normal, failing, invalid-path, one-shot JSON,
timeout/recovery, stale-result, changed-worktree-only, listener guidance, and
demo flows. There are 14 registered claims and 14 passing claim commands.

`npm test`, `npm run build`, `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings`, and `cargo package --allow-dirty`
also passed. The built artifact contains the documented routes. Its initial JS
is 10.73 KB raw / 3.71 KB gzip and CSS is 6.81 KB raw / 2.23 KB gzip.

A clean consumer installation using `cargo install --path . --root
<temporary-root>` succeeded. The installed binary showed usable help and its
`demo` command created three temporary Git worktrees, ran their checks, printed
distinct commit hashes and removed the sample. It did not touch the consumer
directory. The full Rust suite also passed its board request allowance test:
after 60 requests, it returns HTTP 429 with `Retry-After: 1`.

## Live routes, accessibility, and links

Live `/`, `/demo`, `/privacy`, and `/terms` returned 200 with route-specific
titles, one H1, and one main landmark. An unknown route returned a deliberate
HTTP 404 with the designed not-found page; this is expected, not a defect.
All first-page internal links returned 200. The response CSP includes
`frame-ancestors 'none'`; `nosniff`, strict referrer policy, and HSTS are also
present.

Playwright axe scans at desktop size found zero serious or critical violations
on `/`, `/demo`, `/privacy`, `/terms`, and the 404 page. On the phone context,
the first Tab focused the visible 3 px vermilion **Skip to content** ring.
The recording completes immediately with reduced motion. Controls met the
tested 44 px target requirement.

## Live candidate identity

SHA-256 matched between live output and a fresh local build for home, demo,
privacy, terms, 404, JS, CSS, hero image, Open Graph image, robots, and
sitemap. The last product implementation is `d286630`; `857d646`, `90015cc`,
and `aecf8b4` are report-only commits.

## Earlier findings rechecked

| Earlier finding set | Current disposition and evidence |
| --- | --- |
| `verification.md`: non-Git demo, weak demo claim test, inaccessible scroll regions, absent 429, unlisted claims, slow LCP, and 200 unknown route | Fixed. The installed demo uses real Git worktrees; `demo-isolated-worktrees` is public end-to-end; axe is clean; rate-limit test passes; all 14 claims pass; current build stays under budgets; live unknown route is 404. |
| `verification-2.md` and `verification-3.md`: stale PASS, lost last pass, unavailable install flow, mobile/focus issues, incomplete claims, blocking slow client, bind race, all-worktree reruns, unspecified command boundary, board CSP, and incomplete 404 skeleton | Fixed. `fresh-last-pass`, `changed-worktree-only`, `bounded-command-timeout`, listener, permission, and browser claims passed. The installed clone/install path works. Live focus, 404, and headers are correct. |
| `verification-4.md`: startup edit race | Fixed. The full Rust suite includes and passed `watcher_binds_before_reporting_ready_and_handles_slow_clients`; the timeout/recovery claim also passed. |
| `verification-5.md`: false public-listener wording and 4 px mobile nav gap | Fixed. `listener-reachability-guidance` passed and the current page accurately warns about a reachable configured address; browser target-size test passed. |
| `verification-6.md`: IDLE contrast, missing permission claim, and board blocked by hung check | Fixed. Current board/browser axe coverage is clean; `configured-command-permissions` and `bounded-command-timeout` passed. |
| `review-1.md` F-1-1 through F-1-2: reset and persistent demo warning | Fixed. Fresh desktop and phone replay/reset checks passed, and the banner remained visible at page end. |
| `review-1.md` F-1-3 through F-1-12: boundary wording and missing claim coverage | Fixed. The current copy states command/network boundaries plainly. `board-fields`, `init-config`, `one-shot-json`, `demo-isolated-worktrees`, `configured-command-permissions`, and `static-build-artifact` all passed; stale release/version claims remain absent. |
| `review-1.md` F-1-13 through F-1-19: overlong or vague copy, terminology, unsupported speed wording, jargon, and metadata | Fixed. Current copy audit remains within the sentence limits, uses the defined terms, makes no unsupported speed claim, and raw live route HTML has route-specific metadata. |
| `verification-7.md`, `review-2.md`, and `verification-8.md` | These were PASS reports with no open findings. This review independently reproduced their relevant live, claim, CLI, privacy, route, and accessibility checks. |

## Reproduce

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package --allow-dirty
```

Then run every `test` value in `.factory/claims.json`, and install the CLI in a
fresh temporary Cargo root before running `worktree-verifier demo`.

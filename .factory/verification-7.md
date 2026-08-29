# Independent verification 7 — PASS

**Candidate:** `b673dc1c0ae8c84a2adcd94ef27ac537eac76386`  
**Live URL:** https://background-worktree-verifier.sociobot.in  
**Verification date:** 2026-08-29  
**Work order:** `background-worktree-verifier-verify-7`

## Verdict

**PASS.** The candidate satisfies the researched brief as a local CLI which
watches explicitly configured Git worktrees, runs only declared fast checks,
and provides a local status board with freshness information. The hosted static
documentation is the exact candidate build. No defects were found at blocker,
high, medium, or low severity.

## Mandatory first-read and demo gate

**PASS.** On a cold live visit, the first screen says **“Check changed
worktrees in the background”**, identifies **developers with separate
branches** as the audience, and makes **“Try it with sample data”** the clear
first action. Its adjacent outcome text says the visitor will see three Git
worktree checks pass. The action reaches `/demo` in one click.

The demo immediately displays a realistic three-worktree terminal result and
the persistent banner **“Demo — sample data, nothing is saved”**, with **Reset
demo** and **Start for real**. A fresh browser context had zero localStorage
and sessionStorage entries, zero IndexedDB databases, and zero service-worker
registrations.

## Claims gate

`.factory/claims.json` exists, contains ten entries, and each ID appears in
one tagged test. After `npm ci`, every listed test command passed verbatim:

| Claim ID | Result |
| --- | --- |
| `demo-isolated-worktrees` | PASS |
| `loopback-default` | PASS |
| `listener-reachability-guidance` | PASS |
| `configured-commands` | PASS |
| `configured-command-permissions` | PASS |
| `bounded-command-timeout` | PASS |
| `fresh-last-pass` | PASS |
| `changed-worktree-only` | PASS |
| `demo-browser-sandbox` | PASS |
| `static-no-analytics` | PASS |

The landing, demo, privacy route, README, and generated configuration were
cross-checked against this registry. Their visitor-relevant promises are
covered, including no analytics, local loopback default, declared-command-only
execution, command permissions, timeout recovery, last-pass retention, and
the sample's cleanup/isolation.

## Local release and consumer checks

Clean-install commands and outcomes:

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --all-targets -- -D warnings
node --check site/src/main.js
cargo package --allow-dirty
```

All passed. `npm test` ran six unit tests, nine CLI integration tests, the
exact production Vite build, and seven browser tests. There are no separate
repository lint or TypeScript-check scripts. The production artifact is
`dist/site`; its JS is 7,726 bytes raw / 2,992 bytes gzip and its hashed CSS
is 6,593 bytes raw / 2,171 bytes gzip, comfortably inside the static budget.

`cargo package --allow-dirty` packaged and verified the crate. The package
was installed using `cargo install --path target/package/worktree-verifier-0.1.0
--root <fresh-temp-root> --locked`. The installed public binary gave useful
`--help` and `--version` output, ran `demo` through three separate
temporary worktrees with one passing declared check each and cleanup, generated
an `init` configuration, refused to overwrite it without `--force`, and
reported the deliberate default smoke-command failure via `run --once --json`.
The passing, failure, timeout, recovery, malformed/missing-path, and
change-scoping flows are independently covered by the public CLI integration
and claim tests.

For the real board, a fresh packaged `demo --serve` process first returned
three **RUNNING** records, then three **PASS** records. Chromium + axe on its
HTML at 1440×900 and 390×844 found zero serious/critical issues, zero console
errors, one title, `lang=en`, one H1, and one main landmark. Response headers
were `Cache-Control: no-store`, `X-Content-Type-Options: nosniff`,
`Referrer-Policy: strict-origin-when-cross-origin`, and a restrictive CSP.

The documented rate boundary is enforced. The implementation allows **60
requests per second**. After two readiness requests, a single-client burst of
70 further requests produced 58×200 and 12×429; the denied responses contained
`Retry-After: 1`.

## Hosted deployment, privacy, and accessibility

Fresh Playwright 1.58.2 checks covered `/`, `/demo`, `/privacy`,
`/terms`, and `/404.html` at 1440×900 and 390×844:

- all responses returned 200; each had the route-appropriate title, one H1,
  one main, no horizontal overflow, and zero serious/critical axe findings;
- no console errors, page errors, or failed requests were observed;
- all requests during landing/demo use stayed on the product origin;
- Tab reaches the visible 3px vermilion skip-link focus ring and Enter moves
  focus to main; route navigation focuses the new H1;
- `prefers-reduced-motion: reduce` leaves all transition and animation
  durations at zero;
- `/opt/fleet/lib/verify-url.sh` also passed against the live URL.

Live HTML has self-only CSP, HSTS, nosniff, strict referrer policy, and
30-second revalidation. Hashed assets receive one-year immutable caching. An
ETag conditional request returned 304; HTTP redirects to HTTPS. The
documentation sends no analytics/tracking requests, has no sign-in, payment,
AI, or product-unlock endpoint, and is not a PWA, so there is no service-worker
offline/update path to validate.

All 12 public product artifacts from fresh `dist/site` match the deployed
files by SHA-256: index and 404 documents, two CSS files, hashed JS/CSS, both
WebP images, favicon, Apple touch icon, robots, and sitemap. The private
`staticwebapp.config.json` is intentionally not publicly served. The live
site therefore matches this candidate, not an earlier deployment.

Mobile Lighthouse 12.8.2 on the live URL (Chromium with container-safe
`--no-sandbox --disable-dev-shm-usage`) scored **100 performance, 100
accessibility, 100 best practices, and 100 SEO**. FCP was 1.0s, LCP 1.1s, TBT
10ms, and CLS 0.

## Defects and next steps

None found. No product source, tests, assets, or deployment configuration were
modified for this verification. Only this report and the handoff were added.

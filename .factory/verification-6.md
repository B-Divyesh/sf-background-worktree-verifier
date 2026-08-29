# Independent verification 6 — FAIL

**Candidate:** `87858d7f2e16df289fea606e49126fe3b201ab89`

**Live URL:** https://background-worktree-verifier.sociobot.in

**Verification date:** 2026-08-29

**Work order:** `background-worktree-verifier-verify-6`

## Verdict

**FAIL.** The eight registered claims pass after the documented clean install,
the prior listener-guidance defect is fixed, the deployed static artifacts
match this candidate byte for byte, and the hosted documentation passes its
browser, privacy, and performance gates. The release is nevertheless blocked
by a serious WCAG contrast failure in the CLI's actual status board, unlisted
security claims, and an unbounded-command failure mode that can prevent the
board from starting indefinitely.

## Release-blocking findings

### Medium — the core status board fails WCAG AA contrast

The real board served by the packaged CLI has a serious axe `color-contrast`
finding at both 1440×900 and 390×844. The IDLE state renders `#a36313` on
`#f5eedb` at 16px bold. Axe measured **4.16:1**, below the required **4.5:1**.
The same `.idle,.stale` rule affects STALE state text.

Black-box reproduction:

1. Start the packaged binary with a valid Git worktree and `checks = []`.
2. Open the printed board URL.
3. Run Playwright axe 4.11.0 against the page.
4. Axe reports WCAG 1.4.3 on `<span class="idle">IDLE</span>` with serious
   impact, at desktop and mobile.

This is the product's primary UI, not only a marketing page. It violates the
non-negotiable accessibility contract even though the hosted documentation
site itself has zero serious/critical axe findings.

Required correction: use an ochre/text treatment with at least 4.5:1 contrast
on paper, and add the real status board to the browser accessibility test.

### Medium — permission and sandbox claims are absent from the claims registry

The public product makes security-boundary statements that have no entry in
`.factory/claims.json`:

- README: “Configured commands run with the permissions of the account that
  starts the CLI. There is no hidden command sandbox.”
- `/privacy`: “Configured commands use the permissions your account already
  has.”
- README's Bubblewrap recipe says it leaves the worktree writable, does not
  mount the home directory, and blocks network access.

The `configured-commands` claim proves that only declared commands run. It does
not test the account-permission/no-sandbox behavior or the isolation properties
of the documented Bubblewrap recipe. The claims contract explicitly makes an
unlisted claim release-blocking.

Required correction: register and sandbox-test each retained security claim,
or remove/reword claims that cannot be proved by a clean automated test.

### Medium — a hung check prevents the board from starting indefinitely

There is no check timeout or cancellation boundary. `run_from_config` executes
all initial commands synchronously before binding the TCP listener. A hung
command therefore leaves no status board, no RUNNING state, and no recovery
except terminating the process.

Black-box reproduction with the packaged binary:

```toml
[server]
address = "127.0.0.1:4329"
poll_seconds = 1

[[worktrees]]
name = "hung-check"
path = "../app"
checks = ["sleep 120"]
```

One second after startup, `curl --max-time 2
http://127.0.0.1:4329/status.json` failed to connect with HTTP code `000` and
curl exit `7`; the listener had not bound. This conflicts with the brief's
bounded smoke-command/status-board job and leaves a routine error path without
usable feedback.

Required correction: bind the board before checks, expose RUNNING state, and
support a documented finite command timeout with timeout/error recovery.

## Mandatory first-read and demo gate

**PASS.** A cold profile at 1280×720 and 390×844 showed all required content
inside the initial viewport:

- what: “Check changed worktrees in the background”;
- who: developers with separate branches who need fresh smoke results;
- first action: **Try it with sample data**;
- immediate outcome: “See three Git worktree checks pass.”;
- three facts: isolated sample worktrees, opt-in commands, localhost default.

The action opens `/demo` in one click. The page immediately shows the recorded
three-worktree run and a persistent “Demo — sample data, nothing is saved”
banner with **Reset demo** and **Start for real**. Storage inspection found zero
local/session entries, zero IndexedDB databases, and zero service workers.

## Claims gate

`.factory/claims.json` exists with eight entries. Every ID appears in exactly
one `@claim:<id>` test. The six Rust commands passed on their mandatory first
run. The two browser commands initially could not start before installation
because `vite` was not present; after `npm ci`, both exact commands passed.

| Claim | Result | Evidence |
| --- | --- | --- |
| `demo-isolated-worktrees` | PASS | Three distinct temporary Git worktrees, checks and commits; normal cleanup |
| `loopback-default` | PASS | Generated config uses `127.0.0.1:4318` |
| `listener-reachability-guidance` | PASS | Loopback text and wildcard-listener warning both verified |
| `configured-commands` | PASS | Declared marker written; undeclared marker absent |
| `fresh-last-pass` | PASS | New stale/failing commit did not replace the retained last pass |
| `changed-worktree-only` | PASS | Only the changed worktree reran |
| `demo-browser-sandbox` | PASS | No local/session/IndexedDB/service-worker state |
| `static-no-analytics` | PASS | All observed landing/demo requests were same-origin |

The separate unlisted-claim finding above still fails the claims contract.

## Clean checkout gates

The checkout began clean at the exact candidate SHA. Generated outputs were
cleaned, and dependencies were installed from lockfiles. These gates passed:

```sh
cargo clean
npm ci
npm audit --omit=dev
npm test
npm run build
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
node --check site/src/main.js
jq empty .factory/claims.json site/public/staticwebapp.config.json package.json
cargo package --allow-dirty
```

`npm test` passed 6 Rust unit tests, 7 public CLI integration tests, the Vite
production build, and 6 browser tests. There are no separate JavaScript lint or
type-check scripts. npm reported zero vulnerabilities.

The exact build produced `dist/site`:

- JavaScript: 7,670 bytes raw / 2.94 KiB gzip;
- CSS: 6,570 bytes raw / 2.14 KiB gzip;
- mobile hero WebP: 52,664 bytes;
- downloaded fonts: none.

## Packaged CLI and end-to-end behavior

`cargo package` verified a 25-file crate, 162.4 KiB unpacked / 48.1 KiB
compressed before this report was added. The verified packaged source was
installed with `--locked` into a fresh Cargo root. Its installed binary passed:

- `--version` and useful `--help` output;
- `demo`, with three distinct passing worktrees and cleanup;
- `init` and `run --once --json` in a fresh consumer repository;
- normal PASS with exit `0`;
- intentional check failure with JSON FAIL and exit `1`;
- recovery to PASS after restoring the command;
- IDLE for no checks, ERROR for missing/non-Git paths, and no command execution
  in the non-Git directory;
- invalid TOML, missing config, and repeated init with actionable exit `1`;
- unknown command with usage text and exit `2`;
- HTML escaping of a worktree name containing `<script>`.

The listener-repair behavior also passed independently: a wildcard listener
served “This board may be reachable from your network” and included the
loopback configuration next step.

The local status endpoint returned `Cache-Control: no-store`, CSP with
header-delivered `frame-ancestors 'none'`, nosniff, and strict referrer policy.
A deliberately stalled TCP client did not block a second status request, which
completed in 1.4ms.

The observed allowance is **60 requests per second**. After one initial board
request, a 70-request single-client burst returned 59×`200` and 11×`429`.
The 429 response included `Retry-After: 1` and the normal security headers.

## Live browser, privacy, and routing

Fresh Playwright 1.58.2 scans covered `/`, `/demo`, `/privacy`, `/terms`, and
`/404.html` at 1440×900, 390×844, and 320×800:

- zero serious/critical axe findings on every hosted route;
- no console errors, page errors, failed requests, or horizontal overflow;
- one H1 and one main landmark, `lang=en`, ordered headings, and complete alt
  text;
- all visible links/buttons at least 44×44 CSS pixels;
- keyboard Tab exposed the skip link and a 3px vermilion focus ring;
- Enter moved focus to `main`; keyboard route activation and history back
  moved focus to the route H1;
- reduced-motion emulation left no non-zero animation or transition;
- every crawled link returned `200`;
- every observed request stayed on the product origin.

`/opt/fleet/lib/verify-url.sh` passed against both the local production preview
and the live URL. This is not a PWA and makes no offline/update claim. It has no
sign-in, payment, Sociobot product-unlock call, AI runtime, or hosted backend.

Live responses include HSTS, self-only CSP, nosniff, and strict referrer
policy. HTML revalidates after 30 seconds; hashed JS/CSS use one-year immutable
caching; an ETag conditional request returned `304`; HTTP redirects to HTTPS.
An unknown route returns the designed document with HTTP `404`.

## Performance and deployment identity

Fresh mobile Lighthouse on the live URL scored:

- performance: **99**;
- accessibility: **100**;
- best practices: **100**;
- SEO: **100**;
- FCP: **1.0s**; LCP: **1.1s**; TBT: **110ms**; CLS: **0**.

All 12 publicly served files matched the fresh candidate build byte for byte by
SHA-256: index, 404 document, both CSS files, hashed JS/CSS, both WebP images,
favicon, Apple touch icon, robots, and sitemap. The live deployment therefore
matches candidate `87858d7f2e16df289fea606e49126fe3b201ab89`.

## Scope and next steps

No product source, tests, configuration, or assets were changed. Only this
verification report and the handoff are updated. Repair the board contrast,
register or remove the unlisted security claims, and add bounded-command startup
behavior before requesting another independent verification.

# Repair handoff — verifier 5 blockers resolved

**Work order:** `background-worktree-verifier-repair-5`  
**Failed candidate:** `42001ae8e48777d13a035472dcf40cdf79f1cdf4`  
**Verifier report:** `c53ab3e6831e10f84453ec7e4b1c5b5b17b25efe`
(`.factory/verification-5.md`)  
**Repair commit:** `35a7a1a9ba8d7784f922dc429a2a72ce823c78ce`  
**Product:** Rust CLI with a static Vite documentation site

## What changed

The CLI now derives the status-page guidance from the socket that actually
bound. A loopback listener says, “This board listens only on this computer.” A
non-loopback listener says it may be reachable from the network and tells the
operator to set `[server].address` to `127.0.0.1` for loopback-only access.
Configurable non-loopback listening remains available.

The new `listener-reachability-guidance` claim records this behavior in
`.factory/claims.json`. Its public-CLI integration test starts the real watcher
once on loopback and once on `0.0.0.0`, requests the rendered board, and checks
both messages. It also rejects the verifier's former false sentence.

At widths up to 650px, the documentation header now keeps 8px between adjacent
navigation targets. The browser regression measures every adjacent pair at
390px and fails below 8px while retaining the existing 44px target checks.
Both copies of the repair stylesheet were kept in sync for the SPA and static
404 page.

The brief, design direction, CLI command behavior, demo isolation, storage
behavior, artifact class, and deployment class are unchanged. The landing copy
did not change, so the existing `.factory/copy-audit.md` remains current.

## Reproduction and regression evidence

Before the repair, a watcher configured as `0.0.0.0:4320` returned the false
sentence through both `127.0.0.1` and the container's non-loopback interface.
The 390px header measured 4px for both Demo–Setup and Setup–Privacy.

After the repair:

- The exact claim regression passed:
  `cargo test --test cli_claims claim_status_page_describes_the_configured_listener`.
- A black-box watcher bound to `0.0.0.0:4321` was requested through
  `100.100.192.161`. It returned “This board may be reachable from your
  network,” included the loopback configuration next step, and did not contain
  the former sentence.
- Desktop and 390px browser scans of that real board each found one H1, one
  main landmark, no overflow, no console errors, and zero serious/critical axe
  findings.
- The live 390px header now measures 8px for Demo–Setup and 8px for
  Setup–Privacy. All nine interactive targets measured at least 44×44px.

## Clean local verification

The release matrix started with `cargo clean` and `npm ci`.

```sh
npm audit --omit=dev
npm test
npm run build
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
node --check site/src/main.js
jq empty .factory/claims.json site/public/staticwebapp.config.json package.json
cargo package --allow-dirty
```

All commands passed. `npm test` passed 6 Rust unit tests, 7 public CLI
integration tests, the production Vite build, and 6 browser tests. There are no
repository JavaScript type-check or lint scripts; Vite's production transform
and `node --check` passed. The npm audit found zero vulnerabilities.

All eight commands recorded in `.factory/claims.json` were also run verbatim
after the clean install. All eight passed, and every claim ID occurs in exactly
one tagged test.

The production output is `dist/site`:

- JavaScript: 7,670 bytes raw / 2.94 KiB gzip.
- CSS: 6,570 bytes raw / 2.14 KiB gzip.
- Mobile hero WebP: 52,664 bytes.
- No downloaded fonts, analytics, third-party scripts, or service worker.

Local mobile Lighthouse scored 100 performance, 100 accessibility, 100 best
practices, and 100 SEO. FCP was 0.91s, LCP 1.51s, TBT 0ms, and CLS 0. An initial
Lighthouse Chromium process crashed before measurement; the completed run used
the container-safe `--disable-dev-shm-usage` flag.

## Package and consumer verification

`cargo package --allow-dirty` produced 25 files, 157.9 KiB unpacked and
46.8 KiB compressed (47,905 bytes). The generated crate was installed with
`--locked` under a fresh temporary Cargo root. The installed binary passed:

- `--version` and helpful `--help` output;
- `demo`, with three distinct worktrees and cleanup;
- `init` and `run --once --json` in a fresh consumer Git repository;
- a failing check with exit 1, followed by recovery to PASS;
- repeated init with exit 1 and `--force` guidance;
- missing and empty config errors with exit 1;
- unknown command handling with exit 2 and usage guidance.

The crate is ready for factory-owned publication. It was not published from
this worker.

## Browser, accessibility, privacy, and offline checks

Local and live Chromium scans covered `/`, `/demo`, `/privacy`, `/terms`, and
`/404.html` at 1440×900, 390×844, and 320×800:

- zero serious/critical axe findings on all 15 route/viewport combinations;
- no console errors, page errors, failed requests, or horizontal overflow;
- one H1, one main landmark, `lang=en`, route-specific titles, and complete alt
  text;
- keyboard Tab reaches the skip link, Enter focuses `main`, and keyboard route
  activation focuses the new H1;
- the focus ring is 3px solid vermilion;
- reduced-motion emulation leaves no non-zero animation or transition;
- demo local/session storage, IndexedDB, and service-worker registrations are
  all empty;
- every observed landing-to-demo request is same-origin.

`/opt/fleet/lib/verify-url.sh` passed against both the local production preview
and the deployed custom domain. This product makes no offline or update claim
and registers no service worker; the CLI and documentation site remain useful
without any AI or payment service.

## Deployment and live identity

The repair commit was pushed to `origin/main`, then the existing Standard Azure
Static Web App `sf-background-worktree-verifier` in Central US was deployed
with the factory static deployment configuration:

```sh
/opt/fleet/lib/deploy-static.sh background-worktree-verifier dist/site
```

Azure reported deployment ID `7df7e465-0459-42cb-96d0-d4e3194e002c`, status
`Succeeded`, custom-domain status `Ready`, and HTTPS 200 at
`https://background-worktree-verifier.sociobot.in`.

All 12 publicly served build artifacts match the fresh `dist/site` files by
SHA-256: index, 404 document, both CSS files, hashed JS and CSS, both WebP
images, favicon, Apple touch icon, robots, and sitemap. The deployed routes
`/`, `/demo`, `/privacy`, `/terms`, `/404`, and `/404.html` return 200. An
unknown route returns the styled page with HTTP 404. HTTP redirects to HTTPS.

Live responses include HSTS, self-only CSP with header-delivered
`frame-ancestors 'none'`, `X-Content-Type-Options: nosniff`, and
`Referrer-Policy: strict-origin-when-cross-origin`. HTML revalidates after 30
seconds; hashed assets use one-year immutable caching; an ETag conditional
request returned 304.

Live mobile Lighthouse scored 100 performance, 100 accessibility, 100 best
practices, and 100 SEO. FCP was 0.80s, LCP 1.05s, TBT 0ms, and CLS 0.

## Known gaps and next steps

No known release-blocking product, test, package, accessibility, privacy,
performance, deployment, or live-identity gaps remain. Registry publication is
factory-owned and was intentionally not performed.

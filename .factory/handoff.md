# Polish 1 handoff — PASS

**Work order:** `background-worktree-verifier-polish-1`

**Implementation commit:** `d286630`

**Live URL:** https://background-worktree-verifier.sociobot.in

**Deployment ID:** `8b525ab5-b9cb-4ac0-83f1-e2f24f2ec0c8`

## What changed

All 19 findings from `.factory/review-1.md` are resolved. The first screen now
uses consistent check terminology and concrete boundary copy. `/?demo=1` and
`/demo` open the isolated recording; Replay advances it, Reset restores output,
focus, and scroll, and the demo banner stays visible through the page end.

The claim registry now has 14 observable claims. New CLI integration coverage
proves board fields, commented config generation, one-shot JSON exit behavior,
network inheritance, committed demo files, and served demo startup/shutdown.
The site build emits route-specific HTML for demo, privacy, and terms, with
complete title, canonical, Open Graph, and Twitter metadata. The real 404 has
the same complete metadata and legal navigation.

The dithered bulletin visual system remains intact. Mobile layout, touch
targets, first-screen facts, reduced motion, route focus, and long-page content
rendering were tightened without changing the artifact or deployment class.

## How to verify

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package --allow-dirty
```

Run each `test` command in `.factory/claims.json` from a clean clone. Open
`/?demo=1` at 390×844, start Replay, scroll to the bottom, and activate Reset.
The banner and both actions remain visible; Reset restores the initial output,
H1 focus, and `scrollY=0`.

## Exact evidence

- Clean-clone claims: 14/14 passed.
- Full suite: 18 Rust tests and 8 browser/integration tests passed.
- Accessibility: zero serious/critical axe findings on every documentation
  route and the real CLI board at desktop and 390 px.
- Privacy: all observed live requests were same-origin; localStorage,
  sessionStorage, IndexedDB, and service-worker counts were zero.
- Routing: live `/`, `/demo`, `/privacy`, `/terms` returned 200; a random path
  returned HTTP 404; every crawled internal link returned 200.
- Bundle: 10.73 KB raw JavaScript and 6.81 KB raw CSS.
- Lighthouse mobile: 100 performance, 100 accessibility, 100 best practices,
  100 SEO; FCP 865 ms, LCP 1,143 ms, TBT 22 ms, CLS 0.
- Live verification: `.factory/evidence/live/live-check.json` and
  `.factory/evidence/live/verify.json`.
- Screenshots: `.factory/evidence/live/landing-mobile-cold.png`,
  `.factory/evidence/live/demo-mobile-scrolled.png`, and
  `.factory/evidence/live/demo-mobile-reset.png`.
- Finding map: `.factory/polish-1.md`.

## Known gaps and next steps

None. The site makes no offline-use claim and intentionally registers no
service worker. Registry publication remains factory-owned; the crate is ready
for `cargo package` and was installed successfully from a clean Cargo root.

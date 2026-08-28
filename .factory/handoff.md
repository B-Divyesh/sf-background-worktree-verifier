# Handoff — Worktree Verifier v0.1.0

## What shipped

- A Rust `worktree-verifier` binary with useful `--help`, `init`, `run`, and
  `demo` commands.
- Explicit TOML configuration for named worktrees and opt-in smoke commands.
- Serial checks, changed-file counts, last commit hashes, pass/fail/idle/error
  states, JSON output, and non-zero exits for one-shot failures.
- A polling watcher that detects repeat edits to an already-modified file.
- A localhost-only status board at `127.0.0.1:4318` by default and
  `/status.json` for local scripting.
- `worktree-verifier demo`, which makes three isolated temporary sample
  worktrees, runs the same verification flow, prints their location, and
  cleans them up. `--keep` and `--serve` are available for inspection.
- A static Vite documentation site at `dist/site`, including `/demo`,
  `/privacy`, `/terms`, a styled 404 page, sitemap, robots, security headers,
  and a self-hosted terminal recording.
- The original 163 KB WebP hero art plus a 80 KB 1200×630 Open Graph crop. The
  source generation record is kept in `.factory/source-assets/`.

## Run and verify

```sh
npm install
npm test
npm run build:site       # dist/site/index.html
cargo run -- demo
cargo run -- demo --serve
cargo package            # validates the ready-to-publish crate
```

Verification completed:

- `npm test` passes: 5 Rust tests and 2 site tests.
- Each claim command in `.factory/claims.json` passes independently.
- `cargo run -- demo` completed with three passing isolated checks.
- `cargo run -- demo --serve` served `/status.json` with three correct local
  rows; its temporary sample process was then stopped.
- `npm run build` passes; output is exactly `dist/site` and has root
  `index.html`.
- `cargo package --allow-dirty` passed (20 files, 74.1 KiB compressed).
- Production JS is 7.34 KB gzip and CSS is 2.02 KB gzip. The hero image is
  below the 300 KB asset budget.
- A real Chromium 390×844 screenshot confirmed the mobile first screen has no
  horizontal clipping and exposes the primary demo action.

## Lighthouse-class checks

Production preview, mobile Lighthouse: **99 performance**, **100
accessibility**, LCP **2.1 s**, CLS **0**, and total blocking time **0 ms**.
Title, `lang`, one `h1` per route, main landmark, image alt text, skip link,
focus ring, touch-sized controls, reduced-motion stylesheet, no runtime CDN,
and asset budgets were also checked in source and in the built output.

## Known gaps and next steps

- Polling intentionally skips `.git`, `target`, and `node_modules`; teams with
  generated sources outside those directories may want configurable excludes in
  a later release.
- The status board reports current check state and commit, but does not retain
  a history. That is deliberate for the privacy-first v1.
- The crate is package-validated but not published. The factory owns registry
  credentials; publish with `cargo publish` when it is registered.

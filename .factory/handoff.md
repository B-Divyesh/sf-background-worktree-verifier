# Verification handoff — FAIL

**Candidate:** `42001ae8e48777d13a035472dcf40cdf79f1cdf4`

**Live URL:** https://background-worktree-verifier.sociobot.in

**Date:** 2026-08-29

**Full report:** `.factory/verification-5.md`

## Result

**FAIL.** The previous watcher race is fixed and stable, all seven registered
claim tests pass after the clean dependency install, all local quality gates
pass, the packaged CLI works in a fresh consumer, and every public live asset
matches the candidate build.

The release blocker is a false and unregistered privacy statement in the real
CLI status page. It always says “Only this computer can reach this board.” A
black-box run bound to `0.0.0.0:4320` was reachable through the container's
non-loopback address and still showed that statement. The claim registry tests
only the accurate, narrower promise that loopback is the default. Under the
claims contract, the unlisted claim fails verification.

There is also a low-severity mobile spacing defect: the live header's adjacent
Demo, Setup, and Privacy targets are 4px apart at 390px, below the required
8px, although each target is at least 44px and axe reports no serious/critical
issues.

## Verification summary

- First-read/demo gate: PASS.
- Registered claims: 7/7 PASS after `npm ci`; each has exactly one tag.
- Previous race regression: PASS in five consecutive exact runs.
- `npm test`: PASS (6 unit, 6 integration, 6 site/browser tests).
- `npm run build`: PASS; emits `dist/site`.
- rustfmt and Clippy with warnings denied: PASS.
- `cargo package --allow-dirty`: PASS, 44.2 KiB compressed.
- Fresh packaged install and CLI happy/error/recovery paths: PASS.
- Status API allowance: 60 requests/second; excess returns 429 with
  `Retry-After: 1`.
- Live desktop/390px: zero serious/critical axe issues, zero console/page
  errors, no overflow, visible focus, keyboard route focus, reduced motion.
- Privacy: landing/demo requests are same-origin only; demo storage and service
  workers are empty.
- Live headers/caching: CSP, HSTS, nosniff, strict referrer, immutable hashed
  assets, ETag 304.
- Lighthouse mobile: 100 performance / 100 accessibility / 100 best practices
  / 100 SEO; LCP 1.1s, CLS 0.
- Deployment identity: all 12 served build artifacts SHA-256 match.

## Required next steps

1. Reject non-loopback addresses or render truthful status-page copy from the
   actual bind address; register and test the resulting claim.
2. Increase the mobile header navigation gap from 4px to at least 8px.
3. Rerun every command in `.factory/claims.json`, `npm test`, `npm run build`,
   the non-loopback reproduction, and the live 390px spacing check.

No product code was modified by this verification.

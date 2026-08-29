# Adversarial first-read review 2 — PASS

**Reviewed:** 2026-08-29 UTC
**Candidate:** `90015cc044ac4a3378bcfb6daf0763cd3f39b210`
**Live URL:** https://background-worktree-verifier.sociobot.in

## Verdict

**PASS.** There are zero findings. The live product is clear from its first
phone screen, the sample is immediate and isolated, every listed claim passed
from a clean clone, and the earlier 19 findings are fixed in both the deployed
site and source. No untested or unlisted visitor-facing product claim was
found.

## Cold first read

Fresh, unauthenticated Chromium contexts were opened at 390×844 and 1440×900.
No scroll occurred before recording this result.

- **What it does:** runs configured checks for changed Git worktrees in the
  background and shows their results.
- **For whom:** developers working in separate branches/worktrees who need to
  know their current check results without switching worktrees.
- **What to click first:** **Try it with sample data** to see three sample Git
  worktree checks pass.

The answer is explicit on the first screen: “Check changed worktrees in the
background”, “For developers with separate branches who need current check
results without switching worktrees.”, and “Try it with sample data”. The
390px facts block ended at y=652px in an 844px viewport, so the action and all
three plain facts are visible before scrolling. This gate passes.

## Copy audit

Counts are whitespace-delimited. Commands, the example TOML, terminal-table
data, and code blocks are executable/sample data rather than prose. Headings,
labels, buttons, captions, and footer text are included. Every entry is at or
below 22 words. No marketing adjective, banned term, unclear mood heading,
undefined jargon, inconsistent term, or non-result-naming button was found.

### Landing page

| Copy | Words |
| --- | ---: |
| LOCAL CLI | 2 |
| Check changed worktrees in the background | 6 |
| For developers with separate branches who need current check results without switching worktrees. | 13 |
| Try it with sample data | 5 |
| See three Git worktree checks pass. | 6 |
| Sample creates three isolated Git worktrees. | 6 |
| Only configured commands run. | 4 |
| Board starts on this computer. | 5 |
| Three worktree folders feed one verification board. | 7 |
| Three Git worktrees feed one status board. | 7 |
| STATUS BOARD | 2 |
| Current and last passing commits by worktree | 7 |
| The board keeps the last passing commit when a newer check fails. | 12 |
| THREE STEPS | 2 |
| Run checks where the changes live | 6 |
| List worktrees. | 2 |
| Give each Git path and configured command in one file. | 10 |
| Start the watcher. | 3 |
| It reruns checks only for worktrees that changed. | 8 |
| Read the board. | 3 |
| Each result names the snapshot it checked and its last pass. | 11 |
| BOUNDARIES | 1 |
| Command and network boundaries | 4 |
| The CLI runs only commands you put in its config. | 10 |
| The board starts on this computer by default. | 8 |
| Configured commands keep their own network access. | 7 |
| Status for configured Git worktree checks. | 6 |

### README

| Copy | Words |
| --- | ---: |
| Worktree Verifier | 2 |
| Run declared checks in separate Git worktrees. | 7 |
| It is for developers who need current branch results without switching worktrees. | 12 |
| The local board shows each worktree's state, commit, and changed-file count. | 11 |
| Install | 1 |
| Build it from a clone: | 5 |
| Configure and run Worktree Verifier | 5 |
| Create a commented config: | 4 |
| Edit `.worktree-verifier.toml` to list each worktree and its configured checks: | 10 |
| Run once in scripts or CI: | 6 |
| Watch continuously and open the printed localhost URL: | 8 |
| The board starts before the first check and shows `RUNNING` while commands run. | 13 |
| Each command stops after `command_timeout_seconds`. | 5 |
| A timeout appears as `ERROR`; changing that worktree makes the watcher try its checks again. | 15 |
| Worktree Verifier never discovers or runs commands automatically; only commands in your config run. | 14 |
| Review them before use. | 4 |
| Fresh results and command boundaries | 5 |
| Before a check starts, the watcher snapshots the worktree's commit and working state. | 13 |
| If either changes during a check, the watcher marks that result stale. | 12 |
| It checks the new snapshot before reporting a pass. | 9 |
| The local board shows the current snapshot and `last_pass_commit`. | 9 |
| A later failure never erases the last known passing commit. | 10 |
| The watcher reruns only the configured worktree whose Git state changed. | 11 |
| It does not discover commands or repositories for you. | 9 |
| The CLI adds no isolation layer. | 6 |
| Configured commands inherit its user identity, environment, and filesystem access. | 10 |
| Use an operating-system sandbox when a command needs a stricter boundary. | 11 |
| Try the isolated sample | 4 |
| The demo creates `checkout-ui`, `checkout-api`, and `checkout-docs` as separate Git worktrees under a temporary directory. | 15 |
| It commits one sample file in each worktree. | 8 |
| It runs one declared check in each, prints the location, and removes the directory. | 14 |
| Use `cargo run -- demo --keep` to inspect the files. | 10 |
| Use `cargo run -- demo --serve` to open its board on this computer. | 13 |
| Open `/?demo=1` for the hosted recording after running the site locally. | 11 |
| The same recording is also available at `/demo`. | 8 |
| See [.factory/demo.md](.factory/demo.md) for its sandbox details. | 6 |
| Develop, test, and build | 4 |
| To develop the documentation site: | 5 |
| `npm run build` is an alias for `npm run build:site`. | 10 |
| The deploy artifact is `dist/site`, with `index.html` at its root. | 10 |
| Privacy and boundaries | 3 |
| By default, only this computer can open the board at `127.0.0.1`. | 11 |
| The status page warns when another device may reach the configured address. | 12 |
| A configured command keeps its network access. | 7 |
| Review each command before adding it. | 6 |
| The documentation site is static and sends no analytics or tracking requests. | 12 |
| Its `/privacy` and `/terms` routes are included in the built site. | 11 |
| Deploy | 1 |
| Build the static documentation artifact with `npm ci && npm run build`. | 12 |
| Deploy the generated `dist/site` directory with the included `staticwebapp.config.json`; deployment infrastructure is managed by the factory. | 16 |
| License | 1 |
| MIT | 1 |

The public terms are consistent: **worktree**, **check**, **status board**,
**config**, and **last pass**. Each operative statement maps to the relevant
registered claim (`demo-isolated-worktrees`, `configured-commands`,
`loopback-default`, `board-fields`, `fresh-last-pass`,
`changed-worktree-only`, `configured-command-permissions`, or the static-site
claims). The non-claim instructions and legal disclaimer do not promise a
product outcome. No unlisted claim finding applies.

## Demo and sandbox

The landing action entered `/?demo=1` in one click. Its first screen already
showed the named `checkout-ui`, `checkout-api`, and `checkout-docs` sample
worktrees, distinct commits, PASS results, and cleanup. **Replay sample**
changed the terminal recording. **Reset demo** restored the original output,
returned scroll to zero, focused the H1, and restored the replay control.

At 390px after scrolling to the end, the sticky banner remained at y=0 with
both **Reset demo** and **Start for real** fully visible (44px-high controls).
The banner says “Demo — sample data, nothing is saved”. Leaving through
**Start for real** removed the banner and moved focus to the setup H1.

A fresh browser context ended with zero localStorage entries, zero
sessionStorage entries, zero IndexedDB databases, and zero service workers.
The request log for landing, demo, privacy, and terms had only
`background-worktree-verifier.sociobot.in`; no analytics, tracking, external
font, or model request was made. A direct `cargo run -- demo` from an empty
temporary caller directory printed a new `/tmp/worktree-verifier-demo-*`
location, passed all three checks, removed the sample, and left the caller
directory empty.

## Claims verification

All 14 commands in `.factory/claims.json` were run verbatim in a new clone at
`/tmp/wtv-review2-oLKGXe` after `npm ci`. Every result passed.

| Claim ID | Result |
| --- | --- |
| demo-isolated-worktrees | PASS |
| init-config | PASS |
| loopback-default | PASS |
| listener-reachability-guidance | PASS |
| configured-commands | PASS |
| configured-command-permissions | PASS |
| board-fields | PASS |
| one-shot-json | PASS |
| bounded-command-timeout | PASS |
| fresh-last-pass | PASS |
| changed-worktree-only | PASS |
| demo-browser-sandbox | PASS |
| static-no-analytics | PASS |
| static-build-artifact | PASS |

Each ID has exactly one `@claim:<id>` test. The full clean-clone gates also
passed: `npm test` (18 Rust tests and 8 Node tests) and `npm run build` (which
produced `dist/site`).

## Earlier findings rechecked

Every earlier review/polish/handoff document was read. The following table
records a fresh live-site and source check for every finding in review 1; none
was merely accepted on the earlier report's assertion.

| Earlier finding | Fresh confirmation |
| --- | --- |
| F-1-1 | Live Replay changes output; Reset restores output, H1 focus, and top scroll; `resetDemo()` implements the same. |
| F-1-2 | At mobile page end, `.demo-banner` is sticky and both controls remain in view; `repair.css` sets it sticky. |
| F-1-3 | Live heading/footer say “Command and network boundaries” and “Status for configured Git worktree checks”; boundary copy is explicit. |
| F-1-4 | README names state, commit, and changed-file count; `board-fields` passed. |
| F-1-5 | README's commented config is covered by `init-config`, which passed. |
| F-1-6 | README's one-shot JSON workflow is covered by `one-shot-json`, which passed. |
| F-1-7 | README's `--keep` and `--serve` modes are included in `demo-isolated-worktrees`, which passed. |
| F-1-8 | The demo claim/test covers printed location and one committed sample file per worktree; live recording states both. |
| F-1-9 | Network access is named in README, privacy, landing, and the tested permissions claim. |
| F-1-10 | The stale crates.io premise is absent; README says “Build it from a clone.” |
| F-1-11 | The unproved runtime-version sentence is absent. |
| F-1-12 | `static-build-artifact` passed and the fresh build contains index, demo, privacy, and terms HTML. |
| F-1-13 | The old 23-word stale-result sentence is split into two 12/9-word sentences. |
| F-1-14 | The image caption now concretely says “Three Git worktrees feed one status board.” |
| F-1-15 | The live headings are “Current and last passing commits by worktree” and “Configure and run Worktree Verifier.” |
| F-1-16 | Current public copy uses **check** consistently; the former smoke-result wording is absent. |
| F-1-17 | No visitor-facing “fast” command claim remains. |
| F-1-18 | README uses “only this computer” and `127.0.0.1`, while the listener claim passed. |
| F-1-19 | Raw live `/`, `/demo`, `/privacy`, `/terms`, and 404 HTML expose their own title, description, canonical, OG, and Twitter metadata. |

## Structure, accessibility, and product fit

Live `/`, `/demo`, `/privacy`, and `/terms` returned 200. A random path
returned a designed 404 with HTTP 404. The crawled internal navigation targets
returned 200; the 404 page's `#main` skip link is an in-page anchor, not a
network destination. All pages had `lang=en`, one H1, one main landmark, no
390px horizontal overflow, and no serious/critical axe findings at 390px or
desktop.

The live pages have route-specific title, description, canonical, Open Graph,
Twitter, favicon, touch icon, robots, and sitemap support. Browser Back and
client navigation focus the destination H1. The skip link, visible focus ring,
44px mobile targets, and reduced-motion behavior pass the browser suite. No
console errors or failed requests occurred.

The warm-paper, navy, vermilion, moss, mono/Georgia, halftone-folder bulletin
is visibly product-specific and matches `.factory/design.md`; it is not a
generic SaaS hero/card treatment. The first-load built JavaScript is 3.71 KB
gzip and CSS is 2.23 KB gzip. The brief describes a local CLI verifier, so a
model feature, import/export, or sync is not an implied missing capability.

## What would make this perfect

Nothing remains to change for this review's acceptance criteria. Preserve the
current one-click sandbox, explicit command/network boundary, claim tests, and
route metadata as the product evolves.

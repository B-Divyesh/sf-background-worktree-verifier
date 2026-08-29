# Adversarial first-read review 1 — FAIL

**Reviewed:** 2026-08-29

**Live URL:** https://background-worktree-verifier.sociobot.in

**Candidate:** `47c6b2db460693dedb5f019803af3f057b10d583`

**Work order:** `background-worktree-verifier-review-1`

## Verdict

**FAIL.** The core CLI, first screen, routing, accessibility baseline, and all
ten registered claim commands pass. The demo contract does not: its reset
control does not reset anything, and its required demo banner scrolls away.
The public copy also contains unregistered claims, one overlong sentence,
unclear wording, and route metadata that continues to describe the landing
page. PASS requires zero findings.

## Cold first read

Fresh Chromium contexts were opened at 390×844 and 1440×900 without scrolling.

- **What it does:** checks changed Git worktrees in the background.
- **For whom:** developers who use separate branches and want current check
  results without switching worktrees.
- **What to click first:** **Try it with sample data**; the adjacent text says
  it will show three Git worktree checks passing.

This gate passes. The exact first-screen text was “Check changed worktrees in
the background”, “For developers with separate branches who need fresh smoke
results without switching worktrees”, and “Try it with sample data”, followed
by “See three Git worktree checks pass.” The mobile first screen also showed
all three facts before the illustration. There were no console errors or
failed requests on either cold load.

## Findings

### Blocking

#### F-1-1 — Reset demo does not reset the demo

- **Quote/location:** `/demo`, **Reset demo**; implementation at
  `site/src/main.js:61-64`.
- **Observed:** activating the control changes only its own label from “Reset
  demo” to “Demo reset”. The terminal output and every other piece of demo
  state remain byte-for-byte unchanged. A second activation has no additional
  effect.
- **Why this fails:** the required Reset action is present but non-functional.
  A visitor cannot replay or restore the sample. A weak demo is blocking under
  the supplied demo contract.
- **Concrete fix:** make the sample a short replayable terminal sequence and
  have **Reset demo** restore its initial frame, output, focus, and scroll
  position. Add a browser test that changes/advances the demo, resets it, and
  asserts the complete initial state.

#### F-1-2 — The demo banner is not persistent

- **Quote/location:** `/demo`, “Demo — sample data, nothing is saved”; CSS
  `.demo-banner` in `site/src/styles.css`.
- **Observed:** at 390 px, the page is 1,550 px high. At `scrollY = 706`, the
  banner rectangle is from `-636` to `-540` px and is not visible. Its computed
  position is `static`.
- **Why this fails:** the required demo-state warning and exits disappear while
  the visitor is still in the demo.
- **Concrete fix:** keep the banner visible with a sticky treatment that does
  not cover focused content. Add a 390 px test that scrolls to the end of the
  demo and asserts the banner, **Reset demo**, and **Start for real** remain in
  the viewport.

### High

#### F-1-3 — “Local” overstates the boundary and is not a registered claim

- **Quote/location:** landing heading “Your checks stay local and intentional”
  and footer “Local smoke checks for separate Git worktrees.”
- **Why this fails:** `.factory/claims.json` proves a loopback default and
  inherited process permissions, but does not prove that checks stay local.
  README correctly says a configured command may make network requests, so the
  heading can mislead a visitor.
- **Concrete fix:** use “Command and network boundaries” as the heading and
  “Status for configured Git worktree checks” in the footer. State explicitly
  that the board defaults to this computer while configured commands keep
  their own network access.

#### F-1-4 — The board-field claim is only partly registered

- **Quote/location:** README: “It watches configured Git worktrees and serves a
  compact board on `127.0.0.1` with each worktree's state, commit, and
  changed-file count.”
- **Why this fails:** the registry covers the listener and last-pass behavior,
  but no claim entry promises and tests the complete state/commit/changed-file
  payload. “Compact” is also an unmeasured marketing adjective.
- **Concrete fix:** rewrite as “The local board shows each worktree's state,
  commit, and changed-file count.” Add a `board-fields` claim whose public-CLI
  test asserts all three fields, or remove the unregistered fields from the
  sentence.

#### F-1-5 — The generated-commented-config claim is unlisted

- **Quote/location:** README: “Start with a commented config:”
- **Why this fails:** the listed loopback claim inspects one generated value,
  but no registry entry promises or verifies that `init` produces a usable,
  commented configuration.
- **Concrete fix:** add an `init-config` claim and a public CLI test that checks
  the generated comments and parses the file, or rewrite the instruction to
  “Create the config:” without the extra claim.

#### F-1-6 — The one-shot mode is an unlisted public capability

- **Quote/location:** README: “Run once in scripts or CI:” followed by
  `worktree-verifier run --once --json`.
- **Why this fails:** other tests happen to use the flags, but no claim entry
  states and verifies the promised one-shot JSON workflow.
- **Concrete fix:** add a `one-shot-json` claim that asserts one run, valid JSON,
  meaningful PASS/FAIL exit codes, and no watcher left running.

#### F-1-7 — Two demo modes are documented without claim entries

- **Quote/location:** README: “Use `cargo run -- demo --keep` to inspect it or
  `cargo run -- demo --serve` to open its localhost board.”
- **Why this fails:** neither `--keep` nor `--serve` is stated in a claim entry.
  The current demo test inspects `--keep`, but it does not test `--serve`.
- **Concrete fix:** extend the demo claim text and test to cover both modes,
  including board startup and shutdown for `--serve`, or stop promising the
  modes in README.

#### F-1-8 — Two details on the demo page exceed the registered demo claim

- **Quote/location:** `/demo`: “The command prints its temporary sample
  location.” and “Commits one sample file in each worktree.”
- **Why this fails:** `demo-isolated-worktrees` covers three worktrees, one
  declared check in each, and cleanup. Its claim text does not include either
  quoted behavior, and its kept-sample assertions do not check the sample file
  in each worktree.
- **Concrete fix:** add both behaviors to that claim and assert the printed
  path plus the committed sample file in every kept worktree.

#### F-1-9 — The command-network warning is not in the claims registry

- **Quote/location:** README: “A command you configure may make network
  requests; review each command before adding it.”
- **Why this fails:** this is an important execution-boundary statement. The
  permissions claim checks identity, environment, and filesystem access, not
  network access.
- **Concrete fix:** expand `configured-command-permissions` to mention network
  access and test a configured command against a temporary local listener.

#### F-1-10 — The installation publication status is an unlisted, time-sensitive claim

- **Quote/location:** README: “Until published to crates.io, build it from a
  clone:”
- **Why this fails:** this fact can become stale and has no claim entry or
  release check.
- **Concrete fix:** either register a release-status test or remove the
  time-sensitive premise and say “Build it from a clone:”

#### F-1-11 — Runtime-version support is unlisted

- **Quote/location:** README: “Requirements: Rust stable and Node 20+.”
- **Why this fails:** compatibility is a claim a user relies on, but the claim
  registry contains no version-matrix check.
- **Concrete fix:** add a compatibility claim backed by CI on Rust stable and
  the oldest supported Node release, or name only versions actually tested by
  the release gate.

#### F-1-12 — Build and route promises are outside the registry

- **Quote/location:** README: “`npm run build` is an alias for `npm run
  build:site`.”; “The deploy artifact is `dist/site`, with `index.html` at its
  root.”; “Its `/privacy` and `/terms` routes are included in the built site.”;
  and “Build the static documentation artifact with `npm ci && npm run
  build`.”
- **Why this fails:** these observable promises are tested by the general suite
  but have no claims entries, so a verifier cannot map the public statements to
  required tests.
- **Concrete fix:** consolidate them into one `static-build-artifact` claim and
  test the build command, artifact root, and both deep routes from a clean
  checkout.

### Minor

#### F-1-13 — One README sentence exceeds the 22-word cap

- **Quote/location:** README, “If either changes while the command runs, it
  marks that attempt stale and checks the new snapshot before it can report a
  pass.” — **23 words**.
- **Why this fails:** it combines state invalidation and retry behavior in one
  sentence.
- **Concrete rewrite:** “If either changes during a check, the watcher marks
  that result stale. It checks the new snapshot before reporting a pass.”

#### F-1-14 — The illustration caption is a slogan, not a description

- **Quote/location:** landing figure caption: “Separate changes. One fresh
  status board.”
- **Why this fails:** both fragments rely on mood and omit the useful
  relationship shown by the image.
- **Concrete rewrite:** “Three Git worktrees feed one status board.”

#### F-1-15 — Two headings are vague out of context

- **Quote/location:** landing: “See every worktree at a glance”; README: “Use
  it”.
- **Why this fails:** “at a glance” adds no information, and “Use it” does not
  name the section when read in a heading list.
- **Concrete rewrite:** “Current and last passing commits by worktree” and
  “Configure and run Worktree Verifier”.

#### F-1-16 — “Smoke” terminology is introduced as jargon and then varies

- **Quote/location:** landing audience sentence says “fresh smoke results”;
  other copy alternates among “smoke checks”, “checks”, and “results”.
- **Why this fails:** “smoke result” is not defined, and the same operation has
  three labels.
- **Concrete rewrite:** use “checks” consistently. For example: “For developers
  with separate branches who need current check results without switching
  worktrees.” Define “smoke check” once in README only if that technical term is
  necessary.

#### F-1-17 — “Fast” is an unsupported adjective

- **Quote/location:** README “Run fast, declared smoke checks…” and “its fast,
  opt-in checks”; landing “each fast command”.
- **Why this fails:** no duration is stated or tested, and command duration is
  controlled by the user's command.
- **Concrete rewrite:** remove “fast” from all three locations. If speed is a
  product promise, state a measured bound and add it to `claims.json`.

#### F-1-18 — “Loopback” is avoidable jargon in user-facing README copy

- **Quote/location:** “The CLI binds its board to loopback by default.” and
  “The status page confirms a loopback-only listener…”
- **Why this fails:** the landing page already uses the clearer word
  “localhost”.
- **Concrete rewrite:** “By default, only this computer can open the board at
  `127.0.0.1`.” and “The status page warns when another device may reach the
  configured address.”

#### F-1-19 — Secondary routes publish landing-page social metadata

- **Quote/location:** live `/demo`, `/privacy`, and `/terms` all retain
  `og:title="Worktree Verifier — Check changed worktrees"` and the landing
  description. The real 404 document has no Open Graph or Twitter metadata.
- **Why this fails:** titles and canonical URLs change per route, but shared
  previews describe a different page; the 404 metadata set is incomplete.
- **Concrete fix:** emit route-specific HTML metadata (or equivalent edge
  metadata) so link-preview crawlers receive it without running JavaScript;
  also update the DOM during client navigation. Add complete metadata to
  `404.html` and test every route.

## Copy audit

Counts use whitespace-delimited words. Code blocks and status-table data are
excluded because they are executable/sample data, not sentences. Headings,
labels, actions, the image alternative, and sentence fragments are included so
their clarity can be checked.

### Landing page

| Copy | Words | Result |
| --- | ---: | --- |
| LOCAL CLI | 2 | pass |
| Check changed worktrees in the background | 6 | pass |
| For developers with separate branches who need fresh smoke results without switching worktrees. | 13 | F-1-16 |
| Try it with sample data | 5 | pass |
| See three Git worktree checks pass. | 6 | pass |
| Sample uses isolated Git worktrees. | 5 | pass |
| Commands are opt-in. | 3 | pass |
| Board defaults to localhost. | 4 | pass |
| Three worktree folders feed one compact verification board. | 8 | pass (image alternative) |
| Separate changes. | 2 | F-1-14 |
| One fresh status board. | 4 | F-1-14 |
| STATUS BOARD | 2 | pass |
| See every worktree at a glance | 6 | F-1-15 |
| The board keeps the last passing commit when a newer check fails. | 12 | pass |
| THREE STEPS | 2 | pass |
| Run smoke checks where the changes live | 7 | F-1-16 |
| List worktrees. | 2 | pass |
| Give each Git path and each fast command in one file. | 11 | F-1-17 |
| Start the watcher. | 3 | pass |
| It reruns checks only for worktrees that changed. | 8 | pass |
| Read the board. | 3 | pass |
| Each result names the snapshot it checked and its last pass. | 11 | pass |
| BOUNDARIES | 1 | pass |
| Your checks stay local and intentional | 6 | F-1-3 |
| The CLI runs only commands you put in its config. | 10 | pass |
| The status board binds to localhost by default. | 8 | pass |
| Local smoke checks for separate Git worktrees. | 7 | F-1-3, F-1-16 |

### README

| Copy | Words | Result |
| --- | ---: | --- |
| Worktree Verifier | 2 | pass |
| Run fast, declared smoke checks in separate Git worktrees. | 9 | F-1-16, F-1-17 |
| It is for developers who need to know which branch is fresh without switching worktrees. | 15 | pass |
| It watches configured Git worktrees and serves a compact board on 127.0.0.1 with each worktree's state, commit, and changed-file count. | 20 | F-1-4 |
| Install | 1 | pass |
| Until published to crates.io, build it from a clone: | 9 | F-1-10 |
| Use it | 2 | F-1-15 |
| Start with a commented config: | 5 | F-1-5 |
| Edit .worktree-verifier.toml to list each worktree and its fast, opt-in checks: | 11 | F-1-17 |
| Run once in scripts or CI: | 6 | F-1-6 |
| Watch continuously and open the printed localhost URL: | 8 | pass |
| The board starts before the first check and shows RUNNING while commands run. | 13 | pass |
| Each command stops after command_timeout_seconds. | 5 | pass |
| A timeout appears as ERROR; changing that worktree makes the watcher try its checks again. | 15 | pass |
| Worktree Verifier never discovers or runs commands automatically; only commands in your config run. | 14 | pass |
| Review them before use. | 4 | pass |
| Fresh results and command boundaries | 5 | pass |
| Before a smoke check starts, the watcher snapshots the worktree's commit and working state. | 14 | F-1-16 |
| If either changes while the command runs, it marks that attempt stale and checks the new snapshot before it can report a pass. | 23 | F-1-13 |
| The local board shows the current snapshot and last_pass_commit, so a later failure never erases the last known passing commit. | 20 | pass |
| The watcher reruns only the configured worktree whose Git state changed. | 11 | pass |
| It does not discover commands or repositories for you. | 9 | pass |
| The CLI adds no isolation layer. | 6 | pass |
| Configured commands inherit its user identity, environment, and filesystem access. | 10 | pass |
| Use an operating-system sandbox when a command needs a stricter boundary. | 11 | pass |
| Try the isolated sample | 4 | pass |
| The demo creates checkout-ui, checkout-api, and checkout-docs as actual Git worktrees under a temporary directory. | 15 | pass |
| It commits a sample file and runs one declared smoke check in each, prints the location, and removes the directory. | 20 | F-1-8, F-1-16 |
| Use cargo run -- demo --keep to inspect it or cargo run -- demo --serve to open its localhost board. | 20 | F-1-7 |
| The hosted documentation recording is available at /demo after running the site locally. | 13 | pass |
| See .factory/demo.md for its sandbox details. | 6 | pass |
| Develop, test, and build | 4 | pass |
| Requirements: Rust stable and Node 20+. | 6 | F-1-11 |
| To develop the documentation site: | 5 | pass |
| npm run build is an alias for npm run build:site. | 10 | F-1-12 |
| The deploy artifact is dist/site, with index.html at its root. | 10 | F-1-12 |
| Privacy and boundaries | 3 | pass |
| The CLI binds its board to loopback by default. | 9 | F-1-18 |
| The status page confirms a loopback-only listener and warns when your configured address may accept network connections. | 17 | F-1-18 |
| A command you configure may make network requests; review each command before adding it. | 14 | F-1-9 |
| The documentation site is static and sends no analytics or tracking requests. | 12 | pass |
| Its /privacy and /terms routes are included in the built site. | 11 | F-1-12 |
| Deploy | 1 | pass |
| Build the static documentation artifact with npm ci && npm run build. | 11 | F-1-12 |
| Deploy the generated dist/site directory with the included staticwebapp.config.json; deployment infrastructure is managed by the factory. | 16 | pass |
| License | 1 | pass |
| MIT | 1 | pass |

No banned plain-words term appears. The primary landing action is a result-
naming verb. The demo's **Reset demo** label is also clear; its behavior is the
problem recorded in F-1-1.

## Claims verification

All listed commands were run verbatim after `npm ci` in a fresh clone at
`/tmp/wtv-review1-clone-UK7zdn`. The clone began clean at the candidate commit.

| Claim ID | Result | Evidence |
| --- | --- | --- |
| `demo-isolated-worktrees` | PASS | Three distinct Git worktrees and commits; cleanup asserted |
| `loopback-default` | PASS | Generated config contained `127.0.0.1:4318` |
| `listener-reachability-guidance` | PASS | Loopback and wildcard listener messages asserted |
| `configured-commands` | PASS | Declared marker created; undeclared marker absent |
| `configured-command-permissions` | PASS | UID, environment, and adjacent filesystem access asserted |
| `bounded-command-timeout` | PASS | RUNNING, one-second timeout, descendant cancellation, and recovery asserted |
| `fresh-last-pass` | PASS | Checked commit and retained last-pass commit asserted |
| `changed-worktree-only` | PASS | Only the changed worktree reran |
| `demo-browser-sandbox` | PASS | Empty browser storage and no service worker |
| `static-no-analytics` | PASS | Landing and demo requests remained same-origin |

Each registered ID appears in exactly one `@claim:<id>` test, and there are no
extra claim tags. Findings F-1-3 through F-1-12 identify public statements that
the current registry does not completely name.

## Demo and sandbox evidence

- The landing action opened `/demo` in one click and immediately showed three
  realistic named worktrees, distinct commits, PASS results, and cleanup.
- Before and after the demo flow, localStorage, sessionStorage, IndexedDB, and
  service-worker registration counts were all zero.
- The request log contained only the product origin; no analytics, font CDN,
  model endpoint, or other third party was contacted.
- From a fresh temporary working directory, `cargo run -- demo` created its
  sample under `/tmp`, printed three distinct passing commits, removed the
  reported sample root, and left the caller directory empty.
- The CLI demo is useful without AI. The brief does not imply summarisation,
  generation, sync, or import/export, so no missing AI or gateway feature was
  found.

## Structure, routing, accessibility, and visual identity

- `/`, `/demo`, `/privacy`, and `/terms` returned 200. A random path returned
  the designed 404 document with HTTP 404. All crawled internal links returned
  200.
- Every checked route had `lang=en`, one `<h1>`, one `<main>`, ordered headings,
  no horizontal page overflow, and route-appropriate document titles.
- SPA navigation and browser Back moved focus to the new H1. The skip link,
  visible focus styles, 44 px targets, and reduced-motion mode worked.
- Playwright axe 4.11 found zero serious or critical issues on every route at
  390×844 and 1440×900. `/opt/fleet/lib/verify-url.sh` passed the live landing
  page with zero console errors.
- The live request headers included a self-only CSP, `nosniff`, strict-origin
  referrer policy, and HSTS. The first-load JS is 7,726 bytes raw, well below
  the budget.
- The generated 1200×630 Open Graph art, 180×180 touch icon, SVG favicon,
  risograph palette, square stamped controls, editorial type, and halftone
  worktree illustration form a distinct product identity rather than a generic
  SaaS template.
- All 12 public artifacts matched a fresh local build by SHA-256, so the live
  findings apply to the reviewed candidate.

## History check

There are no earlier `.factory/review-*.md` or `.factory/polish-*.md` files.
The earlier handoff claimed no known gaps and a PASS. Its test, deployment,
route, accessibility, and identity statements were rechecked on the live site
and in the code. No prior finding IDs exist to reopen. F-1-1 through F-1-19 are
new findings under this review's stricter demo, copy, claim-listing, and route-
metadata checks.

## What would make this perfect

Make Reset genuinely restore/replay the sample and keep the demo warning and
exits visible while scrolling. Register or remove every public promise listed
above. Replace the overlong, vague, jargon-heavy, and slogan copy with the
provided rewrites. Give each route accurate description/social metadata. Then
rerun all ten claim commands, the complete test/build gates, the live request
log, axe at both sizes, route/back/focus checks, link crawl, and this full
first-read checklist. Nothing else is currently identified.

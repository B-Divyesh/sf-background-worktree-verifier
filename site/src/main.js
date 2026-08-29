import './repair.css';

const app = document.querySelector('#app');
const note = document.querySelector('#route-note');
const canonical = document.querySelector('link[rel="canonical"]');
const pages = { '/': landing, '/demo': demo, '/privacy': privacy, '/terms': terms, '/404': missing };

function shell(body) {
  return `<header class="site-header"><a class="wordmark" href="/" data-route>WORKTREE<br>VERIFIER</a><nav aria-label="Primary"><a href="/demo" data-route>Demo</a><a href="/#setup" data-route>Setup</a><a href="/privacy" data-route>Privacy</a></nav></header><main id="main" tabindex="-1">${body}</main><footer><p>Local smoke checks for separate Git worktrees.</p><p><a href="/privacy" data-route>Privacy</a> · <a href="/terms" data-route>Terms</a> · Built by Param Factory · v0.1.0</p></footer>`;
}

function installSnippet() {
  return `git clone https://github.com/B-Divyesh/sf-background-worktree-verifier.git
cd sf-background-worktree-verifier
cargo install --path .
worktree-verifier init
worktree-verifier run`;
}

function landing() {
  document.title = 'Worktree Verifier — Check changed worktrees';
  return shell(`<section class="hero"><div class="hero-copy"><p class="kicker">LOCAL CLI</p><h1>Check changed worktrees in the background</h1><p class="lede">For developers with separate branches who need fresh smoke results without switching worktrees.</p><p class="actions"><a class="button" href="/demo" data-route>Try it with sample data</a><span>See three Git worktree checks pass.</span></p><ul class="facts"><li>Sample uses isolated Git worktrees.</li><li>Commands are opt-in.</li><li>Board defaults to localhost.</li></ul></div><figure><img src="/halftone-worktrees.webp" width="960" height="640" fetchpriority="high" alt="Three worktree folders feed one compact verification board."><figcaption>Separate changes. One fresh status board.</figcaption></figure></section><section class="board-preview" aria-labelledby="preview-title"><p class="section-label">STATUS BOARD</p><h2 id="preview-title">See every worktree at a glance</h2><div class="status-table" role="table" aria-label="Sample worktree status" tabindex="0"><div role="row" class="table-head"><span role="columnheader">WORKTREE</span><span role="columnheader">CURRENT</span><span role="columnheader">LAST PASS</span><span role="columnheader">STATE</span></div><div role="row"><span role="cell">checkout-ui</span><span role="cell">c72ea1d</span><span role="cell">a1b2c3d</span><strong role="cell" class="stamp pass">PASS</strong></div><div role="row"><span role="cell">checkout-api</span><span role="cell">d4e5f6a</span><span role="cell">d4e5f6a</span><strong role="cell" class="stamp pass">PASS</strong></div><div role="row"><span role="cell">checkout-docs</span><span role="cell">9b8c7d6</span><span role="cell">9b8c7d6</span><strong role="cell" class="stamp idle">FAIL</strong></div></div><p class="caption">The board keeps the last passing commit when a newer check fails.</p></section><section id="setup" class="steps" aria-labelledby="setup-title"><p class="section-label">THREE STEPS</p><h2 id="setup-title">Run smoke checks where the changes live</h2><ol><li><b>List worktrees.</b><span>Give each Git path and each fast command in one file.</span></li><li><b>Start the watcher.</b><span>It reruns checks only for worktrees that changed.</span></li><li><b>Read the board.</b><span>Each result names the snapshot it checked and its last pass.</span></li></ol><pre aria-label="Clone, install, and start commands"><code>${installSnippet()}</code></pre></section><section class="limits" aria-labelledby="limits-title"><p class="section-label">BOUNDARIES</p><h2 id="limits-title">Your checks stay local and intentional</h2><p>The CLI runs only commands you put in its config.</p><p>The status board binds to localhost by default.</p></section>`);
}

function demo() {
  document.title = 'Demo — Worktree Verifier';
  return shell(`<aside class="demo-banner" aria-label="Demo status"><strong>Demo — sample data, nothing is saved</strong><button id="reset-demo">Reset demo</button><a href="/#setup" data-route>Start for real</a></aside><section class="demo-page"><p class="kicker">SAMPLE RUN</p><h1>See three worktrees pass</h1><p class="lede">This recording uses temporary Git worktrees with separate commits.</p><div class="terminal" aria-label="Terminal recording of the sample command"><div class="terminal-bar"><span></span><span></span><span></span><b>worktree-verifier demo</b></div><pre id="demo-output" tabindex="0" aria-label="Recorded terminal output for the sample command"><code>$ worktree-verifier demo
Sample worktrees: /tmp/worktree-verifier-demo-...
checkout-ui      PASS  3e2a61f  0 changed  1 smoke check passed
checkout-api     PASS  7c6b0d2  0 changed  1 smoke check passed
checkout-docs    PASS  b9d40e8  0 changed  1 smoke check passed
Removed sample worktrees.</code></pre></div><p class="demo-actions"><a class="button" href="/#setup" data-route>Read setup steps</a><span>The command prints its temporary sample location.</span></p><section class="demo-notes" aria-labelledby="sample-title"><h2 id="sample-title">What the sample does</h2><ul><li>Creates three separate temporary Git worktrees.</li><li>Commits one sample file in each worktree.</li><li>Runs one declared check in each worktree.</li><li>Removes those worktrees when it finishes.</li></ul></section></section>`);
}

function privacy() {
  document.title = 'Privacy — Worktree Verifier';
  return shell(`<article class="legal"><h1>Privacy for Worktree Verifier</h1><p>The status board binds to 127.0.0.1 by default.</p><p>The documentation site sends no analytics or tracking requests.</p><p>Configured commands use the permissions your account already has. Review each command before adding it.</p></article>`);
}

function terms() {
  document.title = 'Terms — Worktree Verifier';
  return shell(`<article class="legal"><h1>Terms for Worktree Verifier</h1><p>You choose the commands it runs.</p><p>Review those commands before use.</p><p>The software is provided without warranty.</p></article>`);
}

function missing() {
  document.title = 'Page not found — Worktree Verifier';
  return shell(`<section class="not-found"><p class="kicker">404</p><h1>Page not found</h1><p>This page is not available. Return to the documentation home.</p><a class="button" href="/" data-route>Return home</a></section>`);
}

function render({ focusHeading = false } = {}) {
  const path = pages[location.pathname] ? location.pathname : '/404';
  canonical.href = `https://background-worktree-verifier.sociobot.in${path}`;
  app.innerHTML = pages[path]();
  document.querySelector('.steps pre')?.setAttribute('tabindex', '0');
  const heading = document.querySelector('h1');
  note.textContent = heading.textContent;
  if (focusHeading) {
    heading.tabIndex = -1;
    heading.focus({ preventScroll: true });
  }
  if (path === '/demo') {
    document.querySelector('#reset-demo').addEventListener('click', event => {
      event.currentTarget.textContent = 'Demo reset';
    });
  }
}

document.addEventListener('click', event => {
  const link = event.target.closest('a[data-route]');
  if (!link || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;
  event.preventDefault();
  const target = new URL(link.href).hash;
  history.pushState({}, '', link.href);
  render({ focusHeading: true });
  if (target) document.querySelector(target)?.scrollIntoView();
  else scrollTo(0, 0);
});

addEventListener('popstate', () => render({ focusHeading: true }));
render();

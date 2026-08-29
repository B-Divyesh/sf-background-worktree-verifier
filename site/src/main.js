import './repair.css';

const app = document.querySelector('#app');
const note = document.querySelector('#route-note');

const routeMeta = {
  '/': {
    title: 'Worktree Verifier — Check changed worktrees',
    description: 'Run configured checks in separate Git worktrees and see each current result and last passing commit.',
    canonical: '/',
  },
  '/demo': {
    title: 'Demo — Worktree Verifier',
    description: 'Replay a sample Worktree Verifier run with three isolated Git worktrees and no saved browser data.',
    canonical: '/demo',
  },
  '/privacy': {
    title: 'Privacy — Worktree Verifier',
    description: 'Read how the Worktree Verifier CLI and documentation site handle commands, network access, and data.',
    canonical: '/privacy',
  },
  '/terms': {
    title: 'Terms — Worktree Verifier',
    description: 'Read the terms for using Worktree Verifier and the commands you configure it to run.',
    canonical: '/terms',
  },
  '/404': {
    title: 'Page not found — Worktree Verifier',
    description: 'The requested Worktree Verifier documentation page was not found.',
    canonical: '/404',
  },
};

const pages = { '/': landing, '/demo': demo, '/privacy': privacy, '/terms': terms, '/404': missing };
const initialDemoOutput = `$ worktree-verifier demo
Sample worktrees: /tmp/worktree-verifier-demo-...
checkout-ui      PASS  3e2a61f  0 changed  1 check passed
checkout-api     PASS  7c6b0d2  0 changed  1 check passed
checkout-docs    PASS  b9d40e8  0 changed  1 check passed
Removed sample worktrees.`;
const replayFrames = [
  '$ worktree-verifier demo\nCreating three temporary Git worktrees…',
  '$ worktree-verifier demo\nSample worktrees: /tmp/worktree-verifier-demo-...\ncheckout-ui      PASS  3e2a61f  0 changed  1 check passed',
  '$ worktree-verifier demo\nSample worktrees: /tmp/worktree-verifier-demo-...\ncheckout-ui      PASS  3e2a61f  0 changed  1 check passed\ncheckout-api     PASS  7c6b0d2  0 changed  1 check passed',
  initialDemoOutput,
];
let replayTimers = [];

function demoBanner() {
  return `<aside class="demo-banner" aria-label="Demo status"><strong>Demo — sample data, nothing is saved</strong><button id="reset-demo" type="button">Reset demo</button><a href="/#setup" data-route>Start for real</a></aside>`;
}

function shell(body, { isDemo = false } = {}) {
  return `<header class="site-header"><a class="wordmark" href="/" data-route>WORKTREE<br>VERIFIER</a><nav aria-label="Primary"><a href="/?demo=1" data-route>Demo</a><a href="/#setup" data-route>Setup</a><a href="/privacy" data-route>Privacy</a></nav></header>${isDemo ? demoBanner() : ''}<main id="main" tabindex="-1">${body}</main><footer><p>Status for configured Git worktree checks.</p><p><a href="/privacy" data-route>Privacy</a> · <a href="/terms" data-route>Terms</a> · Built by Param Factory · v0.1.0</p></footer>`;
}

function installSnippet() {
  return `git clone https://github.com/B-Divyesh/sf-background-worktree-verifier.git
cd sf-background-worktree-verifier
cargo install --path .
worktree-verifier init
worktree-verifier run`;
}

function landing() {
  return shell(`<section class="hero"><div class="hero-copy"><p class="kicker">LOCAL CLI</p><h1>Check changed worktrees in the background</h1><p class="lede">For developers with separate branches who need current check results without switching worktrees.</p><p class="actions"><a class="button" href="/?demo=1" data-route>Try it with sample data</a><span>See three Git worktree checks pass.</span></p><ul class="facts"><li>Sample creates three isolated Git worktrees.</li><li>Only configured commands run.</li><li>Board starts on this computer.</li></ul></div><figure><img src="/halftone-worktrees.webp" width="960" height="640" fetchpriority="high" alt="Three worktree folders feed one verification board."><figcaption>Three Git worktrees feed one status board.</figcaption></figure></section><section class="board-preview" aria-labelledby="preview-title"><p class="section-label">STATUS BOARD</p><h2 id="preview-title">Current and last passing commits by worktree</h2><div class="status-table" role="table" aria-label="Sample worktree status" tabindex="0"><div role="row" class="table-head"><span role="columnheader">WORKTREE</span><span role="columnheader">CURRENT</span><span role="columnheader">LAST PASS</span><span role="columnheader">STATE</span></div><div role="row"><span role="cell">checkout-ui</span><span role="cell">c72ea1d</span><span role="cell">a1b2c3d</span><strong role="cell" class="stamp pass">PASS</strong></div><div role="row"><span role="cell">checkout-api</span><span role="cell">d4e5f6a</span><span role="cell">d4e5f6a</span><strong role="cell" class="stamp pass">PASS</strong></div><div role="row"><span role="cell">checkout-docs</span><span role="cell">9b8c7d6</span><span role="cell">9b8c7d6</span><strong role="cell" class="stamp idle">FAIL</strong></div></div><p class="caption">The board keeps the last passing commit when a newer check fails.</p></section><section id="setup" class="steps" aria-labelledby="setup-title"><p class="section-label">THREE STEPS</p><h2 id="setup-title">Run checks where the changes live</h2><ol><li><b>List worktrees.</b><span>Give each Git path and configured command in one file.</span></li><li><b>Start the watcher.</b><span>It reruns checks only for worktrees that changed.</span></li><li><b>Read the board.</b><span>Each result names the snapshot it checked and its last pass.</span></li></ol><pre aria-label="Clone, install, and start commands"><code>${installSnippet()}</code></pre></section><section class="limits" aria-labelledby="limits-title"><p class="section-label">BOUNDARIES</p><h2 id="limits-title">Command and network boundaries</h2><p>The CLI runs only commands you put in its config.</p><p>The board starts on this computer by default.</p><p>Configured commands keep their own network access.</p></section>`);
}

function demo() {
  return shell(`<section class="demo-page"><p class="kicker">SAMPLE RUN</p><h1>See three worktrees pass</h1><p class="lede">This recording uses temporary Git worktrees with separate commits.</p><div class="terminal" aria-label="Terminal recording of the sample command"><div class="terminal-bar"><span></span><span></span><span></span><b>worktree-verifier demo</b></div><pre id="demo-output" tabindex="0" aria-label="Recorded terminal output for the sample command"><code>${initialDemoOutput}</code></pre></div><p id="demo-progress" class="sr-only" aria-live="polite">Sample run complete.</p><p class="demo-actions"><button id="replay-demo" type="button">Replay sample</button><span>The command prints its temporary sample location.</span></p><section class="demo-notes" aria-labelledby="sample-title"><h2 id="sample-title">What the sample does</h2><ul><li>Creates three separate temporary Git worktrees.</li><li>Commits one sample file in each worktree.</li><li>Runs one declared check in each worktree.</li><li>Removes those worktrees when it finishes.</li></ul></section></section>`, { isDemo: true });
}

function privacy() {
  return shell(`<article class="legal"><h1>Privacy for Worktree Verifier</h1><p>By default, only this computer can open the board at 127.0.0.1.</p><p>The documentation site sends no analytics or tracking requests.</p><p>The CLI adds no isolation layer. Configured commands inherit its user identity, environment, filesystem access, and network access.</p><p>Review each command before adding it.</p></article>`);
}

function terms() {
  return shell(`<article class="legal"><h1>Terms for Worktree Verifier</h1><p>You choose the commands it runs.</p><p>Review those commands before use.</p><p>The software is provided without warranty.</p></article>`);
}

function missing() {
  return shell(`<section class="not-found"><p class="kicker">404</p><h1>Page not found</h1><p>This page is not available. Return to the documentation home.</p><a class="button" href="/" data-route>Return home</a></section>`);
}

function currentPath() {
  if (location.pathname === '/' && new URLSearchParams(location.search).get('demo') === '1') return '/demo';
  return pages[location.pathname] ? location.pathname : '/404';
}

function setMeta(path) {
  const meta = routeMeta[path];
  const canonicalPath = path === '/demo' && location.pathname === '/' ? '/?demo=1' : meta.canonical;
  const canonicalUrl = `https://background-worktree-verifier.sociobot.in${canonicalPath}`;
  document.title = meta.title;
  document.querySelector('meta[name="description"]').content = meta.description;
  document.querySelector('link[rel="canonical"]').href = canonicalUrl;
  document.querySelector('meta[property="og:title"]').content = meta.title;
  document.querySelector('meta[property="og:description"]').content = meta.description;
  document.querySelector('meta[property="og:url"]').content = canonicalUrl;
  document.querySelector('meta[name="twitter:title"]').content = meta.title;
  document.querySelector('meta[name="twitter:description"]').content = meta.description;
}

function clearReplay() {
  for (const timer of replayTimers) clearTimeout(timer);
  replayTimers = [];
}

function setDemoOutput(output) {
  document.querySelector('#demo-output code').textContent = output;
}

function resetDemo() {
  clearReplay();
  setDemoOutput(initialDemoOutput);
  const replay = document.querySelector('#replay-demo');
  replay.disabled = false;
  replay.textContent = 'Replay sample';
  document.querySelector('#demo-progress').textContent = 'Demo reset. Sample run complete.';
  scrollTo(0, 0);
  requestAnimationFrame(() => {
    const heading = document.querySelector('h1');
    heading.tabIndex = -1;
    heading.focus({ preventScroll: true });
  });
}

function replayDemo() {
  clearReplay();
  const replay = document.querySelector('#replay-demo');
  replay.disabled = true;
  replay.textContent = 'Running sample';
  if (matchMedia('(prefers-reduced-motion: reduce)').matches) {
    setDemoOutput(initialDemoOutput);
    replay.disabled = false;
    replay.textContent = 'Replay sample';
    document.querySelector('#demo-progress').textContent = 'Sample run complete. Three worktrees passed.';
    return;
  }
  replayFrames.forEach((frame, index) => {
    const timer = setTimeout(() => {
      setDemoOutput(frame);
      if (index === replayFrames.length - 1) {
        replay.disabled = false;
        replay.textContent = 'Replay sample';
        document.querySelector('#demo-progress').textContent = 'Sample run complete. Three worktrees passed.';
      }
    }, index * 260);
    replayTimers.push(timer);
  });
}

function render({ focusHeading = false } = {}) {
  clearReplay();
  const path = currentPath();
  setMeta(path);
  app.innerHTML = pages[path]();
  document.querySelector('.steps pre')?.setAttribute('tabindex', '0');
  const heading = document.querySelector('h1');
  note.textContent = heading.textContent;
  if (focusHeading) {
    heading.tabIndex = -1;
    heading.focus({ preventScroll: true });
  }
  if (path === '/demo') {
    document.querySelector('#reset-demo').addEventListener('click', resetDemo);
    document.querySelector('#replay-demo').addEventListener('click', replayDemo);
  }
}

document.addEventListener('click', event => {
  const link = event.target.closest('a[data-route]');
  if (!link || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;
  event.preventDefault();
  const url = new URL(link.href);
  history.pushState({}, '', `${url.pathname}${url.search}${url.hash}`);
  render({ focusHeading: true });
  if (url.hash) document.querySelector(url.hash)?.scrollIntoView();
  else scrollTo(0, 0);
});

addEventListener('popstate', () => render({ focusHeading: true }));
render();

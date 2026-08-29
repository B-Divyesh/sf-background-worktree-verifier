import test, { after } from 'node:test';
import assert from 'node:assert/strict';
import { mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { spawn, spawnSync } from 'node:child_process';
import { createServer } from 'node:net';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { chromium } from 'playwright';
import AxeBuilder from '@axe-core/playwright';

const source = await readFile(new URL('../src/main.js', import.meta.url), 'utf8');
const html = await readFile(new URL('../src/index.html', import.meta.url), 'utf8');
const html404 = await readFile(new URL('../public/404.html', import.meta.url), 'utf8');
const swa = await readFile(new URL('../public/staticwebapp.config.json', import.meta.url), 'utf8');
const packageJson = JSON.parse(await readFile(new URL('../../package.json', import.meta.url), 'utf8'));
const vite = fileURLToPath(new URL('../../node_modules/vite/bin/vite.js', import.meta.url));
const server = spawn(process.execPath, [vite, 'preview', '--config', 'site/vite.config.js', '--host', '127.0.0.1', '--port', '4173'], { stdio: 'ignore' });
const base = 'http://127.0.0.1:4173';

async function waitForSite() {
  for (let attempt = 0; attempt < 50; attempt += 1) {
    try { if ((await fetch(base)).ok) return; } catch {}
    await new Promise(resolve => setTimeout(resolve, 100));
  }
  throw new Error('Vite preview did not start');
}

async function freePort() {
  return new Promise((resolve, reject) => {
    const socket = createServer();
    socket.once('error', reject);
    socket.listen(0, '127.0.0.1', () => {
      const { port } = socket.address();
      socket.close(error => error ? reject(error) : resolve(port));
    });
  });
}

function git(directory, ...args) {
  const result = spawnSync('git', args, { cwd: directory, encoding: 'utf8' });
  assert.equal(result.status, 0, `git ${args.join(' ')} failed: ${result.stderr}`);
}
await waitForSite();
after(() => server.kill());

test('site source keeps local routing, document metadata, and a real static 404 shell', () => {
  assert.match(html, /<html lang="en">/);
  assert.match(html, /<a class="skip" href="#main">/);
  assert.match(html, /<meta name="description"/);
  assert.match(html, /<link rel="canonical"/);
  assert.match(html, /id="app"/);
  assert.ok(!source.includes('fetch('));
  assert.ok(!source.includes('localStorage'));
  assert.ok(!source.includes('sessionStorage'));
  assert.match(source, /heading\.tabIndex = -1/);
  assert.match(source, /heading\.focus/);
  assert.match(html404, /<a class="skip" href="#main">/);
  assert.match(html404, /<nav aria-label="Primary">/);
  assert.match(html404, /<footer>/);
  assert.match(html404, /Privacy/);
  assert.match(html404, /Terms/);
  assert.match(html404, /property="og:title"/);
  assert.match(html404, /name="twitter:title"/);
});

test('known SPA routes are explicit so unknown paths receive a 404 response', () => {
  assert.doesNotMatch(swa, /navigationFallback/);
  for (const route of ['/demo', '/privacy', '/terms']) {
    assert.match(swa, new RegExp(`"route": "${route}", "rewrite": "${route}/index.html"`));
  }
  assert.match(swa, /"404": \{ "rewrite": "\/404.html" \}/);
});

test('@claim:demo-browser-sandbox', { timeout: 60000 }, async () => {
  const browser = await chromium.launch({ headless: true });
  try {
    const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
    const page = await context.newPage();
    await page.goto(base, { waitUntil: 'networkidle' });
    await page.getByRole('link', { name: 'Try it with sample data' }).click();
    await page.waitForURL(`${base}/?demo=1`);
    assert.equal(await page.getByRole('heading', { level: 1 }).textContent(), 'See three worktrees pass');
    assert.equal(await page.locator('.demo-banner').isVisible(), true);
    const initialOutput = await page.locator('#demo-output').textContent();
    await page.getByRole('button', { name: 'Replay sample' }).click();
    await page.waitForFunction(expected => document.querySelector('#demo-output').textContent !== expected, initialOutput);
    await page.evaluate(() => scrollTo(0, document.documentElement.scrollHeight));
    await page.waitForFunction(() => scrollY >= document.documentElement.scrollHeight - innerHeight - 2);
    const bannerBox = await page.locator('.demo-banner').boundingBox();
    assert.ok(bannerBox && bannerBox.y >= 0 && bannerBox.y + bannerBox.height <= 844, `banner left viewport: ${JSON.stringify(bannerBox)}`);
    for (const name of ['Reset demo', 'Start for real']) {
      const control = page.getByRole(name === 'Reset demo' ? 'button' : 'link', { name });
      const box = await control.boundingBox();
      assert.ok(box && box.y >= 0 && box.y + box.height <= 844, `${name} left viewport: ${JSON.stringify(box)}`);
    }
    await page.getByRole('button', { name: 'Reset demo' }).click();
    await page.waitForFunction(() => scrollY === 0 && document.activeElement?.tagName === 'H1');
    assert.equal(await page.locator('#demo-output').textContent(), initialOutput);
    assert.equal(await page.getByRole('button', { name: 'Replay sample' }).isEnabled(), true);
    const storage = await page.evaluate(async () => ({
      local: localStorage.length,
      session: sessionStorage.length,
      indexed: 'databases' in indexedDB ? (await indexedDB.databases()).length : 0,
      workers: 'serviceWorker' in navigator ? (await navigator.serviceWorker.getRegistrations()).length : 0,
    }));
    assert.deepEqual(storage, { local: 0, session: 0, indexed: 0, workers: 0 });
    await page.getByRole('link', { name: 'Start for real' }).click();
    await page.waitForURL(`${base}/#setup`);
    assert.equal(await page.locator('.demo-banner').count(), 0);
    await context.close();
  } finally {
    await browser.close();
  }
});

test('@claim:static-no-analytics', { timeout: 60000 }, async () => {
  const browser = await chromium.launch({ headless: true });
  try {
    const context = await browser.newContext();
    const page = await context.newPage();
    const requests = [];
    page.on('request', request => requests.push(new URL(request.url()).hostname));
    for (const route of ['/', '/?demo=1', '/demo', '/privacy', '/terms']) {
      await page.goto(`${base}${route}`, { waitUntil: 'networkidle' });
    }
    assert.ok(requests.every(host => host === '127.0.0.1'), `unexpected requests: ${requests.join(', ')}`);
    await context.close();
  } finally {
    await browser.close();
  }
});

test('@claim:static-build-artifact', async () => {
  assert.equal(packageJson.scripts.build, 'npm run build:site');
  const expected = [
    ['index.html', 'Worktree Verifier — Check changed worktrees', 'https://background-worktree-verifier.sociobot.in/'],
    ['demo/index.html', 'Demo — Worktree Verifier', 'https://background-worktree-verifier.sociobot.in/demo'],
    ['privacy/index.html', 'Privacy — Worktree Verifier', 'https://background-worktree-verifier.sociobot.in/privacy'],
    ['terms/index.html', 'Terms — Worktree Verifier', 'https://background-worktree-verifier.sociobot.in/terms'],
  ];
  for (const [file, title, canonical] of expected) {
    const built = await readFile(new URL(`../../dist/site/${file}`, import.meta.url), 'utf8');
    assert.match(built, new RegExp(`<title>${title}</title>`));
    assert.match(built, /<meta name="description" content="[^"]+"/);
    assert.match(built, new RegExp(`<link rel="canonical" href="${canonical.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}"`));
    assert.match(built, /property="og:title"/);
    assert.match(built, /property="og:description"/);
    assert.match(built, /name="twitter:title"/);
    assert.match(built, /name="twitter:description"/);
  }
});

test('browser desktop and 390px mobile have no serious axe findings, route focus, or undersized controls', { timeout: 60000 }, async () => {
  const browser = await chromium.launch({ headless: true });
  try {
    for (const viewport of [{ width: 1440, height: 900 }, { width: 390, height: 844 }]) {
      const context = await browser.newContext({ viewport });
      const page = await context.newPage();
      const errors = [];
      page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
      page.on('pageerror', error => errors.push(error.message));
      page.on('requestfailed', request => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
      await page.goto(base, { waitUntil: 'networkidle' });
      assert.equal(await page.title(), 'Worktree Verifier — Check changed worktrees');
      await page.keyboard.press('Tab');
      assert.equal(await page.evaluate(() => document.activeElement?.classList.contains('skip')), true);
      const landing = await new AxeBuilder({ page }).analyze();
      assert.deepEqual(landing.violations.filter(v => ['serious', 'critical'].includes(v.impact)), []);
      if (viewport.width === 390) {
        const firstScreen = await page.locator('.facts').boundingBox();
        assert.ok(firstScreen && firstScreen.y + firstScreen.height <= viewport.height, `first-screen facts are below the fold: ${JSON.stringify(firstScreen)}`);
      }

      await page.locator('a[href="/?demo=1"]').first().click();
      await page.waitForURL(`${base}/?demo=1`);
      assert.equal(await page.title(), 'Demo — Worktree Verifier');
      assert.equal(await page.locator('link[rel="canonical"]').getAttribute('href'), 'https://background-worktree-verifier.sociobot.in/?demo=1');
      assert.equal(await page.evaluate(() => document.activeElement?.tagName), 'H1');
      const demo = await new AxeBuilder({ page }).analyze();
      assert.deepEqual(demo.violations.filter(v => ['serious', 'critical'].includes(v.impact)), []);
      if (viewport.width === 390) {
        for (const selector of ['.wordmark', '.site-header nav a', '#reset-demo', '.demo-banner a', '.button', 'footer a']) {
          const boxes = await page.locator(selector).evaluateAll(nodes => nodes.map(node => {
            const box = node.getBoundingClientRect();
            return { width: box.width, height: box.height };
          }));
          assert.ok(boxes.every(box => box.width >= 44 && box.height >= 44), `${selector} has a sub-44px target: ${JSON.stringify(boxes)}`);
        }
        const navBoxes = await page.locator('.site-header nav a').evaluateAll(nodes => nodes.map(node => {
          const box = node.getBoundingClientRect();
          return { label: node.textContent, left: box.left, right: box.right };
        }));
        for (let index = 1; index < navBoxes.length; index += 1) {
          const gap = navBoxes[index].left - navBoxes[index - 1].right;
          assert.ok(gap >= 8, `${navBoxes[index - 1].label} and ${navBoxes[index].label} are only ${gap}px apart`);
        }
      }
      await page.goBack({ waitUntil: 'networkidle' });
      assert.equal(await page.evaluate(() => document.activeElement?.tagName), 'H1');
      for (const [path, title] of [
        ['/privacy', 'Privacy — Worktree Verifier'],
        ['/terms', 'Terms — Worktree Verifier'],
        ['/404.html', 'Page not found — Worktree Verifier'],
      ]) {
        await page.goto(`${base}${path}`, { waitUntil: 'networkidle' });
        assert.equal(await page.title(), title);
        const scan = await new AxeBuilder({ page }).analyze();
        assert.deepEqual(scan.violations.filter(v => ['serious', 'critical'].includes(v.impact)), []);
        assert.equal(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), true);
      }
      assert.equal(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), true);
      assert.deepEqual(errors, []);
      await context.close();
    }
  } finally {
    await browser.close();
  }
});

test('real CLI status board passes accessibility checks on desktop and 390px mobile', { timeout: 60000 }, async () => {
  const root = await mkdtemp(join(tmpdir(), 'wtv-board-a11y-'));
  const repo = join(root, 'checkout');
  const port = await freePort();
  let watcher;
  let browser;
  try {
    await mkdir(repo);
    git(repo, 'init', '-q');
    git(repo, 'config', 'user.email', 'test@worktree-verifier.local');
    git(repo, 'config', 'user.name', 'Worktree Verifier test');
    await writeFile(join(repo, 'source.txt'), 'board accessibility\n');
    git(repo, 'add', 'source.txt');
    git(repo, 'commit', '-qm', 'Seed board accessibility test');
    const config = join(root, 'worktree-verifier.toml');
    await writeFile(config, `command_timeout_seconds = 2\n\n[server]\naddress = "127.0.0.1:${port}"\npoll_seconds = 1\n\n[[worktrees]]\nname = "idle-check"\npath = ${JSON.stringify(repo)}\nchecks = []\n\n[[worktrees]]\nname = "stale-check"\npath = ${JSON.stringify(repo)}\nchecks = ["printf x >> source.txt"]\n`);
    const binary = fileURLToPath(new URL('../../target/debug/worktree-verifier', import.meta.url));
    watcher = spawn(binary, ['run', '--config', config], { stdio: 'ignore' });
    const board = `http://127.0.0.1:${port}`;
    for (let attempt = 0; attempt < 50; attempt += 1) {
      try {
        const response = await fetch(`${board}/status.json`);
        if (response.ok) {
          const rows = await response.json();
          if (rows[0]?.status === 'idle' && rows[1]?.status === 'stale') break;
        }
      } catch {}
      assert.ok(attempt < 49, 'real status board did not reach IDLE');
      await new Promise(resolve => setTimeout(resolve, 100));
    }

    browser = await chromium.launch({ headless: true });
    for (const viewport of [{ width: 1440, height: 900 }, { width: 390, height: 844 }]) {
      const context = await browser.newContext({ viewport });
      const page = await context.newPage();
      const errors = [];
      page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
      page.on('pageerror', error => errors.push(error.message));
      await page.goto(board, { waitUntil: 'networkidle' });
      assert.equal(await page.locator('html').getAttribute('lang'), 'en');
      assert.equal(await page.locator('h1').count(), 1);
      assert.equal(await page.locator('main').count(), 1);
      assert.equal(await page.locator('.idle').textContent(), 'IDLE');
      assert.equal(await page.locator('.stale').textContent(), 'STALE');
      assert.equal(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), true);
      const scan = await new AxeBuilder({ page }).analyze();
      assert.deepEqual(scan.violations.filter(v => ['serious', 'critical'].includes(v.impact)), []);
      assert.deepEqual(errors, []);
      await context.close();
    }
  } finally {
    if (browser) await browser.close();
    if (watcher) {
      if (watcher.exitCode === null) {
        watcher.kill();
        await new Promise(resolve => watcher.once('exit', resolve));
      }
    }
    await rm(root, { recursive: true, force: true });
  }
});

test('reduced motion removes transitions and animation', { timeout: 60000 }, async () => {
  const browser = await chromium.launch({ headless: true });
  try {
    const context = await browser.newContext({ reducedMotion: 'reduce' });
    const page = await context.newPage();
    await page.goto(base, { waitUntil: 'networkidle' });
    assert.deepEqual(await page.evaluate(() => [...document.querySelectorAll('*')].every(node => {
      const style = getComputedStyle(node);
      return style.transitionDuration === '0s' && style.animationDuration === '0s';
    })), true);
    await page.goto(`${base}/?demo=1`, { waitUntil: 'networkidle' });
    await page.getByRole('button', { name: 'Replay sample' }).click();
    assert.equal(await page.getByRole('button', { name: 'Replay sample' }).isEnabled(), true);
    assert.match(await page.locator('#demo-output').textContent(), /Removed sample worktrees\.$/);
    await context.close();
  } finally {
    await browser.close();
  }
});

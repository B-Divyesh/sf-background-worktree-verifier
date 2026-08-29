import test, { after } from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { chromium } from 'playwright';
import AxeBuilder from '@axe-core/playwright';

const source = await readFile(new URL('../src/main.js', import.meta.url), 'utf8');
const html = await readFile(new URL('../src/index.html', import.meta.url), 'utf8');
const html404 = await readFile(new URL('../public/404.html', import.meta.url), 'utf8');
const swa = await readFile(new URL('../public/staticwebapp.config.json', import.meta.url), 'utf8');
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
});

test('known SPA routes are explicit so unknown paths receive a 404 response', () => {
  assert.doesNotMatch(swa, /navigationFallback/);
  for (const route of ['/demo', '/privacy', '/terms']) assert.match(swa, new RegExp(`"route": "${route}"`));
  assert.match(swa, /"404": \{ "rewrite": "\/404.html" \}/);
});

test('@claim:demo-browser-sandbox', { timeout: 60000 }, async () => {
  const browser = await chromium.launch({ headless: true });
  try {
    const context = await browser.newContext();
    const page = await context.newPage();
    await page.goto(`${base}/demo`, { waitUntil: 'networkidle' });
    const storage = await page.evaluate(async () => ({
      local: localStorage.length,
      session: sessionStorage.length,
      indexed: 'databases' in indexedDB ? (await indexedDB.databases()).length : 0,
      workers: 'serviceWorker' in navigator ? (await navigator.serviceWorker.getRegistrations()).length : 0,
    }));
    assert.deepEqual(storage, { local: 0, session: 0, indexed: 0, workers: 0 });
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
    await page.goto(base, { waitUntil: 'networkidle' });
    await page.goto(`${base}/demo`, { waitUntil: 'networkidle' });
    assert.ok(requests.every(host => host === '127.0.0.1'), `unexpected requests: ${requests.join(', ')}`);
    await context.close();
  } finally {
    await browser.close();
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
      await page.goto(base, { waitUntil: 'networkidle' });
      await page.keyboard.press('Tab');
      assert.equal(await page.evaluate(() => document.activeElement?.classList.contains('skip')), true);
      const landing = await new AxeBuilder({ page }).analyze();
      assert.deepEqual(landing.violations.filter(v => ['serious', 'critical'].includes(v.impact)), []);

      await page.locator('a[href="/demo"]').first().click();
      await page.waitForURL(`${base}/demo`);
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
      }
      await page.goBack({ waitUntil: 'networkidle' });
      assert.equal(await page.evaluate(() => document.activeElement?.tagName), 'H1');
      for (const path of ['/privacy', '/terms', '/404.html']) {
        await page.goto(`${base}${path}`, { waitUntil: 'networkidle' });
        const scan = await new AxeBuilder({ page }).analyze();
        assert.deepEqual(scan.violations.filter(v => ['serious', 'critical'].includes(v.impact)), []);
      }
      assert.equal(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), true);
      assert.deepEqual(errors, []);
      await context.close();
    }
  } finally {
    await browser.close();
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
    await context.close();
  } finally {
    await browser.close();
  }
});

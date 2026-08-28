import test, { after } from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { chromium } from 'playwright';
import AxeBuilder from '@axe-core/playwright';

const source = await readFile(new URL('../src/main.js', import.meta.url), 'utf8');
const html = await readFile(new URL('../src/index.html', import.meta.url), 'utf8');
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

test('@claim:demo-recording-shows-isolated-sample', () => {
  assert.match(source, /Creates three separate temporary Git worktrees/);
  assert.match(source, /temporary Git worktrees with separate commits/);
  assert.doesNotMatch(source, /no commit/);
  assert.match(source, /Demo — sample data, nothing is saved/);
});
test('site has language, a main target, and no runtime network code', () => {
  assert.match(html, /<html lang="en">/);
  assert.match(html, /id="app"/);
  assert.ok(!source.includes('fetch('));
});
test('known SPA routes are explicit so unknown paths receive a 404 response', () => {
  assert.doesNotMatch(swa, /navigationFallback/);
  for (const route of ['/demo', '/privacy', '/terms']) assert.match(swa, new RegExp(`"route": "${route}"`));
  assert.match(swa, /"404": \{ "rewrite": "\/404.html" \}/);
});
test('scrollable regions are keyboard focus targets', () => {
  assert.match(source, /class="status-table"[^>]*tabindex="0"/);
  assert.match(source, /id="demo-output" tabindex="0"/);
});
test('browser desktop and 390px mobile have no serious axe findings or third-party requests', { timeout: 60000 }, async () => {
  const browser = await chromium.launch({ headless: true });
  try {
    for (const viewport of [{ width: 1440, height: 900 }, { width: 390, height: 844 }]) {
      const context = await browser.newContext({ viewport });
      const page = await context.newPage();
      const requests = [];
      page.on('request', request => requests.push(new URL(request.url()).hostname));
      await page.goto(base, { waitUntil: 'networkidle' });
      await page.keyboard.press('Tab');
      await expectFocusedSkipLink(page);
      const landing = await new AxeBuilder({ page }).analyze();
      assert.deepEqual(landing.violations.filter(v => ['serious', 'critical'].includes(v.impact)), []);
      await page.goto(`${base}/demo`, { waitUntil: 'networkidle' });
      const demo = await new AxeBuilder({ page }).analyze();
      assert.deepEqual(demo.violations.filter(v => ['serious', 'critical'].includes(v.impact)), []);
      for (const path of ['/privacy', '/terms']) {
        await page.goto(`${base}${path}`, { waitUntil: 'networkidle' });
        const legal = await new AxeBuilder({ page }).analyze();
        assert.deepEqual(legal.violations.filter(v => ['serious', 'critical'].includes(v.impact)), []);
      }
      assert.ok(requests.every(host => host === '127.0.0.1'), `unexpected requests: ${requests.join(', ')}`);
      assert.equal(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), true);
      await context.close();
    }
  } finally {
    await browser.close();
  }
});

async function expectFocusedSkipLink(page) {
  assert.equal(await page.evaluate(() => document.activeElement?.classList.contains('skip')), true);
}

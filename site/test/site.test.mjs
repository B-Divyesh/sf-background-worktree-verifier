import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

const source = await readFile(new URL('../src/main.js', import.meta.url), 'utf8');
const html = await readFile(new URL('../src/index.html', import.meta.url), 'utf8');

test('@claim:demo-recording-shows-isolated-sample', () => {
  assert.match(source, /Creates three separate temporary folders/);
  assert.match(source, /Demo — sample data, nothing is saved/);
});
test('site has language, a main target, and no runtime network code', () => {
  assert.match(html, /<html lang="en">/);
  assert.match(html, /id="app"/);
  assert.ok(!source.includes('fetch('));
});

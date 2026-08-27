import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

const html = readFileSync(new URL('../site/index.html', import.meta.url), 'utf8');
const css = readFileSync(new URL('../site/src/style.css', import.meta.url), 'utf8');

test('landing page has the required accessibility landmarks', () => {
  assert.match(html, /<html lang="en">/);
  assert.equal((html.match(/<h1[ >]/g) ?? []).length, 1);
  assert.match(html, /<main id="main">/);
  assert.match(html, /class="skip-link"/);
  assert.match(html, /<img[^>]+alt="[^"]+"/);
  assert.match(css, /:focus-visible/);
  assert.match(css, /prefers-reduced-motion:reduce/);
});

test('landing page does not load third-party runtime assets', () => {
  assert.doesNotMatch(html, /(?:src|href)="https:\/\/(?!github\.com)/);
  assert.doesNotMatch(html, /google-analytics|googletagmanager|fonts\.googleapis/i);
});

test('hero declares dimensions and high fetch priority', () => {
  assert.match(html, /cassette-handoff\.webp[^>]+width="1200"[^>]+height="800"[^>]+fetchpriority="high"/);
});

import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';
import { writeServiceWorker } from '../scripts/service-worker.mjs';

const html = readFileSync(new URL('../site/index.html', import.meta.url), 'utf8');
const css = readFileSync(new URL('../site/src/style.css', import.meta.url), 'utf8');
const staticWebAppConfig = JSON.parse(readFileSync(new URL('../site/public/staticwebapp.config.json', import.meta.url), 'utf8'));
const notFoundHtml = readFileSync(new URL('../site/public/404.html', import.meta.url), 'utf8');
const notFoundCss = readFileSync(new URL('../site/public/not-found.css', import.meta.url), 'utf8');

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

test('Azure Static Web Apps response policy protects every response, including controlled 404s', () => {
  assert.equal(staticWebAppConfig.globalHeaders['Content-Security-Policy'], "default-src 'self'; img-src 'self' data:; script-src 'self'; style-src 'self'; object-src 'none'; base-uri 'self'; form-action 'self'; frame-ancestors 'none'");
  assert.equal(staticWebAppConfig.globalHeaders['Permissions-Policy'], 'camera=(), microphone=(), geolocation=()');
  assert.equal(staticWebAppConfig.globalHeaders['Cache-Control'], 'public, max-age=0, must-revalidate');
  assert.equal(staticWebAppConfig.globalHeaders['X-Content-Type-Options'], 'nosniff');
  assert.equal(staticWebAppConfig.globalHeaders['Referrer-Policy'], 'strict-origin-when-cross-origin');
  const headersFor = (route) => staticWebAppConfig.routes.find((entry) => entry.route === route)?.headers;
  assert.equal(headersFor('/assets/*')['Cache-Control'], 'public, max-age=31536000, immutable');
  assert.equal(headersFor('/cassette-handoff.webp')['Cache-Control'], 'public, max-age=31536000, immutable');
  assert.equal(headersFor('/sw.js')['Cache-Control'], 'no-cache');
  assert.deepEqual(staticWebAppConfig.responseOverrides['404'], {
    rewrite: '/404.html',
    statusCode: 404,
  });
  assert.match(notFoundHtml, /<html lang="en">/);
  assert.match(notFoundHtml, /<main>/);
  assert.equal((notFoundHtml.match(/<h1[ >]/g) ?? []).length, 1);
  assert.match(notFoundHtml, /href="\/not-found\.css"/);
  assert.match(notFoundCss, /:focus-visible/);
  assert.match(notFoundCss, /min-height:44px/);
});

test('release service worker fingerprints the shell and refreshes it before taking control', async () => {
  const output = mkdtempSync(join(tmpdir(), 'khb-service-worker-'));
  const template = new URL('../site/sw.template.js', import.meta.url);
  try {
    for (const file of ['index.html', 'privacy/index.html', 'terms/index.html', 'cassette-handoff.webp']) {
      const destination = join(output, file);
      const parent = destination.slice(0, destination.lastIndexOf('/'));
      mkdirSync(parent, { recursive: true });
      writeFileSync(destination, `first release ${file}`);
    }
    const firstVersion = await writeServiceWorker(output, template);
    const firstWorker = readFileSync(join(output, 'sw.js'), 'utf8');
    writeFileSync(join(output, 'index.html'), 'second release');
    const secondVersion = await writeServiceWorker(output, template);
    const secondWorker = readFileSync(join(output, 'sw.js'), 'utf8');
    assert.notEqual(firstVersion, secondVersion);
    assert.match(firstWorker, new RegExp(`khb-site-${firstVersion}`));
    assert.match(secondWorker, new RegExp(`khb-site-${secondVersion}`));
    assert.match(secondWorker, /new Request\(url, \{ cache: 'reload' \}\)/);
    assert.match(secondWorker, /key\.startsWith\('khb-site-'\)/);
    assert.match(secondWorker, /event\.request\.mode === 'navigate'/);
  } finally {
    rmSync(output, { recursive: true, force: true });
  }
});

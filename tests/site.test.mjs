import assert from 'node:assert/strict';
import { createServer } from 'node:http';
import { readFileSync, statSync } from 'node:fs';
import { normalize, join, relative } from 'node:path';
import { spawnSync } from 'node:child_process';
import test, { after } from 'node:test';
import { chromium } from 'playwright';
import { writeServiceWorker } from '../scripts/service-worker.mjs';
import { mkdtempSync, mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';

const root = new URL('..', import.meta.url).pathname;
const build = spawnSync('npm', ['run', 'build'], { cwd: root, encoding: 'utf8' });
assert.equal(build.status, 0, build.stderr || build.stdout);
const directory = join(root, 'dist/site');
const contentTypes = { '.css': 'text/css', '.js': 'text/javascript', '.json': 'application/json', '.png': 'image/png', '.webp': 'image/webp', '.xml': 'application/xml', '.txt': 'text/plain' };
const server = createServer((request, response) => {
  const pathname = new URL(request.url, 'http://localhost').pathname;
  const requested = pathname === '/' ? 'index.html' : pathname.replace(/^\/+/, '');
  let target = normalize(join(directory, requested));
  try {
    if (relative(directory, target).startsWith('..')) throw new Error('outside output');
    if (statSync(target).isDirectory()) target = join(target, 'index.html');
    response.writeHead(200, { 'Content-Type': contentTypes[target.slice(target.lastIndexOf('.'))] ?? 'text/html' });
    response.end(readFileSync(target));
  } catch {
    response.writeHead(404, { 'Content-Type': 'text/html' });
    response.end(readFileSync(join(directory, '404.html')));
  }
});
await new Promise((resolve) => server.listen(0, '127.0.0.1', resolve));
const base = `http://127.0.0.1:${server.address().port}`;

test('built routes provide usable metadata, landmarks, and mobile touch controls', { concurrency: false }, async () => {
  const browser = await chromium.launch();
  try {
    const expectedTitles = new Map([
      ['/', 'Knowledge Handoff Bundle — Build project handoffs'],
      ['/demo/', 'Demo — Knowledge Handoff Bundle'],
      ['/privacy/', 'Privacy — Knowledge Handoff Bundle'],
      ['/terms/', 'Terms — Knowledge Handoff Bundle'],
    ]);
    for (const [route, title] of expectedTitles) {
      const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
      const page = await context.newPage();
      const messages = [];
      page.on('console', (message) => { if (message.type() === 'error') messages.push(message.text()); });
      await page.goto(`${base}${route}`, { waitUntil: 'networkidle' });
      assert.equal(await page.title(), title);
      assert.equal(await page.locator('html').getAttribute('lang'), 'en');
      assert.equal(await page.locator('main').count(), 1);
      assert.equal(await page.locator('h1').count(), 1);
      assert.ok(await page.locator('link[rel="canonical"]').count());
      assert.ok(await page.locator('meta[property="og:image"]').count());
      await page.keyboard.press('Tab');
      assert.match(await page.locator(':focus').textContent(), /Skip to/);
      await page.keyboard.press('Enter');
      assert.equal(await page.locator('main').evaluate((element) => element === document.activeElement), true);
      const targets = await page.locator('a:visible, button:visible').evaluateAll((nodes) => nodes.map((node) => {
        const box = node.getBoundingClientRect();
        return { label: node.textContent?.trim(), width: box.width, height: box.height };
      }));
      for (const target of targets) {
        assert.ok(target.height >= 44, `${route} ${target.label} height was ${target.height}`);
        assert.ok(target.width >= 44, `${route} ${target.label} width was ${target.width}`);
      }
      assert.deepEqual(messages, []);
      await context.close();
    }
  } finally { await browser.close(); }
});

test('the built 404 route keeps the site skeleton and a route home', async () => {
  const response = await fetch(`${base}/missing-route`);
  assert.equal(response.status, 404);
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage();
    await page.goto(`${base}/missing-route`);
    assert.equal(await page.title(), 'Page not found — Knowledge Handoff Bundle');
    assert.equal(await page.locator('header nav').count(), 1);
    assert.equal(await page.locator('footer').count(), 1);
    assert.equal(await page.getByRole('link', { name: 'Go to home' }).count(), 1);
  } finally { await browser.close(); }
});

test('the desktop demo exposes one banner landmark', async () => {
  const browser = await chromium.launch();
  try {
    const context = await browser.newContext({ viewport: { width: 1366, height: 900 } });
    const page = await context.newPage();
    await page.goto(`${base}/demo/`, { waitUntil: 'networkidle' });
    assert.equal(await page.getByRole('banner').count(), 1);
    await context.close();
  } finally { await browser.close(); }
});

test('a shell change produces a new service-worker cache identity', async () => {
  const output = mkdtempSync(join(tmpdir(), 'khb-site-worker-'));
  const template = new URL('../site/sw.template.js', import.meta.url);
  try {
    for (const file of ['index.html', 'privacy/index.html', 'terms/index.html', 'cassette-handoff.webp']) {
      const destination = join(output, file);
      mkdirSync(destination.slice(0, destination.lastIndexOf('/')), { recursive: true });
      writeFileSync(destination, `first ${file}`);
    }
    const first = await writeServiceWorker(output, template);
    writeFileSync(join(output, 'index.html'), 'second release');
    const second = await writeServiceWorker(output, template);
    assert.notEqual(first, second);
  } finally { rmSync(output, { recursive: true, force: true }); }
});

after(async () => { await new Promise((resolve) => server.close(resolve)); });

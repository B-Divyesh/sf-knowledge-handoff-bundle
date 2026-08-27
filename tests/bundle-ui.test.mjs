import assert from 'node:assert/strict';
import { createServer } from 'node:http';
import { mkdtempSync, readFileSync, rmSync, statSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, normalize, relative } from 'node:path';
import { spawnSync } from 'node:child_process';
import test from 'node:test';
import { chromium } from 'playwright';

const repositoryRoot = new URL('..', import.meta.url).pathname;

function serve(directory) {
  const server = createServer((request, response) => {
    const pathname = new URL(request.url, 'http://localhost').pathname;
    const relativePath = pathname === '/' ? 'index.html' : pathname.replace(/^\/+/, '');
    const target = normalize(join(directory, relativePath));
    if (relative(directory, target).startsWith('..') || !statSync(target, { throwIfNoEntry: false })?.isFile()) {
      response.writeHead(404).end();
      return;
    }
    const contentType = target.endsWith('.css') ? 'text/css' : target.endsWith('.js') ? 'text/javascript' : 'text/html';
    response.writeHead(200, { 'Content-Type': contentType });
    response.end(readFileSync(target));
  });
  return new Promise((resolve) => server.listen(0, '127.0.0.1', () => resolve(server)));
}

test('generated artifact actions are at least 44 CSS pixels tall at 390px and desktop widths', { timeout: 30_000 }, async () => {
  const output = mkdtempSync(join(tmpdir(), 'khb-bundle-ui-'));
  const build = spawnSync(
    join(repositoryRoot, 'target/debug/khb'),
    ['build', 'examples/atlas/handoff.yaml', '--output', output],
    { cwd: repositoryRoot, encoding: 'utf8' },
  );
  assert.equal(build.status, 0, build.stderr || build.stdout);

  const server = await serve(output);
  const port = server.address().port;
  const browser = await chromium.launch();
  try {
    for (const viewport of [{ width: 390, height: 844 }, { width: 1366, height: 900 }]) {
      const context = await browser.newContext({ viewport });
      const page = await context.newPage();
      await page.goto(`http://127.0.0.1:${port}/`, { waitUntil: 'networkidle' });
      const links = await page.locator('.artifact-link').evaluateAll((nodes) => nodes.map((node) => {
        const box = node.getBoundingClientRect();
        return { label: node.textContent?.trim(), height: box.height, width: box.width };
      }));
      assert.equal(links.length, 3);
      for (const link of links) {
        assert.ok(link.height >= 44, `${viewport.width}px ${link.label} height was ${link.height}px`);
        assert.ok(link.width >= 44, `${viewport.width}px ${link.label} width was ${link.width}px`);
      }
      await context.close();
    }
  } finally {
    await browser.close();
    await new Promise((resolve) => server.close(resolve));
    rmSync(output, { recursive: true, force: true });
  }
});

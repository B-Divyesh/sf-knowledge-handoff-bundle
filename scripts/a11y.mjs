import AxeBuilder from '@axe-core/playwright';
import { createServer } from 'node:http';
import { readFile, stat } from 'node:fs/promises';
import { extname, join, normalize, relative } from 'node:path';
import { chromium } from 'playwright';

const routes = ['/', '/demo/', '/privacy/', '/terms/', '/404.html', '/does-not-exist'];
const contentTypes = { '.css': 'text/css', '.js': 'text/javascript', '.json': 'application/json', '.png': 'image/png', '.webp': 'image/webp', '.xml': 'application/xml', '.txt': 'text/plain' };

function staticServer(directory) {
  return createServer(async (request, response) => {
    const pathname = new URL(request.url, 'http://localhost').pathname;
    const requested = pathname === '/' ? 'index.html' : pathname.replace(/^\/+/, '');
    let target = normalize(join(directory, requested));
    if (!relative(directory, target).startsWith('..')) {
      try {
        if ((await stat(target)).isDirectory()) target = join(target, 'index.html');
        const body = await readFile(target);
        response.writeHead(200, { 'Content-Type': contentTypes[extname(target)] ?? 'text/html' });
        response.end(body);
        return;
      } catch {}
    }
    response.writeHead(404, { 'Content-Type': 'text/html' });
    response.end(await readFile(join(directory, '404.html')));
  });
}

let server;
let baseUrl = process.argv[2];
if (!baseUrl) {
  server = staticServer(new URL('../dist/site/', import.meta.url).pathname);
  await new Promise((resolve) => server.listen(0, '127.0.0.1', resolve));
  baseUrl = `http://127.0.0.1:${server.address().port}`;
}

const browser = await chromium.launch();
let failed = false;
try {
  for (const route of routes) {
    const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
    const page = await context.newPage();
    await page.goto(new URL(route, baseUrl).href, { waitUntil: 'networkidle' });
    const results = await new AxeBuilder({ page }).analyze();
    const serious = results.violations.filter(({ impact }) => impact === 'serious' || impact === 'critical');
    console.log(`${route}: ${results.violations.length} total, ${serious.length} serious/critical`);
    for (const violation of results.violations) console.log(`  ${violation.id}: ${violation.help} (${violation.nodes.map((node) => node.target.join(' ')).join(', ')})`);
    for (const violation of serious) console.error(`  ${violation.id}: ${violation.help}`);
    failed ||= serious.length > 0;
    await context.close();
  }
} finally {
  await browser.close();
  if (server) await new Promise((resolve) => server.close(resolve));
}
process.exitCode = failed ? 1 : 0;

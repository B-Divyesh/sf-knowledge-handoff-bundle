import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { createServer } from 'node:http';
import { mkdtempSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, normalize, relative } from 'node:path';
import { spawn, spawnSync } from 'node:child_process';
import test, { after } from 'node:test';
import { chromium } from 'playwright';

const root = new URL('..', import.meta.url).pathname;
const binary = join(root, 'target/debug/khb');

function temp(prefix) { return mkdtempSync(join(tmpdir(), `${prefix}-`)); }
function run(args, options = {}) {
  return spawnSync(binary, args, { cwd: root, encoding: 'utf8', ...options });
}
function runAsync(args, options = {}) {
  return new Promise((resolve, reject) => {
    const child = spawn(binary, args, { cwd: root, ...options });
    let stdout = '';
    let stderr = '';
    child.stdout.on('data', (chunk) => { stdout += chunk; });
    child.stderr.on('data', (chunk) => { stderr += chunk; });
    child.on('error', reject);
    child.on('close', (status) => resolve({ status, stdout, stderr }));
  });
}
function writeSimpleHandoff(directory, artifacts, extra = '') {
  writeFileSync(join(directory, 'handoff.yaml'), `project:\n  title: Claim test\n  summary: A portable project handoff.\n  owner: { name: Priya }\n  prepared_at: 2026-09-05\n${extra}sections:\n  - title: Sources\n    artifacts:\n${artifacts.map((artifact) => `      - ${artifact}`).join('\n')}\ngaps: []\n`);
  return join(directory, 'handoff.yaml');
}
function serve(handler) {
  const server = createServer(handler);
  return new Promise((resolve) => server.listen(0, '127.0.0.1', () => resolve(server)));
}
function close(server) { return new Promise((resolve) => server.close(resolve)); }
function staticServer(directory) {
  return serve(async (request, response) => {
    const pathname = new URL(request.url, 'http://localhost').pathname;
    const requested = pathname === '/' ? 'index.html' : pathname.replace(/^\/+/, '');
    let target = normalize(join(directory, requested));
    try {
      if (relative(directory, target).startsWith('..')) throw new Error('outside site');
      if (statSync(target).isDirectory()) target = join(target, 'index.html');
      const types = { '.css': 'text/css', '.js': 'text/javascript', '.json': 'application/json', '.webp': 'image/webp', '.png': 'image/png' };
      const extension = target.slice(target.lastIndexOf('.'));
      response.writeHead(200, { 'Content-Type': types[extension] ?? 'text/html' });
      response.end(readFileSync(target));
    } catch {
      response.writeHead(404, { 'Content-Type': 'text/html' });
      response.end(readFileSync(join(directory, '404.html')));
    }
  });
}
let site;
async function localSite() {
  if (site) return site;
  const build = spawnSync('npm', ['run', 'build'], { cwd: root, encoding: 'utf8' });
  assert.equal(build.status, 0, build.stderr || build.stdout);
  const server = await staticServer(join(root, 'dist/site'));
  site = { server, base: `http://127.0.0.1:${server.address().port}` };
  return site;
}
async function withPage(callback) {
  const { base } = await localSite();
  const browser = await chromium.launch();
  const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
  const page = await context.newPage();
  try { return await callback({ base, page, context }); }
  finally { await context.close(); await browser.close(); }
}

test('@claim:build-static-bundle builds a portable static handoff from YAML, files, and URLs', () => {
  const directory = temp('khb-build');
  try {
    writeFileSync(join(directory, 'notes.md'), '# Source of truth\n');
    const yaml = writeSimpleHandoff(directory, ['{ id: notes, title: Notes, kind: file, path: notes.md, owner: Priya, required: true }']);
    const output = join(directory, 'bundle');
    const result = run(['build', yaml, '--output', output]);
    assert.equal(result.status, 0, result.stderr);
    assert.ok(statSync(join(output, 'index.html')).isFile());
    assert.ok(statSync(join(output, 'manifest.json')).isFile());
  } finally { rmSync(directory, { recursive: true, force: true }); }
});

test('@claim:copied-file-hash copies local files and records SHA-256 hashes', () => {
  const directory = temp('khb-hash');
  try {
    const source = join(directory, 'notes.md');
    writeFileSync(source, '# Current truth\n');
    const yaml = writeSimpleHandoff(directory, ['{ id: notes, title: Notes, kind: file, path: notes.md, owner: Priya }']);
    const output = join(directory, 'bundle');
    assert.equal(run(['--json', 'build', yaml, '--output', output]).status, 0);
    const manifest = JSON.parse(readFileSync(join(output, 'manifest.json')));
    const artifact = manifest.sections[0].artifacts[0];
    assert.equal(artifact.sha256, createHash('sha256').update(readFileSync(source)).digest('hex'));
    assert.equal(readFileSync(join(output, artifact.href), 'utf8'), '# Current truth\n');
  } finally { rmSync(directory, { recursive: true, force: true }); }
});

test('@claim:expiry-warning shows expiry warnings', () => {
  const directory = temp('khb-expiry');
  try {
    const date = new Date(Date.now() + 5 * 86_400_000).toISOString().slice(0, 10);
    const yaml = writeSimpleHandoff(directory, ['{ id: source, title: Source, kind: url, url: https://example.test/source, owner: Priya, expires_at: ' + date + ' }']);
    const output = run(['--json', 'check', yaml]);
    assert.equal(output.status, 0);
    const result = JSON.parse(output.stdout);
    assert.equal(result.result.warnings, 1);
    assert.equal(result.result.findings[0].code, 'expiry.soon');
  } finally { rmSync(directory, { recursive: true, force: true }); }
});

test('@claim:static-expiry-current recalculates expiry status when a recipient opens an older bundle', async () => {
  const directory = temp('khb-static-expiry');
  try {
    writeFileSync(join(directory, 'notes.md'), 'source');
    const yaml = writeSimpleHandoff(directory, ['{ id: notes, title: Notes, kind: file, path: notes.md, owner: Priya, expires_at: 2099-01-01 }']);
    const output = join(directory, 'bundle');
    assert.equal(run(['build', yaml, '--output', output]).status, 0);
    const browser = await chromium.launch();
    const context = await browser.newContext();
    const page = await context.newPage();
    await page.addInitScript(() => {
      const RealDate = Date;
      class FutureDate extends RealDate {
        constructor(...args) { super(...(args.length ? args : ['2100-01-01T00:00:00Z'])); }
        static now() { return new RealDate('2100-01-01T00:00:00Z').valueOf(); }
      }
      window.Date = FutureDate;
    });
    await page.goto(`file://${join(output, 'index.html')}`);
    await assert.doesNotReject(page.getByText('Expired on 2099-01-01; replace or confirm it.').waitFor());
    assert.match(await page.locator('#bundle-health').textContent(), /Attention needed/);
    await context.close();
    await browser.close();
  } finally { rmSync(directory, { recursive: true, force: true }); }
});

test('@claim:opt-in-link-check checks public links only when requested', () => {
  const directory = temp('khb-opt-in');
  try {
    const yaml = writeSimpleHandoff(directory, ['{ id: source, title: Source, kind: url, url: http://127.0.0.1:9/no-server, owner: Priya }']);
    const skipped = run(['--json', 'check', yaml]);
    assert.equal(skipped.status, 0);
    assert.equal(JSON.parse(skipped.stdout).result.findings.length, 1);
    const checked = run(['--json', 'check', yaml, '--check-links']);
    assert.equal(checked.status, 3);
    assert.match(checked.stdout, /link\.network/);
  } finally { rmSync(directory, { recursive: true, force: true }); }
});

test('@claim:origin-rate-limit honors a Retry-After window before the next origin request', () => {
  const result = spawnSync('cargo', ['test', '--locked', 'link_checker_honors_retry_after_before_the_next_request'], { cwd: root, encoding: 'utf8' });
  assert.equal(result.status, 0, result.stderr || result.stdout);
});

test('@claim:robots-respected respects robots.txt', async () => {
  const paths = [];
  const server = await serve((request, response) => {
    paths.push(request.url);
    if (request.url === '/robots.txt') response.end('User-agent: *\nDisallow: /private\n');
    else response.end('should not be reached');
  });
  const directory = temp('khb-robots');
  try {
    const port = server.address().port;
    const yaml = writeSimpleHandoff(directory, [`{ id: private, title: Private, kind: url, url: http://127.0.0.1:${port}/private/notes, owner: Priya }`]);
    const output = await runAsync(['--json', 'check', yaml, '--check-links']);
    assert.equal(output.status, 0, output.stderr);
    assert.match(output.stdout, /link\.robots_denied/);
    assert.deepEqual(paths, ['/robots.txt']);
  } finally { await close(server); rmSync(directory, { recursive: true, force: true }); }
});

test('@claim:credential-urls-rejected rejects user-info and credential-query URLs before fetch or output', async () => {
  const paths = [];
  const server = await serve((request, response) => {
    paths.push(request.url);
    response.end('should not be reached');
  });
  const directory = temp('khb-credentials');
  try {
    const port = server.address().port;
    const cases = [
      {
        url: `http://claim-user:claim-password@127.0.0.1:${port}/userinfo`,
        code: 'artifact.url_credentials',
        secret: 'claim-password',
        output: join(directory, 'userinfo-bundle'),
      },
      {
        url: `http://127.0.0.1:${port}/query?access_token=claim-query-secret`,
        code: 'artifact.url_secret',
        secret: 'claim-query-secret',
        output: join(directory, 'query-bundle'),
      },
    ];
    for (const example of cases) {
      const yaml = writeSimpleHandoff(directory, [`{ id: secret, title: Secret, kind: url, url: "${example.url}", owner: Priya }`]);
      const result = await runAsync(['--json', 'build', yaml, '--output', example.output, '--check-links']);
      assert.equal(result.status, 2, result.stderr);
      const parsed = JSON.parse(result.stdout);
      assert.equal(parsed.ok, false);
      assert.ok(parsed.result.findings.some((finding) => finding.code === example.code));
      assert.doesNotMatch(`${result.stdout}${result.stderr}`, new RegExp(example.secret));
      assert.equal(statSync(example.output, { throwIfNoEntry: false }), undefined);
    }
    assert.deepEqual(paths, []);
  } finally { await close(server); rmSync(directory, { recursive: true, force: true }); }
});

test('@claim:no-link-crawling checks only listed URLs and robots.txt', async () => {
  const paths = [];
  const server = await serve((request, response) => {
    paths.push(request.url);
    if (request.url === '/robots.txt') response.end('User-agent: *\nAllow: /\n');
    else response.end('<a href="/unlisted">ignored</a>');
  });
  const directory = temp('khb-crawl');
  try {
    const port = server.address().port;
    const yaml = writeSimpleHandoff(directory, [`{ id: listed, title: Listed, kind: url, url: http://127.0.0.1:${port}/listed, owner: Priya }`]);
    const output = await runAsync(['check', yaml, '--check-links']);
    assert.equal(output.status, 0, output.stderr);
    assert.deepEqual(paths, ['/robots.txt', '/listed']);
  } finally { await close(server); rmSync(directory, { recursive: true, force: true }); }
});

test('@claim:bundle-offline works from disk while the browser is offline', async () => {
  const directory = temp('khb-offline');
  try {
    const output = join(directory, 'bundle');
    assert.equal(run(['demo', '--output', output]).status, 0);
    const browser = await chromium.launch();
    const context = await browser.newContext();
    await context.setOffline(true);
    const page = await context.newPage();
    await page.goto(`file://${join(output, 'index.html')}`);
    assert.equal(await page.locator('.track').count(), 4);
    await context.close();
    await browser.close();
  } finally { rmSync(directory, { recursive: true, force: true }); }
});

test('@claim:static-self-contained needs no hosted runtime or database', async () => {
  const directory = temp('khb-static');
  try {
    const output = join(directory, 'bundle');
    assert.equal(run(['demo', '--output', output]).status, 0);
    const browser = await chromium.launch();
    const context = await browser.newContext();
    const page = await context.newPage();
    const requests = [];
    page.on('request', (request) => requests.push(request.url()));
    await page.goto(`file://${join(output, 'index.html')}`);
    assert.equal(await page.title(), 'Atlas reporting migration — Knowledge handoff');
    assert.ok(requests.every((url) => url.startsWith('file:')));
    await context.close();
    await browser.close();
  } finally { rmSync(directory, { recursive: true, force: true }); }
});

test('@claim:review-state-local keeps review state in the recipient browser', async () => {
  const directory = temp('khb-review');
  try {
    const output = join(directory, 'bundle');
    assert.equal(run(['demo', '--output', output]).status, 0);
    const browser = await chromium.launch();
    const context = await browser.newContext();
    const page = await context.newPage();
    await page.goto(`file://${join(output, 'index.html')}`);
    await page.locator('.review input').first().check();
    await page.reload();
    assert.equal(await page.locator('.review input').first().isChecked(), true);
    const keys = await page.evaluate(() => Object.keys(localStorage));
    assert.equal(keys.length, 1);
    assert.ok(keys[0].startsWith('khb:'));
    await context.close();
    await browser.close();
  } finally { rmSync(directory, { recursive: true, force: true }); }
});

test('@claim:acknowledgement-local-json exports local JSON tied to the manifest hash', () => {
  const directory = temp('khb-ack');
  try {
    const output = join(directory, 'bundle');
    assert.equal(run(['demo', '--output', output]).status, 0);
    const receipt = join(directory, 'ack.json');
    assert.equal(run(['acknowledge', join(output, 'manifest.json'), '--recipient', 'Sam Rivera', '--accept', 'architecture', '--output', receipt]).status, 0);
    const ack = JSON.parse(readFileSync(receipt));
    assert.equal(ack.accepted[0], 'architecture');
    assert.equal(ack.manifest_sha256, createHash('sha256').update(readFileSync(join(output, 'manifest.json'))).digest('hex'));
  } finally { rmSync(directory, { recursive: true, force: true }); }
});

test('@claim:no-cli-telemetry makes no proxy request during the bundled demo', async () => {
  let proxyRequests = 0;
  const proxy = await serve((_request, response) => { proxyRequests += 1; response.end(); });
  try {
    const output = await runAsync(['demo'], { env: { ...process.env, HTTP_PROXY: `http://127.0.0.1:${proxy.address().port}`, HTTPS_PROXY: `http://127.0.0.1:${proxy.address().port}` } });
    assert.equal(output.status, 0, output.stderr);
    assert.equal(proxyRequests, 0);
  } finally { await close(proxy); }
});

test('@claim:scoped-file-access copies only listed files to the chosen output', () => {
  const directory = temp('khb-scope');
  try {
    writeFileSync(join(directory, 'listed.md'), 'listed');
    writeFileSync(join(directory, 'unlisted.md'), 'do not copy');
    const yaml = writeSimpleHandoff(directory, ['{ id: listed, title: Listed, kind: file, path: listed.md, owner: Priya }']);
    const output = join(directory, 'chosen-output');
    assert.equal(run(['build', yaml, '--output', output]).status, 0);
    assert.deepEqual(readdirSync(join(output, 'files')), ['listed-listed.md']);
    assert.equal(readFileSync(join(directory, 'unlisted.md'), 'utf8'), 'do not copy');
  } finally { rmSync(directory, { recursive: true, force: true }); }
});

test('@claim:landing-storage-disclosed leaves user-data stores empty and caches only same-origin site files', { concurrency: false }, async () => {
  await withPage(async ({ base, page }) => {
    await page.goto(base, { waitUntil: 'networkidle' });
    await page.evaluate(() => navigator.serviceWorker.ready);
    await page.reload({ waitUntil: 'networkidle' });
    const state = await page.evaluate(async () => {
      const cacheNames = await caches.keys();
      const cachedUrls = (await Promise.all(cacheNames.map(async (name) => {
        const cache = await caches.open(name);
        return (await cache.keys()).map((request) => request.url);
      }))).flat();
      return {
        localStorageKeys: Object.keys(localStorage),
        sessionStorageKeys: Object.keys(sessionStorage),
        databases: await indexedDB.databases(),
        registrations: (await navigator.serviceWorker.getRegistrations()).length,
        controlled: navigator.serviceWorker.controller !== null,
        cacheNames,
        cachedUrls,
      };
    });
    assert.deepEqual(state.localStorageKeys, []);
    assert.deepEqual(state.sessionStorageKeys, []);
    assert.deepEqual(state.databases, []);
    assert.equal((await page.context().cookies()).length, 0);
    assert.equal(state.registrations, 1);
    assert.equal(state.controlled, true);
    assert.equal(state.cacheNames.length, 1);
    assert.ok(state.cacheNames[0].startsWith('khb-site-'));
    assert.ok(state.cachedUrls.length >= 4);
    assert.ok(state.cachedUrls.every((url) => new URL(url).origin === base));
    for (const pathname of ['/', '/cassette-handoff.webp', '/privacy/', '/terms/']) {
      assert.ok(state.cachedUrls.includes(new URL(pathname, base).href), `offline cache omitted ${pathname}`);
    }
  });
});

test('@claim:landing-no-third-party loads no analytics, trackers, remote scripts, or remote fonts', { concurrency: false }, async () => {
  await withPage(async ({ base, page }) => {
    const requests = [];
    page.on('request', (request) => requests.push(request.url()));
    await page.goto(base, { waitUntil: 'networkidle' });
    assert.ok(requests.every((url) => new URL(url).origin === base));
    assert.equal((await page.context().cookies()).length, 0);
  });
});

test('@claim:json-output supports JSON output on every command', () => {
  const directory = temp('khb-json');
  try {
    const init = run(['--json', 'init', join(directory, 'new.yaml')]);
    assert.equal(JSON.parse(init.stdout).command, 'init');
    const yaml = writeSimpleHandoff(directory, ['{ id: source, title: Source, kind: url, url: https://example.test, owner: Priya }']);
    const output = join(directory, 'bundle');
    for (const args of [['--json', 'check', yaml], ['--json', 'build', yaml, '--output', output], ['--json', 'demo'], ['--json', 'acknowledge', join(output, 'manifest.json'), '--recipient', 'Sam', '--output', join(directory, 'ack.json')]]) {
      const result = run(args);
      assert.equal(result.status, 0, result.stderr);
      assert.equal(JSON.parse(result.stdout).ok, true);
    }
  } finally { rmSync(directory, { recursive: true, force: true }); }
});

test('@claim:ci-warnings-fail makes warnings fail validation in CI mode', () => {
  const directory = temp('khb-ci');
  try {
    const date = new Date(Date.now() + 5 * 86_400_000).toISOString().slice(0, 10);
    const yaml = writeSimpleHandoff(directory, ['{ id: source, title: Source, kind: url, url: https://example.test, owner: Priya, expires_at: ' + date + ' }']);
    const output = run(['--json', '--ci', 'check', yaml]);
    assert.equal(output.status, 2);
    assert.equal(JSON.parse(output.stdout).ok, false);
  } finally { rmSync(directory, { recursive: true, force: true }); }
});

test('@claim:init-no-overwrite refuses to overwrite an existing file', () => {
  const directory = temp('khb-init');
  try {
    const file = join(directory, 'handoff.yaml');
    writeFileSync(file, 'keep this');
    const output = run(['init', file]);
    assert.equal(output.status, 4);
    assert.equal(readFileSync(file, 'utf8'), 'keep this');
  } finally { rmSync(directory, { recursive: true, force: true }); }
});

test('@claim:exit-codes returns documented 0, 2, 3, and 4 exit codes', () => {
  const directory = temp('khb-exits');
  try {
    assert.equal(run(['demo']).status, 0);
    writeFileSync(join(directory, 'bad.yaml'), 'not: [valid');
    assert.equal(run(['check', join(directory, 'bad.yaml')]).status, 2);
    const linkYaml = writeSimpleHandoff(directory, ['{ id: source, title: Source, kind: url, url: http://127.0.0.1:9/no-server, owner: Priya }']);
    assert.equal(run(['check', linkYaml, '--check-links']).status, 3);
    const exists = join(directory, 'exists.yaml');
    writeFileSync(exists, 'existing');
    assert.equal(run(['init', exists]).status, 4);
  } finally { rmSync(directory, { recursive: true, force: true }); }
});

test('@claim:mit-free is free software under the MIT license', () => {
  const metadata = spawnSync('cargo', ['metadata', '--no-deps', '--format-version', '1'], { cwd: root, encoding: 'utf8' });
  assert.equal(metadata.status, 0, metadata.stderr);
  assert.equal(JSON.parse(metadata.stdout).packages[0].license, 'MIT');
  assert.match(readFileSync(join(root, 'LICENSE'), 'utf8'), /Permission is hereby granted, free of charge/);
});

test('@claim:no-account runs the sample without an account or configuration', () => {
  const output = run(['--json', 'demo'], { env: { PATH: process.env.PATH } });
  assert.equal(output.status, 0, output.stderr);
  assert.equal(JSON.parse(output.stdout).result.sample, true);
});

test('@claim:demo-sandbox uses a separate sample namespace and reset controls', { concurrency: false }, async () => {
  await withPage(async ({ base, page }) => {
    await page.goto(`${base}/demo/`, { waitUntil: 'networkidle' });
    await assert.doesNotReject(page.getByText('Demo — sample data, nothing is saved').waitFor());
    await page.locator('.review input').first().check();
    await page.locator('#recipient').fill('Sample recipient');
    const keys = await page.evaluate(() => Object.keys(localStorage));
    assert.equal(keys.length, 1);
    assert.ok(keys[0].startsWith('demo:'));
    await page.getByRole('button', { name: 'Reset demo' }).click();
    assert.equal(await page.locator('.review input').first().isChecked(), false);
    assert.equal(await page.locator('#recipient').inputValue(), '');
  });
});

test('@claim:demo-sample-results shows recorded reachable and broken link outcomes', { concurrency: false }, async () => {
  await withPage(async ({ base, page }) => {
    await page.goto(`${base}/demo/`, { waitUntil: 'networkidle' });
    await assert.doesNotReject(page.getByText('Recorded sample check: reachable (HTTP 200)').waitFor());
    await assert.doesNotReject(page.getByText('Recorded sample check: HTTP 404; replace this URL').waitFor());
    assert.equal(await page.getByText('Link needs replacement').count(), 1);
  });
});

after(async () => { if (site) await close(site.server); });

import { createHash } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const SHELL_FILES = [
  'index.html',
  'privacy/index.html',
  'terms/index.html',
  'cassette-handoff.webp',
];

/**
 * Write a release-specific service worker after Vite has emitted the shell.
 *
 * A shell fingerprint deliberately becomes part of the worker source. Browsers
 * therefore see a worker update whenever a page or the hero image changes,
 * and the worker uses a new cache rather than reusing stale entries.
 */
export async function writeServiceWorker(outputDirectory, templatePath) {
  const files = await Promise.all(
    SHELL_FILES.map(async (file) => ({
      file,
      content: await readFile(join(outputDirectory, file)),
    })),
  );
  const fingerprint = createHash('sha256');
  for (const { file, content } of files) {
    fingerprint.update(file);
    fingerprint.update(content);
  }

  const template = await readFile(templatePath, 'utf8');
  const cacheVersion = fingerprint.digest('hex').slice(0, 16);
  const worker = template.replaceAll('__CACHE_VERSION__', cacheVersion);
  await writeFile(join(outputDirectory, 'sw.js'), worker);
  return cacheVersion;
}

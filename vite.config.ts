import { defineConfig } from 'vite';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { writeServiceWorker } from './scripts/service-worker.mjs';

const repositoryRoot = fileURLToPath(new URL('.', import.meta.url));

export default defineConfig({
  root: 'site',
  plugins: [
    {
      name: 'release-service-worker',
      async closeBundle() {
        await writeServiceWorker(
          resolve(repositoryRoot, 'dist/site'),
          resolve(repositoryRoot, 'site/sw.template.js'),
        );
      },
    },
  ],
  build: {
    outDir: '../dist/site',
    emptyOutDir: true,
    target: 'es2022',
    cssCodeSplit: false,
    rollupOptions: {
      input: {
        index: resolve(repositoryRoot, 'site/index.html'),
        privacy: resolve(repositoryRoot, 'site/privacy/index.html'),
        terms: resolve(repositoryRoot, 'site/terms/index.html'),
      },
    },
  },
});

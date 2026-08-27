import { defineConfig } from 'vite';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = fileURLToPath(new URL('.', import.meta.url));

export default defineConfig({
  root: 'site',
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

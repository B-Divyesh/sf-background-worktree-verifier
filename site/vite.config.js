import { defineConfig } from 'vite';
import { resolve } from 'node:path';

export default defineConfig({
  root: resolve(import.meta.dirname, 'src'),
  publicDir: resolve(import.meta.dirname, 'public'),
  build: {
    outDir: resolve(import.meta.dirname, '../dist/site'),
    emptyOutDir: true,
    target: 'es2022',
    rollupOptions: {
      input: [
        resolve(import.meta.dirname, 'src/index.html'),
        resolve(import.meta.dirname, 'src/demo/index.html'),
        resolve(import.meta.dirname, 'src/privacy/index.html'),
        resolve(import.meta.dirname, 'src/terms/index.html'),
      ],
    },
  },
});

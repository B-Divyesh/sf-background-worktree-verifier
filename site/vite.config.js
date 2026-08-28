import { defineConfig } from 'vite';
import { resolve } from 'node:path';

export default defineConfig({
  root: resolve(import.meta.dirname, 'src'),
  publicDir: resolve(import.meta.dirname, 'public'),
  build: { outDir: resolve(import.meta.dirname, '../dist/site'), emptyOutDir: true, target: 'es2022' }
});

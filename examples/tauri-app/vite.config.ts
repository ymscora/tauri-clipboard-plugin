import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import path from 'node:path';

const rootDir = path.resolve(__dirname, '../..');

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@clipboard-api': path.resolve(rootDir, 'guest-js/index.ts'),
      '@tauri-apps/api/core': path.resolve(__dirname, 'node_modules/@tauri-apps/api/core.js'),
      '@tauri-apps/api/event': path.resolve(__dirname, 'node_modules/@tauri-apps/api/event.js'),
    },
  },
  server: {
    fs: {
      allow: [rootDir],
    },
    port: 1420,
    strictPort: true,
  },
});

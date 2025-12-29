import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  plugins: [
    wasm(),
    topLevelAwait(),
  ],
  build: {
    target: 'esnext',
  },
  optimizeDeps: {
    exclude: ['rusheet-wasm'],
  },
  resolve: {
    alias: {
      '@': '/src',
    },
  },
  test: {
    globals: true,
    environment: 'happy-dom',
    setupFiles: ['./src/__tests__/setup.ts'],
    pool: 'forks', // Use forks for better WASM support
    poolOptions: {
      forks: {
        singleFork: true, // Run tests in single fork for WASM shared memory
      },
    },
    server: {
      deps: {
        inline: ['rusheet-wasm'], // Inline WASM module in tests
      },
    },
  },
  server: {
    fs: {
      allow: ['..'], // Allow serving files from parent directory (for pkg/)
    },
  },
});

import { defineConfig } from 'vite'

// https://vite.dev/config/
export default defineConfig({
  server: {
    proxy: {
      '/blockchain': 'http://localhost:3000', // Adaptează portul la cel pe care rulează Axum
      '/wallet': 'http://localhost:3000',
    },
  },
});

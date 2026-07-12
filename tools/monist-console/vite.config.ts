import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  build: {
    outDir: '../../../first-synth/console',
    emptyOutDir: true,
  },
  base: '/console/',
})

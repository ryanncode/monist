import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import fs from 'fs'
import path from 'path'

const firstSynthParent = path.resolve(__dirname, '../../../first-synth');
const outDir = fs.existsSync(firstSynthParent) ? path.resolve(firstSynthParent, 'console') : 'dist';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  build: {
    outDir,
    emptyOutDir: true,
  },
  base: '/console/',
})

import { resolve } from 'path';
import copy from 'rollup-plugin-copy';
import { defineConfig } from 'vite';

export default defineConfig({
    build: {
        rollupOptions: {
            input: {
                script: resolve(__dirname, 'src/resources/js/script.js'),
                styles: resolve(__dirname, 'src/resources/css/styles.css'),
            },
            output: {
                entryFileNames: '[name]-[hash].js',
                chunkFileNames: '[name]-[hash].js',
                assetFileNames: '[name]-[hash][extname]',
            },
        },
        outDir: 'dist',
        emptyOutDir: true,
        cssCodeSplit: true,
        manifest: true,
    },
    plugins: [
        copy({
            targets: [
                {
                    src: 'src/resources/imgs/**/*',
                    dest: 'dist/imgs',
                },
            ],
            hook: 'writeBundle',
        }),
    ],
});

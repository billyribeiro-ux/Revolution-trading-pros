import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import tailwindcss from '@tailwindcss/vite';
import viteCompression from 'vite-plugin-compression';

export default defineConfig({
	plugins: [
		tailwindcss(),
		sveltekit(),
		// Brotli compression (best compression)
		viteCompression({
			algorithm: 'brotliCompress',
			ext: '.br',
			threshold: 1024, // Only compress files > 1KB
			deleteOriginFile: false
		}),
		// Gzip compression (fallback for older browsers)
		viteCompression({
			algorithm: 'gzip',
			ext: '.gz',
			threshold: 1024,
			deleteOriginFile: false
		})
	],
	server: {
		port: 5174
	},
	build: {
		// Optimize chunk size
		chunkSizeWarningLimit: 1000,
		// Enable minification with esbuild (faster than terser)
		minify: 'esbuild',
		// Optimize chunk splitting
		rollupOptions: {
			output: {
				manualChunks: (id) => {
					// Only apply manual chunks to client-side builds
					if (id.includes('node_modules')) {
						if (id.includes('svelte')) return 'vendor-svelte';
						if (id.includes('lightweight-charts')) return 'vendor-charts';
						if (id.includes('three') || id.includes('@threlte')) return 'vendor-three';
						return 'vendor';
					}
				}
			}
		},
		// Enable source maps for debugging (can disable in production)
		sourcemap: false,
		// Target modern browsers for smaller bundle
		target: 'es2020'
	},
	optimizeDeps: {
		// Pre-bundle these dependencies
		include: [
			'svelte',
			'gsap',
			'lightweight-charts',
			'three',
			'@threlte/core',
			'@threlte/extras'
		],
		// Exclude large dependencies that should be lazy loaded
		exclude: ['@fortawesome/fontawesome-svg-core']
	},
	// Enable CSS code splitting
	css: {
		devSourcemap: false
	}
});

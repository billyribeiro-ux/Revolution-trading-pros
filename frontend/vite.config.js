import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import tailwindcss from '@tailwindcss/vite';
import viteCompression from 'vite-plugin-compression';

export default defineConfig({
	// Vitest configuration
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}'],
		environment: 'node',
		globals: true,
		setupFiles: ['src/lib/observability/__tests__/setup.ts'],
		// Mock SvelteKit modules
		alias: {
			'$app/environment': new URL('./src/lib/observability/__tests__/mocks/app-environment.ts', import.meta.url).pathname,
		},
	},
	plugins: [
		tailwindcss(),
		sveltekit(),
		// Brotli compression (best compression)
		viteCompression({
			algorithm: 'brotliCompress',
			ext: '.br',
			threshold: 512, // Compress smaller files
			deleteOriginFile: false
		}),
		// Gzip compression (fallback for older browsers)
		viteCompression({
			algorithm: 'gzip',
			ext: '.gz',
			threshold: 512,
			deleteOriginFile: false
		})
	],
	server: {
		port: 5174,
		// Enable HTTP/2 for better performance
		https: false,
		// Strict localhost binding for consistent HMR
		host: 'localhost',
		strictPort: true,
		// ICT9+ HMR Configuration - explicit WebSocket settings
		hmr: {
			protocol: 'ws',
			host: 'localhost',
			port: 5174,
			overlay: true // Enable overlay to see errors clearly
		},
		// Proxy API requests to Laravel backend
		proxy: {
			'/api': {
				target: 'http://localhost:8000',
				changeOrigin: true,
				secure: false
			}
		}
	},
	build: {
		// Optimize chunk size
		chunkSizeWarningLimit: 500,
		// Enable minification with esbuild (faster than terser)
		minify: 'esbuild',
		// Optimize CSS minification
		cssMinify: 'esbuild',
		// Optimize chunk splitting - CRITICAL FOR PERFORMANCE
		rollupOptions: {
			output: {
				manualChunks: (id) => {
					// Aggressive code splitting for faster initial load
					if (id.includes('node_modules')) {
						// Core framework
						if (id.includes('svelte')) return 'vendor-svelte';
						
						// Heavy animation libraries - lazy load
						if (id.includes('gsap')) return 'vendor-gsap';
						if (id.includes('lightweight-charts')) return 'vendor-charts';
						if (id.includes('three') || id.includes('@threlte')) return 'vendor-three';
						
						// Icon libraries - split by usage
						if (id.includes('@tabler/icons-svelte')) return 'vendor-icons';
						
						// Chart libraries
						if (id.includes('chart.js') || id.includes('apexcharts')) return 'vendor-charts-alt';
						
						// Animation libraries
						if (id.includes('lottie') || id.includes('anime')) return 'vendor-animation';
						
						// D3 and data viz
						if (id.includes('d3')) return 'vendor-d3';
						
						// Everything else
						return 'vendor';
					}
				},
				// Optimize chunk file names
				chunkFileNames: 'chunks/[name]-[hash].js',
				entryFileNames: 'entries/[name]-[hash].js',
				assetFileNames: 'assets/[name]-[hash].[ext]'
			}
		},
		// Disable source maps in production
		sourcemap: false,
		// Target modern browsers for smaller bundle
		target: 'es2020',
		// Enable module preload polyfill
		modulePreload: {
			polyfill: true
		},
		// Optimize asset inlining
		assetsInlineLimit: 4096 // 4KB
	},
	optimizeDeps: {
		// Pre-bundle only critical dependencies
		include: [
			'svelte',
			// svelte-email@0.0.4 has svelte field but no exports condition
			// Include it here to avoid vite-plugin-svelte warning
			// See: https://github.com/sveltejs/vite-plugin-svelte/blob/main/docs/faq.md#missing-exports-condition
			'svelte-email'
		],
		// Exclude heavy dependencies - lazy load them
		exclude: [
			'@fortawesome/fontawesome-svg-core',
			'gsap',
			'lightweight-charts',
			'three',
			'@threlte/core',
			'@threlte/extras',
			'lottie-web',
			'animejs',
			'chart.js',
			'apexcharts',
			'd3'
		],
		// Force optimization
		force: false,
		// Optimize dependency discovery
		entries: [
			'src/routes/**/*.svelte'
		],
		// ICT8-11+ Performance: Disable esbuild optimization for faster dev startup
		esbuildOptions: {
			target: 'esnext'
		}
	},
	// Enable CSS code splitting
	css: {
		devSourcemap: false,
		// Optimize CSS processing
		preprocessorOptions: {},
		// Enable CSS modules optimization
		modules: {
			localsConvention: 'camelCase'
		}
	},
	// Performance optimizations
	esbuild: {
		// Drop console and debugger in production
		drop: process.env.NODE_ENV === 'production' ? ['console', 'debugger'] : [],
		// Optimize for speed
		legalComments: 'none',
		// Tree shaking
		treeShaking: true
	},
	// Resolve optimizations
	resolve: {
		// Optimize module resolution
		dedupe: ['svelte']
	}
});

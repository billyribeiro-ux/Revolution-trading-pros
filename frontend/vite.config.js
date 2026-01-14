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
		// Brotli compression for production
		viteCompression({
			algorithm: 'brotliCompress',
			ext: '.br',
			threshold: 512,
			deleteOriginFile: false
		}),
		// Gzip fallback
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
		// Proxy API requests to deployed Rust API on Fly.io
		proxy: {
			'/api': {
				target: 'https://revolution-trading-pros-api.fly.dev',
				changeOrigin: true,
				secure: true,
				rewrite: (path) => path // Keep path as-is
			},
			// ICT11+ Pattern: Proxy admin routes to prevent CORS
			'/admin': {
				target: 'https://revolution-trading-pros-api.fly.dev',
				changeOrigin: true,
				secure: true
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
		// Disable source maps in production
		sourcemap: false,
		// Target modern browsers for smaller bundle
		target: 'es2020',
		// Apple ICT 7 Principal Engineer Solution: Disable modulePreload entirely
		// SvelteKit's native lazy loading is superior - no preload warnings
		modulePreload: false,
		// Optimize asset inlining
		assetsInlineLimit: 4096, // 4KB
		// NOTE: cssCodeSplit is controlled by SvelteKit, not Vite
		// CORB prevention is handled via kit.inlineStyleThreshold in svelte.config.js
		rollupOptions: {
			external: ['tus-js-client'],
			output: {
				// Optimized chunk naming for production
				chunkFileNames: '_app/immutable/chunks/[name]-[hash].js',
				assetFileNames: '_app/immutable/assets/[name]-[hash][extname]',
				// Apple ICT 7: Aggressive code splitting for optimal caching
				manualChunks: (id) => {
					// Vendor chunks by package for granular caching
					if (id.includes('node_modules')) {
						// Split large vendor packages into separate chunks
						if (id.includes('svelte')) return 'vendor-svelte';
						if (id.includes('@sveltejs')) return 'vendor-sveltekit';
						return 'vendor';
					}
				}
			}
		}
	},
	// SSR configuration to handle CSS properly
	ssr: {
		// Ensure external packages with CSS are handled correctly
		noExternal: ['@tailwindcss/postcss', 'tus-js-client']
	},
	optimizeDeps: {
		// Pre-bundle only critical dependencies
		include: [
			'svelte'
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
		]
	},
	// CSS configuration - CORB prevention handled by SvelteKit's inlineStyleThreshold
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

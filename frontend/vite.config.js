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
		// ICT 11+ Fix: Optimize module preload to reduce "preloaded but not used" warnings
		modulePreload: {
			polyfill: true,
			// Only preload modules that are immediately needed
			resolveDependencies: (filename, deps, { hostId, hostType }) => {
				// Filter out CSS files from preload unless they're critical
				return deps.filter(dep => {
					// Always preload JS modules
					if (dep.endsWith('.js')) return true;
					// Only preload CSS for the current route, not all routes
					if (dep.endsWith('.css')) {
						// Only preload CSS if it's for the same route chunk
						return filename.includes(dep.split('.')[0]);
					}
					return true;
				});
			}
		},
		// Optimize asset inlining
		assetsInlineLimit: 4096, // 4KB
		// ICT 11+ Fix: Optimize CSS code splitting to reduce unused preloads
		cssCodeSplit: true,
		rollupOptions: {
			output: {
				// Better chunk naming for debugging
				chunkFileNames: '_app/immutable/chunks/[name]-[hash].js',
				assetFileNames: '_app/immutable/assets/[name]-[hash][extname]',
				// Optimize manual chunks to reduce CSS splitting issues
				manualChunks: (id) => {
					// Keep vendor code separate
					if (id.includes('node_modules')) {
						return 'vendor';
					}
					// Group dashboard routes together to reduce CSS chunks
					if (id.includes('/routes/dashboard/')) {
						return 'dashboard';
					}
				}
			}
		}
	},
	// SSR configuration to handle CSS properly
	ssr: {
		// Ensure external packages with CSS are handled correctly
		noExternal: ['@tailwindcss/postcss']
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

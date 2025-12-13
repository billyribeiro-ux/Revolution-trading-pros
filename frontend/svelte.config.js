/**
 * Revolution Trading Pros - SvelteKit Configuration
 * ⚡ LIGHTNING STACK - ICT 11+ Principal Engineer Configuration
 *
 * Default: Vercel (development) - simple deployment
 * Production: Set DEPLOY_TARGET=cloudflare for maximum speed
 *
 * @version 3.1.0 - Lightning Stack Edition
 * @author Revolution Trading Pros
 */

import adapterCloudflare from '@sveltejs/adapter-cloudflare';
import adapterVercel from '@sveltejs/adapter-vercel';
import adapterStatic from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

// Determine which adapter to use based on environment
// Default: Vercel for easy development, switch to cloudflare for production
const getAdapter = () => {
	const target = process.env.DEPLOY_TARGET || 'vercel';

	switch (target) {
		case 'cloudflare':
			return adapterCloudflare({
				// Enable edge SSR for dynamic pages
				routes: {
					include: ['/*'],
					exclude: ['<all>']
				},
				// Platform-specific optimizations
				platformProxy: {
					configPath: 'wrangler.toml',
					experimentalJsonConfig: false,
					persist: false
				}
			});
		case 'vercel':
			return adapterVercel({
				runtime: 'nodejs22.x', // Node.js runtime for full compatibility
				regions: ['iad1'], // Primary region
				split: false
			});
		case 'static':
			return adapterStatic({
				pages: 'build',
				assets: 'build',
				fallback: 'index.html',
				precompress: true, // Enable brotli/gzip
				strict: true
			});
		default:
			return adapterCloudflare();
	}
};

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	onwarn: (warning, handler) => {
		// ICT11+ Production Configuration - Suppress non-critical warnings
		const suppressedCodes = [
			'css_unused_selector',
			'css_unknown_at_rule', // Tailwind @apply/@reference directives
			'css_empty_rule', // Empty CSS rulesets used as placeholders
			'state_referenced_locally', // Svelte 5 false positive for closures
			'a11y_click_events_have_key_events', // Handled by component logic
			'a11y_no_static_element_interactions' // Handled by component logic
		];
		if (suppressedCodes.includes(warning.code)) return;
		handler(warning);
	},
	kit: {
		adapter: getAdapter(),
		prerender: {
			handleHttpError: ({ status, path }) => {
				// Ignore 404 errors for unimplemented routes
				if (status === 404) {
					return;
				}
				// Let other errors through
				throw new Error(`${status} ${path}`);
			},
			handleMissingId: 'ignore', // Ignore missing anchor IDs (skip-to-content links)
			handleUnseenRoutes: 'ignore',
			// Concurrent prerendering for faster builds
			concurrency: 8, // Increased from 4 for faster builds
			crawl: true, // Auto-discover linked pages
			entries: ['*', '/sitemap.xml', '/robots.txt']
		},
		// Inline small CSS files for faster initial render
		inlineStyleThreshold: 2048, // Increased from 1024 for better FCP
		// Content Security Policy - WCAG/OWASP compliant
		csp: {
			mode: 'auto',
			directives: {
				'default-src': ['self'],
				'script-src': ['self', 'unsafe-inline', 'https://www.googletagmanager.com', 'https://www.google-analytics.com'],
				'style-src': ['self', 'unsafe-inline', 'fonts.googleapis.com'],
				'font-src': ['self', 'fonts.gstatic.com', 'data:'],
				'img-src': ['self', 'data:', 'https:', 'blob:'],
				'connect-src': ['self', 'ws:', 'wss:', 'https:', 'http://localhost:8000', 'http://127.0.0.1:8000', 'http://localhost:8001', 'http://127.0.0.1:8001'],
				'frame-ancestors': ['none'],
				'base-uri': ['self'],
				'form-action': ['self']
			}
		},
		// Alias for cleaner imports
		alias: {
			$components: 'src/lib/components',
			$stores: 'src/lib/stores',
			$utils: 'src/lib/utils',
			$api: 'src/lib/api'
		},
		// Service worker for offline support and caching
		serviceWorker: {
			register: true
		},
		// Environment variable prefix
		env: {
			publicPrefix: 'PUBLIC_'
		}
	},
	// Compile-time optimizations
	compilerOptions: {
		// CSS scoping - each component gets its own scoped styles
		// This ensures complete design freedom per page/section
		// Note: hydratable and immutable options removed in Svelte 5
		// Components are always hydratable, and immutable has no effect in runes mode
	}
};

export default config;

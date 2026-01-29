/**
 * Revolution Trading Pros - SvelteKit Configuration
 * ⚡ LIGHTNING STACK - ICT 11+ Principal Engineer Configuration
 *
 * Default: Cloudflare for maximum performance
 * Alternative: Set DEPLOY_TARGET=static for static site generation
 *
 * @version 3.2.0 - Lightning Stack Edition (Cloudflare-first)
 * @author Revolution Trading Pros
 */

import adapterCloudflare from '@sveltejs/adapter-cloudflare';
import adapterStatic from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

// Determine which adapter to use based on environment
// Default: Cloudflare for maximum edge performance
const getAdapter = () => {
	const target = process.env.DEPLOY_TARGET || 'cloudflare';

	switch (target) {
		case 'cloudflare':
			// ICT 7 FIX: Don't exclude offline.html - it's needed by service worker
			return adapterCloudflare({
				routes: {
					include: ['/*'],
					exclude: ['<all>']
				},
				platformProxy: {
					configPath: 'wrangler.toml',
					experimentalJsonConfig: false,
					persist: false
				}
			});
		case 'static':
			return adapterStatic({
				pages: 'build',
				assets: 'build',
				fallback: 'index.html',
				precompress: true,
				strict: true
			});
		default:
			return adapterCloudflare();
	}
};

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	// Svelte 5: Let compiler auto-detect runes mode per-file
	// This allows third-party libraries using legacy $$props to work
	// while project files using $state/$derived/$effect use runes mode
	compilerOptions: {
		// Do NOT set runes globally - Svelte 5 auto-detects per file
		// Optimize for modern browsers
		dev: process.env.NODE_ENV !== 'production',
		// Svelte 5.48+ compatibility
		compatibility: {
			componentApi: 5
		}
	},
	onwarn: (warning, handler) => {
		const suppressedCodes = [
			'css_unused_selector',
			'css_unknown_at_rule',
			'css_empty_rule',
			'state_referenced_locally',
			// Svelte 5 specific warnings
			'component_type_invalid',
			'export_let_unused',
			// Modern CSS properties not yet recognized by parser
			'css_unknown_property'
		];
		if (suppressedCodes.includes(warning.code)) return;
		handler(warning);
	},
	kit: {
		adapter: getAdapter(),
		prerender: {
			handleHttpError: ({ status, path }) => {
				if (status === 404) {
					return;
				}
				// Ignore 500 errors for authenticated routes during prerendering
				if (status === 500 && path.startsWith('/dashboard')) {
					return;
				}
				throw new Error(`${status} ${path}`);
			},
			handleMissingId: 'ignore',
			handleUnseenRoutes: 'ignore',
			concurrency: 8,
			crawl: true,
			entries: ['*', '/sitemap.xml', '/robots.txt']
		},
		// Disable CSS inlining to allow proper Svelte scoped CSS
		// The aggressive inlining was breaking scoped CSS in production builds
		inlineStyleThreshold: 0, // Disabled - use separate CSS files
		csp: {
			mode: 'auto',
			directives: {
				'default-src': ['self'],
				'script-src': [
					'self',
					'unsafe-inline',
					'https://www.googletagmanager.com',
					'https://www.google-analytics.com',
					'https://static.cloudflareinsights.com',
					'https://apis.google.com'
				],
				'style-src': ['self', 'unsafe-inline', 'fonts.googleapis.com'],
				'font-src': ['self', 'fonts.gstatic.com', 'data:'],
				'img-src': ['self', 'data:', 'https:', 'blob:'],
				'media-src': [
					'self',
					'https://*.mediadelivery.net',
					'https://vz-5a23b520-193.b-cdn.net',
					'https://*.b-cdn.net',
					'https://pub-2e5bd1b702b440bd888a0fc47f3493ae.r2.dev'
				],
				'connect-src': [
					'self',
					'ws:',
					'wss:',
					'https:',
					'https://revolution-trading-pros-api.fly.dev',
					'https://revolution-trading-pros.pages.dev',
					'https://www.googleapis.com',
					'https://*.mediadelivery.net',
					'https://vz-5a23b520-193.b-cdn.net'
				],
				'frame-src': ['self', 'https://iframe.mediadelivery.net', 'https://*.mediadelivery.net'],
				'frame-ancestors': ['none'],
				'base-uri': ['self'],
				'form-action': ['self']
			}
		},
		alias: {
			$components: 'src/lib/components',
			$stores: 'src/lib/stores',
			$utils: 'src/lib/utils',
			$api: 'src/lib/api'
		},
		serviceWorker: {
			register: true
		},
		env: {
			publicPrefix: 'PUBLIC_'
		},
		// Apple ICT 7: Output configuration per SvelteKit 2.13+ docs
		// https://svelte.dev/docs/kit/configuration#output
		output: {
			// Preload strategy for modern browsers
			preloadStrategy: 'modulepreload'
			// NOTE: bundleStrategy: 'single' removed - caused scroll/CSS issues
			// CORB prevention handled via inlineStyleThreshold: 10240 above
		}
	}
};

export default config;

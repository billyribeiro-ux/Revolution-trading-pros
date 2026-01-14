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
		dev: process.env.NODE_ENV !== 'production'
	},
	onwarn: (warning, handler) => {
		const suppressedCodes = [
			'css_unused_selector',
			'css_unknown_at_rule', 
			'css_empty_rule',
			'state_referenced_locally',
			'a11y_click_events_have_key_events',
			'a11y_no_static_element_interactions',
			// Svelte 5 specific warnings
			'component_type_invalid',
			'export_let_unused'
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
		// Apple ICT 7 CORB Fix: Aggressively inline CSS to prevent cross-origin blocking
		// Higher threshold = more CSS inlined = fewer external CSS files = no CORB
		inlineStyleThreshold: 10240, // 10KB - inline most component CSS
		csp: {
			mode: 'auto',
			directives: {
				'default-src': ['self'],
				'script-src': ['self', 'unsafe-inline', 'https://www.googletagmanager.com', 'https://www.google-analytics.com', 'https://static.cloudflareinsights.com', 'https://apis.google.com'],
				'style-src': ['self', 'unsafe-inline', 'fonts.googleapis.com'],
				'font-src': ['self', 'fonts.gstatic.com', 'data:'],
				'img-src': ['self', 'data:', 'https:', 'blob:'],
				'media-src': ['self', 'https://s3.amazonaws.com', 'https://simpler-options.s3.amazonaws.com', 'https://simpler-cdn.s3.amazonaws.com', 'https://*.s3.amazonaws.com', 'https://cloud-streaming.s3.amazonaws.com', 'blob:'],
				'connect-src': ['self', 'ws:', 'wss:', 'https:', 'https://revolution-trading-pros-api.fly.dev', 'https://revolution-trading-pros.pages.dev', 'https://www.googleapis.com', 'https://pendo-data-prod.box.com'],
				'frame-src': ['self', 'https://simplertrading.app.box.com', 'https://*.box.com', 'https://www.youtube.com', 'https://player.vimeo.com'],
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
		// Apple ICT 7 CORB Fix: Output configuration per SvelteKit 2.13+ docs
		// https://svelte.dev/docs/kit/configuration#output
		output: {
			// Preload strategy for modern browsers
			preloadStrategy: 'modulepreload',
			// CRITICAL: 'single' creates ONE CSS file instead of chunks
			// This eliminates CORB by avoiding cross-origin CSS chunk requests
			bundleStrategy: 'single'
		}
	}
};

export default config;

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
	onwarn: (warning, handler) => {
		const suppressedCodes = [
			'css_unused_selector',
			'css_unknown_at_rule',
			'css_empty_rule',
			'state_referenced_locally',
			'a11y_click_events_have_key_events',
			'a11y_no_static_element_interactions'
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
				throw new Error(`${status} ${path}`);
			},
			handleMissingId: 'ignore',
			handleUnseenRoutes: 'ignore',
			concurrency: 8,
			crawl: true,
			entries: ['*', '/sitemap.xml', '/robots.txt']
		},
		inlineStyleThreshold: 2048,
		csp: {
			mode: 'auto',
			directives: {
				'default-src': ['self'],
				'script-src': ['self', 'unsafe-inline', 'https://www.googletagmanager.com', 'https://www.google-analytics.com', 'https://static.cloudflareinsights.com'],
				'style-src': ['self', 'unsafe-inline', 'fonts.googleapis.com'],
				'font-src': ['self', 'fonts.gstatic.com', 'data:'],
				'img-src': ['self', 'data:', 'https:', 'blob:'],
				'connect-src': ['self', 'ws:', 'wss:', 'https:', 'http://localhost:8000', 'http://127.0.0.1:8000', 'http://localhost:8001', 'http://127.0.0.1:8001'],
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
		}
	},
	compilerOptions: {}
};

export default config;

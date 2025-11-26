import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import sveltePreprocess from 'svelte-preprocess';

/**
 * Revolution Trading Pros - SvelteKit Configuration
 * Enterprise-grade SSR/SSG configuration with performance optimizations
 *
 * @version 2.0.0 - L8 Principal Engineer
 * @author Revolution Trading Pros
 */

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: [
		vitePreprocess(),
		sveltePreprocess({
			postcss: true
		})
	],
	onwarn: (warning, handler) => {
		// Allow accessibility warnings for WCAG compliance
		// Only suppress non-critical warnings that don't affect accessibility
		if (warning.code === 'css_unused_selector') return;
		handler(warning);
	},
	kit: {
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: 'index.html',
			strict: false
		}),
		prerender: {
			handleHttpError: ({ status, path }) => {
				// Ignore 404 errors for unimplemented routes
				if (status === 404) {
					return;
				}
				// Let other errors through
				throw new Error(`${status} ${path}`);
			},
			handleUnseenRoutes: 'ignore',
			// Concurrent prerendering for faster builds
			concurrency: 8, // Increased from 4 for faster builds
			crawl: true, // Auto-discover linked pages
			entries: ['*', '/sitemap.xml', '/robots.txt']
		},
		// Inline small CSS files for faster initial render
		inlineStyleThreshold: 1024,
		// Content Security Policy for better security
		csp: {
			mode: 'auto',
			directives: {
				'default-src': ['self'],
				'script-src': ['self', 'unsafe-inline'],
				'style-src': ['self', 'unsafe-inline', 'fonts.googleapis.com'],
				'font-src': ['self', 'fonts.gstatic.com'],
				'img-src': ['self', 'data:', 'https:']
			}
		},
		// Alias for cleaner imports
		alias: {
			$components: 'src/lib/components',
			$stores: 'src/lib/stores',
			$utils: 'src/lib/utils',
			$api: 'src/lib/api'
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

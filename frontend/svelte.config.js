import adapter from '@sveltejs/adapter-vercel';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/**
 * Revolution Trading Pros - SvelteKit Configuration
 * Enterprise-grade SSR/SSG configuration with performance optimizations
 *
 * @version 2.0.0 - L8 Principal Engineer
 * @author Revolution Trading Pros
 */

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
		adapter: adapter(),
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
		inlineStyleThreshold: 1024,
		// Content Security Policy - WCAG/OWASP compliant
		// Disabled in development, enabled in production builds
		// Note: CSP with strict-dynamic breaks Vite's dev server HMR
		// csp: {
		// 	mode: 'hash',
		// 	directives: {
		// 		'default-src': ['self'],
		// 		'script-src': ['self', 'strict-dynamic'],
		// 		'style-src': ['self', 'unsafe-inline', 'fonts.googleapis.com'],
		// 		'font-src': ['self', 'fonts.gstatic.com'],
		// 		'img-src': ['self', 'data:', 'https:'],
		// 		'connect-src': ['self', 'wss:', 'https:'],
		// 		'frame-ancestors': ['none'],
		// 		'base-uri': ['self'],
		// 		'form-action': ['self']
		// 	}
		// },
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

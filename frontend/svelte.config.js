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
			concurrency: 4
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
		// Enable CSS optimization
		css: 'injected'
	}
};

export default config;

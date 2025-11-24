import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import sveltePreprocess from 'svelte-preprocess';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: [
		vitePreprocess(),
		sveltePreprocess({
			postcss: true
		})
	],
	onwarn: (warning, handler) => {
		// Disable all a11y warnings
		if (warning.code && warning.code.startsWith('a11y_')) return;
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
			handleUnseenRoutes: 'ignore'
		}
	}
};

export default config;

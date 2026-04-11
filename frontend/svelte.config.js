import adapter from '@sveltejs/adapter-cloudflare';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	// Opt into Svelte 5.36+ top-level `await` in components, `$derived(await ...)`,
	// and async expressions in markup. Required for `<svelte:boundary pending>` flows
	// and for fully-streamed SSR with `$effect.pending()` / `settled()`.
	// The flag will be removed in Svelte 6 once stable.
	compilerOptions: {
		experimental: {
			async: true
		}
	},
	dynamicCompileOptions({ filename }) {
		if (filename.includes('node_modules')) {
			return { runes: false };
		}
		return { runes: true };
	},
	kit: {
		experimental: {
			remoteFunctions: true
		},
		adapter: adapter({
			routes: {
				include: ['/*'],
				// Use the placeholder groups exposed by `@sveltejs/adapter-cloudflare`
				// so newly added static files and prerendered routes are excluded
				// from the Cloudflare Function automatically. The previous explicit
				// list missed `/avatars/*`, `/data/*`, `/textures/*`, `/icon-{192,512}.png`,
				// `/grid-pattern.svg`, `/offline.html`, `/trading-room-rules.pdf`, plus
				// every prerendered route (sitemaps, blog, etc.), causing them to be
				// served by the Worker instead of straight from the edge.
				//
				//   <build>       — `.svelte-kit` build artifacts (covers `/_app/*`)
				//   <files>       — contents of `static/`
				//   <prerendered> — all prerendered routes (sitemap.xml, robots.txt, posts, ...)
				//   <redirects>   — paths declared in the root `_redirects` file
				exclude: ['<build>', '<files>', '<prerendered>', '<redirects>']
			}
		}),
		alias: {
			$lib: 'src/lib',
			$components: 'src/lib/components',
			$stores: 'src/lib/stores',
			$utils: 'src/lib/utils',
			$api: 'src/lib/api'
		},
		prerender: {
			handleHttpError: ({ status, path }) => {
				if (status === 404) {
					return;
				}
				if (status === 500 && path.startsWith('/dashboard')) {
					return;
				}
				if (status === 500 && path.startsWith('/tools/options-calculator')) {
					return;
				}
				throw new Error(`${status} ${path}`);
			},
			handleMissingId: 'ignore',
			handleUnseenRoutes: 'ignore',
			concurrency: 8,
			crawl: true,
			entries: ['*', '/robots.txt']
		},
		// Inline component styles smaller than this size (bytes) directly into
		// the HTML instead of emitting a separate render-blocking <link>. This
		// significantly improves FCP / LCP on slow networks because small per-
		// component CSS (most of our marketing sections) ships with the document.
		// 5 KB is the industry-standard sweet spot (PageSpeed / web.dev).
		inlineStyleThreshold: 5120,
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
					'https://revolution-trading-pros-api.fly.dev',
					'https://revolution-trading-pros.pages.dev',
					'https://www.googleapis.com',
					'https://www.google-analytics.com',
					'https://static.cloudflareinsights.com',
					'https://*.mediadelivery.net',
					'https://vz-5a23b520-193.b-cdn.net'
				],
				'frame-src': ['self', 'https://iframe.mediadelivery.net', 'https://*.mediadelivery.net'],
				'frame-ancestors': ['none'],
				'worker-src': ['self', 'blob:'],
				'base-uri': ['self'],
				'form-action': ['self']
			}
		},
		serviceWorker: {
			register: true
		},
		env: {
			publicPrefix: 'PUBLIC_'
		},
		output: {
			preloadStrategy: 'modulepreload'
		}
	}
};

export default config;

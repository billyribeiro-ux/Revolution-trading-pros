import adapter from '@sveltejs/adapter-cloudflare';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
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
				exclude: [
					'/favicon.ico',
					'/favicon.png',
					'/apple-touch-icon.png',
					'/apple-touch-icon-precomposed.png',
					'/manifest.json',
					'/robots.txt',
					'/sitemap.xml',
					'/sitemap-*.xml',
					'/_app/*',
					'/images/*',
					'/icons/*',
					'/logos/*',
					'/fonts/*',
					'/uploads/*',
					'/static/*'
				]
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
		inlineStyleThreshold: 0,
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
					'https://apis.google.com',
					'https://www.gstatic.com',
					'https://js.stripe.com'
				],
				'style-src': ['self', 'unsafe-inline', 'https://fonts.googleapis.com'],
				'font-src': ['self', 'https://fonts.gstatic.com', 'data:'],
				'img-src': ['self', 'data:', 'https:', 'blob:'],
				'media-src': [
					'self',
					'https://*.mediadelivery.net',
					'https://vz-5a23b520-193.b-cdn.net',
					'https://*.b-cdn.net',
					'https://video.bunnycdn.com',
					'https://pub-2e5bd1b702b440bd888a0fc47f3493ae.r2.dev',
					// Legacy course / partner VOD (e.g. Moxie quickstart MP4s)
					'https://*.s3.amazonaws.com'
				],
				'connect-src': [
					'self',
					'ws:',
					'wss:',
					'wss://revolution-trading-pros-api.fly.dev',
					'https://revolution-trading-pros-api.fly.dev',
					'https://revolution-trading-pros.pages.dev',
					'https://revolutiontradingpros.com',
					'https://www.revolutiontradingpros.com',
					'https://www.googleapis.com',
					'https://oauth2.googleapis.com',
					'https://accounts.google.com',
					'https://www.google-analytics.com',
					'https://analytics.google.com',
					'https://*.google-analytics.com',
					'https://stats.g.doubleclick.net',
					'https://www.googletagmanager.com',
					'https://static.cloudflareinsights.com',
					'https://cloudflareinsights.com',
					'https://*.mediadelivery.net',
					'https://vz-5a23b520-193.b-cdn.net',
					'https://video.bunnycdn.com',
					'https://api.stripe.com',
					'https://m.stripe.com',
					'https://r.stripe.com',
					'https://q.stripe.com',
					'https://merchant-ui-api.stripe.com'
				],
				'frame-src': [
					'self',
					'https://iframe.mediadelivery.net',
					'https://*.mediadelivery.net',
					'https://accounts.google.com',
					'https://docs.google.com',
					'https://js.stripe.com',
					'https://hooks.stripe.com',
					'https://www.paypal.com',
					'https://www.sandbox.paypal.com'
				],
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

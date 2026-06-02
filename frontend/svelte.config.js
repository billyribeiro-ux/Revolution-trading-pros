import adapter from '@sveltejs/adapter-cloudflare';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

// Dev-mode detection. NODE_ENV is unreliable here: Vite sets it AFTER
// svelte.config.js evaluates, so reading process.env.NODE_ENV at this
// scope returns undefined for `vite dev`. Detect via the CLI argv instead,
// which is populated before any module-load. Covers `vite dev`, `vite dev --host`,
// `pnpm dev`, and the `dev:fast` alias.
const isDev = process.env.NODE_ENV === 'development' || process.argv.includes('dev');

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	// Remote functions require BOTH `kit.experimental.remoteFunctions` AND
	// `compilerOptions.experimental.async` (per the SvelteKit remote-functions
	// docs). This repo previously enabled only the former, so any *client-side*
	// remote-query resolution (e.g. `await getAlerts()` in `onMount`) threw
	// `experimental_async_required` at runtime — silently breaking the dashboard
	// queries (explosive-swings threw it 29× on load). Server-resolved queries
	// (awaited in `load`, like memberships/favorites) were unaffected, which is
	// why the gap went unnoticed by build/typecheck. Enabling async fixes the
	// runtime with check 0/0 + production build green (verified 2026-06-02).
	// To revert: remove this `compilerOptions` block (client-side queries break
	// again; server-resolved ones keep working).
	compilerOptions: {
		experimental: {
			async: true
		}
	},
	// @sveltejs/vite-plugin-svelte v7 (Vite 8): inspector is first-party,
	// configured here. Disabled — use browser devtools instead.
	vitePlugin: {
		inspector: false
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
				exclude: ['<all>']
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
			// Serialized: adapter-cloudflare spins an emulated Miniflare/
			// workerd per prerender worker; concurrent workerd instances
			// contend on workerd's internal SQLite lock and crash the build
			// with `SQLITE_BUSY`/ERR_RUNTIME_FAILURE (CI-only; the real
			// Cloudflare runtime is unaffected). Build-time cost only.
			concurrency: 1,
			crawl: true,
			entries: ['*', '/robots.txt']
		},
		inlineStyleThreshold: 0,
		// FIX-M-1 (2026-04-29): tightened frontend CSP.
		//   - Added Stripe.js to script-src and api.stripe.com to connect-src
		//     (required for checkout flows; absence would silently break
		//     payments). frame-src includes js.stripe.com for Stripe Elements.
		//   - Removed the always-on http://localhost:8080 from connect-src.
		//     The dev-only branch below already adds it when NODE_ENV=development.
		//     Leaving it in production was harmless (browsers won't connect
		//     localhost from a remote origin) but it bloated the allowlist.
		//   - Kept 'unsafe-inline' on style-src: Svelte component-scoped
		//     styles compile to `style=""` attributes which require it.
		//     mode:'auto' adds nonces to <style> blocks but cannot rewrite
		//     inline style attributes. Removing it would break the UI.
		csp: {
			mode: 'auto',
			directives: {
				'default-src': ['self'],
				'script-src': [
					'self',
					'https://js.stripe.com',
					'https://www.googletagmanager.com',
					'https://www.google-analytics.com',
					'https://static.cloudflareinsights.com',
					'https://apis.google.com'
				],
				'style-src': ['self', 'unsafe-inline', 'fonts.googleapis.com'],
				'font-src': ['self', 'fonts.gstatic.com', 'data:'],
				'img-src': ['self', 'data:', 'https:', 'blob:'],
				'worker-src': ['self', ...(isDev ? ['blob:'] : [])],
				'media-src': [
					'self',
					'https://*.mediadelivery.net',
					'https://vz-5a23b520-193.b-cdn.net',
					'https://*.b-cdn.net',
					'https://pub-2e5bd1b702b440bd888a0fc47f3493ae.r2.dev'
				],
				'connect-src': [
					'self',
					'wss:',
					'https://revolution-trading-pros.pages.dev',
					'https://api.stripe.com',
					'https://www.googleapis.com',
					'https://www.google-analytics.com',
					'https://static.cloudflareinsights.com',
					'https://*.mediadelivery.net',
					'https://vz-5a23b520-193.b-cdn.net',
					// Dev-only: allow direct calls to the local Rust API,
					// Vite dev servers, and ws/wss for HMR. isDev is detected
					// from process.argv (NODE_ENV isn't set yet at config load).
					...(isDev
						? ['ws:', 'http://localhost:8080', 'http://localhost:5173', 'http://localhost:5174']
						: [])
				],
				'frame-src': [
					'self',
					'https://js.stripe.com',
					'https://hooks.stripe.com',
					'https://iframe.mediadelivery.net',
					'https://*.mediadelivery.net'
				],
				'frame-ancestors': ['none'],
				'base-uri': ['self'],
				'form-action': ['self']
			}
		},
		serviceWorker: {
			register: process.env.NODE_ENV !== 'development'
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

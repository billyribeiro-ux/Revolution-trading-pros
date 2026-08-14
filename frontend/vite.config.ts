import adapter from '@sveltejs/adapter-cloudflare';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import devtoolsJson from 'vite-plugin-devtools-json';
import tailwindcss from '@tailwindcss/vite';

// Dev-mode detection. NODE_ENV is unreliable here: Vite sets it AFTER this
// module evaluates, so reading process.env.NODE_ENV at this scope returns
// undefined for `vite dev`. Detect via the CLI argv instead, which is
// populated before any module-load. Covers `vite dev`, `vite dev --host`,
// `pnpm dev`, and the `dev:fast` alias.
const isDev = process.env.NODE_ENV === 'development' || process.argv.includes('dev');

// Build-unique app version, shared by Kit and the service worker.
//
// The service worker CANNOT get this from `$app/env`. In SvelteKit 3 that
// export is `BROWSER ? payload.version : __SVELTEKIT_APP_VERSION__`, and
// `payload` is only ever populated by the client app boot — never in a
// ServiceWorkerGlobalScope. So inside the worker it resolves to `undefined`,
// which silently pins every cache name to `cache-undefined` for all time and
// makes the `activate` purge a permanent no-op (it can never distinguish the
// previous deploy's cache from its own). `$service-worker` used to supply a
// real string in SvelteKit 2; nothing in `$app/*` replaces it today.
//
// Vite's `define` replacements ARE applied to the service-worker bundle, per
// the official service-workers docs, so that is the supported channel. Feeding
// the same value to `kit.version.name` keeps Kit's own version polling and the
// worker's cache names on one identifier.
//
// Prefer the deploy commit SHA (Cloudflare Pages and GitHub Actions both
// provide one) and fall back to a timestamp, which is what Kit itself defaults
// to.
const APP_VERSION =
	process.env.CF_PAGES_COMMIT_SHA || process.env.GITHUB_SHA || Date.now().toString();

// SvelteKit 3 takes its config as options to the `sveltekit()` Vite plugin —
// the old svelte.config.js is gone. Callback form so `mode` is available;
// dev-only plugins gated on mode === 'development'.
export default defineConfig(({ mode }) => ({
	plugins: [
		tailwindcss(),
		sveltekit({
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
			compilerOptions: { experimental: { async: true } },
			// `dynamicCompileOptions` removed here. It used to force
			// `runes: false` for everything under node_modules, which now breaks
			// runes-based dependencies: @tabler/icons-svelte-runes calls `$props()`,
			// and compiling it in legacy mode threw "Cannot access 'props' before
			// initialization" at render time (342 unit tests across 14 files).
			// Svelte 5 auto-detects runes mode per component, so first-party code
			// still compiles as runes without the override.
			experimental: { remoteFunctions: true },
			adapter: adapter({ routes: { include: ['/*'], exclude: ['<all>'] } }),
			// `alias` dropped in the SvelteKit 3 migration: `$lib` is replaced by
			// the `#lib/*` subpath import declared in package.json#imports, and
			// $components/$stores/$utils/$api had zero usages left in src/.
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
					'worker-src': ['self', ...(isDev ? (['blob:'] as const) : [])],
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
							? ([
									'ws:',
									'http://localhost:8080',
									'http://localhost:5173',
									'http://localhost:5174'
								] as const)
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
			serviceWorker: { register: process.env.NODE_ENV !== 'development' },
			// Pinned so the worker's `__APP_VERSION__` and Kit's version polling
			// agree on one identifier. Kit would otherwise default to its own
			// timestamp, which the worker has no way to read.
			version: { name: APP_VERSION }
			// `env.publicPrefix` and `output.preloadStrategy` are both gone in
			// SvelteKit 3. Client exposure is now declared per-variable in
			// src/env.ts (`public: true`) instead of by name prefix, and
			// 'modulepreload' was already the preload default.
		}),
		...(mode === 'development' ? [devtoolsJson()] : [])
	],
	// Applied to the service-worker bundle as well as client/server — the only
	// way to get a real version string into the worker. See APP_VERSION above.
	define: {
		__APP_VERSION__: JSON.stringify(APP_VERSION)
	},
	resolve: {
		// Force Svelte client bundle in tests (default resolves to server bundle
		// which doesn't have mount/unmount needed by @testing-library/svelte)
		conditions: ['browser']
	},
	test: {
		globals: true,
		environment: 'jsdom',
		setupFiles: ['./src/test/setup.ts'],
		include: ['src/**/*.{test,spec}.{js,ts}'],
		coverage: {
			provider: 'v8',
			reporter: ['text', 'json', 'html', 'lcov'],
			include: ['src/lib/**/*.{js,ts,svelte}'],
			exclude: ['**/*.test.{js,ts}', '**/*.spec.{js,ts}', '**/types.ts', '**/__tests__/**'],
			thresholds: {
				lines: 80,
				functions: 80,
				branches: 75,
				statements: 80
			}
		},
		mockReset: true,
		restoreMocks: true
	},
	server: {
		// Prefer 5173 (matches docs + tooling defaults) but fall back to the
		// next free port if it's taken. strictPort:true previously hard-failed
		// when 5173 was in use — that broke the "just run another dev server"
		// flow. Downstream tooling (playwright, scripts/preview-component.js,
		// SvelteKit +server.ts proxies) all read FRONTEND_URL/E2E_BASE_URL/
		// API_BASE_URL from env, so an alternate port is opt-in via env, not
		// a code change.
		port: 5173,
		strictPort: false,
		host: 'localhost'
	},
	optimizeDeps: {
		// Dynamically imported in-page (maintenance motion layer, charts). Pre-bundle
		// them so first discovery doesn't trigger a dev-server full page reload.
		include: ['gsap', 'gsap/ScrollTrigger', 'lightweight-charts']
	},
	build: {
		target: 'es2022',
		// Surface3D is an intentionally lazy WebGL/Three tab in the options
		// calculator. Its minified chunk is larger than Vite's generic 500 kB
		// default, but it is not part of the initial calculator route payload.
		chunkSizeWarningLimit: 700,
		// Vite 8 default CSS minifier is Lightning CSS (via Rolldown). Keep
		// esbuild until component <style> blocks are audited for Lightning CSS
		// compatibility — calc() inside media queries (e.g. Tailwind breakpoint
		// helpers) currently fails Lightning CSS strict parsing.
		// Revisit when Lightning CSS calc-in-media-query support lands.
		cssMinify: 'esbuild',
		rolldownOptions: {
			checks: {
				pluginTimings: false
			}
		}
	}
}));

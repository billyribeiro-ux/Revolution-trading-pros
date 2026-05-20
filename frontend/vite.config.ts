import { sveltekit } from '@sveltejs/kit/vite';
import { svelteInspector } from '@sveltejs/vite-plugin-svelte-inspector';
import { defineConfig } from 'vitest/config';
import devtoolsJson from 'vite-plugin-devtools-json';
import tailwindcss from '@tailwindcss/vite';

// FIX-2026-04-26: converted to callback form so `mode` is available; dev-only
// plugins are now gated on mode === 'development' (Phase 1.3).
export default defineConfig(({ mode }) => ({
	plugins: [
		tailwindcss(),
		sveltekit(),
		// FIX-2026-04-26: was unconditional — moved inside dev guard below
		// svelteInspector({ toggleKeyCombo: 'meta-shift', showToggleButton: 'always', toggleButtonPos: 'bottom-right' }),
		// FIX-2026-04-26: was unconditional — moved inside dev guard below
		// devtoolsJson(),
		...(mode === 'development'
			? [
					svelteInspector({
						toggleKeyCombo: 'meta-shift',
						showToggleButton: 'always',
						toggleButtonPos: 'bottom-right'
					}),
					devtoolsJson()
				]
			: [])
	],
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
	build: {
		target: 'es2022',
		chunkSizeWarningLimit: 500
	}
}));

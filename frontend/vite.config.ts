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
		port: 5173,
		strictPort: true,
		host: 'localhost'
	},
	build: {
		target: 'es2022',
		chunkSizeWarningLimit: 500
	}
}));

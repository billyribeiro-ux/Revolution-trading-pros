import { sveltekit } from '@sveltejs/kit/vite';
import { svelteInspector } from '@sveltejs/vite-plugin-svelte-inspector';
import { defineConfig } from 'vitest/config';
import devtoolsJson from 'vite-plugin-devtools-json';
import { fileURLToPath } from 'node:url';

export default defineConfig({
	plugins: [
		sveltekit(),
		svelteInspector({
			toggleKeyCombo: 'meta-shift',
			showToggleButton: 'always',
			toggleButtonPos: 'bottom-right'
		}),
		devtoolsJson()
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
		alias: {
			'@iconify/svelte': fileURLToPath(
				new URL('./src/test/__mocks__/IconifyMock.svelte', import.meta.url)
			)
		},
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
	build: {
		target: 'es2022',
		chunkSizeWarningLimit: 500
	},
	optimizeDeps: {
		exclude: [
			'@iconify/svelte',
			'svelte-inspect-value',
			'svelte-render-scan',
			'phosphor-svelte',
			'bits-ui',
			'@threlte/extras',
			'@threlte/core',
			'@melt-ui/svelte',
			'formsnap',
			'sveltekit-superforms',
			'@sveltejs/enhanced-img'
		]
	}
});

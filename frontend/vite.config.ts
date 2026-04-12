import { fileURLToPath } from 'node:url';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	// Only pin `browser` during Vitest — a global `conditions: ['browser']` breaks SSR
	// resolution for packages with Node/export maps (e.g. @iconify-json/*/icons.json).
	resolve: process.env.VITEST ? { conditions: ['browser'] } : undefined,
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
		chunkSizeWarningLimit: 500,
		rollupOptions: {
			output: {
				manualChunks(id) {
					if (id.includes('three') || id.includes('@threlte')) return 'vendor-three';
					if (id.includes('d3-') || id.includes('d3/')) return 'vendor-d3';
					if (id.includes('gsap')) return 'vendor-gsap';
					if (id.includes('options-calculator/engine')) return 'options-engine';
					return undefined;
				}
			}
		}
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

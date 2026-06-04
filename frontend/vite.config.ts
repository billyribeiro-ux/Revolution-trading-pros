import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import devtoolsJson from 'vite-plugin-devtools-json';
import tailwindcss from '@tailwindcss/vite';

// Callback form so `mode` is available; dev-only plugins gated on
// mode === 'development'. Using @sveltejs/vite-plugin-svelte v7 (Vite 8).
// Inspector configured via svelte.config.js -> vitePlugin.inspector.
export default defineConfig(({ mode }) => ({
	plugins: [tailwindcss(), sveltekit(), ...(mode === 'development' ? [devtoolsJson()] : [])],
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
		// Surface3D is an intentionally lazy WebGL/Three tab in the options
		// calculator. Its minified chunk is larger than Vite's generic 500 kB
		// default, but it is not part of the initial calculator route payload.
		chunkSizeWarningLimit: 700,
		// Vite 8 default CSS minifier is Lightning CSS (via Rolldown). Keep
		// esbuild until component <style> blocks are audited for Lightning CSS
		// compatibility — calc() inside media queries (e.g. Tailwind breakpoint
		// helpers) currently fails Lightning CSS strict parsing.
		// Revisit when Lightning CSS calc-in-media-query support lands.
		cssMinify: 'esbuild'
	}
}));

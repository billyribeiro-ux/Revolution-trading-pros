import { defineConfig, devices } from '@playwright/test';

// Dev server URL. Precedence: FRONTEND_URL > E2E_BASE_URL > localhost:5173.
// vite.config.ts uses strictPort:false now, so if 5173 is taken vite rolls
// to the next free port — set FRONTEND_URL (or E2E_BASE_URL) to the actual
// printed URL when running playwright against a non-default port.
const BASE_URL = process.env.FRONTEND_URL || process.env.E2E_BASE_URL || 'http://localhost:5173';

// Live Playwright surface (2026-05-19 — see PR for full audit):
//   frontend/tests/sitemap.test.ts                  — sitemap.xml validation
//   frontend/tests/accessibility/a11y-audit.spec.ts — axe-core a11y audit
// Both specs run chromium-only via `pnpm test:a11y` (a11y) or `pnpm exec
// playwright test tests/sitemap` (sitemap). tests/e2e/ was removed
// 2026-05-19; frontend/e2e/blog-editor/ was removed 2026-05-17. The blog
// editor SOURCE (frontend/src/lib/components/blog/BlockEditor/) is fully
// intact and shipping — only its old test scaffolding is gone.
// firefox/webkit/Mobile Chrome/Mobile Safari device projects were trimmed
// because the surviving specs don't exercise them, and 6 dead
// `test:blog-editor*` package.json scripts were removed alongside.
export default defineConfig({
	testDir: './tests',
	fullyParallel: true,
	forbidOnly: !!process.env.CI,
	retries: process.env.CI ? 2 : 0,
	workers: process.env.CI ? 1 : 4,
	timeout: 60_000,
	expect: { timeout: 10_000 },
	reporter: [
		['list'],
		['html', { open: 'never' }],
		['json', { outputFile: 'test-results/results.json' }],
		['junit', { outputFile: 'test-results/junit.xml' }]
	],
	use: {
		baseURL: BASE_URL,
		trace: 'on-first-retry',
		screenshot: 'only-on-failure',
		video: 'retain-on-failure',
		actionTimeout: 15_000,
		navigationTimeout: 30_000
	},
	projects: [
		{
			name: 'chromium',
			use: { ...devices['Desktop Chrome'] }
		}
	],
	webServer: {
		command: 'pnpm dev',
		url: BASE_URL,
		reuseExistingServer: !process.env.CI,
		timeout: 120_000
	}
});

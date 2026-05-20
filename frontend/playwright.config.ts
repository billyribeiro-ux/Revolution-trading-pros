import { defineConfig, devices } from '@playwright/test';

// Dev server URL. Precedence: FRONTEND_URL > E2E_BASE_URL > localhost:5173.
// vite.config.ts uses strictPort:false now, so if 5173 is taken vite rolls
// to the next free port — set FRONTEND_URL (or E2E_BASE_URL) to the actual
// printed URL when running playwright against a non-default port.
const BASE_URL = process.env.FRONTEND_URL || process.env.E2E_BASE_URL || 'http://localhost:5173';

export default defineConfig({
	testDir: './tests/e2e',
	// FIX-2026-04-26 (admin-audit): default to chromium-only when running the
	// admin sweep so we get one fast end-to-end pass instead of 5 redundant ones.
	// Opt into other browsers via `--project=firefox` etc.
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
		},
		{
			name: 'firefox',
			use: { ...devices['Desktop Firefox'] }
		},
		{
			name: 'webkit',
			use: { ...devices['Desktop Safari'] }
		},
		{
			name: 'Mobile Chrome',
			use: { ...devices['Pixel 5'] }
		},
		{
			name: 'Mobile Safari',
			use: { ...devices['iPhone 12'] }
		}
	],
	webServer: {
		command: 'pnpm dev',
		url: BASE_URL,
		reuseExistingServer: !process.env.CI,
		timeout: 120_000
	}
});

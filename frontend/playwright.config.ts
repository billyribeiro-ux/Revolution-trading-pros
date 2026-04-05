/// <reference types="node" />
import { defineConfig, devices } from '@playwright/test';

const isCI = !!process.env.CI;
/** CI workflows start `vite preview` and set E2E_BASE_URL (e.g. http://localhost:4173) — do not spawn a second dev server */
const baseURL = process.env.E2E_BASE_URL || 'http://localhost:5173';
const useExternalServer = !!process.env.E2E_BASE_URL;

/** Avoid saturating the Vite dev server with too many concurrent browsers (Playwright runs in Node; keep config free of node:os for editor TS) */
const localWorkers = (() => {
	const n = Number(process.env.PLAYWRIGHT_LOCAL_WORKERS);
	if (Number.isFinite(n) && n > 0) return Math.min(12, Math.floor(n));
	return 4;
})();
/** Full matrix: 300+ route × many browsers is slow; default to Chromium unless E2E_ALL_BROWSERS=1 */
const allBrowsers = process.env.E2E_ALL_BROWSERS === '1';
/** GitHub Actions e2e.yml uses --project=chromium|firefox|webkit and mobile projects; local dev defaults to Chromium only */
const ciBrowserProjects = [
	{ name: 'chromium', use: { ...devices['Desktop Chrome'] } },
	{ name: 'firefox', use: { ...devices['Desktop Firefox'] } },
	{ name: 'webkit', use: { ...devices['Desktop Safari'] } },
	{ name: 'Mobile Chrome', use: { ...devices['Pixel 5'] } },
	{ name: 'Mobile Safari', use: { ...devices['iPhone 12'] } }
];
const browserProjects = allBrowsers
	? ciBrowserProjects
	: isCI
		? ciBrowserProjects
		: [{ name: 'chromium', use: { ...devices['Desktop Chrome'] } }];

export default defineConfig({
	testDir: './tests/e2e',
	globalSetup: './tests/e2e/global-setup.ts',
	fullyParallel: true,
	forbidOnly: isCI,
	retries: isCI ? 2 : 0,
	workers: isCI ? 1 : localWorkers,
	timeout: 90_000,
	expect: { timeout: 20_000 },
	reporter: [
		['html'],
		['json', { outputFile: 'test-results/results.json' }],
		['junit', { outputFile: 'test-results/junit.xml' }]
	],
	use: {
		baseURL,
		trace: 'on-first-retry',
		screenshot: 'only-on-failure',
		video: 'retain-on-failure'
	},
	projects: browserProjects,
	...(useExternalServer
		? {}
		: {
				webServer: {
					command: 'pnpm run dev',
					url: 'http://localhost:5173',
					reuseExistingServer: !isCI,
					timeout: 180000,
					stdout: 'pipe',
					stderr: 'pipe'
				}
			})
});

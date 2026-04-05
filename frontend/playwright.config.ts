import { cpus } from 'node:os';
import { defineConfig, devices } from '@playwright/test';

const isCI = !!process.env.CI;
/** Avoid saturating the Vite dev server with too many concurrent browsers */
const localWorkers = Math.min(6, Math.max(2, cpus().length));
/** Full matrix: 300+ route × many browsers is slow; default to Chromium unless E2E_ALL_BROWSERS=1 */
const allBrowsers = process.env.E2E_ALL_BROWSERS === '1';
const browserProjects = allBrowsers
	? [
			{ name: 'chromium', use: { ...devices['Desktop Chrome'] } },
			{ name: 'firefox', use: { ...devices['Desktop Firefox'] } },
			{ name: 'webkit', use: { ...devices['Desktop Safari'] } },
			{ name: 'Mobile Chrome', use: { ...devices['Pixel 5'] } },
			{ name: 'Mobile Safari', use: { ...devices['iPhone 12'] } }
		]
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
		baseURL: 'http://localhost:5173',
		trace: 'on-first-retry',
		screenshot: 'only-on-failure',
		video: 'retain-on-failure'
	},
	projects: browserProjects,
	webServer: {
		command: 'pnpm run dev',
		url: 'http://localhost:5173',
		reuseExistingServer: !process.env.CI,
		timeout: 180000,
		stdout: 'pipe',
		stderr: 'pipe'
	}
});

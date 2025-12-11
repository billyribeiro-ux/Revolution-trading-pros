/**
 * Revolution Trading Pros - Playwright E2E Test Configuration
 *
 * Netflix L11+ Principal Engineer Grade Configuration
 * Designed for enterprise-scale testing with:
 * - Multi-browser support (Chromium, Firefox, WebKit)
 * - Parallel execution with smart sharding
 * - Comprehensive artifact collection on failures
 * - Environment-based configuration for local/staging/production
 * - Performance timing thresholds
 * - Accessibility testing integration hooks
 *
 * @see https://playwright.dev/docs/test-configuration
 */

import { defineConfig, devices } from '@playwright/test';

// Environment configuration with sensible defaults
const BASE_URL = process.env.E2E_BASE_URL || 'http://localhost:5174';
const API_URL = process.env.E2E_API_URL || 'http://localhost:8000/api';
const CI = !!process.env.CI;
const DEBUG = process.env.E2E_DEBUG === 'true';

// Timeout configuration (ms)
const TIMEOUTS = {
	test: CI ? 60_000 : 30_000,
	expect: CI ? 10_000 : 5_000,
	action: CI ? 30_000 : 15_000,
	navigation: CI ? 30_000 : 15_000,
	webServer: 180_000
};

export default defineConfig({
	// ========================================
	// TEST DIRECTORY & DISCOVERY
	// ========================================
	testDir: './tests/e2e',
	testMatch: /.*\.spec\.ts$/,

	// Run tests in files in parallel for speed
	fullyParallel: true,

	// Fail the build on CI if test.only is left in code
	forbidOnly: CI,

	// Retry on CI for flake resilience (Netflix standard: 2 retries)
	retries: CI ? 2 : 0,

	// Parallel workers - Netflix recommendation: half of CPUs on CI
	workers: CI ? '50%' : undefined,

	// Global timeout for entire test run (5 minutes on CI)
	globalTimeout: CI ? 300_000 : undefined,

	// ========================================
	// REPORTING
	// ========================================
	reporter: CI
		? [
				['list'],
				['html', { outputFolder: 'playwright-report', open: 'never' }],
				['json', { outputFile: 'test-results/results.json' }],
				['junit', { outputFile: 'test-results/junit.xml' }]
			]
		: [
				['list', { printSteps: true }],
				['html', { outputFolder: 'playwright-report', open: 'on-failure' }]
			],

	// Output directory for test artifacts
	outputDir: 'test-results',

	// ========================================
	// SHARED TEST SETTINGS
	// ========================================
	use: {
		// Base URL for all tests
		baseURL: BASE_URL,

		// Extra HTTP headers for API requests
		extraHTTPHeaders: {
			Accept: 'application/json',
			'X-Requested-With': 'Playwright'
		},

		// Collect trace on first retry (Netflix: balance debugging vs speed)
		trace: CI ? 'on-first-retry' : 'retain-on-failure',

		// Screenshot on failure
		screenshot: 'only-on-failure',

		// Video recording on failure
		video: CI ? 'on-first-retry' : 'retain-on-failure',

		// Timeouts
		actionTimeout: TIMEOUTS.action,
		navigationTimeout: TIMEOUTS.navigation,

		// Viewport size (standard desktop)
		viewport: { width: 1280, height: 720 },

		// Locale and timezone for consistent test behavior
		locale: 'en-US',
		timezoneId: 'America/New_York',

		// Ignore HTTPS errors for testing
		ignoreHTTPSErrors: true,

		// Bypass CSP for testing
		bypassCSP: true,

		// Color scheme
		colorScheme: 'light'
	},

	// Per-test timeout
	timeout: TIMEOUTS.test,

	// Expect timeout
	expect: {
		timeout: TIMEOUTS.expect,
		// Screenshot comparison thresholds
		toHaveScreenshot: {
			maxDiffPixels: 100,
			threshold: 0.2
		},
		toMatchSnapshot: {
			threshold: 0.2
		}
	},

	// ========================================
	// BROWSER PROJECTS
	// ========================================
	projects: [
		// ----------------
		// DESKTOP BROWSERS
		// ----------------
		{
			name: 'chromium',
			use: {
				...devices['Desktop Chrome'],
				channel: 'chromium'
			}
		},
		{
			name: 'firefox',
			use: {
				...devices['Desktop Firefox']
			}
		},
		{
			name: 'webkit',
			use: {
				...devices['Desktop Safari']
			}
		},

		// ----------------
		// MOBILE BROWSERS
		// ----------------
		{
			name: 'mobile-chrome',
			use: {
				...devices['Pixel 5']
			}
		},
		{
			name: 'mobile-safari',
			use: {
				...devices['iPhone 13']
			}
		},

		// ----------------
		// TABLET
		// ----------------
		{
			name: 'tablet',
			use: {
				...devices['iPad Pro 11']
			}
		},

		// ----------------
		// API TESTS (no browser needed)
		// ----------------
		{
			name: 'api',
			testMatch: /.*\.api\.spec\.ts$/,
			use: {
				baseURL: API_URL
			}
		}
	],

	// ========================================
	// WEB SERVER CONFIGURATION
	// ========================================
	webServer: {
		command: 'npm run dev',
		url: BASE_URL,
		reuseExistingServer: !CI,
		timeout: TIMEOUTS.webServer,
		stdout: DEBUG ? 'pipe' : 'ignore',
		stderr: 'pipe'
	},

	// ========================================
	// GLOBAL SETUP/TEARDOWN
	// ========================================
	globalSetup: './tests/e2e/global-setup.ts',
	globalTeardown: './tests/e2e/global-teardown.ts',

	// ========================================
	// METADATA
	// ========================================
	metadata: {
		project: 'Revolution Trading Pros',
		environment: CI ? 'ci' : 'local',
		baseUrl: BASE_URL,
		apiUrl: API_URL
	}
});

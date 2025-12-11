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
import * as dotenv from 'dotenv';
import * as path from 'path';

// Load environment variables from .env.e2e if it exists
dotenv.config({ path: path.resolve(__dirname, '.env.e2e') });

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
	reporter: [
		// Console output - minimal on CI, verbose locally
		['list', { printSteps: !CI }],
		// HTML report for detailed analysis
		[
			'html',
			{
				outputFolder: 'playwright-report',
				open: CI ? 'never' : 'on-failure'
			}
		],
		// JSON report for CI parsing
		['json', { outputFile: 'test-results/results.json' }],
		// JUnit for CI integration (GitHub Actions, Jenkins, etc.)
		['junit', { outputFile: 'test-results/junit.xml' }]
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

		// Ignore HTTPS errors for local development
		ignoreHTTPSErrors: !CI,

		// Bypass CSP for testing (dev only)
		bypassCSP: !CI,

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
		// SETUP PROJECT (runs first, used for auth state)
		// ----------------
		{
			name: 'setup',
			testMatch: /.*\.setup\.ts$/,
			use: {
				...devices['Desktop Chrome']
			}
		},

		// ----------------
		// DESKTOP BROWSERS
		// ----------------
		{
			name: 'chromium',
			use: {
				...devices['Desktop Chrome'],
				channel: 'chromium'
			},
			dependencies: ['setup']
		},
		{
			name: 'firefox',
			use: {
				...devices['Desktop Firefox']
			},
			dependencies: ['setup']
		},
		{
			name: 'webkit',
			use: {
				...devices['Desktop Safari']
			},
			dependencies: ['setup']
		},

		// ----------------
		// MOBILE BROWSERS
		// ----------------
		{
			name: 'mobile-chrome',
			use: {
				...devices['Pixel 5']
			},
			dependencies: ['setup']
		},
		{
			name: 'mobile-safari',
			use: {
				...devices['iPhone 13']
			},
			dependencies: ['setup']
		},

		// ----------------
		// TABLET
		// ----------------
		{
			name: 'tablet',
			use: {
				...devices['iPad Pro 11']
			},
			dependencies: ['setup']
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
	webServer: [
		// Frontend dev server
		{
			command: 'npm run dev',
			url: BASE_URL,
			reuseExistingServer: !CI,
			timeout: TIMEOUTS.webServer,
			stdout: DEBUG ? 'pipe' : 'ignore',
			stderr: 'pipe',
			env: {
				NODE_ENV: 'test'
			}
		}
		// Uncomment to also start backend:
		// {
		//   command: 'cd ../backend && php artisan serve',
		//   url: API_URL.replace('/api', '') + '/api/health/live',
		//   reuseExistingServer: !CI,
		//   timeout: TIMEOUTS.webServer,
		//   stdout: DEBUG ? 'pipe' : 'ignore',
		//   stderr: 'pipe',
		// },
	],

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

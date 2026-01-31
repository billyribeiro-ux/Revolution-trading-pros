/// <reference types="node" />
/**
 * Revolution Trading Pros - Playwright E2E Test Configuration
 * ============================================================
 *
 * Comprehensive test configuration for Playwright 1.50+ (January 2026)
 * Designed for enterprise-scale testing with:
 * - Multi-browser support (Chromium, Firefox, WebKit)
 * - Parallel execution with smart sharding
 * - Comprehensive artifact collection on failures
 * - Environment-based configuration for local/staging/production
 * - Performance timing thresholds
 * - Accessibility testing integration hooks
 * - Visual regression testing with screenshot comparisons
 *
 * @version 3.0.0
 * @see https://playwright.dev/docs/test-configuration
 */

import { defineConfig, devices } from '@playwright/test';

// =============================================================================
// Environment Configuration
// =============================================================================

const BASE_URL = process.env.E2E_BASE_URL || 'http://localhost:5174';
const API_URL = process.env.E2E_API_URL || 'https://revolution-trading-pros-api.fly.dev/api';
const CI = !!process.env.CI;
const DEBUG = process.env.E2E_DEBUG === 'true';
const SLOW_MO = DEBUG ? 100 : 0;

// =============================================================================
// Timeout Configuration (ms)
// =============================================================================

const TIMEOUTS = {
  test: CI ? 60_000 : 30_000,
  expect: CI ? 10_000 : 5_000,
  action: CI ? 30_000 : 15_000,
  navigation: CI ? 30_000 : 15_000,
  webServer: 180_000
};

// =============================================================================
// Performance Thresholds
// =============================================================================

const PERFORMANCE_THRESHOLDS = {
  editorRender: 100, // ms - Editor should render under 100ms
  blockCreation: 50, // ms - Block creation under 50ms
  pageLoad: 3000,    // ms - Page load under 3 seconds
  interaction: 100   // ms - UI interactions under 100ms
};

// =============================================================================
// Main Configuration
// =============================================================================

export default defineConfig({
  // ===========================================================================
  // TEST DIRECTORY & DISCOVERY
  // ===========================================================================
  testDir: './tests/e2e',

  // Also include the blog-editor e2e tests
  testMatch: [
    /.*\.spec\.ts$/,
    '../e2e/blog-editor/**/*.spec.ts'
  ],

  // Run tests in files in parallel for speed
  fullyParallel: true,

  // Fail the build on CI if test.only is left in code
  forbidOnly: CI,

  // Retry on CI for flake resilience
  retries: CI ? 2 : 0,

  // Parallel workers - half of CPUs on CI, auto locally
  workers: CI ? '50%' : undefined,

  // Global timeout for entire test run (10 minutes on CI)
  globalTimeout: CI ? 600_000 : undefined,

  // ===========================================================================
  // REPORTING
  // ===========================================================================
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

  // ===========================================================================
  // SHARED TEST SETTINGS
  // ===========================================================================
  use: {
    // Base URL for all tests
    baseURL: BASE_URL,

    // Collect trace on first retry (balance debugging vs speed)
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
    colorScheme: 'light',

    // Slow down actions for debugging
    launchOptions: {
      slowMo: SLOW_MO
    }
  },

  // Per-test timeout
  timeout: TIMEOUTS.test,

  // Expect timeout and screenshot thresholds
  expect: {
    timeout: TIMEOUTS.expect,
    // Screenshot comparison thresholds
    toHaveScreenshot: {
      maxDiffPixels: 100,
      maxDiffPixelRatio: 0.05,
      threshold: 0.2,
      animations: 'disabled'
    },
    toMatchSnapshot: {
      threshold: 0.2
    }
  },

  // ===========================================================================
  // BROWSER PROJECTS
  // ===========================================================================
  projects: [
    // -------------------------------------------------------------------------
    // SETUP - Global authentication
    // -------------------------------------------------------------------------
    {
      name: 'setup',
      testMatch: /global-setup\.ts/,
      teardown: 'cleanup'
    },
    {
      name: 'cleanup',
      testMatch: /global-teardown\.ts/
    },

    // -------------------------------------------------------------------------
    // DESKTOP BROWSERS
    // -------------------------------------------------------------------------
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

    // -------------------------------------------------------------------------
    // MOBILE BROWSERS
    // -------------------------------------------------------------------------
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

    // -------------------------------------------------------------------------
    // TABLET
    // -------------------------------------------------------------------------
    {
      name: 'tablet',
      use: {
        ...devices['iPad Pro 11']
      },
      dependencies: ['setup']
    },

    // -------------------------------------------------------------------------
    // BLOG EDITOR SPECIFIC - Desktop Only (Chromium)
    // -------------------------------------------------------------------------
    {
      name: 'blog-editor',
      testDir: './e2e/blog-editor',
      testMatch: /.*\.spec\.ts$/,
      use: {
        ...devices['Desktop Chrome'],
        channel: 'chromium',
        // Blog editor specific viewport
        viewport: { width: 1440, height: 900 },
        // Longer timeouts for editor operations
        actionTimeout: 20_000,
        navigationTimeout: 30_000
      },
      dependencies: ['setup']
    },

    // -------------------------------------------------------------------------
    // BLOG EDITOR - Firefox (Cross-browser)
    // -------------------------------------------------------------------------
    {
      name: 'blog-editor-firefox',
      testDir: './e2e/blog-editor',
      testMatch: /.*\.spec\.ts$/,
      use: {
        ...devices['Desktop Firefox'],
        viewport: { width: 1440, height: 900 }
      },
      dependencies: ['setup']
    },

    // -------------------------------------------------------------------------
    // BLOG EDITOR - Mobile (Responsive)
    // -------------------------------------------------------------------------
    {
      name: 'blog-editor-mobile',
      testDir: './e2e/blog-editor',
      testMatch: /.*\.spec\.ts$/,
      use: {
        ...devices['iPhone 13'],
        // Mobile specific settings
        hasTouch: true,
        isMobile: true
      },
      dependencies: ['setup']
    },

    // -------------------------------------------------------------------------
    // API TESTS (no browser needed)
    // -------------------------------------------------------------------------
    {
      name: 'api',
      testMatch: /.*\.api\.spec\.ts$/,
      use: {
        baseURL: API_URL
      }
    },

    // -------------------------------------------------------------------------
    // VISUAL REGRESSION ONLY
    // -------------------------------------------------------------------------
    {
      name: 'visual-regression',
      testDir: './e2e/blog-editor',
      testMatch: /visual-regression/,
      use: {
        ...devices['Desktop Chrome'],
        viewport: { width: 1920, height: 1080 }
      },
      dependencies: ['setup']
    },

    // -------------------------------------------------------------------------
    // ACCESSIBILITY TESTS
    // -------------------------------------------------------------------------
    {
      name: 'accessibility',
      testDir: './e2e/blog-editor',
      testMatch: /accessibility/,
      use: {
        ...devices['Desktop Chrome']
      },
      dependencies: ['setup']
    }
  ],

  // ===========================================================================
  // WEB SERVER CONFIGURATION
  // ===========================================================================
  webServer: {
    command: CI ? 'npm run preview' : 'npm run dev',
    url: BASE_URL,
    reuseExistingServer: !CI,
    timeout: TIMEOUTS.webServer,
    stdout: DEBUG ? 'pipe' : 'ignore',
    stderr: 'pipe'
  },

  // ===========================================================================
  // GLOBAL SETUP/TEARDOWN
  // ===========================================================================
  globalSetup: './tests/e2e/global-setup.ts',
  globalTeardown: './tests/e2e/global-teardown.ts',

  // ===========================================================================
  // SNAPSHOT SETTINGS
  // ===========================================================================
  snapshotDir: './test-results/snapshots',
  snapshotPathTemplate: '{testDir}/{testFilePath}-snapshots/{arg}{ext}',

  // ===========================================================================
  // METADATA
  // ===========================================================================
  metadata: {
    project: 'Revolution Trading Pros',
    environment: CI ? 'ci' : 'local',
    baseUrl: BASE_URL,
    apiUrl: API_URL,
    playwrightVersion: '1.50.0',
    performanceThresholds: PERFORMANCE_THRESHOLDS
  }
});

// =============================================================================
// Export Performance Thresholds for Tests
// =============================================================================
export { PERFORMANCE_THRESHOLDS };

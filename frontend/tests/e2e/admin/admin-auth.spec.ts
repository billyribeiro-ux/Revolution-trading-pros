/**
 * Revolution Trading Pros - Admin Authentication E2E Tests
 *
 * Apple ICT11+ Principal Engineer Grade Tests
 *
 * Comprehensive E2E tests for admin authentication:
 * - Admin login flow
 * - Admin API calls with authentication
 * - Unauthenticated access handling
 * - Token-based API requests
 * - Session management
 *
 * Note: Tests requiring backend API are automatically skipped when
 * backend is not available (e.g., in CI environments).
 *
 * @version 1.0.0
 */

import { test, expect, type Page, type ConsoleMessage } from '@playwright/test';
import { shouldSkipBackendTests, getBackendSkipReason } from '../helpers';

const BASE_URL = process.env.E2E_BASE_URL || 'http://localhost:5174';
const API_URL = process.env.E2E_API_URL || 'http://localhost:8000/api';

// Admin credentials from environment
const ADMIN_EMAIL = process.env.E2E_ADMIN_EMAIL || 'welberribeirodrums@gmail.com';
const ADMIN_PASSWORD = process.env.E2E_ADMIN_PASSWORD || '';

test.describe('Admin Authentication Flow', () => {
	test.describe('Unauthenticated Access', () => {
		test('admin dashboard redirects to login when not authenticated', async ({ page }) => {
			await page.goto(`${BASE_URL}/admin`);
			await page.waitForLoadState('networkidle');

			// Should redirect to login
			const url = page.url();
			expect(url).toMatch(/login/);
		});

		test('admin blog page redirects to login when not authenticated', async ({ page }) => {
			await page.goto(`${BASE_URL}/admin/blog`);
			await page.waitForLoadState('networkidle');

			const url = page.url();
			expect(url).toMatch(/login/);
		});

		test('admin settings page redirects to login when not authenticated', async ({ page }) => {
			await page.goto(`${BASE_URL}/admin/settings`);
			await page.waitForLoadState('networkidle');

			const url = page.url();
			expect(url).toMatch(/login/);
		});
	});

	test.describe('Admin API Responses', () => {
		test('admin API returns 401 JSON for unauthenticated requests', async ({ request }) => {
			// Skip if backend not available
			test.skip(shouldSkipBackendTests(), getBackendSkipReason());

			const response = await request.get(`${API_URL}/admin/posts`, {
				headers: {
					'Accept': 'application/json'
				}
			});

			expect(response.status()).toBe(401);

			const contentType = response.headers()['content-type'];
			expect(contentType).toContain('application/json');

			const data = await response.json();
			expect(data).toHaveProperty('message');
		});

		test('admin API returns 401 JSON for invalid token', async ({ request }) => {
			// Skip if backend not available
			test.skip(shouldSkipBackendTests(), getBackendSkipReason());

			const response = await request.get(`${API_URL}/admin/posts`, {
				headers: {
					'Accept': 'application/json',
					'Authorization': 'Bearer invalid-token-12345'
				}
			});

			expect(response.status()).toBe(401);

			const contentType = response.headers()['content-type'];
			expect(contentType).toContain('application/json');
		});
	});

	test.describe('Console Error Monitoring', () => {
		test('no JSON parse errors on admin pages when unauthenticated', async ({ page }) => {
			const consoleErrors: string[] = [];

			page.on('console', (msg: ConsoleMessage) => {
				if (msg.type() === 'error') {
					consoleErrors.push(msg.text());
				}
			});

			await page.goto(`${BASE_URL}/admin`);
			await page.waitForLoadState('networkidle');
			await page.waitForTimeout(2000);

			// Filter for JSON parse errors specifically
			const jsonErrors = consoleErrors.filter(
				(err) => err.includes('JSON') || err.includes('Unexpected token')
			);

			expect(jsonErrors).toHaveLength(0);
		});

		test('no history.pushState warnings on page load', async ({ page }) => {
			const consoleWarnings: string[] = [];

			page.on('console', (msg: ConsoleMessage) => {
				if (msg.type() === 'warning') {
					consoleWarnings.push(msg.text());
				}
			});

			await page.goto(`${BASE_URL}/`);
			await page.waitForLoadState('networkidle');
			await page.waitForTimeout(3000); // Wait for GA4 to potentially load

			// Filter for history.pushState warnings
			const historyWarnings = consoleWarnings.filter(
				(warn) => warn.includes('history.pushState') || warn.includes('history.replaceState')
			);

			expect(historyWarnings).toHaveLength(0);
		});

		test('no history.pushState warnings during navigation', async ({ page }) => {
			const consoleWarnings: string[] = [];

			page.on('console', (msg: ConsoleMessage) => {
				if (msg.type() === 'warning') {
					consoleWarnings.push(msg.text());
				}
			});

			// Navigate through multiple pages
			await page.goto(`${BASE_URL}/`);
			await page.waitForLoadState('networkidle');
			
			await page.goto(`${BASE_URL}/about`);
			await page.waitForLoadState('networkidle');
			
			await page.goto(`${BASE_URL}/contact`);
			await page.waitForLoadState('networkidle');

			await page.waitForTimeout(2000);

			const historyWarnings = consoleWarnings.filter(
				(warn) => warn.includes('history.pushState') || warn.includes('history.replaceState')
			);

			expect(historyWarnings).toHaveLength(0);
		});
	});
});

test.describe('Admin Authenticated Flow', () => {
	// Skip entire describe block if backend not available or no admin password
	test.skip(
		shouldSkipBackendTests() || !ADMIN_PASSWORD,
		shouldSkipBackendTests()
			? getBackendSkipReason()
			: 'Skipping authenticated tests - no admin password configured'
	);

	test.describe('Login and Access', () => {
		test('admin can login successfully', async ({ page }) => {
			await page.goto(`${BASE_URL}/login`);
			await page.waitForLoadState('networkidle');

			await page.locator('input[type="email"], input[name="email"]').fill(ADMIN_EMAIL);
			await page.locator('input[type="password"], input[name="password"]').fill(ADMIN_PASSWORD);
			await page.locator('button[type="submit"]').click();

			// Wait for redirect away from login
			await page.waitForURL((url) => !url.pathname.includes('/login'), {
				timeout: 15000
			});

			// Should be logged in
			const url = page.url();
			expect(url).not.toContain('/login');
		});

		test('admin can access dashboard after login', async ({ page }) => {
			// Login first
			await page.goto(`${BASE_URL}/login`);
			await page.locator('input[type="email"]').fill(ADMIN_EMAIL);
			await page.locator('input[type="password"]').fill(ADMIN_PASSWORD);
			await page.locator('button[type="submit"]').click();

			await page.waitForURL((url) => !url.pathname.includes('/login'), {
				timeout: 15000
			});

			// Navigate to admin
			await page.goto(`${BASE_URL}/admin`);
			await page.waitForLoadState('networkidle');

			// Should stay on admin page
			const url = page.url();
			expect(url).toContain('/admin');
		});

		test('admin API calls succeed with valid token', async ({ page }) => {
			// Login via UI to get token
			await page.goto(`${BASE_URL}/login`);
			await page.locator('input[type="email"]').fill(ADMIN_EMAIL);
			await page.locator('input[type="password"]').fill(ADMIN_PASSWORD);
			await page.locator('button[type="submit"]').click();

			await page.waitForURL((url) => !url.pathname.includes('/login'), {
				timeout: 15000
			});

			// Navigate to admin blog page
			await page.goto(`${BASE_URL}/admin/blog`);
			await page.waitForLoadState('networkidle');

			// Check for no errors in console
			const errors: string[] = [];
			page.on('console', (msg) => {
				if (msg.type() === 'error') {
					errors.push(msg.text());
				}
			});

			await page.waitForTimeout(3000);

			// Filter for API-related errors
			const apiErrors = errors.filter(
				(err) => err.includes('401') || err.includes('Unauthorized') || err.includes('JSON')
			);

			expect(apiErrors).toHaveLength(0);
		});
	});
});

test.describe('GA4 Integration', () => {
	test('GA4 loads without console warnings', async ({ page }) => {
		const warnings: string[] = [];

		page.on('console', (msg) => {
			if (msg.type() === 'warning') {
				warnings.push(msg.text());
			}
		});

		await page.goto(`${BASE_URL}/`);
		await page.waitForLoadState('networkidle');
		
		// Wait for GA4 to potentially initialize (deferred loading)
		await page.waitForTimeout(5000);

		// Check for SvelteKit router conflict warnings
		const routerWarnings = warnings.filter(
			(w) => w.includes('history.pushState') || 
			       w.includes('history.replaceState') ||
			       w.includes('SvelteKit')
		);

		expect(routerWarnings).toHaveLength(0);
	});

	test('GA4 gtag function is available after page load', async ({ page }) => {
		await page.goto(`${BASE_URL}/`);
		await page.waitForLoadState('networkidle');
		
		// Wait for deferred GA4 loading
		await page.waitForTimeout(3000);

		const hasGtag = await page.evaluate(() => {
			return typeof window.gtag === 'function';
		});

		// gtag should be available (even if GA4 ID is not configured, the stub should exist)
		expect(hasGtag).toBe(true);
	});

	test('dataLayer is initialized', async ({ page }) => {
		await page.goto(`${BASE_URL}/`);
		await page.waitForLoadState('networkidle');
		
		await page.waitForTimeout(3000);

		const hasDataLayer = await page.evaluate(() => {
			return Array.isArray(window.dataLayer);
		});

		expect(hasDataLayer).toBe(true);
	});
});

test.describe('adminFetch Utility', () => {
	test('adminFetch redirects to login on 401', async ({ page }) => {
		// This test verifies the adminFetch utility behavior
		// We'll simulate an unauthenticated admin page access
		
		await page.goto(`${BASE_URL}/admin/blog`);
		await page.waitForLoadState('networkidle');

		// Should redirect to login
		const url = page.url();
		expect(url).toMatch(/login/);
	});

	test('adminFetch includes redirect parameter', async ({ page }) => {
		await page.goto(`${BASE_URL}/admin/blog`);
		await page.waitForLoadState('networkidle');

		const url = page.url();
		
		// Should include redirect parameter pointing back to admin
		if (url.includes('/login')) {
			expect(url).toMatch(/redirect.*admin/);
		}
	});
});

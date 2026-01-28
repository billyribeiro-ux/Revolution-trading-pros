/// <reference types="node" />
/**
 * Authentication Fixtures for E2E Tests
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 *
 * Custom Playwright fixtures for authenticated testing:
 * - memberPage: Auto-login as regular member user
 * - adminPage: Auto-login as admin user
 * - unauthenticatedPage: Ensure clean unauthenticated state
 *
 * Usage:
 * ```typescript
 * import { test, expect } from '../fixtures/auth.fixture';
 *
 * test('member can view dashboard', async ({ memberPage }) => {
 *   await memberPage.goto('/dashboard');
 *   await expect(memberPage.locator('.dashboard')).toBeVisible();
 * });
 *
 * test('admin can create alert', async ({ adminPage }) => {
 *   await adminPage.goto('/dashboard/explosive-swings');
 *   await expect(adminPage.locator('[data-testid="create-alert-btn"]')).toBeVisible();
 * });
 * ```
 */

import { test as base, Page, expect } from '@playwright/test';

// Test user credentials from environment or defaults
const TEST_CREDENTIALS = {
	member: {
		email: process.env.E2E_TEST_EMAIL || 'test@example.com',
		password: process.env.E2E_TEST_PASSWORD || 'testpassword'
	},
	admin: {
		email: process.env.E2E_ADMIN_EMAIL || 'admin@example.com',
		password: process.env.E2E_ADMIN_PASSWORD || 'adminpassword'
	}
};

// Base URL from environment
const BASE_URL = process.env.E2E_BASE_URL || 'http://localhost:5174';

/**
 * Custom test fixtures extending Playwright's base test
 */
export const test = base.extend<{
	memberPage: Page;
	adminPage: Page;
	unauthenticatedPage: Page;
}>({
	/**
	 * memberPage fixture - automatically logs in as a member user
	 */
	memberPage: async ({ page }, use) => {
		await loginUser(page, TEST_CREDENTIALS.member);
		await use(page);
	},

	/**
	 * adminPage fixture - automatically logs in as an admin user
	 */
	adminPage: async ({ page }, use) => {
		await loginUser(page, TEST_CREDENTIALS.admin);
		await use(page);
	},

	/**
	 * unauthenticatedPage fixture - ensures clean unauthenticated state
	 */
	unauthenticatedPage: async ({ page }, use) => {
		// Clear any existing auth state
		await page.context().clearCookies();

		// Navigate to home to establish clean state
		await page.goto('/');
		await page.waitForLoadState('domcontentloaded');

		await use(page);
	}
});

/**
 * Helper function to perform login
 */
async function loginUser(
	page: Page,
	credentials: { email: string; password: string }
): Promise<void> {
	try {
		// Navigate to login page
		await page.goto('/login');
		await page.waitForLoadState('domcontentloaded');

		// Look for email input
		const emailInput = page
			.locator('input[type="email"], input[name*="email"], input[placeholder*="email" i]')
			.first();
		const hasEmailInput = await emailInput.isVisible().catch(() => false);

		if (!hasEmailInput) {
			console.log('Login form not found - may already be authenticated or page structure different');
			return;
		}

		// Fill email
		await emailInput.fill(credentials.email);

		// Fill password
		const passwordInput = page.locator('input[type="password"]').first();
		if (await passwordInput.isVisible().catch(() => false)) {
			await passwordInput.fill(credentials.password);
		}

		// Submit login form
		const submitBtn = page
			.locator(
				'button[type="submit"], button:has-text("Login"), button:has-text("Sign in"), button:has-text("Log in")'
			)
			.first();

		if (await submitBtn.isVisible().catch(() => false)) {
			await submitBtn.click();

			// Wait for navigation or response
			await page.waitForLoadState('networkidle').catch(() => {
				// Timeout is acceptable - may have been redirected already
			});

			// Brief wait for any auth state to settle
			await page.waitForTimeout(500);
		}

		// Verify we're logged in (not on login page anymore)
		const currentUrl = page.url();
		if (currentUrl.includes('/login') || currentUrl.includes('/auth')) {
			console.log('Still on login page after login attempt - credentials may be invalid');
		} else {
			console.log('Login successful - navigated away from login page');
		}
	} catch (error) {
		console.log('Login error:', (error as Error).message?.slice(0, 100));
	}
}

/**
 * Re-export expect from Playwright
 */
export { expect };

/**
 * Helper to check if user is authenticated
 */
export async function isAuthenticated(page: Page): Promise<boolean> {
	// Check for common auth indicators
	const authIndicators = [
		// User menu/avatar
		page.locator('[data-testid="user-menu"], .user-avatar, [class*="user-menu"]').first(),
		// Logout button
		page.locator('button:has-text("Logout"), button:has-text("Sign out"), [href="/logout"]').first(),
		// Dashboard access
		page.locator('[href="/dashboard"], a:has-text("Dashboard")').first()
	];

	for (const indicator of authIndicators) {
		if (await indicator.isVisible().catch(() => false)) {
			return true;
		}
	}

	// Check if we're on a protected route successfully
	const currentUrl = page.url();
	if (currentUrl.includes('/dashboard') && !currentUrl.includes('/login')) {
		return true;
	}

	return false;
}

/**
 * Helper to check if user has admin privileges
 */
export async function isAdmin(page: Page): Promise<boolean> {
	// Navigate to a page that shows admin controls
	await page.goto('/dashboard/explosive-swings');
	await page.waitForLoadState('domcontentloaded');

	// Look for admin-only controls
	const adminIndicators = [
		page.locator('[data-testid="create-alert-btn"]').first(),
		page.locator('button:has-text("Create Alert"), button:has-text("New Alert")').first(),
		page.locator('[href*="/admin"], a:has-text("Admin")').first(),
		page.locator('[data-testid="admin-panel"]').first()
	];

	for (const indicator of adminIndicators) {
		if (await indicator.isVisible().catch(() => false)) {
			return true;
		}
	}

	return false;
}

/**
 * Helper to get user role from page
 */
export async function getUserRole(page: Page): Promise<'admin' | 'member' | 'guest'> {
	if (await isAdmin(page)) {
		return 'admin';
	}

	if (await isAuthenticated(page)) {
		return 'member';
	}

	return 'guest';
}

/**
 * Storage state paths for authentication caching
 * Can be used to speed up tests by reusing auth state
 */
export const AUTH_STATE_PATHS = {
	member: '.auth/member-state.json',
	admin: '.auth/admin-state.json'
};

/**
 * Setup function to create authenticated storage states
 * Run this before tests to cache auth sessions
 */
export async function setupAuthStates(): Promise<void> {
	const { chromium } = await import('@playwright/test');

	const browser = await chromium.launch();

	// Setup member auth state
	try {
		const memberContext = await browser.newContext();
		const memberPage = await memberContext.newPage();
		await loginUser(memberPage, TEST_CREDENTIALS.member);

		if (await isAuthenticated(memberPage)) {
			await memberContext.storageState({ path: AUTH_STATE_PATHS.member });
			console.log('Member auth state saved');
		}
		await memberContext.close();
	} catch (error) {
		console.log('Failed to setup member auth state:', (error as Error).message);
	}

	// Setup admin auth state
	try {
		const adminContext = await browser.newContext();
		const adminPage = await adminContext.newPage();
		await loginUser(adminPage, TEST_CREDENTIALS.admin);

		if (await isAuthenticated(adminPage)) {
			await adminContext.storageState({ path: AUTH_STATE_PATHS.admin });
			console.log('Admin auth state saved');
		}
		await adminContext.close();
	} catch (error) {
		console.log('Failed to setup admin auth state:', (error as Error).message);
	}

	await browser.close();
}

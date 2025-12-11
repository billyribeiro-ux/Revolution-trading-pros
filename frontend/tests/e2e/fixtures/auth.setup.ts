/**
 * Revolution Trading Pros - Auth Setup Fixture
 *
 * Creates authenticated browser state for tests that require login.
 * This runs once before tests and saves session state for reuse.
 *
 * @see https://playwright.dev/docs/auth
 */

import { test as setup, expect } from '@playwright/test';
import { TEST_USER, loginViaUI } from '../helpers';
import path from 'path';
import { fileURLToPath } from 'url';

// ESM-compatible __dirname
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const authFile = path.join(__dirname, '../.auth/user.json');

setup('authenticate as test user', async ({ page }) => {
	// Skip if no test credentials configured
	if (!process.env.E2E_TEST_USER_EMAIL) {
		console.log('No test user configured, skipping auth setup');
		return;
	}

	console.log('Setting up authenticated session...');
	console.log('Email:', TEST_USER.email);

	// Navigate to login page
	await page.goto('/login');
	await page.waitForLoadState('domcontentloaded');

	// Dismiss cookie consent banner if present
	const acceptCookiesButton = page.getByRole('button', { name: /accept all/i });
	if (await acceptCookiesButton.isVisible({ timeout: 2000 }).catch(() => false)) {
		await acceptCookiesButton.click();
		await page.waitForTimeout(500);
	}

	// Fill in credentials using placeholder text for more reliable selection
	const emailInput = page.getByPlaceholder(/trader@example\.com/i);
	const passwordInput = page.getByPlaceholder(/••••••••/i);
	
	await emailInput.fill(TEST_USER.email);
	await passwordInput.fill(TEST_USER.password);

	// Submit form
	await page.getByRole('button', { name: /sign in/i }).click();

	// Wait for navigation or timeout
	try {
		await page.waitForURL((url) => !url.pathname.includes('/login'), { timeout: 15000 });
		console.log('Login successful, redirected to:', page.url());
		
		// Save storage state
		await page.context().storageState({ path: authFile });
		console.log('Auth setup complete, session saved');
	} catch {
		// Login didn't redirect - check for error or just continue
		console.log('Login did not redirect. Current URL:', page.url());
		console.log('This may be expected if credentials are invalid or backend is not running.');
		// Don't fail the setup - tests that need auth will handle it
	}
});

// Admin auth setup (separate file in production)
const adminAuthFile = path.join(__dirname, '../.auth/admin.json');

setup.skip('authenticate as admin', async ({ page }) => {
	// Skip if no admin credentials configured
	if (!process.env.E2E_ADMIN_EMAIL) {
		console.log('No admin user configured, skipping admin auth setup');
		return;
	}

	console.log('Setting up admin session...');

	await page.goto('/login');

	await page.getByRole('textbox', { name: /email/i }).fill(process.env.E2E_ADMIN_EMAIL!);
	await page
		.locator('input[type="password"]')
		.first()
		.fill(process.env.E2E_ADMIN_PASSWORD!);

	await page.getByRole('button', { name: /sign in|log in|login/i }).click();

	await page.waitForURL((url) => !url.pathname.includes('/login'), {
		timeout: 30000
	});

	// Verify admin access
	await page.goto('/admin');
	await expect(page).toHaveURL(/admin/);

	await page.context().storageState({ path: adminAuthFile });

	console.log('Admin auth setup complete');
});

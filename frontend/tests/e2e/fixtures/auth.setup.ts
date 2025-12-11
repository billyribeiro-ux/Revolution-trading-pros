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

	// Navigate to login page
	await page.goto('/login');

	// Fill in credentials
	await page.getByRole('textbox', { name: /email/i }).fill(TEST_USER.email);
	await page
		.locator('input[type="password"]')
		.first()
		.fill(TEST_USER.password);

	// Submit form
	await page.getByRole('button', { name: /sign in|log in|login/i }).click();

	// Wait for login to complete
	await page.waitForURL((url) => !url.pathname.includes('/login'), {
		timeout: 30000
	});

	// Verify login was successful
	await expect(
		page.locator('[data-testid="user-menu"], .user-menu, .dashboard')
	).toBeVisible({ timeout: 10000 });

	// Save storage state
	await page.context().storageState({ path: authFile });

	console.log('Auth setup complete, session saved');
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

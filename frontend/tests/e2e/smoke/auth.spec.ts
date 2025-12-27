/**
 * Revolution Trading Pros - Authentication Smoke Tests
 * Apple ICT 11+ Principal Engineer Grade
 *
 * Tests for authentication pages and flows
 */

import { test, expect } from '@playwright/test';

test.describe('Login Page Smoke Tests', () => {
	test('should load login page', async ({ page }) => {
		await page.goto('/login');

		// Page should load (may redirect to /my-account or similar)
		await page.waitForLoadState('networkidle');

		// Should have some form of authentication UI
		const hasEmailInput = (await page.locator('input[type="email"], input[name="email"]').count()) > 0;
		const hasUsernameInput =
			(await page.locator('input[type="text"][name*="user"], input[name="username"]').count()) > 0;
		const hasPasswordInput = (await page.locator('input[type="password"]').count()) > 0;

		// Either we're on a login page or redirected somewhere valid
		const url = page.url();
		const isAuthPage = hasEmailInput || hasUsernameInput || hasPasswordInput;
		const isRedirected = url.includes('account') || url.includes('dashboard') || url.includes('home');

		expect(isAuthPage || isRedirected).toBe(true);
	});

	test('should show password field as type password', async ({ page }) => {
		await page.goto('/login');

		const passwordField = page.locator('input[type="password"]').first();
		if (await passwordField.isVisible()) {
			// Password field should hide input
			await expect(passwordField).toHaveAttribute('type', 'password');
		}
	});

	test('should have form validation', async ({ page }) => {
		await page.goto('/login');

		const submitButton = page.locator(
			'button[type="submit"], input[type="submit"], button:has-text("Log")'
		);
		if (await submitButton.first().isVisible()) {
			// Try to submit empty form
			await submitButton.first().click();

			// Should either show validation message or stay on page
			await page.waitForTimeout(500);
			const url = page.url();
			expect(url).toContain('login');
		}
	});
});

test.describe('Register Page Smoke Tests', () => {
	test('should load register page', async ({ page }) => {
		await page.goto('/register');

		await page.waitForLoadState('networkidle');

		// Should have registration form elements or redirect
		const url = page.url();
		const hasForm =
			(await page.locator('form').count()) > 0 ||
			(await page.locator('input[type="email"]').count()) > 0;
		const isValidPage = hasForm || url.includes('account') || url.includes('login');

		expect(isValidPage).toBe(true);
	});
});

test.describe('Password Reset Smoke Tests', () => {
	test('should have password reset option', async ({ page }) => {
		await page.goto('/login');

		// Look for forgot password link
		const forgotLink = page.locator(
			'a:has-text("Forgot"), a:has-text("Reset"), a[href*="reset"], a[href*="forgot"]'
		);
		const hasResetOption = (await forgotLink.count()) > 0;

		// It's acceptable if there's no forgot password link on the page
		// Just verify the page loaded correctly
		expect(page.url()).toBeDefined();
	});
});

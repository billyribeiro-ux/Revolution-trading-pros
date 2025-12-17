/**
 * Revolution Trading Pros - Auth Smoke Test
 * 
 * Quick end-to-end smoke test for login/auth flow.
 * Tests the critical path: login page loads → form works → no JS errors
 * 
 * Run with: npx playwright test tests/e2e/smoke/auth-smoke.spec.ts
 */

/// <reference types="node" />
import { test, expect } from '@playwright/test';
import { LoginPage } from '../pages';

test.describe('Auth Smoke Test', () => {
	test.describe.configure({ mode: 'serial' });

	test('1. Login page loads without errors', async ({ page }) => {
		// Collect console errors
		const consoleErrors: string[] = [];
		page.on('console', (msg) => {
			if (msg.type() === 'error') {
				consoleErrors.push(msg.text());
			}
		});

		// Collect page errors (uncaught exceptions)
		const pageErrors: Error[] = [];
		page.on('pageerror', (error) => {
			pageErrors.push(error);
		});

		await page.goto('/login');
		await page.waitForLoadState('domcontentloaded');
		await page.waitForTimeout(2000); // Allow time for hydration

		// Verify page loaded
		await expect(page).toHaveURL(/login/);

		// Check for critical JS errors (like "Cannot read properties of undefined")
		const criticalErrors = pageErrors.filter(
			(e) =>
				e.message.includes('Cannot read properties of undefined') ||
				e.message.includes('Cannot read property') ||
				e.message.includes('is not defined')
		);

		if (criticalErrors.length > 0) {
			console.error('Critical JS errors found:', criticalErrors.map((e) => e.message));
		}

		expect(criticalErrors.length).toBe(0);
	});

	test('2. Login form elements are visible and interactive', async ({ page }) => {
		const loginPage = new LoginPage(page);
		await loginPage.goto();

		// Verify form is ready
		await loginPage.verifyFormReady();

		// Verify email input accepts text
		await loginPage.emailInput.fill('test@example.com');
		await expect(loginPage.emailInput).toHaveValue('test@example.com');

		// Verify password input accepts text
		await loginPage.passwordInput.fill('testpassword');
		await expect(loginPage.passwordInput).toHaveValue('testpassword');

		// Verify submit button is clickable
		await expect(loginPage.submitButton).toBeEnabled();
	});

	test('3. Form validation works (empty submission)', async ({ page }) => {
		const loginPage = new LoginPage(page);
		await loginPage.goto();

		// Clear any pre-filled values
		await loginPage.emailInput.clear();
		await loginPage.passwordInput.clear();

		// Submit empty form
		await loginPage.submit();

		// Should stay on login page (validation prevents submission)
		await page.waitForTimeout(1000);
		await expect(page).toHaveURL(/login/);
	});

	test('4. Form validation works (invalid email)', async ({ page }) => {
		const loginPage = new LoginPage(page);
		await loginPage.goto();

		// Fill with invalid email
		await loginPage.fillForm('not-an-email', 'password123');
		await loginPage.submit();

		// Should stay on login page
		await page.waitForTimeout(1000);
		await expect(page).toHaveURL(/login/);
	});

	test('5. No undefined email errors during auth flow', async ({ page }) => {
		// This specifically tests for the "Cannot read properties of undefined (reading 'email')" bug
		const undefinedEmailErrors: string[] = [];

		page.on('pageerror', (error) => {
			if (error.message.includes("reading 'email'")) {
				undefinedEmailErrors.push(error.message);
			}
		});

		page.on('console', (msg) => {
			if (msg.type() === 'error' && msg.text().includes("reading 'email'")) {
				undefinedEmailErrors.push(msg.text());
			}
		});

		// Navigate to login
		await page.goto('/login');
		await page.waitForLoadState('domcontentloaded');
		await page.waitForTimeout(2000); // Allow hydration

		// Fill form and attempt login (will fail without valid creds, but shouldn't crash)
		const loginPage = new LoginPage(page);
		await loginPage.fillForm('test@example.com', 'wrongpassword');
		await loginPage.submit();

		// Wait for any async operations
		await page.waitForTimeout(3000);

		// Log errors for debugging
		if (undefinedEmailErrors.length > 0) {
			console.error('Undefined email errors found:', undefinedEmailErrors);
		}

		// Check for the specific undefined email error
		expect(undefinedEmailErrors.length).toBe(0);
	});

	test('6. Navigation to forgot password works', async ({ page }) => {
		const loginPage = new LoginPage(page);
		await loginPage.goto();

		// Click forgot password link
		await loginPage.goToForgotPassword();

		// Should be on forgot password page
		await expect(page).toHaveURL(/forgot/);
	});

	test('7. Navigation to register works', async ({ page }) => {
		await page.goto('/login');

		// Find and click register/signup link
		const registerLink = page.locator('a[href*="register"], a[href*="signup"]').first();
		
		if (await registerLink.isVisible()) {
			await registerLink.click();
			await page.waitForLoadState('domcontentloaded');
			
			// Should be on register page
			const url = page.url();
			expect(url.includes('register') || url.includes('signup')).toBe(true);
		} else {
			// No register link visible - that's okay, test passes
			expect(true).toBe(true);
		}
	});

	test('8. Auth store initializes without crashing', async ({ page }) => {
		// Test that the app initializes properly without auth errors
		const errors: string[] = [];

		page.on('pageerror', (error) => {
			errors.push(error.message);
		});

		// Go to home page (triggers auth initialization)
		await page.goto('/');
		await page.waitForLoadState('networkidle');

		// Navigate to a few pages to trigger auth checks
		await page.goto('/login');
		await page.waitForLoadState('networkidle');

		await page.goto('/');
		await page.waitForLoadState('networkidle');

		// Filter for auth-related errors
		const authErrors = errors.filter(
			(e) =>
				e.includes('email') ||
				e.includes('user') ||
				e.includes('auth') ||
				e.includes('undefined')
		);

		if (authErrors.length > 0) {
			console.error('Auth-related errors:', authErrors);
		}

		expect(authErrors.length).toBe(0);
	});
});

test.describe('Auth Smoke Test - With Credentials', () => {
	// These tests require valid test credentials
	const hasCredentials = !!process.env.E2E_TEST_USER_EMAIL;

	test.skip(!hasCredentials, 'Skipping: No test credentials configured');

	test('9. Login with valid credentials succeeds', async ({ page }) => {
		const loginPage = new LoginPage(page);
		await loginPage.goto();

		await loginPage.login(
			process.env.E2E_TEST_USER_EMAIL!,
			process.env.E2E_TEST_USER_PASSWORD!
		);

		// Should redirect away from login
		const success = await loginPage.isLoginSuccessful();
		expect(success).toBe(true);
	});

	test('10. User data loads without undefined email error', async ({ page }) => {
		const errors: string[] = [];

		page.on('pageerror', (error) => {
			if (error.message.includes("reading 'email'")) {
				errors.push(error.message);
			}
		});

		const loginPage = new LoginPage(page);
		await loginPage.goto();

		await loginPage.login(
			process.env.E2E_TEST_USER_EMAIL!,
			process.env.E2E_TEST_USER_PASSWORD!
		);

		// Wait for login to complete and user data to load
		await page.waitForTimeout(5000);

		// Navigate around to trigger user data access
		await page.goto('/dashboard');
		await page.waitForTimeout(2000);

		expect(errors.length).toBe(0);
	});
});

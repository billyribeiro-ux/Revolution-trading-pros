/**
 * Revolution Trading Pros - Authentication Helper
 *
 * Provides reusable authentication utilities for E2E tests:
 * - Login/logout flows
 * - Session management
 * - Test user creation
 * - Auth state persistence
 *
 * Netflix L11+ Pattern: Centralized auth handling for consistency
 */

import { type Page, type BrowserContext, expect } from '@playwright/test';

/** Test user credentials configuration */
export interface TestUserCredentials {
	email: string;
	password: string;
	name?: string;
}

/** Authentication state that can be stored/restored */
export interface AuthState {
	cookies: Array<{
		name: string;
		value: string;
		domain: string;
		path: string;
	}>;
	localStorage: Record<string, string>;
}

/** Default test user (configurable via environment) */
export const TEST_USER: TestUserCredentials = {
	email: process.env.E2E_TEST_USER_EMAIL || 'test@example.com',
	password: process.env.E2E_TEST_USER_PASSWORD || 'TestPassword123!',
	name: process.env.E2E_TEST_USER_NAME || 'E2E Test User'
};

/** Admin test user */
export const ADMIN_USER: TestUserCredentials = {
	email: process.env.E2E_ADMIN_EMAIL || 'admin@example.com',
	password: process.env.E2E_ADMIN_PASSWORD || 'AdminPassword123!',
	name: 'Admin User'
};

/**
 * Logs in a user via the login form
 * Handles the full login flow including redirects
 */
export async function loginViaUI(
	page: Page,
	credentials: TestUserCredentials = TEST_USER
): Promise<void> {
	// Navigate to login page
	await page.goto('/login');
	await page.waitForLoadState('networkidle');

	// Fill in credentials
	await page.getByRole('textbox', { name: /email/i }).fill(credentials.email);
	await page.getByRole('textbox', { name: /password/i }).fill(credentials.password);

	// Submit form
	await page.getByRole('button', { name: /sign in|log in|login/i }).click();

	// Wait for navigation away from login page
	await page.waitForURL((url) => !url.pathname.includes('/login'), {
		timeout: 15000
	});

	// Verify we're logged in by checking for user-specific content
	await expect(page.locator('[data-testid="user-menu"], .user-menu, .dashboard')).toBeVisible({
		timeout: 10000
	});
}

/**
 * Logs in via API directly (faster, for setup)
 * Returns auth token for API requests
 * Returns null if backend is not available
 */
export async function loginViaAPI(
	page: Page,
	credentials: TestUserCredentials = TEST_USER
): Promise<string | null> {
	const apiUrl = process.env.E2E_API_URL || 'http://localhost:8000/api';

	// Skip if backend is not available
	if (process.env.BACKEND_AVAILABLE === 'false') {
		console.log('Skipping API login - backend not available');
		return null;
	}

	try {
		const response = await page.request.post(`${apiUrl}/login`, {
			data: {
				email: credentials.email,
				password: credentials.password
			},
			timeout: 5000
		});

		if (response.ok()) {
			const data = await response.json();
			const token = data.token || data.access_token;

			// Store token in localStorage for subsequent requests
			if (token) {
				await page.evaluate((t) => {
					localStorage.setItem('auth_token', t);
					localStorage.setItem('user_authenticated', 'true');
				}, token);
			}

			return token;
		}

		console.warn(`Login API returned ${response.status()}`);
		return null;
	} catch (error) {
		const errorMessage = error instanceof Error ? error.message : String(error);
		console.warn('API login failed:', errorMessage);
		return null;
	}
}

/**
 * Logs out the current user
 * Works even if backend is not available
 */
export async function logout(page: Page): Promise<void> {
	// Try API logout first if backend is available
	if (process.env.BACKEND_AVAILABLE !== 'false') {
		const apiUrl = process.env.E2E_API_URL || 'http://localhost:8000/api';

		try {
			await page.request.post(`${apiUrl}/logout`, { timeout: 3000 });
		} catch {
			// API might not be available - continue with client-side cleanup
		}
	}

	// Clear local storage
	await page.evaluate(() => {
		localStorage.removeItem('auth_token');
		localStorage.removeItem('user_authenticated');
		localStorage.clear();
	});

	// Clear cookies
	const context = page.context();
	await context.clearCookies();

	// Navigate to home page
	await page.goto('/');
}

/**
 * Saves auth state for reuse across tests
 */
export async function saveAuthState(context: BrowserContext, path: string): Promise<void> {
	await context.storageState({ path });
}

/**
 * Checks if user is currently authenticated
 */
export async function isAuthenticated(page: Page): Promise<boolean> {
	const token = await page.evaluate(() => {
		return localStorage.getItem('auth_token');
	});

	return !!token;
}

/**
 * Registers a new user via the registration form
 */
export async function registerViaUI(
	page: Page,
	credentials: TestUserCredentials & { confirmPassword?: string }
): Promise<void> {
	await page.goto('/register');
	await page.waitForLoadState('networkidle');

	// Fill in registration form
	if (credentials.name) {
		const nameInput = page.getByRole('textbox', { name: /name/i });
		if (await nameInput.isVisible()) {
			await nameInput.fill(credentials.name);
		}
	}

	await page.getByRole('textbox', { name: /email/i }).fill(credentials.email);
	await page.locator('input[type="password"]').first().fill(credentials.password);

	// Handle confirm password if present
	const confirmPasswordInput = page.locator('input[name="password_confirmation"], input[name="confirmPassword"]');
	if (await confirmPasswordInput.isVisible()) {
		await confirmPasswordInput.fill(credentials.confirmPassword || credentials.password);
	}

	// Accept terms if checkbox exists
	const termsCheckbox = page.getByRole('checkbox', { name: /terms|agree/i });
	if (await termsCheckbox.isVisible()) {
		await termsCheckbox.check();
	}

	// Submit
	await page.getByRole('button', { name: /register|sign up|create account/i }).click();

	// Wait for success (redirect or confirmation)
	await page.waitForURL(
		(url) => !url.pathname.includes('/register') || url.searchParams.has('success'),
		{ timeout: 15000 }
	);
}

/**
 * Registers a new user via API
 * Returns success: false if backend is not available
 */
export async function registerViaAPI(
	page: Page,
	credentials: TestUserCredentials
): Promise<{ success: boolean; userId?: number; message?: string }> {
	const apiUrl = process.env.E2E_API_URL || 'http://localhost:8000/api';

	// Skip if backend is not available
	if (process.env.BACKEND_AVAILABLE === 'false') {
		return { success: false, message: 'Backend not available' };
	}

	try {
		const response = await page.request.post(`${apiUrl}/register`, {
			data: {
				name: credentials.name,
				email: credentials.email,
				password: credentials.password,
				password_confirmation: credentials.password
			},
			timeout: 5000
		});

		const data = await response.json();

		if (response.ok()) {
			return { success: true, userId: data.user?.id };
		}

		return { success: false, message: data.message || 'Registration failed' };
	} catch (error) {
		return { success: false, message: String(error) };
	}
}

/**
 * Initiates password reset flow
 */
export async function requestPasswordReset(page: Page, email: string): Promise<void> {
	await page.goto('/forgot-password');
	await page.waitForLoadState('networkidle');

	await page.getByRole('textbox', { name: /email/i }).fill(email);
	await page.getByRole('button', { name: /reset|send|submit/i }).click();

	// Wait for confirmation
	await expect(page.getByText(/check your email|sent|success/i)).toBeVisible({
		timeout: 10000
	});
}

/**
 * Generates a unique test email
 */
export function generateTestEmail(prefix: string = 'test'): string {
	const timestamp = Date.now();
	const random = Math.random().toString(36).substring(7);
	return `${prefix}-${timestamp}-${random}@test.example.com`;
}

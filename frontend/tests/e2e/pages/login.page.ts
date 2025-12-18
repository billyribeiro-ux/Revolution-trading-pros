/**
 * Revolution Trading Pros - Login Page Object
 *
 * Page object for the login page with:
 * - Form interactions
 * - Validation handling
 * - Error message checks
 * - Success navigation
 */

import { type Page, type Locator, expect } from '@playwright/test';
import { BasePage } from './base.page';

export class LoginPage extends BasePage {
	// Form elements
	readonly emailInput: Locator;
	readonly passwordInput: Locator;
	readonly submitButton: Locator;
	readonly rememberMeCheckbox: Locator;
	readonly forgotPasswordLink: Locator;
	readonly registerLink: Locator;

	// Messages
	readonly errorMessage: Locator;
	readonly successMessage: Locator;

	// Social login buttons
	readonly googleLoginButton: Locator;

	constructor(page: Page) {
		super(page);

		// Form elements - using ID-based selection for reliability
		this.emailInput = page.locator('#email');
		this.passwordInput = page.locator('#password');
		this.submitButton = page.locator('button[type="submit"].submit-btn').or(
			page.getByRole('button', { name: /sign in to trade/i })
		).first();
		this.rememberMeCheckbox = page.locator(
			'input[type="checkbox"][name="remember"], [data-testid="remember-me"]'
		);
		this.forgotPasswordLink = page.locator(
			'a[href*="forgot"], a:has-text("Forgot"), [data-testid="forgot-password"]'
		);
		this.registerLink = page.locator(
			'a[href*="register"], a[href*="signup"], a:has-text("Sign Up"), a:has-text("Register")'
		);

		// Messages
		this.errorMessage = page.locator(
			'.error, .alert-error, [data-testid="error-message"], [role="alert"]'
		);
		this.successMessage = page.locator(
			'.success, .alert-success, [data-testid="success-message"]'
		);

		// Social login
		this.googleLoginButton = page.locator(
			'button:has-text("Google"), [data-testid="google-login"]'
		);
	}

	get path(): string {
		return '/login';
	}

	/**
	 * Fills in the login form
	 */
	async fillForm(email: string, password: string): Promise<void> {
		await this.emailInput.waitFor({ state: 'visible', timeout: 10000 });
		await this.emailInput.fill(email);
		await this.passwordInput.fill(password);
	}

	/**
	 * Submits the login form
	 */
	async submit(): Promise<void> {
		await this.submitButton.waitFor({ state: 'visible', timeout: 10000 });
		await this.submitButton.click();
		// Wait for form submission to start processing
		await this.page.waitForTimeout(1000);
	}

	/**
	 * Complete login flow
	 */
	async login(email: string, password: string, options?: { rememberMe?: boolean }): Promise<void> {
		await this.fillForm(email, password);

		if (options?.rememberMe) {
			await this.rememberMeCheckbox.check();
		}

		await this.submit();
	}

	/**
	 * Waits for successful login (navigation away from login page)
	 */
	async waitForLoginSuccess(timeout: number = 15000): Promise<void> {
		await this.page.waitForURL((url) => !url.pathname.includes('/login'), { timeout });
	}

	/**
	 * Checks if login was successful
	 */
	async isLoginSuccessful(): Promise<boolean> {
		try {
			await this.waitForLoginSuccess(20000);
			return true;
		} catch {
			return false;
		}
	}

	/**
	 * Gets error message text
	 */
	async getErrorMessage(): Promise<string | null> {
		try {
			await this.errorMessage.waitFor({ state: 'visible', timeout: 5000 });
			return await this.errorMessage.textContent();
		} catch {
			return null;
		}
	}

	/**
	 * Verifies error message is displayed
	 */
	async verifyErrorDisplayed(expectedText?: string | RegExp): Promise<void> {
		await expect(this.errorMessage).toBeVisible();
		if (expectedText) {
			await expect(this.errorMessage).toContainText(expectedText);
		}
	}

	/**
	 * Verifies form is visible and ready
	 */
	async verifyFormReady(): Promise<void> {
		await expect(this.emailInput).toBeVisible();
		await expect(this.passwordInput).toBeVisible();
		await expect(this.submitButton).toBeVisible();
		await expect(this.submitButton).toBeEnabled();
	}

	/**
	 * Navigates to forgot password page
	 */
	async goToForgotPassword(): Promise<void> {
		await this.forgotPasswordLink.click();
		await this.page.waitForURL('**/forgot-password**');
	}

	/**
	 * Navigates to registration page
	 */
	async goToRegister(): Promise<void> {
		await this.registerLink.click();
		await this.page.waitForURL('**/register**');
	}

	/**
	 * Tests form validation by submitting empty form
	 */
	async verifyRequiredValidation(): Promise<void> {
		// Clear inputs and submit
		await this.emailInput.clear();
		await this.passwordInput.clear();
		await this.submit();

		// Should either show validation error or stay on page
		const url = this.page.url();
		expect(url).toContain('/login');
	}

	/**
	 * Tests email format validation
	 */
	async verifyEmailFormatValidation(): Promise<void> {
		await this.fillForm('invalid-email', 'password123');
		await this.submit();

		// Should show validation error
		const hasError = await this.errorMessage.isVisible().catch(() => false);
		const stayedOnPage = this.page.url().includes('/login');

		expect(hasError || stayedOnPage).toBe(true);
	}

	/**
	 * Checks if form inputs have proper labels for accessibility
	 */
	async verifyAccessibility(): Promise<void> {
		// Check for associated labels or aria-labels
		const emailLabel = this.page.locator('label[for="email"], label:has-text("Email")');
		const passwordLabel = this.page.locator('label[for="password"], label:has-text("Password")');

		const hasEmailLabel = await emailLabel.isVisible().catch(() => false);
		const hasPasswordLabel = await passwordLabel.isVisible().catch(() => false);

		// Or check aria attributes
		const emailAriaLabel = await this.emailInput.getAttribute('aria-label');
		const passwordAriaLabel = await this.passwordInput.getAttribute('aria-label');

		expect(hasEmailLabel || emailAriaLabel).toBeTruthy();
		expect(hasPasswordLabel || passwordAriaLabel).toBeTruthy();
	}
}

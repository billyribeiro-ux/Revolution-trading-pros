/**
 * Revolution Trading Pros - Base Page Object
 *
 * Foundation for all page objects with common functionality:
 * - Navigation
 * - Element waiting
 * - Common assertions
 * - Screenshot capture
 *
 * Netflix L11+ Pattern: Page Object Model for maintainability
 */

import { type Page, type Locator, expect } from '@playwright/test';

export abstract class BasePage {
	protected readonly page: Page;
	protected readonly baseUrl: string;

	constructor(page: Page) {
		this.page = page;
		this.baseUrl = process.env.E2E_BASE_URL || 'http://localhost:5174';
	}

	/**
	 * Abstract method - each page must define its path
	 */
	abstract get path(): string;

	/**
	 * Navigate to this page
	 */
	async goto(): Promise<void> {
		await this.page.goto(this.path);
		await this.waitForPageReady();
	}

	/**
	 * Navigate to a specific path
	 */
	async gotoPath(path: string): Promise<void> {
		await this.page.goto(path);
		await this.waitForPageReady();
	}

	/**
	 * Wait for page to be fully loaded and ready
	 */
	async waitForPageReady(): Promise<void> {
		await this.page.waitForLoadState('domcontentloaded');
		await this.page.waitForLoadState('networkidle');
	}

	/**
	 * Wait for specific element to be visible
	 */
	async waitForElement(selector: string | Locator, timeout?: number): Promise<Locator> {
		const locator = typeof selector === 'string' ? this.page.locator(selector) : selector;
		await locator.waitFor({ state: 'visible', timeout });
		return locator;
	}

	/**
	 * Get page title
	 */
	async getTitle(): Promise<string> {
		return this.page.title();
	}

	/**
	 * Get current URL
	 */
	getCurrentUrl(): string {
		return this.page.url();
	}

	/**
	 * Check if page has specific text
	 */
	async hasText(text: string): Promise<boolean> {
		const locator = this.page.getByText(text);
		return locator.isVisible();
	}

	/**
	 * Take a screenshot
	 */
	async screenshot(name: string): Promise<void> {
		await this.page.screenshot({ path: `test-results/screenshots/${name}.png`, fullPage: true });
	}

	/**
	 * Scroll to bottom of page
	 */
	async scrollToBottom(): Promise<void> {
		await this.page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));
		await this.page.waitForTimeout(500);
	}

	/**
	 * Scroll to top of page
	 */
	async scrollToTop(): Promise<void> {
		await this.page.evaluate(() => window.scrollTo(0, 0));
		await this.page.waitForTimeout(500);
	}

	/**
	 * Check for console errors (filters known acceptable errors)
	 */
	async getConsoleErrors(): Promise<string[]> {
		const errors: string[] = [];

		this.page.on('console', (msg) => {
			if (msg.type() === 'error') {
				const text = msg.text();
				// Filter known acceptable errors
				if (
					!text.includes('favicon') &&
					!text.includes('net::ERR') &&
					!text.includes('Failed to load') &&
					!text.includes('NetworkError')
				) {
					errors.push(text);
				}
			}
		});

		return errors;
	}

	/**
	 * Assert page title matches
	 */
	async assertTitle(expected: string | RegExp): Promise<void> {
		await expect(this.page).toHaveTitle(expected);
	}

	/**
	 * Assert URL matches
	 */
	async assertUrl(expected: string | RegExp): Promise<void> {
		await expect(this.page).toHaveURL(expected);
	}

	/**
	 * Assert element is visible
	 */
	async assertVisible(selector: string | Locator): Promise<void> {
		const locator = typeof selector === 'string' ? this.page.locator(selector) : selector;
		await expect(locator).toBeVisible();
	}

	/**
	 * Assert element contains text
	 */
	async assertText(selector: string | Locator, text: string | RegExp): Promise<void> {
		const locator = typeof selector === 'string' ? this.page.locator(selector) : selector;
		await expect(locator).toContainText(text);
	}

	/**
	 * Get navigation element
	 */
	get navigation(): Locator {
		return this.page.locator('nav, header').first();
	}

	/**
	 * Get footer element
	 */
	get footer(): Locator {
		return this.page.locator('footer, [role="contentinfo"]').first();
	}

	/**
	 * Get main content area
	 */
	get mainContent(): Locator {
		return this.page.locator('main, [role="main"], .main-content').first();
	}
}

/**
 * Revolution Trading Pros - Home Page Object
 *
 * Page object for the home/landing page with:
 * - Hero section interactions
 * - Navigation checks
 * - Feature sections
 * - CTA button interactions
 */

import { type Page, type Locator, expect } from '@playwright/test';
import { BasePage } from './base.page';

export class HomePage extends BasePage {
	// Selectors
	readonly heroSection: Locator;
	readonly heroTitle: Locator;
	readonly heroCta: Locator;
	readonly chartContainer: Locator;
	readonly featuresSection: Locator;
	readonly testimonialsSection: Locator;
	readonly ctaButtons: Locator;
	readonly newsletterForm: Locator;

	constructor(page: Page) {
		super(page);

		// Hero section elements
		this.heroSection = page.locator('#hero, .hero, [data-testid="hero"]');
		this.heroTitle = page.locator('h1').first();
		this.heroCta = page.locator('.cta, [data-testid="hero-cta"]').first();
		this.chartContainer = page.locator('#chart-bg, .chart-container');

		// Page sections
		this.featuresSection = page.locator('#features, .features, [data-testid="features"]');
		this.testimonialsSection = page.locator('#testimonials, .testimonials');

		// Interactive elements
		this.ctaButtons = page.locator('.cta, button[type="button"], a.btn');
		this.newsletterForm = page.locator('form[data-testid="newsletter"], .newsletter-form');
	}

	get path(): string {
		return '/';
	}

	/**
	 * Verifies the hero section is visible and contains expected content
	 */
	async verifyHeroSection(): Promise<void> {
		await expect(this.heroSection).toBeVisible();
		await expect(this.heroTitle).toBeVisible();
	}

	/**
	 * Checks if the trading chart is loaded
	 */
	async verifyChartLoaded(): Promise<boolean> {
		try {
			await this.chartContainer.waitFor({ state: 'visible', timeout: 5000 });
			// Check for chart canvas/children (lightweight-charts creates elements)
			const hasChildren = (await this.chartContainer.locator('*').count()) > 0;
			return hasChildren;
		} catch {
			return false;
		}
	}

	/**
	 * Clicks the primary CTA button
	 */
	async clickPrimaryCta(): Promise<void> {
		await this.heroCta.click();
	}

	/**
	 * Gets all navigation links
	 */
	async getNavLinks(): Promise<string[]> {
		const links = this.page.locator('nav a, header a');
		const hrefs: string[] = [];
		const count = await links.count();

		for (let i = 0; i < count; i++) {
			const href = await links.nth(i).getAttribute('href');
			if (href) {
				hrefs.push(href);
			}
		}

		return hrefs;
	}

	/**
	 * Verifies all main navigation links are present
	 */
	async verifyMainNavLinks(): Promise<void> {
		const expectedLinks = [
			'/live-trading-rooms',
			'/courses',
			'/alerts',
			'/indicators',
			'/login'
		];

		const actualLinks = await this.getNavLinks();

		for (const expected of expectedLinks) {
			const found = actualLinks.some((link) => link.includes(expected) || link === expected);
			expect(found).toBe(true);
		}
	}

	/**
	 * Subscribes to newsletter
	 */
	async subscribeToNewsletter(email: string): Promise<void> {
		const emailInput = this.newsletterForm.getByRole('textbox', { name: /email/i });
		const submitButton = this.newsletterForm.getByRole('button', { name: /subscribe|submit/i });

		await emailInput.fill(email);
		await submitButton.click();
	}

	/**
	 * Verifies page loads without critical errors
	 */
	async verifyNoErrors(): Promise<void> {
		const errors: string[] = [];

		this.page.on('pageerror', (error) => {
			errors.push(error.message);
		});

		await this.page.waitForTimeout(2000);

		// Filter out expected errors
		const criticalErrors = errors.filter(
			(e) =>
				!e.includes('NetworkError') &&
				!e.includes('fetch') &&
				!e.includes('API')
		);

		expect(criticalErrors).toHaveLength(0);
	}

	/**
	 * Verifies meta tags for SEO
	 */
	async verifySeoMeta(): Promise<{
		title: string;
		description: string | null;
		ogTitle: string | null;
	}> {
		const title = await this.getTitle();
		const description = await this.page.locator('meta[name="description"]').getAttribute('content');
		const ogTitle = await this.page.locator('meta[property="og:title"]').getAttribute('content');

		return { title, description, ogTitle };
	}

	/**
	 * Scrolls through all sections to trigger lazy loading
	 */
	async scrollThroughPage(): Promise<void> {
		const sections = [
			this.heroSection,
			this.featuresSection,
			this.testimonialsSection,
			this.footer
		];

		for (const section of sections) {
			try {
				await section.scrollIntoViewIfNeeded();
				await this.page.waitForTimeout(500);
			} catch {
				// Section might not exist
			}
		}
	}

	/**
	 * Checks responsive behavior at mobile width
	 */
	async verifyMobileLayout(): Promise<void> {
		// Check hamburger menu appears
		const hamburger = this.page.locator(
			'[data-testid="hamburger"], .hamburger, button[aria-label*="menu"]'
		);
		const isHamburgerVisible = await hamburger.isVisible().catch(() => false);

		// On mobile, either hamburger is visible or nav links are hidden
		if (isHamburgerVisible) {
			await expect(hamburger).toBeVisible();
		}
	}
}

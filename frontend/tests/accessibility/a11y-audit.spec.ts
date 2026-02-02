/**
 * Accessibility Audit Tests
 * ═══════════════════════════════════════════════════════════════════════════
 * Automated WCAG 2.1 AA compliance testing
 */

import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test.describe('CMS Editor Accessibility', () => {
	test.beforeEach(async ({ page }) => {
		// Navigate to editor page
		await page.goto('/admin/blog/new');
		await page.waitForLoadState('networkidle');
	});

	test('should not have any WCAG 2.1 AA violations', async ({ page }) => {
		const accessibilityScanResults = await new AxeBuilder({ page })
			.withTags(['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa'])
			.exclude('.third-party-widget') // Exclude third-party elements
			.analyze();

		// Log any violations for debugging
		if (accessibilityScanResults.violations.length > 0) {
			console.log('Accessibility violations:', JSON.stringify(accessibilityScanResults.violations, null, 2));
		}

		expect(accessibilityScanResults.violations).toEqual([]);
	});

	test('should have proper heading hierarchy', async ({ page }) => {
		const headings = await page.$$eval('h1, h2, h3, h4, h5, h6', (elements) =>
			elements.map((el) => ({
				level: parseInt(el.tagName[1]),
				text: el.textContent?.trim()
			}))
		);

		// Check heading order
		let lastLevel = 0;
		for (const heading of headings) {
			// Should not skip more than one level
			expect(heading.level).toBeLessThanOrEqual(lastLevel + 2);
			lastLevel = heading.level;
		}
	});

	test('all interactive elements should be keyboard accessible', async ({ page }) => {
		// Get all interactive elements
		const interactiveElements = await page.$$('button, a, input, select, textarea, [tabindex]');

		for (const element of interactiveElements) {
			const isVisible = await element.isVisible();
			if (!isVisible) continue;

			// Check that element can receive focus
			await element.focus();
			const isFocused = await element.evaluate((el) => document.activeElement === el);
			expect(isFocused).toBe(true);

			// Check for visible focus indicator
			const focusStyles = await element.evaluate((el) => {
				const styles = window.getComputedStyle(el);
				return {
					outline: styles.outline,
					boxShadow: styles.boxShadow,
					border: styles.border
				};
			});

			// Should have some form of focus indicator
			const hasFocusIndicator =
				focusStyles.outline !== 'none' ||
				focusStyles.boxShadow !== 'none' ||
				focusStyles.border !== 'none';

			if (!hasFocusIndicator) {
				console.warn(`Element may lack focus indicator: ${await element.evaluate((el) => el.outerHTML.slice(0, 100))}`);
			}
		}
	});

	test('all images should have alt text', async ({ page }) => {
		const images = await page.$$('img');

		for (const img of images) {
			const alt = await img.getAttribute('alt');
			const role = await img.getAttribute('role');

			// Images should have alt text or be marked as decorative
			if (role !== 'presentation' && role !== 'none') {
				expect(alt).not.toBeNull();
			}
		}
	});

	test('form inputs should have associated labels', async ({ page }) => {
		const inputs = await page.$$('input:not([type="hidden"]), select, textarea');

		for (const input of inputs) {
			const id = await input.getAttribute('id');
			const ariaLabel = await input.getAttribute('aria-label');
			const ariaLabelledBy = await input.getAttribute('aria-labelledby');

			// Should have a label, aria-label, or aria-labelledby
			if (id) {
				const label = await page.$(`label[for="${id}"]`);
				const hasLabel = label !== null || ariaLabel !== null || ariaLabelledBy !== null;
				expect(hasLabel).toBe(true);
			} else {
				// If no id, must have aria-label or aria-labelledby
				const hasAccessibleName = ariaLabel !== null || ariaLabelledBy !== null;
				if (!hasAccessibleName) {
					// Check if wrapped in label
					const isWrappedInLabel = await input.evaluate((el) => el.closest('label') !== null);
					expect(isWrappedInLabel).toBe(true);
				}
			}
		}
	});

	test('color contrast should meet WCAG AA standards', async ({ page }) => {
		const results = await new AxeBuilder({ page })
			.withTags(['wcag2aa'])
			.options({ rules: { 'color-contrast': { enabled: true } } })
			.analyze();

		const contrastViolations = results.violations.filter((v) => v.id === 'color-contrast');

		if (contrastViolations.length > 0) {
			console.log('Contrast violations:', JSON.stringify(contrastViolations, null, 2));
		}

		expect(contrastViolations).toEqual([]);
	});

	test('page should have proper landmarks', async ({ page }) => {
		// Check for main landmark
		const main = await page.$('main, [role="main"]');
		expect(main).not.toBeNull();

		// Check for navigation landmark
		const nav = await page.$('nav, [role="navigation"]');
		expect(nav).not.toBeNull();
	});

	test('modals should trap focus', async ({ page }) => {
		// Open a modal if there's one available
		const modalTrigger = await page.$('[data-modal-trigger]');
		if (!modalTrigger) {
			test.skip();
			return;
		}

		await modalTrigger.click();
		await page.waitForSelector('[role="dialog"]');

		// Get all focusable elements in modal
		const modal = await page.$('[role="dialog"]');
		if (!modal) {
			test.skip();
			return;
		}

		const focusableInModal = await modal.$$('button, a, input, select, textarea, [tabindex]:not([tabindex="-1"])');

		// Tab through all elements
		for (let i = 0; i < focusableInModal.length + 2; i++) {
			await page.keyboard.press('Tab');

			// Check that focus stays within modal
			const activeElement = await page.evaluate(() => document.activeElement?.closest('[role="dialog"]'));
			expect(activeElement).not.toBeNull();
		}
	});

	test('skip link should be present and functional', async ({ page }) => {
		const skipLink = await page.$('a[href="#main-content"], a[href="#main"]');

		if (skipLink) {
			// Focus skip link
			await page.keyboard.press('Tab');
			const isSkipLinkFocused = await skipLink.evaluate((el) => document.activeElement === el);

			if (isSkipLinkFocused) {
				// Activate skip link
				await page.keyboard.press('Enter');

				// Check that focus moved to main content
				const focusedElement = await page.evaluate(() => document.activeElement?.id);
				expect(['main-content', 'main']).toContain(focusedElement);
			}
		}
	});

	test('ARIA attributes should be valid', async ({ page }) => {
		const results = await new AxeBuilder({ page })
			.options({
				rules: {
					'aria-allowed-attr': { enabled: true },
					'aria-valid-attr': { enabled: true },
					'aria-valid-attr-value': { enabled: true }
				}
			})
			.analyze();

		const ariaViolations = results.violations.filter((v) => v.id.startsWith('aria-'));

		if (ariaViolations.length > 0) {
			console.log('ARIA violations:', JSON.stringify(ariaViolations, null, 2));
		}

		expect(ariaViolations).toEqual([]);
	});
});

test.describe('Block Component Accessibility', () => {
	test('accordion blocks should be keyboard navigable', async ({ page }) => {
		await page.goto('/admin/blog/new');

		// Add an accordion block if possible
		const addBlockBtn = await page.$('[data-add-block]');
		if (addBlockBtn) {
			await addBlockBtn.click();
			const accordionOption = await page.$('[data-block-type="accordion"]');
			if (accordionOption) {
				await accordionOption.click();
				await page.waitForSelector('.accordion-block');

				// Test keyboard navigation
				const accordionHeaders = await page.$$('.accordion-header');
				if (accordionHeaders.length > 0) {
					await accordionHeaders[0].focus();

					// Enter to toggle
					await page.keyboard.press('Enter');

					// Arrow keys to navigate
					await page.keyboard.press('ArrowDown');
				}
			}
		}
	});

	test('tabs blocks should support arrow key navigation', async ({ page }) => {
		await page.goto('/admin/blog/new');

		// Find or create tabs block
		const tabsBlock = await page.$('.tabs-block');
		if (!tabsBlock) {
			test.skip();
			return;
		}

		const tabs = await tabsBlock.$$('[role="tab"]');
		if (tabs.length > 1) {
			await tabs[0].focus();

			// Right arrow should move to next tab
			await page.keyboard.press('ArrowRight');
			const focused = await page.evaluate(() => document.activeElement?.getAttribute('aria-selected'));
			expect(focused).toBeTruthy();
		}
	});
});

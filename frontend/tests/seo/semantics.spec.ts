import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { REPRESENTATIVE_URLS } from './test-helpers.js';

test.describe('Semantic Landmarks, Heading Hierarchy & A11y (SEM-01..05)', () => {
	for (const sample of REPRESENTATIVE_URLS) {
		test(`Landmarks, H1..H6 hierarchy & axe-core on ${sample.path}`, async ({ page }) => {
			await page.goto(sample.path, { waitUntil: 'domcontentloaded' });

			await expect(page.locator('main')).toHaveCount(1);
			expect(await page.locator('header').count()).toBeGreaterThanOrEqual(1);
			expect(await page.locator('nav').count()).toBeGreaterThanOrEqual(1);
			expect(await page.locator('footer').count()).toBeGreaterThanOrEqual(1);
			await expect(page.locator('h1')).toHaveCount(1);

			const headingLevels = await page.evaluate(() =>
				Array.from(document.querySelectorAll('h1, h2, h3, h4, h5, h6')).map((el) =>
					Number(el.tagName.slice(1))
				)
			);
			for (let i = 1; i < headingLevels.length; i++) {
				expect(
					headingLevels[i] - headingLevels[i - 1],
					`Heading skip H${headingLevels[i - 1]} -> H${headingLevels[i]} on ${sample.path}`
				).toBeLessThanOrEqual(1);
			}

			const axeResults = await new AxeBuilder({ page })
				.withTags(['wcag2a', 'wcag2aa', 'best-practice'])
				.analyze();
			const seriousOrCritical = axeResults.violations.filter(
				(v) => v.impact === 'critical' || v.impact === 'serious'
			);
			expect(
				seriousOrCritical.map((v) => `${v.id}: ${v.help}`),
				`Axe serious/critical violations on ${sample.path}`
			).toEqual([]);
		});
	}
});


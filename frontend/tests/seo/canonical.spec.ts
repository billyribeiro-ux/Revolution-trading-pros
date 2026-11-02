import { test, expect } from '@playwright/test';
import { REPRESENTATIVE_URLS } from './test-helpers.js';

test.describe('Canonical URL Hygiene & Tracking Param Stripping (URL-01..05)', () => {
	for (const sample of REPRESENTATIVE_URLS) {
		test(`Canonical on ${sample.path} is absolute HTTPS production origin without trailing slash`, async ({
			page
		}) => {
			await page.goto(sample.path, { waitUntil: 'domcontentloaded' });
			const canonicals = page.locator('head link[rel="canonical"]');
			await expect(canonicals).toHaveCount(1);

			const href = await canonicals.first().getAttribute('href');
			const expected = `https://revolutiontradingpros.com${sample.path}`;
			expect(href).toBe(expected);
		});
	}

	test('Tracking parameters (utm_source, gclid, fbclid) are stripped from canonical', async ({
		page
	}) => {
		await page.goto('/pricing?utm_source=test&utm_medium=cpc&gclid=123&fbclid=456', {
			waitUntil: 'domcontentloaded'
		});
		const href = await page.locator('head link[rel="canonical"]').first().getAttribute('href');
		expect(href).toBe('https://revolutiontradingpros.com/pricing');
	});
});


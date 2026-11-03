import { test, expect } from '@playwright/test';
import { REPRESENTATIVE_URLS } from './test-helpers.js';

test.describe('Metadata, Open Graph & Twitter Card Completeness (META-01..07)', () => {
	const titles = new Map<string, string>();
	const descriptions = new Map<string, string>();

	for (const sample of REPRESENTATIVE_URLS) {
		test(`Meta & OG/Twitter tags on ${sample.path}`, async ({ page, request }) => {
			await page.goto(sample.path, { waitUntil: 'domcontentloaded' });

			const title = (await page.title()).trim();
			expect(title.length).toBeGreaterThanOrEqual(15);
			expect(title).not.toMatch(/^(Home|Untitled|Page)$/i);

			const desc =
				(
					await page.locator('head meta[name="description"]').first().getAttribute('content')
				)?.trim() ?? '';
			expect(desc.length).toBeGreaterThanOrEqual(50);

			for (const [otherPath, otherTitle] of titles.entries()) {
				if (otherPath !== sample.path) {
					expect(title, `Duplicate title between ${sample.path} and ${otherPath}`).not.toBe(
						otherTitle
					);
				}
			}
			titles.set(sample.path, title);

			for (const [otherPath, otherDesc] of descriptions.entries()) {
				if (otherPath !== sample.path) {
					expect(desc, `Duplicate description between ${sample.path} and ${otherPath}`).not.toBe(
						otherDesc
					);
				}
			}
			descriptions.set(sample.path, desc);

			const ogTitle = await page.locator('head meta[property="og:title"]').getAttribute('content');
			const ogDesc = await page
				.locator('head meta[property="og:description"]')
				.getAttribute('content');
			const ogUrl = await page.locator('head meta[property="og:url"]').getAttribute('content');
			const ogImage = await page.locator('head meta[property="og:image"]').getAttribute('content');
			const ogSiteName = await page
				.locator('head meta[property="og:site_name"]')
				.getAttribute('content');
			const twitterCard = await page
				.locator('head meta[name="twitter:card"]')
				.getAttribute('content');

			expect(ogTitle?.length).toBeGreaterThan(0);
			expect(ogDesc?.length).toBeGreaterThan(0);
			expect(ogUrl).toMatch(/^https:\/\/revolutiontradingpros\.com/);
			expect(ogImage).toMatch(/^https:\/\//);
			expect(ogSiteName).toBe('Revolution Trading Pros');
			expect(twitterCard).toBe('summary_large_image');

			if (ogImage?.startsWith('https://revolutiontradingpros.com/')) {
				const localPath = new URL(ogImage).pathname;
				const imgRes = await request.get(localPath);
				expect(imgRes.status()).toBe(200);
			}
		});
	}
});

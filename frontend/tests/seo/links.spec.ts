import { test, expect } from '@playwright/test';
import { REPRESENTATIVE_URLS } from './test-helpers.js';

const BANNED_ANCHORS = new Set([
	'click here',
	'here',
	'read more',
	'learn more',
	'more',
	'link',
	'this'
]);

test.describe('Internal Linking & Anchor Text Quality (LINK-01..05)', () => {
	for (const sample of REPRESENTATIVE_URLS) {
		test(`Crawlable <a href> links and descriptive anchor text on ${sample.path}`, async ({
			page
		}) => {
			await page.goto(sample.path, { waitUntil: 'domcontentloaded' });

			const anchors = await page.evaluate(() =>
				Array.from(document.querySelectorAll('a[href]')).map((a) => ({
					href: a.getAttribute('href') || '',
					text: (
						a.textContent ||
						a.getAttribute('aria-label') ||
						a.querySelector('img')?.getAttribute('alt') ||
						''
					)
						.replace(/\s+/g, ' ')
						.trim()
				}))
			);

			expect(anchors.length).toBeGreaterThan(0);

			for (const a of anchors) {
				expect(a.href).not.toMatch(/^javascript:/i);
				expect(a.href).not.toBe('#');
				expect(a.text.length, `Empty anchor text for href="${a.href}" on ${sample.path}`).toBeGreaterThan(
					0
				);
				expect(
					BANNED_ANCHORS.has(a.text.toLowerCase()),
					`Generic anchor text "${a.text}" for href="${a.href}" on ${sample.path}`
				).toBe(false);
			}
		});
	}
});


import { test, expect } from '@playwright/test';
import { REPRESENTATIVE_URLS } from './test-helpers.js';

test.describe('Internationalization & Document Language (I18N-01..03)', () => {
	for (const sample of REPRESENTATIVE_URLS) {
		test(`<html lang> and dir attributes on ${sample.path}`, async ({ page, request }) => {
			const res = await request.get(sample.path);
			const ssrHtml = await res.text();
			expect(ssrHtml).toMatch(/<html[^>]+lang=["']en["']/i);
			expect(ssrHtml).toMatch(/<html[^>]+dir=["']ltr["']/i);

			await page.goto(sample.path, { waitUntil: 'domcontentloaded' });
			const lang = await page.locator('html').getAttribute('lang');
			const dir = await page.locator('html').getAttribute('dir');
			expect(lang).toBe('en');
			expect(dir).toBe('ltr');
		});
	}
});


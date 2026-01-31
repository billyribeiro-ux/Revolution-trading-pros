/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Sitemap Validation Test
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Validates sitemap.xml structure and content
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ Standards
 */

import { expect, test } from '@playwright/test';

test.describe('Sitemap', () => {
	test('sitemap.xml is valid and accessible', async ({ page }) => {
		const response = await page.goto('/sitemap.xml');
		
		// Should return 200 OK
		expect(response?.status()).toBe(200);
		
		// Should have correct content type
		expect(response?.headers()['content-type']).toContain('application/xml');
	});

	test('sitemap.xml contains valid XML structure', async ({ page }) => {
		await page.goto('/sitemap.xml');
		
		// Ensure XML is valid - Playwright parses the XML here and will error if invalid
		const urls = await page.$$eval('url', (urls) =>
			urls.map((url) => ({
				loc: url.querySelector('loc')?.textContent || ''
			}))
		);

		// Sanity check - should have at least 10 public pages
		expect(urls.length).toBeGreaterThan(10);
		
		console.log(`✅ Sitemap contains ${urls.length} URLs`);
	});

	test('sitemap.xml URLs are valid and well-formed', async ({ page }) => {
		await page.goto('/sitemap.xml');
		
		const urls = await page.$$eval('url', (urls) =>
			urls.map((url) => ({
				loc: url.querySelector('loc')?.textContent || ''
			}))
		);

		// Validate each URL
		for (const url of urls) {
			// Should have a loc element
			expect(url.loc).toBeTruthy();
			
			// Should be a valid URL
			expect(() => new URL(url.loc)).not.toThrow();
			
			// Should use HTTPS
			expect(url.loc).toMatch(/^https:\/\//);
			
			// Should be from correct domain
			expect(url.loc).toContain('revolution-trading-pros');
		}
	});

	test('sitemap.xml excludes private routes', async ({ page }) => {
		await page.goto('/sitemap.xml');
		
		const urls = await page.$$eval('url', (urls) =>
			urls.map((url) => url.querySelector('loc')?.textContent || '')
		);

		// Should NOT contain dashboard routes
		const dashboardUrls = urls.filter(url => url.includes('/dashboard'));
		expect(dashboardUrls.length).toBe(0);
		
		// Should NOT contain admin routes
		const adminUrls = urls.filter(url => url.includes('/admin'));
		expect(adminUrls.length).toBe(0);
		
		// Should NOT contain API routes
		const apiUrls = urls.filter(url => url.includes('/api'));
		expect(apiUrls.length).toBe(0);
		
		// Should NOT contain auth routes
		const authUrls = urls.filter(url => url.includes('/auth'));
		expect(authUrls.length).toBe(0);
	});

	test('sitemap.xml includes important public pages', async ({ page }) => {
		await page.goto('/sitemap.xml');
		
		const urls = await page.$$eval('url', (urls) =>
			urls.map((url) => url.querySelector('loc')?.textContent || '')
		);

		// Should contain homepage
		expect(urls.some(url => url.endsWith('/'))).toBe(true);
		
		// Should contain about page
		expect(urls.some(url => url.includes('/about'))).toBe(true);
		
		// Should contain alerts pages
		expect(urls.some(url => url.includes('/alerts'))).toBe(true);
		
		// Should contain courses pages
		expect(urls.some(url => url.includes('/courses'))).toBe(true);
		
		console.log('✅ All important public pages are included');
	});

	test('sitemap.xml is sorted alphabetically', async ({ page }) => {
		await page.goto('/sitemap.xml');
		
		const urls = await page.$$eval('url', (urls) =>
			urls.map((url) => url.querySelector('loc')?.textContent || '')
		);

		// Check if URLs are sorted
		const sortedUrls = [...urls].sort();
		expect(urls).toEqual(sortedUrls);
		
		console.log('✅ URLs are sorted alphabetically');
	});
});

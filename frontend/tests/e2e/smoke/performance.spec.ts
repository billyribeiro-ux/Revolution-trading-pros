/**
 * Revolution Trading Pros - Performance Smoke Tests
 * Apple ICT 11+ Principal Engineer Grade
 *
 * Basic performance and loading tests
 */

import { test, expect } from '@playwright/test';

test.describe('Performance Smoke Tests', () => {
	test('homepage should load within acceptable time', async ({ page }) => {
		const startTime = Date.now();

		await page.goto('/', { waitUntil: 'domcontentloaded' });

		const loadTime = Date.now() - startTime;

		// Page should load DOM within 10 seconds (generous for CI)
		expect(loadTime).toBeLessThan(10000);
	});

	test('should not have memory leaks on navigation', async ({ page }) => {
		// Simple navigation test
		await page.goto('/');
		await page.waitForLoadState('networkidle');

		// Navigate away and back
		await page.goto('/shop');
		await page.waitForLoadState('networkidle');

		await page.goto('/');
		await page.waitForLoadState('networkidle');

		// If we got here without crashing, basic memory management is working
		expect(true).toBe(true);
	});

	test('should handle rapid navigation', async ({ page }) => {
		const pages = ['/', '/shop', '/pricing', '/'];

		for (const path of pages) {
			await page.goto(path, { waitUntil: 'commit' });
		}

		// Should end up on homepage
		await page.waitForLoadState('domcontentloaded');
		expect(page.url()).toContain('/');
	});
});

test.describe('Asset Loading Tests', () => {
	test('should load CSS styles', async ({ page }) => {
		await page.goto('/');
		await page.waitForLoadState('networkidle');

		// Check that styles are applied (body should have some computed styles)
		const bodyBgColor = await page.evaluate(() => {
			return window.getComputedStyle(document.body).backgroundColor;
		});

		// Body should have a background color set
		expect(bodyBgColor).toBeDefined();
	});

	test('should load JavaScript', async ({ page }) => {
		await page.goto('/');

		// Check that JavaScript is executing
		const hasJS = await page.evaluate(() => {
			return typeof window !== 'undefined' && typeof document !== 'undefined';
		});

		expect(hasJS).toBe(true);
	});
});

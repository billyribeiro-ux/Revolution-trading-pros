/// <reference types="node" />
/**
 * Dashboard E2E — smoke checks beyond the route matrix (see route-matrix.spec.ts).
 */

import { test, expect } from '@playwright/test';

test.describe('Dashboard smoke', () => {
	test('home and dashboard URLs respond without server error', async ({ page }) => {
		for (const path of ['/', '/dashboard']) {
			const response = await page.goto(path, { waitUntil: 'domcontentloaded' });
			expect(response?.status() ?? 0).toBeLessThan(500);
			await expect(page.locator('body')).toBeVisible();
		}
	});
});

/// <reference types="node" />
/**
 * Dashboard E2E Tests
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 *
 * End-to-end tests for dashboard functionality
 */

import { test, expect } from '@playwright/test';

test.describe('Dashboard Tests', () => {
	test('public pages load without authentication', async ({ page }) => {
		// Test homepage loads
		const response = await page.goto('/');
		expect(response?.status()).toBeLessThan(400);

		// Body should be visible
		await expect(page.locator('body')).toBeVisible();
	});

	test('page renders content', async ({ page }) => {
		await page.goto('/');

		// Wait for content to load
		await page.waitForLoadState('domcontentloaded');

		// Should have some text content
		const bodyText = await page.locator('body').textContent();
		expect(bodyText?.length).toBeGreaterThan(0);
	});
});

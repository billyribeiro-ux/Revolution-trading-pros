/// <reference types="node" />
/**
 * Homepage Smoke Tests
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 * 
 * Fast smoke tests to verify basic site functionality:
 * - Page loads without errors
 * - Critical elements are visible
 * - No JavaScript console errors
 */

import { test, expect } from '@playwright/test';

test.describe('Homepage Smoke Tests', () => {
	test('homepage loads successfully', async ({ page }) => {
		// Navigate to homepage
		const response = await page.goto('/');
		
		// Verify successful response
		expect(response?.status()).toBeLessThan(400);
		
		// Verify page has loaded (check for body)
		await expect(page.locator('body')).toBeVisible();
	});

	test('homepage has correct title', async ({ page }) => {
		await page.goto('/');
		
		// Check page title contains site name
		await expect(page).toHaveTitle(/Revolution Trading|Trading/i);
	});

	test('no critical JavaScript errors', async ({ page }) => {
		const errors: string[] = [];
		
		// Collect console errors
		page.on('console', (msg) => {
			if (msg.type() === 'error') {
				errors.push(msg.text());
			}
		});
		
		await page.goto('/');
		
		// Wait for page to settle
		await page.waitForLoadState('networkidle');
		
		// Filter out known non-critical errors (CORS, extensions, etc.)
		const criticalErrors = errors.filter(err => 
			!err.includes('favicon') &&
			!err.includes('CORS') &&
			!err.includes('extension') &&
			!err.includes('third-party')
		);
		
		// Should have no critical errors
		expect(criticalErrors).toHaveLength(0);
	});

	test('navigation is visible', async ({ page }) => {
		await page.goto('/');
		
		// Check that navigation/header exists
		const nav = page.locator('nav, header, [role="navigation"]').first();
		await expect(nav).toBeVisible({ timeout: 10000 });
	});
});

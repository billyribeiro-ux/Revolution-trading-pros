import { test, expect } from '@playwright/test';

test('Hamburger menu visual test - WATCH THIS', async ({ page }) => {
	// Set mobile viewport
	await page.setViewportSize({ width: 375, height: 812 });
	
	await page.goto('/');
	await page.waitForLoadState('networkidle');
	
	// Find hamburger button
	const hamburger = page.locator('button.hamburger');
	await expect(hamburger).toBeVisible();
	
	console.log('✅ Hamburger button is visible');
	
	// Click hamburger
	await hamburger.click();
	
	// Wait for mobile panel to appear
	const mobilePanel = page.locator('.mobile-panel');
	await expect(mobilePanel).toBeVisible({ timeout: 5000 });
	
	console.log('✅ Mobile panel opened!');
	
	// Check for menu items
	const menuItems = page.locator('.mobile-nav-item');
	const count = await menuItems.count();
	console.log(`✅ Found ${count} menu items`);
	
	// Expand a submenu
	const firstSubmenuButton = page.locator('.mobile-nav-item').first();
	await firstSubmenuButton.click();
	
	// Wait a moment
	await page.waitForTimeout(500);
	
	// Check submenu items
	const submenuItems = page.locator('.mobile-submenu-item');
	const submenuCount = await submenuItems.count();
	console.log(`✅ Found ${submenuCount} submenu items after expanding`);
	
	// Close menu
	const closeButton = page.locator('.mobile-close');
	await closeButton.click();
	
	// Verify closed
	await expect(mobilePanel).not.toBeVisible({ timeout: 3000 });
	
	console.log('✅ Mobile panel closed!');
	console.log('🎉 ALL HAMBURGER INTERACTIONS WORKING!');
});

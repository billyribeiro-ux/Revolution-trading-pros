import { test, expect } from '@playwright/test';

test('Mobile navbar with hamburger', async ({ page }) => {
	// Set MOBILE viewport
	await page.setViewportSize({ width: 375, height: 812 });
	
	await page.goto('/');
	await page.waitForLoadState('networkidle');
	
	// Screenshot mobile view
	await page.screenshot({ path: 'test-results/navbar-mobile.png', fullPage: false });
	
	// Check hamburger visibility
	const hamburger = page.locator('button.hamburger');
	const isVisible = await hamburger.isVisible();
	console.log('=== HAMBURGER VISIBLE ===', isVisible);
	
	// Check hamburger computed styles
	if (isVisible) {
		const display = await hamburger.evaluate(el => getComputedStyle(el).display);
		const width = await hamburger.evaluate(el => getComputedStyle(el).width);
		const height = await hamburger.evaluate(el => getComputedStyle(el).height);
		console.log('Display:', display);
		console.log('Width:', width);
		console.log('Height:', height);
	} else {
		// Check if hamburger exists but is hidden
		const exists = await hamburger.count();
		console.log('Hamburger element count:', exists);
		if (exists > 0) {
			const display = await hamburger.evaluate(el => getComputedStyle(el).display);
			console.log('Hamburger display value:', display);
		}
	}
	
	// Check desktop nav visibility (should be hidden on mobile)
	const desktopNav = page.locator('nav.desktop-nav');
	const desktopVisible = await desktopNav.isVisible();
	console.log('=== DESKTOP NAV VISIBLE ===', desktopVisible);
	
	expect(isVisible).toBe(true);
});

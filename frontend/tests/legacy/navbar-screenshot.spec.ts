import { test, expect } from '@playwright/test';

test('Screenshot navbar to see what it looks like', async ({ page }) => {
	await page.goto('/');
	await page.waitForLoadState('networkidle');
	
	// Take full page screenshot
	await page.screenshot({ path: 'test-results/navbar-full.png', fullPage: false });
	
	// Get the navbar HTML
	const navbarHTML = await page.locator('header').first().innerHTML();
	console.log('=== NAVBAR HTML ===');
	console.log(navbarHTML.substring(0, 2000));
	
	// Check what classes are on the header
	const headerClasses = await page.locator('header').first().getAttribute('class');
	console.log('=== HEADER CLASSES ===');
	console.log(headerClasses);
	
	// Check computed styles
	const bgColor = await page.locator('header').first().evaluate(el => getComputedStyle(el).backgroundColor);
	const height = await page.locator('header').first().evaluate(el => getComputedStyle(el).height);
	console.log('=== COMPUTED STYLES ===');
	console.log('Background:', bgColor);
	console.log('Height:', height);
});

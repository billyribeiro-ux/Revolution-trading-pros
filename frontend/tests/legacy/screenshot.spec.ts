import { test, expect } from '@playwright/test';

test('Take screenshot of home page', async ({ page }) => {
	// Capture ALL console output
	const logs: string[] = [];
	page.on('console', msg => {
		logs.push(`[${msg.type()}] ${msg.text()}`);
	});
	page.on('pageerror', err => {
		logs.push(`[PAGE ERROR] ${err.message}`);
	});

	await page.goto('/', { waitUntil: 'networkidle' });
	await page.waitForTimeout(2000);

	// Take screenshot
	await page.screenshot({ 
		path: 'test-results/homepage-screenshot.png', 
		fullPage: true 
	});

	// Print all logs
	console.log('\n=== ALL BROWSER LOGS ===');
	logs.forEach(l => console.log(l));

	// Check what's visible
	const heroVisible = await page.locator('#hero').isVisible();
	const chartVisible = await page.locator('#chart-bg').isVisible();
	const slideVisible = await page.locator('.slide--active').isVisible();
	
	console.log(`\nHero visible: ${heroVisible}`);
	console.log(`Chart visible: ${chartVisible}`);
	console.log(`Active slide visible: ${slideVisible}`);

	// Get computed styles of hero
	const heroStyles = await page.locator('#hero').evaluate(el => {
		const styles = window.getComputedStyle(el);
		return {
			display: styles.display,
			visibility: styles.visibility,
			opacity: styles.opacity,
			height: styles.height,
			position: styles.position
		};
	});
	console.log('\nHero computed styles:', heroStyles);

	// Check slide styles
	const slideStyles = await page.locator('.slide--active').evaluate(el => {
		const styles = window.getComputedStyle(el);
		return {
			display: styles.display,
			visibility: styles.visibility,
			opacity: styles.opacity,
			transform: styles.transform
		};
	});
	console.log('Active slide styles:', slideStyles);
});

import { test, expect } from '@playwright/test';

/**
 * DIAGNOSTIC TEST - Identify exact rendering issues
 */
test.describe('Diagnostic Tests', () => {
	test('Capture all console errors and warnings on home page', async ({ page }) => {
		const errors: string[] = [];
		const warnings: string[] = [];
		
		page.on('console', msg => {
			if (msg.type() === 'error') {
				errors.push(msg.text());
			}
			if (msg.type() === 'warning') {
				warnings.push(msg.text());
			}
		});

		page.on('pageerror', err => {
			errors.push(`Page Error: ${err.message}`);
		});

		await page.goto('/');
		await page.waitForLoadState('networkidle');
		await page.waitForTimeout(3000); // Wait for animations

		console.log('\n=== CONSOLE ERRORS ===');
		errors.forEach(e => console.log(e));
		
		console.log('\n=== CONSOLE WARNINGS ===');
		warnings.forEach(w => console.log(w));

		// Filter out expected backend errors
		const criticalErrors = errors.filter(e => 
			!e.includes('ECONNREFUSED') && 
			!e.includes('fetch failed') &&
			!e.includes('Failed to load resource')
		);

		expect(criticalErrors.length).toBe(0);
	});

	test('Check Hero section DOM structure', async ({ page }) => {
		await page.goto('/');
		await page.waitForLoadState('domcontentloaded');

		// Check hero section exists
		const hero = page.locator('#hero');
		await expect(hero).toBeVisible();
		console.log('✓ Hero section visible');

		// Check chart container
		const chartBg = page.locator('#chart-bg');
		await expect(chartBg).toBeVisible();
		console.log('✓ Chart container visible');

		// Check slides exist
		const slides = page.locator('[data-slide]');
		const slideCount = await slides.count();
		console.log(`✓ Found ${slideCount} slides`);
		expect(slideCount).toBe(4);

		// Check active slide
		const activeSlide = page.locator('.slide--active');
		await expect(activeSlide).toBeVisible();
		console.log('✓ Active slide visible');

		// Check slide content
		const h1 = activeSlide.locator('h1');
		const h1Text = await h1.textContent();
		console.log(`✓ H1 text: "${h1Text}"`);
		expect(h1Text).toBeTruthy();

		// Check CTA buttons
		const ctas = activeSlide.locator('.cta');
		const ctaCount = await ctas.count();
		console.log(`✓ Found ${ctaCount} CTA buttons`);
		expect(ctaCount).toBe(2);
	});

	test('Check navigation dropdown functionality', async ({ page }) => {
		await page.goto('/');
		await page.waitForLoadState('networkidle');

		// Find nav items with dropdowns
		const navItems = page.locator('nav .nav-item, nav [class*="dropdown"], header button, header [class*="menu"]');
		const navCount = await navItems.count();
		console.log(`Found ${navCount} potential nav items`);

		// Check for main navigation
		const mainNav = page.locator('header nav, .marketing-nav, [class*="nav"]').first();
		const navExists = await mainNav.count();
		console.log(`Main nav exists: ${navExists > 0}`);

		// Look for dropdown triggers
		const dropdownTriggers = page.locator('[aria-haspopup], [data-dropdown], button:has(svg), .dropdown-trigger');
		const triggerCount = await dropdownTriggers.count();
		console.log(`Found ${triggerCount} dropdown triggers`);

		// Try to find and click a dropdown
		const liveRoomsLink = page.locator('text=Live Rooms, text=Trading Rooms, text=Rooms').first();
		if (await liveRoomsLink.count() > 0) {
			console.log('Found Live Rooms link');
		}
	});

	test('Check if chart canvas is created', async ({ page }) => {
		await page.goto('/');
		await page.waitForLoadState('networkidle');
		await page.waitForTimeout(2000);

		// Check for canvas elements (lightweight-charts creates these)
		const canvases = page.locator('#chart-bg canvas, #chart-bg table');
		const canvasCount = await canvases.count();
		console.log(`Chart canvases/tables found: ${canvasCount}`);

		// Check chart container children
		const chartChildren = page.locator('#chart-bg > *');
		const childCount = await chartChildren.count();
		console.log(`Chart container children: ${childCount}`);

		// Get chart container HTML for inspection
		const chartHtml = await page.locator('#chart-bg').innerHTML();
		console.log(`Chart container HTML length: ${chartHtml.length} chars`);
		if (chartHtml.length < 100) {
			console.log(`Chart HTML: ${chartHtml}`);
		}
	});

	test('Check sections below hero', async ({ page }) => {
		await page.goto('/');
		await page.waitForLoadState('networkidle');

		// Check for sections after hero
		const sections = page.locator('section');
		const sectionCount = await sections.count();
		console.log(`Total sections on page: ${sectionCount}`);

		// List section IDs/classes
		for (let i = 0; i < Math.min(sectionCount, 10); i++) {
			const section = sections.nth(i);
			const id = await section.getAttribute('id');
			const className = await section.getAttribute('class');
			console.log(`Section ${i}: id="${id}" class="${className?.slice(0, 50)}..."`);
		}

		// Check page height (if content is rendering)
		const bodyHeight = await page.evaluate(() => document.body.scrollHeight);
		console.log(`Page body height: ${bodyHeight}px`);
		expect(bodyHeight).toBeGreaterThan(500);
	});

	test('Test slide transition', async ({ page }) => {
		await page.goto('/');
		await page.waitForLoadState('networkidle');

		// Get initial slide
		const getActiveSlideIndex = async () => {
			const activeSlide = page.locator('.slide--active');
			return await activeSlide.getAttribute('data-slide');
		};

		const initialSlide = await getActiveSlideIndex();
		console.log(`Initial slide: ${initialSlide}`);

		// Wait for slide transition (7 seconds interval)
		await page.waitForTimeout(8000);

		const nextSlide = await getActiveSlideIndex();
		console.log(`After 8s, slide: ${nextSlide}`);

		// Slides should have changed
		expect(nextSlide).not.toBe(initialSlide);
	});
});

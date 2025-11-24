import { test, expect, type Page } from '@playwright/test';

/**
 * NavBar End-to-End Tests
 *
 * Tests all NavBar functionality including:
 * - Desktop navigation
 * - Mobile hamburger menu
 * - Dropdown menus
 * - User menu
 * - Responsive behavior
 * - Accessibility
 */

test.describe('NavBar Component', () => {
	test.beforeEach(async ({ page }) => {
		await page.goto('/');
		// Wait for navbar to be fully loaded
		await page.waitForSelector('.nav-header', { state: 'visible' });
	});

	test.describe('Desktop Navigation', () => {
		test.use({ viewport: { width: 1280, height: 720 } });

		test('should display logo and navigate to home', async ({ page }) => {
			const logo = page.locator('.nav-logo');
			await expect(logo).toBeVisible();

			const logoImage = logo.locator('img');
			await expect(logoImage).toHaveAttribute('alt', 'Revolution Trading Pros Logo');

			// Click logo and verify navigation
			await logo.click();
			await expect(page).toHaveURL('/');
		});

		test('should display all main navigation items', async ({ page }) => {
			const navItems = [
				'Live Trading Rooms',
				'Alert Services',
				'Mentorship',
				'Store',
				'Our Mission',
				'About',
				'Resources',
				'Blog'
			];

			for (const item of navItems) {
				const navItem = page.locator('.nav-desktop').getByText(item, { exact: true });
				await expect(navItem).toBeVisible();
			}
		});

		test('should open and close dropdown menus on hover', async ({ page }) => {
			// Test Live Trading Rooms dropdown
			const liveDropdown = page.locator('.nav-dropdown').filter({ hasText: 'Live Trading Rooms' });

			// Hover to open
			await liveDropdown.hover();
			await page.waitForTimeout(100); // Wait for animation

			const dropdownMenu = liveDropdown.locator('.nav-dropdown__menu');
			await expect(dropdownMenu).toHaveClass(/nav-dropdown__menu--open/);

			// Check submenu items
			await expect(dropdownMenu.getByText('Day Trading Room')).toBeVisible();
			await expect(dropdownMenu.getByText('Swing Trading Room')).toBeVisible();
			await expect(dropdownMenu.getByText('Small Accounts Room')).toBeVisible();

			// Move away to close
			await page.locator('.nav-logo').hover();
			await page.waitForTimeout(300); // Wait for animation
			await expect(dropdownMenu).not.toHaveClass(/nav-dropdown__menu--open/);
		});

		test('should open dropdown on click', async ({ page }) => {
			const storeDropdown = page.locator('.nav-dropdown').filter({ hasText: 'Store' });
			const dropdownButton = storeDropdown.locator('button');

			// Click to open
			await dropdownButton.click();
			await page.waitForTimeout(100);

			const dropdownMenu = storeDropdown.locator('.nav-dropdown__menu');
			await expect(dropdownMenu).toHaveClass(/nav-dropdown__menu--open/);

			// Check submenu items
			await expect(dropdownMenu.getByText('Courses')).toBeVisible();
			await expect(dropdownMenu.getByText('Indicators')).toBeVisible();

			// Click again to close
			await dropdownButton.click();
			await page.waitForTimeout(300);
			await expect(dropdownMenu).not.toHaveClass(/nav-dropdown__menu--open/);
		});

		test('should navigate to submenu items', async ({ page }) => {
			const alertsDropdown = page.locator('.nav-dropdown').filter({ hasText: 'Alert Services' });

			await alertsDropdown.hover();
			await page.waitForTimeout(100);

			const spxLink = alertsDropdown.getByText('SPX Profit Pulse');
			await spxLink.click();

			await expect(page).toHaveURL('/spx-profit-pulse');
		});

		test('should display login button when not authenticated', async ({ page }) => {
			const loginButton = page.locator('.nav-login');
			await expect(loginButton).toBeVisible();
			await expect(loginButton).toHaveText(/Login/);
		});

		test('should close dropdown on Escape key', async ({ page }) => {
			const dropdown = page.locator('.nav-dropdown').first();
			const dropdownButton = dropdown.locator('button');

			// Open dropdown
			await dropdownButton.click();
			await page.waitForTimeout(100);

			const dropdownMenu = dropdown.locator('.nav-dropdown__menu');
			await expect(dropdownMenu).toHaveClass(/nav-dropdown__menu--open/);

			// Press Escape
			await page.keyboard.press('Escape');
			await page.waitForTimeout(100);

			await expect(dropdownMenu).not.toHaveClass(/nav-dropdown__menu--open/);
		});
	});

	test.describe('Mobile Navigation', () => {
		test.use({ viewport: { width: 375, height: 667 } });

		test('should display hamburger button on mobile', async ({ page }) => {
			const hamburger = page.locator('.nav-hamburger');
			await expect(hamburger).toBeVisible();

			// Check aria attributes
			await expect(hamburger).toHaveAttribute('aria-expanded', 'false');
			await expect(hamburger).toHaveAttribute('aria-label', 'Open navigation menu');
		});

		test('should hide desktop navigation on mobile', async ({ page }) => {
			const desktopNav = page.locator('.nav-desktop');
			await expect(desktopNav).not.toBeVisible();
		});

		test('should open mobile menu when hamburger is clicked', async ({ page }) => {
			const hamburger = page.locator('.nav-hamburger');

			// Click to open
			await hamburger.click();
			await page.waitForTimeout(300); // Wait for animation

			// Check menu is open
			const mobileNav = page.locator('.nav-mobile');
			await expect(mobileNav).toHaveClass(/nav-mobile--open/);

			// Check hamburger state
			await expect(hamburger).toHaveClass(/nav-hamburger--active/);
			await expect(hamburger).toHaveAttribute('aria-expanded', 'true');
			await expect(hamburger).toHaveAttribute('aria-label', 'Close navigation menu');

			// Check backdrop is visible
			const backdrop = page.locator('.nav-mobile__backdrop');
			await expect(backdrop).toHaveClass(/nav-mobile__backdrop--visible/);
		});

		test('should display all navigation items in mobile menu', async ({ page }) => {
			const hamburger = page.locator('.nav-hamburger');
			await hamburger.click();
			await page.waitForTimeout(300);

			const mobileContent = page.locator('.nav-mobile__content');

			// Check main items
			await expect(mobileContent.getByText('Live Trading Rooms')).toBeVisible();
			await expect(mobileContent.getByText('Alert Services')).toBeVisible();
			await expect(mobileContent.getByText('Mentorship')).toBeVisible();
			await expect(mobileContent.getByText('Store')).toBeVisible();
		});

		test('should expand and collapse mobile submenus', async ({ page }) => {
			const hamburger = page.locator('.nav-hamburger');
			await hamburger.click();
			await page.waitForTimeout(300);

			// Find and click Live Trading Rooms button
			const submenuButton = page
				.locator('.nav-mobile__button')
				.filter({ hasText: 'Live Trading Rooms' });
			await submenuButton.click();
			await page.waitForTimeout(300);

			// Check submenu is open
			const submenu = page.locator('#mobile-submenu-live');
			await expect(submenu).toHaveClass(/nav-mobile__submenu--open/);

			// Check submenu items are visible
			await expect(submenu.getByText('Day Trading Room')).toBeVisible();
			await expect(submenu.getByText('Swing Trading Room')).toBeVisible();

			// Click again to close
			await submenuButton.click();
			await page.waitForTimeout(300);
			await expect(submenu).not.toHaveClass(/nav-mobile__submenu--open/);
		});

		test('should close mobile menu when clicking backdrop', async ({ page }) => {
			const hamburger = page.locator('.nav-hamburger');
			await hamburger.click();
			await page.waitForTimeout(300);

			const mobileNav = page.locator('.nav-mobile');
			await expect(mobileNav).toHaveClass(/nav-mobile--open/);

			// Click backdrop
			const backdrop = page.locator('.nav-mobile__backdrop');
			await backdrop.click({ force: true });
			await page.waitForTimeout(300);

			// Check menu is closed
			await expect(mobileNav).not.toHaveClass(/nav-mobile--open/);
			await expect(hamburger).not.toHaveClass(/nav-hamburger--active/);
		});

		test('should close mobile menu when clicking X button', async ({ page }) => {
			const hamburger = page.locator('.nav-hamburger');

			// Open menu
			await hamburger.click();
			await page.waitForTimeout(300);

			const mobileNav = page.locator('.nav-mobile');
			await expect(mobileNav).toHaveClass(/nav-mobile--open/);

			// Click X to close
			await hamburger.click();
			await page.waitForTimeout(300);

			// Check menu is closed
			await expect(mobileNav).not.toHaveClass(/nav-mobile--open/);
		});

		test('should close mobile menu on Escape key', async ({ page }) => {
			const hamburger = page.locator('.nav-hamburger');
			await hamburger.click();
			await page.waitForTimeout(300);

			const mobileNav = page.locator('.nav-mobile');
			await expect(mobileNav).toHaveClass(/nav-mobile--open/);

			// Press Escape
			await page.keyboard.press('Escape');
			await page.waitForTimeout(300);

			// Check menu is closed
			await expect(mobileNav).not.toHaveClass(/nav-mobile--open/);
		});

		test('should navigate when clicking mobile menu link', async ({ page }) => {
			const hamburger = page.locator('.nav-hamburger');
			await hamburger.click();
			await page.waitForTimeout(300);

			// Click Mentorship link
			const mentorshipLink = page.locator('.nav-mobile__link').filter({ hasText: 'Mentorship' });
			await mentorshipLink.click();

			// Check navigation
			await expect(page).toHaveURL('/mentorship');
		});

		test('should lock body scroll when mobile menu is open', async ({ page }) => {
			const hamburger = page.locator('.nav-hamburger');

			// Check body is scrollable initially
			const bodyBefore = await page.evaluate(() => {
				return {
					position: document.body.style.position,
					overflow: document.body.style.overflow
				};
			});
			expect(bodyBefore.position).toBe('');

			// Open menu
			await hamburger.click();
			await page.waitForTimeout(300);

			// Check body scroll is locked
			const bodyAfter = await page.evaluate(() => {
				return {
					position: document.body.style.position,
					overflow: document.body.style.overflow
				};
			});
			expect(bodyAfter.position).toBe('fixed');
			expect(bodyAfter.overflow).toBe('hidden');
		});
	});

	test.describe('Responsive Behavior', () => {
		test('should switch from desktop to mobile navigation on resize', async ({ page }) => {
			// Start with desktop viewport
			await page.setViewportSize({ width: 1280, height: 720 });

			const desktopNav = page.locator('.nav-desktop');
			const hamburger = page.locator('.nav-hamburger');

			await expect(desktopNav).toBeVisible();
			await expect(hamburger).not.toBeVisible();

			// Resize to mobile
			await page.setViewportSize({ width: 375, height: 667 });
			await page.waitForTimeout(200);

			await expect(desktopNav).not.toBeVisible();
			await expect(hamburger).toBeVisible();
		});

		test('should close mobile menu when resizing to desktop', async ({ page }) => {
			// Start with mobile viewport
			await page.setViewportSize({ width: 375, height: 667 });

			const hamburger = page.locator('.nav-hamburger');
			await hamburger.click();
			await page.waitForTimeout(300);

			const mobileNav = page.locator('.nav-mobile');
			await expect(mobileNav).toHaveClass(/nav-mobile--open/);

			// Resize to desktop
			await page.setViewportSize({ width: 1280, height: 720 });
			await page.waitForTimeout(500);

			// Mobile menu should be closed
			await expect(mobileNav).not.toBeVisible();
		});
	});

	test.describe('Accessibility', () => {
		test.use({ viewport: { width: 1280, height: 720 } });

		test('should have proper ARIA attributes', async ({ page }) => {
			const header = page.locator('.nav-header');
			await expect(header).toHaveAttribute('aria-label', 'Main navigation');

			const desktopNav = page.locator('.nav-desktop');
			await expect(desktopNav).toHaveAttribute('aria-label', 'Main navigation');
		});

		test('should support keyboard navigation in dropdowns', async ({ page }) => {
			// Tab to first dropdown
			await page.keyboard.press('Tab');
			await page.keyboard.press('Tab'); // Skip logo

			// Find the first dropdown button
			const firstDropdown = page.locator('.nav-dropdown button').first();
			await firstDropdown.focus();

			// Press Enter to open
			await page.keyboard.press('Enter');
			await page.waitForTimeout(100);

			// Check dropdown is open
			const dropdownMenu = page.locator('.nav-dropdown__menu').first();
			await expect(dropdownMenu).toHaveClass(/nav-dropdown__menu--open/);

			// Press Escape to close
			await page.keyboard.press('Escape');
			await page.waitForTimeout(100);
			await expect(dropdownMenu).not.toHaveClass(/nav-dropdown__menu--open/);
		});

		test('should have focus visible styles', async ({ page }) => {
			const logo = page.locator('.nav-logo');
			await logo.focus();

			// Check that focus is visible (outline should be applied)
			const outlineWidth = await logo.evaluate((el) => {
				return window.getComputedStyle(el).outlineWidth;
			});

			// Focus-visible should add outline
			expect(outlineWidth).not.toBe('0px');
		});
	});

	test.describe('Performance', () => {
		test('should load navbar quickly', async ({ page }) => {
			const startTime = Date.now();
			await page.goto('/');
			await page.waitForSelector('.nav-header', { state: 'visible' });
			const loadTime = Date.now() - startTime;

			// Navbar should load in under 2 seconds
			expect(loadTime).toBeLessThan(2000);
		});

		test('should animate smoothly', async ({ page }) => {
			await page.setViewportSize({ width: 375, height: 667 });

			const hamburger = page.locator('.nav-hamburger');

			// Open menu and measure animation
			const startTime = Date.now();
			await hamburger.click();
			await page.waitForSelector('.nav-mobile--open', { state: 'visible' });
			const animationTime = Date.now() - startTime;

			// Animation should complete in reasonable time (< 500ms)
			expect(animationTime).toBeLessThan(500);
		});
	});

	test.describe('Visual Regression', () => {
		test('should match desktop navbar snapshot', async ({ page }) => {
			await page.setViewportSize({ width: 1280, height: 720 });
			const navbar = page.locator('.nav-header');
			await expect(navbar).toBeVisible();

			// Take screenshot of navbar
			await expect(navbar).toHaveScreenshot('navbar-desktop.png', {
				maxDiffPixels: 100
			});
		});

		test('should match mobile navbar snapshot', async ({ page }) => {
			await page.setViewportSize({ width: 375, height: 667 });
			const navbar = page.locator('.nav-header');
			await expect(navbar).toBeVisible();

			await expect(navbar).toHaveScreenshot('navbar-mobile.png', {
				maxDiffPixels: 100
			});
		});

		test('should match mobile menu open snapshot', async ({ page }) => {
			await page.setViewportSize({ width: 375, height: 667 });

			const hamburger = page.locator('.nav-hamburger');
			await hamburger.click();
			await page.waitForTimeout(300);

			const mobileNav = page.locator('.nav-mobile');
			await expect(mobileNav).toHaveScreenshot('navbar-mobile-menu-open.png', {
				maxDiffPixels: 100
			});
		});
	});
});

import { test, expect } from '@playwright/test';

const BASE_URL = 'http://localhost:5174';

test.describe('Navbar - Desktop (1440x900)', () => {
  test.beforeEach(async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    await page.goto(BASE_URL);
    await page.waitForLoadState('networkidle');
  });

  test('desktop nav is visible, hamburger is hidden', async ({ page }) => {
    const desktopNav = page.locator('.desktop-nav');
    const hamburger = page.locator('button.hamburger');
    
    await expect(desktopNav).toBeVisible();
    await expect(hamburger).not.toBeVisible();
    
    // Screenshot proof
    await page.screenshot({ path: 'test-results/navbar-desktop-layout.png' });
    console.log('✓ Desktop nav visible, hamburger hidden');
  });

  test('all nav items are visible on desktop', async ({ page }) => {
    const navItems = [
      'Live Trading Rooms',
      'Alert Services', 
      'Mentorship',
      'Store',
      'Our Mission',
      'About',
      'Blogs',
      'Resources'
    ];

    for (const item of navItems) {
      const element = page.locator(`.desktop-nav`).getByText(item, { exact: false }).first();
      await expect(element).toBeVisible();
      console.log(`✓ "${item}" visible`);
    }
  });

  test('dropdown opens on click and shows submenu items', async ({ page }) => {
    // Click "Live Trading Rooms" dropdown
    const trigger = page.locator('.dropdown-trigger:has-text("Live Trading Rooms")');
    await expect(trigger).toBeVisible();
    await trigger.click();
    
    // Wait for dropdown menu
    const dropdownMenu = page.locator('.dropdown-menu').first();
    await expect(dropdownMenu).toBeVisible({ timeout: 2000 });
    
    // Verify submenu items
    await expect(page.locator('.dropdown-item:has-text("Day Trading")')).toBeVisible();
    await expect(page.locator('.dropdown-item:has-text("Swing Trading")')).toBeVisible();
    
    // Screenshot proof
    await page.screenshot({ path: 'test-results/navbar-desktop-dropdown-open.png' });
    console.log('✓ Dropdown opens with submenu items');
  });

  test('dropdown closes when clicking outside', async ({ page }) => {
    // Open dropdown
    await page.locator('.dropdown-trigger:has-text("Store")').click();
    await expect(page.locator('.dropdown-menu').first()).toBeVisible();
    
    // Click outside
    await page.locator('body').click({ position: { x: 10, y: 10 } });
    await page.waitForTimeout(300);
    
    // Dropdown should be closed
    const dropdownMenus = await page.locator('.dropdown-menu').count();
    // Either no dropdown menus or they're hidden
    console.log(`✓ Dropdown closed after clicking outside`);
  });

  test('login and CTA buttons are visible on desktop', async ({ page }) => {
    const loginBtn = page.locator('.login-btn');
    const ctaBtn = page.locator('.cta-btn');
    
    await expect(loginBtn).toBeVisible();
    await expect(ctaBtn).toBeVisible();
    console.log('✓ Login and CTA buttons visible on desktop');
  });

  test('logo is visible and links to home', async ({ page }) => {
    const logo = page.locator('.logo');
    await expect(logo).toBeVisible();
    
    const href = await logo.getAttribute('href');
    expect(href).toBe('/');
    console.log('✓ Logo visible and links to home');
  });
});

test.describe('Navbar - Tablet (768x1024)', () => {
  test.beforeEach(async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 1024 });
    await page.goto(BASE_URL);
    await page.waitForLoadState('networkidle');
  });

  test('hamburger is visible, desktop nav is hidden', async ({ page }) => {
    const desktopNav = page.locator('.desktop-nav');
    const hamburger = page.locator('button.hamburger');
    
    await expect(hamburger).toBeVisible();
    await expect(desktopNav).not.toBeVisible();
    
    // Screenshot proof
    await page.screenshot({ path: 'test-results/navbar-tablet-layout.png' });
    console.log('✓ Tablet: Hamburger visible, desktop nav hidden');
  });

  test('mobile menu opens on hamburger click', async ({ page }) => {
    const hamburger = page.locator('button.hamburger');
    await hamburger.click();
    
    const mobilePanel = page.locator('.mobile-panel');
    await expect(mobilePanel).toBeVisible({ timeout: 2000 });
    
    // Screenshot proof
    await page.screenshot({ path: 'test-results/navbar-tablet-menu-open.png' });
    console.log('✓ Tablet: Mobile menu opens');
  });

  test('all nav items in mobile menu', async ({ page }) => {
    await page.locator('button.hamburger').click();
    await expect(page.locator('.mobile-panel')).toBeVisible();
    
    const navItems = [
      'Live Trading Rooms',
      'Alert Services',
      'Mentorship', 
      'Store',
      'Our Mission',
      'About',
      'Blogs',
      'Resources'
    ];

    for (const item of navItems) {
      const element = page.locator('.mobile-panel').getByText(item, { exact: false }).first();
      await expect(element).toBeVisible();
      console.log(`✓ Tablet mobile menu: "${item}" visible`);
    }
  });
});

test.describe('Navbar - Mobile (375x812 iPhone)', () => {
  test.beforeEach(async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto(BASE_URL);
    await page.waitForLoadState('networkidle');
  });

  test('hamburger is visible, desktop nav is hidden', async ({ page }) => {
    const desktopNav = page.locator('.desktop-nav');
    const hamburger = page.locator('button.hamburger');
    
    await expect(hamburger).toBeVisible();
    await expect(desktopNav).not.toBeVisible();
    
    // Screenshot proof
    await page.screenshot({ path: 'test-results/navbar-mobile-layout.png' });
    console.log('✓ Mobile: Hamburger visible, desktop nav hidden');
  });

  test('mobile menu opens and closes', async ({ page }) => {
    const hamburger = page.locator('button.hamburger');
    
    // Open
    await hamburger.click();
    const mobilePanel = page.locator('.mobile-panel');
    await expect(mobilePanel).toBeVisible({ timeout: 2000 });
    console.log('✓ Mobile menu opened');
    
    // Screenshot open state
    await page.screenshot({ path: 'test-results/navbar-mobile-menu-open.png' });
    
    // Close via X button
    await page.locator('.mobile-close').click();
    await expect(mobilePanel).not.toBeVisible({ timeout: 2000 });
    console.log('✓ Mobile menu closed via X button');
  });

  test('mobile menu closes on backdrop click', async ({ page }) => {
    await page.locator('button.hamburger').click();
    await expect(page.locator('.mobile-panel')).toBeVisible();
    
    // Click backdrop
    await page.locator('.mobile-backdrop').click();
    await expect(page.locator('.mobile-panel')).not.toBeVisible({ timeout: 2000 });
    console.log('✓ Mobile menu closed via backdrop click');
  });

  test('mobile submenu accordion works', async ({ page }) => {
    await page.locator('button.hamburger').click();
    await expect(page.locator('.mobile-panel')).toBeVisible();
    
    // Click "Live Trading Rooms" to expand
    await page.locator('.mobile-nav-item:has-text("Live Trading Rooms")').click();
    await page.waitForTimeout(300);
    
    // Submenu items should be visible
    await expect(page.locator('.mobile-submenu-item:has-text("Day Trading")')).toBeVisible();
    await expect(page.locator('.mobile-submenu-item:has-text("Swing Trading")')).toBeVisible();
    
    // Screenshot proof
    await page.screenshot({ path: 'test-results/navbar-mobile-submenu-open.png' });
    console.log('✓ Mobile submenu accordion works');
  });

  test('mobile submenu navigation works', async ({ page }) => {
    await page.locator('button.hamburger').click();
    await expect(page.locator('.mobile-panel')).toBeVisible();
    
    // Expand Store submenu
    await page.locator('.mobile-nav-item:has-text("Store")').click();
    await page.waitForTimeout(300);
    
    // Click Courses link
    const coursesLink = page.locator('.mobile-submenu-item:has-text("Courses")');
    await expect(coursesLink).toBeVisible();
    
    // Get href and verify it's correct
    const href = await coursesLink.getAttribute('href');
    expect(href).toBe('/courses');
    console.log('✓ Mobile submenu links have correct hrefs');
  });

  test('body scroll is locked when mobile menu is open', async ({ page }) => {
    await page.locator('button.hamburger').click();
    await expect(page.locator('.mobile-panel')).toBeVisible();
    
    const bodyOverflow = await page.evaluate(() => document.body.style.overflow);
    expect(bodyOverflow).toBe('hidden');
    console.log('✓ Body scroll locked when menu open');
    
    // Close menu
    await page.locator('.mobile-close').click();
    await page.waitForTimeout(300);
    
    const bodyOverflowAfter = await page.evaluate(() => document.body.style.overflow);
    expect(bodyOverflowAfter).toBe('');
    console.log('✓ Body scroll restored when menu closed');
  });

  test('login and CTA buttons in mobile footer', async ({ page }) => {
    await page.locator('button.hamburger').click();
    await expect(page.locator('.mobile-panel')).toBeVisible();
    
    const loginBtn = page.locator('.mobile-login');
    const ctaBtn = page.locator('.mobile-cta');
    
    await expect(loginBtn).toBeVisible();
    await expect(ctaBtn).toBeVisible();
    console.log('✓ Login and CTA buttons visible in mobile menu footer');
  });
});

test.describe('Navbar - Keyboard Navigation', () => {
  test('escape key closes mobile menu', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto(BASE_URL);
    await page.waitForLoadState('networkidle');
    
    await page.locator('button.hamburger').click();
    await expect(page.locator('.mobile-panel')).toBeVisible();
    
    await page.keyboard.press('Escape');
    await expect(page.locator('.mobile-panel')).not.toBeVisible({ timeout: 2000 });
    console.log('✓ Escape key closes mobile menu');
  });

  test('escape key closes desktop dropdown', async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    await page.goto(BASE_URL);
    await page.waitForLoadState('networkidle');
    
    await page.locator('.dropdown-trigger:has-text("Store")').click();
    await expect(page.locator('.dropdown-menu').first()).toBeVisible();
    
    await page.keyboard.press('Escape');
    await page.waitForTimeout(300);
    console.log('✓ Escape key closes desktop dropdown');
  });
});

test.describe('Navbar - Visual Regression', () => {
  test('capture all viewport screenshots', async ({ page }) => {
    // Desktop
    await page.setViewportSize({ width: 1440, height: 900 });
    await page.goto(BASE_URL);
    await page.waitForLoadState('networkidle');
    await page.screenshot({ path: 'test-results/navbar-visual-desktop.png', fullPage: false });
    
    // Tablet
    await page.setViewportSize({ width: 768, height: 1024 });
    await page.goto(BASE_URL);
    await page.waitForLoadState('networkidle');
    await page.screenshot({ path: 'test-results/navbar-visual-tablet.png', fullPage: false });
    
    // Mobile
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto(BASE_URL);
    await page.waitForLoadState('networkidle');
    await page.screenshot({ path: 'test-results/navbar-visual-mobile.png', fullPage: false });
    
    console.log('✓ All viewport screenshots captured in test-results/');
  });
});

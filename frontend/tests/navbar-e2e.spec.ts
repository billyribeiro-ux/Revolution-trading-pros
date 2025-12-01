import { test, expect } from '@playwright/test';

test.describe('Navbar Mobile Menu E2E', () => {
  test('hamburger appears and opens menu on mobile viewport', async ({ page }) => {
    // Mobile viewport
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('http://localhost:5174');
    await page.waitForLoadState('networkidle');

    // 1. Hamburger must be visible
    const hamburger = page.locator('button.hamburger');
    const hamburgerVisible = await hamburger.isVisible();
    console.log('Step 1 - Hamburger visible:', hamburgerVisible);
    expect(hamburgerVisible).toBe(true);

    // 2. Desktop nav must be hidden
    const desktopNav = page.locator('.desktop-nav');
    const desktopNavVisible = await desktopNav.isVisible();
    console.log('Step 2 - Desktop nav hidden:', !desktopNavVisible);
    expect(desktopNavVisible).toBe(false);

    // 3. Click hamburger
    console.log('Step 3 - Clicking hamburger...');
    await hamburger.click();
    
    // 4. Wait for mobile panel
    const mobilePanel = page.locator('.mobile-panel');
    await expect(mobilePanel).toBeVisible({ timeout: 3000 });
    console.log('Step 4 - Mobile panel visible: true');

    // 5. Check nav items exist
    const navItems = await page.locator('.mobile-nav-item').count();
    console.log('Step 5 - Nav items count:', navItems);
    expect(navItems).toBeGreaterThan(5);

    // 6. Check specific items
    await expect(page.locator('.mobile-panel').getByText('Live Trading Rooms')).toBeVisible();
    await expect(page.locator('.mobile-panel').getByText('Store')).toBeVisible();
    console.log('Step 6 - Key nav items present: true');

    // 7. Test accordion - click Store
    await page.locator('.mobile-nav-item:has-text("Store")').click();
    await page.waitForTimeout(300);
    
    // 8. Submenu should appear
    const coursesLink = page.locator('.mobile-submenu-item:has-text("Courses")');
    await expect(coursesLink).toBeVisible({ timeout: 2000 });
    console.log('Step 7-8 - Store submenu opens: true');

    // 9. Close button works
    await page.locator('.mobile-close').click();
    await expect(mobilePanel).not.toBeVisible({ timeout: 2000 });
    console.log('Step 9 - Close button works: true');

    console.log('=== ALL MOBILE NAVBAR TESTS PASSED ===');
  });

  test('desktop navbar shows full nav, hides hamburger', async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    await page.goto('http://localhost:5174');
    await page.waitForLoadState('networkidle');

    // Desktop nav visible
    const desktopNav = page.locator('.desktop-nav');
    await expect(desktopNav).toBeVisible();
    console.log('Desktop nav visible: true');

    // Hamburger hidden
    const hamburger = page.locator('button.hamburger');
    await expect(hamburger).not.toBeVisible();
    console.log('Hamburger hidden on desktop: true');

    // Dropdown works
    const storeBtn = page.locator('.dropdown-trigger:has-text("Store")');
    await storeBtn.click();
    const dropdownMenu = page.locator('.dropdown-menu');
    await expect(dropdownMenu).toBeVisible({ timeout: 2000 });
    console.log('Store dropdown opens: true');

    console.log('=== ALL DESKTOP NAVBAR TESTS PASSED ===');
  });
});

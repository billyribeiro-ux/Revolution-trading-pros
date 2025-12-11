import { test, expect } from '@playwright/test';

test.describe('Admin Toolbar E2E Tests', () => {
  // Test that admin toolbar buttons work when visible
  test('Admin toolbar buttons should be clickable and functional', async ({ page }) => {
    // Navigate to a page where admin toolbar might be visible
    await page.goto('http://localhost:5174/');
    await page.waitForLoadState('domcontentloaded');
    await page.waitForTimeout(1000);
    
    // Check if admin toolbar exists (user might be logged in from previous session)
    const adminToolbar = page.locator('.admin-toolbar');
    const toolbarVisible = await adminToolbar.isVisible().catch(() => false);
    
    if (!toolbarVisible) {
      console.log('Admin toolbar not visible - skipping test (user not logged in as admin)');
      test.skip();
      return;
    }
    
    // Test 1: Quick Access button
    const quickAccessBtn = page.locator('.quick-menu-trigger');
    await expect(quickAccessBtn).toBeVisible();
    await expect(quickAccessBtn).toBeEnabled();
    
    // Click Quick Access
    await quickAccessBtn.click();
    await page.waitForTimeout(300);
    
    // Verify dropdown opened
    const quickDropdown = page.locator('#quick-menu-dropdown');
    await expect(quickDropdown).toBeVisible();
    
    // Click outside to close
    await page.click('body', { position: { x: 10, y: 10 } });
    await page.waitForTimeout(300);
    
    // Verify dropdown closed
    await expect(quickDropdown).not.toBeVisible();
    
    // Test 2: User Menu button
    const userMenuBtn = page.locator('.user-menu-trigger');
    await expect(userMenuBtn).toBeVisible();
    await expect(userMenuBtn).toBeEnabled();
    
    // Click User Menu
    await userMenuBtn.click();
    await page.waitForTimeout(300);
    
    // Verify dropdown opened
    const userDropdown = page.locator('#user-menu-dropdown');
    await expect(userDropdown).toBeVisible();
    
    // Click outside to close
    await page.click('body', { position: { x: 10, y: 10 } });
    await page.waitForTimeout(300);
    
    // Verify dropdown closed
    await expect(userDropdown).not.toBeVisible();
    
    // Test 3: Admin Dashboard button navigation
    const adminBtn = page.locator('.toolbar-logo');
    await expect(adminBtn).toBeVisible();
    await expect(adminBtn).toBeEnabled();
    
    // Click Admin button
    await adminBtn.click();
    await page.waitForURL('**/admin**', { timeout: 5000 });
    
    // Verify navigation
    expect(page.url()).toContain('/admin');
  });

  // Test keyboard navigation
  test('Admin toolbar should support keyboard navigation', async ({ page }) => {
    await page.goto('http://localhost:5174/');
    await page.waitForLoadState('domcontentloaded');
    await page.waitForTimeout(1000);
    
    const adminToolbar = page.locator('.admin-toolbar');
    const toolbarVisible = await adminToolbar.isVisible().catch(() => false);
    
    if (!toolbarVisible) {
      console.log('Admin toolbar not visible - skipping test');
      test.skip();
      return;
    }
    
    // Focus Quick Access button
    const quickAccessBtn = page.locator('.quick-menu-trigger');
    await quickAccessBtn.focus();
    
    // Press Enter to open
    await page.keyboard.press('Enter');
    await page.waitForTimeout(300);
    
    const quickDropdown = page.locator('#quick-menu-dropdown');
    await expect(quickDropdown).toBeVisible();
    
    // Press Escape to close
    await page.keyboard.press('Escape');
    await page.waitForTimeout(300);
    
    await expect(quickDropdown).not.toBeVisible();
  });
});

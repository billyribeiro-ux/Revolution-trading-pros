import { test, expect } from '@playwright/test';

test.describe('Admin Toolbar Click Debug', () => {
  test('Login and test admin toolbar buttons', async ({ page }) => {
    const logs: string[] = [];
    
    // Capture console messages
    page.on('console', msg => {
      const text = msg.text();
      logs.push(`[${msg.type().toUpperCase()}] ${text}`);
    });
    
    // Go to login page
    await page.goto('http://localhost:5174/login');
    await page.waitForLoadState('domcontentloaded');
    await page.waitForTimeout(500);
    
    // Fill login form
    await page.fill('input[type="email"]', 'welberribeirodrums@gmail.com');
    await page.fill('input[type="password"]', 'password123');
    
    // Submit
    await page.click('button[type="submit"]');
    
    // Wait for navigation
    await page.waitForTimeout(3000);
    
    // Check current URL
    const currentUrl = page.url();
    console.log('Current URL after login:', currentUrl);
    
    // Check for admin toolbar
    const adminToolbar = page.locator('.admin-toolbar');
    const toolbarCount = await adminToolbar.count();
    console.log('Admin toolbar count:', toolbarCount);
    
    if (toolbarCount > 0) {
      const isVisible = await adminToolbar.isVisible();
      console.log('Admin toolbar visible:', isVisible);
      
      // Test Quick Access button
      const quickAccessBtn = page.locator('.quick-menu-trigger');
      const quickBtnCount = await quickAccessBtn.count();
      console.log('Quick Access button count:', quickBtnCount);
      
      if (quickBtnCount > 0) {
        // Check if button is disabled
        const isDisabled = await quickAccessBtn.isDisabled();
        console.log('Quick Access disabled:', isDisabled);
        
        // Get button bounding box
        const box = await quickAccessBtn.boundingBox();
        console.log('Quick Access bounding box:', JSON.stringify(box));
        
        // Click the button
        console.log('Clicking Quick Access button...');
        await quickAccessBtn.click();
        await page.waitForTimeout(500);
        
        // Check if dropdown appeared
        const dropdown = page.locator('#quick-menu-dropdown');
        const dropdownVisible = await dropdown.isVisible().catch(() => false);
        console.log('Quick menu dropdown visible:', dropdownVisible);
        
        // Check showQuickMenu state via console logs
        const relevantLogs = logs.filter(l => l.includes('AdminToolbar'));
        console.log('\n=== AdminToolbar Logs ===');
        relevantLogs.forEach(l => console.log(l));
      }
      
      // Test User Menu button
      const userMenuBtn = page.locator('.user-menu-trigger');
      const userBtnCount = await userMenuBtn.count();
      console.log('\nUser Menu button count:', userBtnCount);
      
      if (userBtnCount > 0) {
        const isDisabled = await userMenuBtn.isDisabled();
        console.log('User Menu disabled:', isDisabled);
        
        console.log('Clicking User Menu button...');
        await userMenuBtn.click();
        await page.waitForTimeout(500);
        
        const userDropdown = page.locator('#user-menu-dropdown');
        const userDropdownVisible = await userDropdown.isVisible().catch(() => false);
        console.log('User menu dropdown visible:', userDropdownVisible);
      }
      
      // Test Admin Dashboard button
      const adminBtn = page.locator('.toolbar-logo');
      const adminBtnCount = await adminBtn.count();
      console.log('\nAdmin Dashboard button count:', adminBtnCount);
      
      if (adminBtnCount > 0) {
        const isDisabled = await adminBtn.isDisabled();
        console.log('Admin Dashboard disabled:', isDisabled);
        
        console.log('Clicking Admin Dashboard button...');
        await adminBtn.click();
        await page.waitForTimeout(1000);
        
        const newUrl = page.url();
        console.log('URL after Admin click:', newUrl);
        const navigatedToAdmin = newUrl.includes('/admin');
        console.log('Navigated to admin:', navigatedToAdmin);
      }
    } else {
      console.log('Admin toolbar not found - user may not be logged in or not admin');
      
      // Print all AdminToolbar related logs
      const relevantLogs = logs.filter(l => 
        l.includes('AdminToolbar') || 
        l.includes('Admin check') ||
        l.includes('isAdmin')
      );
      console.log('\n=== Relevant Logs ===');
      relevantLogs.forEach(l => console.log(l));
    }
  });
});

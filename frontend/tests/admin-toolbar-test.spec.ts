import { test, expect } from '@playwright/test';

test.describe('Admin Toolbar Investigation', () => {
  test('Check admin toolbar button functionality', async ({ page }) => {
    const errors: string[] = [];
    const logs: string[] = [];
    
    // Capture console messages
    page.on('console', msg => {
      if (msg.type() === 'error') {
        errors.push(`[ERROR] ${msg.text()}`);
      } else {
        logs.push(`[${msg.type().toUpperCase()}] ${msg.text()}`);
      }
    });
    
    // Capture page errors
    page.on('pageerror', err => {
      errors.push(`[PAGE ERROR] ${err.message}`);
    });
    
    // Set up auth token to simulate logged in state
    await page.goto('http://localhost:5174/login');
    await page.waitForLoadState('networkidle');
    
    // Check if there's a login form and try to login
    const emailInput = page.locator('input[type="email"], input[name="email"]').first();
    const passwordInput = page.locator('input[type="password"]').first();
    
    if (await emailInput.isVisible()) {
      console.log('Login form found, attempting login...');
      await emailInput.fill('welberribeirodrums@gmail.com');
      await passwordInput.fill('test123');
      
      const submitBtn = page.locator('button[type="submit"]').first();
      if (await submitBtn.isVisible()) {
        await submitBtn.click();
        await page.waitForTimeout(2000);
      }
    }
    
    // Navigate to home page
    await page.goto('http://localhost:5174/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);
    
    // Check for admin toolbar
    const adminToolbar = page.locator('.admin-toolbar');
    const toolbarVisible = await adminToolbar.isVisible().catch(() => false);
    console.log('Admin toolbar visible:', toolbarVisible);
    
    if (toolbarVisible) {
      // Check Quick Access button
      const quickAccessBtn = page.locator('.quick-menu-trigger');
      const quickAccessVisible = await quickAccessBtn.isVisible().catch(() => false);
      console.log('Quick Access button visible:', quickAccessVisible);
      
      if (quickAccessVisible) {
        // Check if button is disabled
        const isDisabled = await quickAccessBtn.isDisabled().catch(() => false);
        console.log('Quick Access button disabled:', isDisabled);
        
        // Try clicking
        await quickAccessBtn.click({ force: true });
        await page.waitForTimeout(500);
        
        // Check if dropdown opened
        const dropdown = page.locator('#quick-menu-dropdown');
        const dropdownVisible = await dropdown.isVisible().catch(() => false);
        console.log('Quick Access dropdown visible after click:', dropdownVisible);
      }
      
      // Check User Menu button
      const userMenuBtn = page.locator('.user-menu-trigger');
      const userMenuVisible = await userMenuBtn.isVisible().catch(() => false);
      console.log('User menu button visible:', userMenuVisible);
      
      if (userMenuVisible) {
        const isDisabled = await userMenuBtn.isDisabled().catch(() => false);
        console.log('User menu button disabled:', isDisabled);
        
        // Check the username text
        const userName = page.locator('.user-menu-trigger .user-name');
        const userNameText = await userName.textContent().catch(() => 'N/A');
        console.log('Username text:', userNameText);
        
        // Try clicking
        await userMenuBtn.click({ force: true });
        await page.waitForTimeout(500);
        
        const userDropdown = page.locator('#user-menu-dropdown');
        const userDropdownVisible = await userDropdown.isVisible().catch(() => false);
        console.log('User dropdown visible after click:', userDropdownVisible);
      }
    } else {
      console.log('Admin toolbar not visible - user may not be logged in as admin');
    }
    
    // Print errors
    if (errors.length > 0) {
      console.log('\n=== ERRORS ===');
      errors.forEach(e => console.log(e));
    }
    
    // Print relevant logs
    const relevantLogs = logs.filter(l => 
      l.includes('AdminToolbar') || 
      l.includes('auth') || 
      l.includes('user') ||
      l.includes('Admin')
    );
    if (relevantLogs.length > 0) {
      console.log('\n=== RELEVANT LOGS ===');
      relevantLogs.forEach(l => console.log(l));
    }
  });
});

import { test, expect } from '@playwright/test';

test.describe('Admin Toolbar Debug', () => {
  test('Debug admin toolbar button clicks with injected auth', async ({ page }) => {
    const errors: string[] = [];
    const logs: string[] = [];
    
    // Capture all console messages
    page.on('console', msg => {
      const text = msg.text();
      if (msg.type() === 'error') {
        errors.push(`[ERROR] ${text}`);
      }
      logs.push(`[${msg.type().toUpperCase()}] ${text}`);
    });
    
    page.on('pageerror', err => {
      errors.push(`[PAGE ERROR] ${err.message}`);
    });
    
    // Navigate to home page
    await page.goto('http://localhost:5174/', { timeout: 10000 });
    await page.waitForLoadState('domcontentloaded');
    
    // Inject mock auth state to simulate logged-in admin
    await page.evaluate(() => {
      const mockUser = {
        id: 1,
        name: 'Test Admin',
        email: 'welberribeirodrums@gmail.com',
        roles: ['super-admin'],
        is_admin: true,
        permissions: []
      };
      
      // Set session storage (used by auth store)
      sessionStorage.setItem('auth_session_id', 'test-session-123');
      sessionStorage.setItem('auth_user', JSON.stringify(mockUser));
      
      // Try to access the auth store directly if available
      if ((window as any).__SVELTE_AUTH_STORE__) {
        console.log('Found Svelte auth store');
      }
    });
    
    // Reload to pick up the auth state
    await page.reload({ timeout: 10000 });
    await page.waitForLoadState('domcontentloaded');
    await page.waitForTimeout(1000);
    
    // Check for admin toolbar
    const adminToolbar = page.locator('.admin-toolbar');
    const toolbarExists = await adminToolbar.count();
    console.log('Admin toolbar elements found:', toolbarExists);
    
    if (toolbarExists > 0) {
      const toolbarVisible = await adminToolbar.isVisible();
      console.log('Admin toolbar visible:', toolbarVisible);
      
      // Get computed styles
      const toolbarStyles = await adminToolbar.evaluate((el) => {
        const styles = window.getComputedStyle(el);
        return {
          display: styles.display,
          visibility: styles.visibility,
          opacity: styles.opacity,
          zIndex: styles.zIndex,
          position: styles.position,
          pointerEvents: styles.pointerEvents
        };
      });
      console.log('Toolbar computed styles:', JSON.stringify(toolbarStyles));
      
      // Check Quick Access button
      const quickAccessBtn = page.locator('.quick-menu-trigger');
      const quickAccessExists = await quickAccessBtn.count();
      console.log('Quick Access button count:', quickAccessExists);
      
      if (quickAccessExists > 0) {
        const btnStyles = await quickAccessBtn.evaluate((el) => {
          const styles = window.getComputedStyle(el);
          const rect = el.getBoundingClientRect();
          return {
            display: styles.display,
            visibility: styles.visibility,
            opacity: styles.opacity,
            pointerEvents: styles.pointerEvents,
            cursor: styles.cursor,
            disabled: (el as HTMLButtonElement).disabled,
            rect: { top: rect.top, left: rect.left, width: rect.width, height: rect.height }
          };
        });
        console.log('Quick Access button styles:', JSON.stringify(btnStyles));
        
        // Try clicking with force
        console.log('Attempting to click Quick Access button...');
        await quickAccessBtn.click({ force: true, timeout: 5000 }).catch(e => {
          console.log('Click failed:', e.message);
        });
        
        await page.waitForTimeout(500);
        
        // Check if dropdown appeared
        const dropdown = page.locator('#quick-menu-dropdown');
        const dropdownVisible = await dropdown.isVisible().catch(() => false);
        console.log('Dropdown visible after click:', dropdownVisible);
      }
      
      // Check User Menu button
      const userMenuBtn = page.locator('.user-menu-trigger');
      const userMenuExists = await userMenuBtn.count();
      console.log('User menu button count:', userMenuExists);
      
      if (userMenuExists > 0) {
        const btnStyles = await userMenuBtn.evaluate((el) => {
          const styles = window.getComputedStyle(el);
          return {
            display: styles.display,
            visibility: styles.visibility,
            opacity: styles.opacity,
            pointerEvents: styles.pointerEvents,
            cursor: styles.cursor,
            disabled: (el as HTMLButtonElement).disabled
          };
        });
        console.log('User menu button styles:', JSON.stringify(btnStyles));
        
        // Get username text
        const userNameEl = page.locator('.user-menu-trigger .user-name');
        const userName = await userNameEl.textContent().catch(() => 'N/A');
        console.log('Username displayed:', userName);
        
        // Try clicking
        console.log('Attempting to click User Menu button...');
        await userMenuBtn.click({ force: true, timeout: 5000 }).catch(e => {
          console.log('Click failed:', e.message);
        });
        
        await page.waitForTimeout(500);
        
        const userDropdown = page.locator('#user-menu-dropdown');
        const userDropdownVisible = await userDropdown.isVisible().catch(() => false);
        console.log('User dropdown visible after click:', userDropdownVisible);
      }
    } else {
      console.log('Admin toolbar NOT in DOM - checking why...');
      
      // Check if isAdmin condition is false
      const bodyClasses = await page.locator('body').getAttribute('class');
      console.log('Body classes:', bodyClasses);
      
      const hasAdminToolbarClass = await page.locator('.has-admin-toolbar').count();
      console.log('Has admin toolbar class:', hasAdminToolbarClass > 0);
    }
    
    // Print all errors
    if (errors.length > 0) {
      console.log('\n=== ALL ERRORS ===');
      errors.forEach(e => console.log(e));
    }
    
    // Print relevant logs
    const relevantLogs = logs.filter(l => 
      l.includes('AdminToolbar') || 
      l.includes('auth') || 
      l.includes('Admin') ||
      l.includes('isAdmin') ||
      l.includes('user')
    );
    if (relevantLogs.length > 0) {
      console.log('\n=== RELEVANT LOGS ===');
      relevantLogs.slice(0, 30).forEach(l => console.log(l));
    }
  });
});

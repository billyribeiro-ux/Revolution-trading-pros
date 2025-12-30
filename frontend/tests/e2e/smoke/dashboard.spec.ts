import { test, expect } from '@playwright/test';

/**
 * Smoke Test: Dashboard
 * Quick validation that the dashboard route exists and loads
 */

test.describe('Dashboard Smoke Tests', () => {
  test('should load dashboard page', async ({ page }) => {
    await page.goto('/dashboard');
    
    // Wait for page to load
    await page.waitForLoadState('networkidle');
    
    // Verify we're on a dashboard-related page (might redirect to login)
    const url = page.url();
    expect(url).toMatch(/dashboard|login/i);
  });

  test('should handle unauthenticated access', async ({ page }) => {
    await page.goto('/dashboard');
    
    // Should either show login or redirect
    const hasLoginForm = await page.locator('form[action*="login"], input[type="email"]').count() > 0;
    const hasLoginUrl = page.url().includes('login');
    
    expect(hasLoginForm || hasLoginUrl).toBeTruthy();
  });
});

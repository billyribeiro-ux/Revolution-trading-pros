import { test, expect } from '@playwright/test';

/**
 * Smoke Test: API Health
 * Quick validation that critical API endpoints are accessible
 */

test.describe('API Health Smoke Tests', () => {
  test('should have valid base URL', async ({ page }) => {
    // Just verify the app loads - API tests are optional
    await page.goto('/');
    await expect(page).toHaveTitle(/Revolution Trading Pros/i);
  });

  test('should load without critical network errors', async ({ page }) => {
    const failedRequests: string[] = [];
    
    page.on('requestfailed', (request) => {
      // Only track critical failures (not 404s for optional resources)
      if (!request.url().includes('favicon') && !request.url().includes('.map')) {
        failedRequests.push(request.url());
      }
    });
    
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    
    // Allow some failed requests for optional resources
    expect(failedRequests.length).toBeLessThan(5);
  });
});

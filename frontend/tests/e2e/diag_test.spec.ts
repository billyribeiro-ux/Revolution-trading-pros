import { test } from '@playwright/test';

const BASE = 'http://localhost:5174';

test('deep diagnose edit page', async ({ page }) => {
  const consoleLogs: string[] = [];
  
  page.on('console', msg => {
    consoleLogs.push(`[${msg.type()}] ${msg.text().slice(0, 300)}`);
  });

  await page.goto(`${BASE}/login`);
  await page.waitForLoadState('networkidle');
  await page.locator('#email').fill('welberribeirodrums@gmail.com');
  await page.locator('#password').fill('Davedicenso01!');
  await page.click('.submit-btn');
  await page.waitForURL(/\/(dashboard|admin)/, { timeout: 25000 });

  // Navigate to edit page (post 19 should exist)
  await page.goto(`${BASE}/admin/blog/edit/19`);
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(4000);
  
  const titleVal = await page.locator('.title-input').inputValue().catch(() => 'NOT FOUND');
  console.log('TITLE VALUE:', titleVal);
  console.log('ALL CONSOLE:', consoleLogs.join('\n'));
  await page.screenshot({ path: '/tmp/diag_edit.png' });
});

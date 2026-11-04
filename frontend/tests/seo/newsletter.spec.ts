import { expect, test } from '@playwright/test';

for (const action of ['confirm', 'unsubscribe']) {
	test(`Newsletter ${action} invalid link is private and server-rendered`, async ({ page }) => {
		const response = await page.goto(`/newsletter/${action}`, { waitUntil: 'domcontentloaded' });
		expect(response?.status()).toBe(400);
		expect(response?.headers()['x-robots-tag']).toContain('noindex');
		expect(response?.headers()['cache-control']).toContain('no-store');
		expect(response?.headers()['referrer-policy']).toBe('no-referrer');
		await expect(
			page.getByRole('heading', {
				name: 'Unable to update your newsletter subscription',
				exact: true
			})
		).toBeVisible();
		await expect(page.locator('head title')).toHaveCount(1);
		await expect(page.locator('meta[name="robots"]')).toHaveAttribute('content', /noindex/);
		expect(await page.locator('script:not([type="application/ld+json"])').count()).toBe(0);
		expect(await page.locator('link[rel="canonical"]').getAttribute('href')).not.toContain('?');
	});
}

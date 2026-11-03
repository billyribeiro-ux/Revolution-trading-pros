import { test, expect } from '@playwright/test';

test.describe('Robots.txt, Meta Robots & X-Robots-Tag (BOT-01..05)', () => {
	test('/robots.txt returns 200 text/plain with Sitemap and AI bot policy', async ({ request }) => {
		const res = await request.get('/robots.txt');
		expect(res.status()).toBe(200);
		expect(res.headers()['content-type']).toContain('text/plain');
		const body = await res.text();

		expect(body).toContain('User-agent: *');
		expect(body).toContain('Sitemap: https://revolutiontradingpros.com/sitemap.xml');
		expect(body).toContain('User-agent: GPTBot');
		expect(body).toContain('User-agent: Google-Extended');
		expect(body).toContain('User-agent: OAI-SearchBot');
		expect(body).toContain('User-agent: PerplexityBot');
		expect(body).not.toContain('Crawl-delay');
		expect(body).not.toContain('Disallow: /_app/');
	});

	test('Indexable pages emit index,follow and max-image-preview:large', async ({ page }) => {
		await page.goto('/about', { waitUntil: 'domcontentloaded' });
		const robotsContent = await page.locator('head meta[name="robots"]').getAttribute('content');
		expect(robotsContent).toContain('index');
		expect(robotsContent).toContain('follow');
		expect(robotsContent).toContain('max-image-preview:large');
	});

	test('Auth/private pages emit noindex,nofollow and X-Robots-Tag header', async ({
		page,
		request
	}) => {
		const res = await request.get('/login');
		expect(res.headers()['x-robots-tag']).toContain('noindex');

		await page.goto('/login', { waitUntil: 'domcontentloaded' });
		const robotsContent = await page.locator('head meta[name="robots"]').getAttribute('content');
		expect(robotsContent).toContain('noindex');
	});
});

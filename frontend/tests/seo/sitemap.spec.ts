import { test, expect } from '@playwright/test';

test.describe('XML Sitemap Integrity (MAP-01..06)', () => {
	test('/sitemap.xml returns valid XML with only canonical indexable URLs', async ({ request }) => {
		const res = await request.get('/sitemap.xml');
		expect(res.status()).toBe(200);
		expect(res.headers()['content-type']).toContain('application/xml');

		const xml = await res.text();
		expect(xml).toContain('xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"');

		const locs = Array.from(xml.matchAll(/<loc>([^<]+)<\/loc>/g)).map((m) => m[1]);
		expect(locs.length).toBeGreaterThanOrEqual(40);
		expect(locs.length).toBeLessThanOrEqual(50_000);

		const seen = new Set<string>();
		for (const loc of locs) {
			expect(loc).toMatch(/^https:\/\/revolutiontradingpros\.com/);
			expect(seen.has(loc), `Duplicate <loc> ${loc}`).toBe(false);
			seen.add(loc);
		}

		expect(locs).not.toContain('https://revolutiontradingpros.com/login');
		expect(locs).not.toContain('https://revolutiontradingpros.com/register');
		expect(locs).not.toContain('https://revolutiontradingpros.com/signup');
		expect(locs).toContain('https://revolutiontradingpros.com/authors/billy-ribeiro');
	});

	test('/news-sitemap.xml and /video-sitemap.xml do not emit fabricated URLs', async ({
		request
	}) => {
		for (const ep of ['/news-sitemap.xml', '/video-sitemap.xml']) {
			const res = await request.get(ep);
			expect(res.status()).toBe(200);
			expect(res.headers()['x-robots-tag']).toContain('noindex');
			const xml = await res.text();
			expect(xml).not.toContain('<loc>');
		}
	});
});

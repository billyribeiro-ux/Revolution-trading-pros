import { test, expect } from '@playwright/test';
import { PRODUCTION_ORIGIN } from '../../src/lib/seo/config.js';
function record(value: unknown): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		throw new Error('Expected actual published source record');
	return value as Record<string, unknown>;
}

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

	test('/news-sitemap.xml and /video-sitemap.xml match current saved published sources', async ({
		request
	}) => {
		for (const kind of ['news', 'video']) {
			const sourceResponse = await request.get(`/api/seo/sitemaps/${kind}?per_page=1000`);
			expect(sourceResponse.status()).toBe(200);
			const source = record(await sourceResponse.json()),
				entries = record(source.entries),
				meta = record(entries.meta);
			expect(source.metrics_scope).toBe('published_sitemap_sources');
			if (!Array.isArray(entries.data)) throw new Error('Actual published source page is missing');
			const sources = entries.data.map((value) => {
				const entry = record(value);
				expect(entry.eligible).toBe(true);
				expect(entry.exclusion_reason).toBeNull();
				return record(entry.record);
			});
			const response = await request.get(`/${kind}-sitemap.xml`);
			expect(response.status()).toBe(200);
			expect(response.headers()['x-robots-tag']).toContain('noindex');
			let xml = await response.text();
			if (Number(meta.last_page) > 1) {
				expect(xml).toContain('<sitemapindex');
				expect(Array.from(xml.matchAll(/<loc>/g))).toHaveLength(Number(meta.last_page));
				const first = await request.get(`/${kind}-sitemap.xml?page=1`);
				expect(first.status()).toBe(200);
				xml = await first.text();
			}
			const locations = Array.from(xml.matchAll(/<loc>([^<]+)<\/loc>/g)).map((match) => match[1]);
			expect(locations).toEqual(sources.map((source) => `${PRODUCTION_ORIGIN}${source.path}`));
			expect(xml).not.toMatch(/<priority>|<changefreq>/);
			if (record(source.settings).enabled === false) {
				expect(meta.total).toBe(0);
				expect(locations).toHaveLength(0);
			}
			for (const source of sources.slice(0, 3)) {
				const page = await request.get(String(source.path));
				expect(page.status()).toBe(200);
				const html = await page.text();
				expect(html).toContain(
					String(source.title)
						.replaceAll('&', '&amp;')
						.replaceAll('<', '&lt;')
						.replaceAll('>', '&gt;')
				);
				expect(html).toContain(`/authors/${source.author_slug}`);
				expect(typeof source.published_at).toBe('string');
			}
		}
	});
});

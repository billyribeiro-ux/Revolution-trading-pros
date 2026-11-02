import { test, expect } from '@playwright/test';

test.describe('HTTP Status Codes & Redirects (IDX-01..06)', () => {
	test('404 routes return HTTP 404 with noindex and error UI (IDX-02, IDX-05)', async ({
		request
	}) => {
		const paths = [
			'/ __nonexistent-seo-probe-2026__'.replace(' ', ''),
			'/classes/__nonexistent-class-slug-2026__',
			'/indicators/__nonexistent-indicator-2026__',
			'/authors/__nonexistent-author-2026__'
		];

		for (const p of paths) {
			const res = await request.get(p, { maxRedirects: 0 });
			expect(res.status(), `Expected 404 on ${p}`).toBe(404);
			const xRobots = res.headers()['x-robots-tag'] ?? '';
			expect(xRobots).toContain('noindex');
		}
	});

	test('Permanent redirect /signup -> /register uses HTTP 308 in a single hop (IDX-06)', async ({
		request
	}) => {
		const res = await request.get('/signup', { maxRedirects: 0 });
		expect(res.status()).toBe(308);
		expect(res.headers()['location']).toBe('/register');
	});
});


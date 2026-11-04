import { expect, test } from '@playwright/test';
import { PRODUCTION_ORIGIN, STATIC_NOINDEX_HOST_PATTERNS } from '../../src/lib/seo/config.js';

test('real local Cloudflare asset serving applies preview noindex without changing production marketing policy', async ({
	request,
	baseURL
}) => {
	expect(baseURL).toBe('http://127.0.0.1:5276');
	for (const rule of STATIC_NOINDEX_HOST_PATTERNS) {
		const hostname = rule
			.slice('https://'.length, -2)
			.replace(':project', 'admin-fixture')
			.replace(':version', 'revision-fixture');
		// The socket always connects to the loopback fixture. No DNS/provider request.
		const response = await request.get('/about', { headers: { Host: hostname }, maxRedirects: 0 });
		expect(response.status(), hostname).toBe(200);
		expect(response.headers()['x-robots-tag'], hostname).toContain('noindex');
	}
	const productionHost = new URL(PRODUCTION_ORIGIN).hostname;
	const publicPage = await request.get('/about', {
		headers: { Host: productionHost },
		maxRedirects: 0
	});
	expect(publicPage.status()).toBe(200);
	expect(publicPage.headers()['x-robots-tag']).toBeUndefined();
	const internalDemo = await request.get('/popup-demo', {
		headers: { Host: productionHost },
		maxRedirects: 0
	});
	expect(internalDemo.status()).toBe(200);
	expect(internalDemo.headers()['x-robots-tag']).toContain('noindex');
	const privatePage = await request.get('/admin', { maxRedirects: 0 });
	expect(privatePage.status()).toBe(303);
	expect(privatePage.headers()['cache-control']).toBe('no-store');
	expect(privatePage.headers()['x-robots-tag']).toContain('noindex');
	const missing = await request.get('/about/missing-fixture-page', { maxRedirects: 0 });
	expect(missing.status()).toBe(404);
	expect(missing.headers()['x-robots-tag']).toContain('noindex');
});

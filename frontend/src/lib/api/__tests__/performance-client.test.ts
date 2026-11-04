import { afterEach, expect, it, vi } from 'vite-plus/test';
vi.mock('#lib/stores/auth.svelte.js', () => ({ getAuthToken: () => null }));
import { bingSeoApi } from '../bing-seo.js';
afterEach(() => vi.unstubAllGlobals());
it('constructs same-origin performance URLs and preserves filters and envelopes', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValue(Response.json({ data: { fixture: true } }));
	vi.stubGlobal('fetch', fetch);
	expect(
		await bingSeoApi.getPerformanceDashboard({
			url: 'https://example.test/a?x=1',
			device: 'mobile',
			days: 7
		})
	).toEqual({ fixture: true });
	expect(fetch.mock.calls[0]?.[0]).toBe(
		'/api/performance/dashboard?url=https%3A%2F%2Fexample.test%2Fa%3Fx%3D1&device=mobile&days=7'
	);
	expect(fetch.mock.calls[0]?.[1]?.credentials).toBe('include');
});
it('surfaces structured backend errors instead of an empty measurement report', async () => {
	vi.stubGlobal(
		'fetch',
		vi
			.fn<typeof globalThis.fetch>()
			.mockResolvedValue(
				Response.json({ error: { message: 'Performance storage unavailable' } }, { status: 503 })
			)
	);
	await expect(bingSeoApi.getPerformanceDashboard()).rejects.toThrow(
		'Performance storage unavailable'
	);
});

// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';

vi.mock('$app/env/private', () => ({ API_BASE_URL: 'http://127.0.0.1:8281', BACKEND_URL: '' }));

import { GET } from './+server';

const token = '01234567-89ab-cdef-0123-456789abcdef';

function event(id = token, width = '768', accept = 'image/avif,image/webp') {
	const url = new URL(`http://fixture.test/api/media/image/${id}?width=${width}`);
	const fetch = vi.fn<typeof globalThis.fetch>();
	return {
		fetch,
		event: {
			params: { token: id },
			url,
			request: new Request(url, { headers: { Accept: accept } }),
			fetch
		} as unknown as Parameters<typeof GET>[0]
	};
}

afterEach(() => vi.clearAllMocks());

it('forwards format and width negotiation and returns only a bounded public redirect', async () => {
	const { event: request, fetch } = event();
	fetch.mockResolvedValue(
		new Response(null, {
			status: 302,
			headers: {
				Location: 'https://assets.example.test/media/variants/42/fixture.webp',
				'Cache-Control': 'public, max-age=30, s-maxage=30'
			}
		})
	);
	const response = await GET(request);
	expect(fetch).toHaveBeenCalledWith(
		`http://127.0.0.1:8281/api/media/image/${token}?width=768`,
		expect.objectContaining({
			redirect: 'manual',
			headers: { Accept: 'image/avif,image/webp' },
			signal: expect.any(AbortSignal)
		})
	);
	expect(response.status).toBe(302);
	expect(response.headers.get('location')).toBe(
		'https://assets.example.test/media/variants/42/fixture.webp'
	);
	expect(response.headers.get('cache-control')).toBe('public, max-age=30, s-maxage=30');
	expect(response.headers.get('vary')).toBe('Accept');
});

it('rejects malformed selectors and fails closed when the backend is unavailable', async () => {
	const invalid = event('not-a-token');
	expect((await GET(invalid.event)).status).toBe(404);
	expect(invalid.fetch).not.toHaveBeenCalled();
	const badWidth = event(token, '9999');
	expect((await GET(badWidth.event)).status).toBe(400);
	expect(badWidth.fetch).not.toHaveBeenCalled();
	const unavailable = event();
	unavailable.fetch.mockRejectedValue(new Error('backend unavailable'));
	expect((await GET(unavailable.event)).status).toBe(503);
});

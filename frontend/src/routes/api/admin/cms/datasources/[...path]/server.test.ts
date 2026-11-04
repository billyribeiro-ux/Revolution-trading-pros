// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
vi.mock('$app/env/private', () => ({ API_BASE_URL: 'http://127.0.0.1:8281', BACKEND_URL: '' }));
const auth = vi.hoisted(() => vi.fn(() => ({ token: 'fixture-access-token' })));
vi.mock('#lib/server/auth.js', () => ({ requireAdmin: auth }));
import { DELETE, GET } from './+server';
afterEach(() => {
	vi.unstubAllGlobals();
	vi.clearAllMocks();
});
function event(method: string, path: string, body?: string) {
	const url = new URL(`http://fixture.example.test/api/admin/cms/datasources/${path}?limit=2`);
	return {
		params: { path },
		url,
		request: new Request(url, {
			method,
			...(body ? { body, headers: { 'Content-Type': 'application/json' } } : {})
		})
	} as unknown as Parameters<typeof GET>[0];
}
it('uses the mounted CMS namespace and preserves CSV bytes, metadata and authenticated access', async () => {
	const fetch = vi.fn<typeof globalThis.fetch>().mockResolvedValue(
		new Response('name,value\nFixture,one\n', {
			headers: {
				'Content-Type': 'text/csv',
				'Content-Disposition': 'attachment; filename="fixture.csv"'
			}
		})
	);
	vi.stubGlobal('fetch', fetch);
	const response = await GET(event('GET', 'fixture/export'));
	expect(auth).toHaveBeenCalledOnce();
	expect(fetch).toHaveBeenCalledWith(
		'http://127.0.0.1:8281/api/cms/datasources/fixture/export?limit=2',
		expect.objectContaining({ method: 'GET', signal: expect.any(AbortSignal) })
	);
	expect(new Headers(fetch.mock.calls[0]?.[1]?.headers).get('Authorization')).toBe(
		'Bearer fixture-access-token'
	);
	expect(response.headers.get('Content-Disposition')).toBe('attachment; filename="fixture.csv"');
	expect(response.headers.get('Cache-Control')).toBe('no-store');
	expect(await response.text()).toBe('name,value\nFixture,one\n');
});
it('preserves a DELETE payload and accepts an actual empty 204 response', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValue(new Response(null, { status: 204 }));
	vi.stubGlobal('fetch', fetch);
	const response = await DELETE(event('DELETE', 'fixture', '{"reason":"fixture"}'));
	expect(response.status).toBe(204);
	expect(await response.text()).toBe('');
	const body = fetch.mock.calls[0]?.[1]?.body;
	if (!(body instanceof ArrayBuffer)) throw new Error('Expected the original request bytes');
	expect(new TextDecoder().decode(body)).toBe('{"reason":"fixture"}');
});
it('reports an unavailable upstream without inventing a successful response', async () => {
	vi.stubGlobal(
		'fetch',
		vi.fn<typeof globalThis.fetch>().mockRejectedValue(new TypeError('Fixture unavailable'))
	);
	const response = await GET(event('GET', ''));
	expect(response.status).toBe(503);
	expect((await response.json()).success).toBe(false);
});

// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
vi.mock('$app/env/private', () => ({ API_BASE_URL: 'http://127.0.0.1:8281', BACKEND_URL: '' }));
const auth = vi.hoisted(() => vi.fn(() => ({ token: 'fixture-access-token' })));
vi.mock('#lib/server/auth.js', () => ({ requireAdmin: auth }));
import { DELETE, GET, POST } from './+server';
afterEach(() => {
	vi.unstubAllGlobals();
	vi.clearAllMocks();
});
function event(method: string, path: string, body?: string) {
	const url = new URL(
		`http://fixture.example.test/api/admin/email/subscribers/${path}?status=pending`
	);
	return {
		params: { path },
		url,
		request: new Request(url, {
			method,
			...(body ? { body, headers: { 'Content-Type': 'application/json' } } : {})
		})
	} as unknown as Parameters<typeof GET>[0];
}
it('preserves filtered CSV bytes, metadata and authentication', async () => {
	const fetch = vi.fn<typeof globalThis.fetch>().mockResolvedValue(
		new Response('email,status\r\nfixture@example.test,pending\r\n', {
			headers: {
				'Content-Type': 'text/csv',
				'Content-Disposition': 'attachment; filename="subscribers.csv"'
			}
		})
	);
	vi.stubGlobal('fetch', fetch);
	const response = await GET(event('GET', 'export'));
	expect(auth).toHaveBeenCalledOnce();
	expect(fetch).toHaveBeenCalledWith(
		'http://127.0.0.1:8281/api/admin/email/subscribers/export?status=pending',
		expect.objectContaining({ method: 'GET', signal: expect.any(AbortSignal), redirect: 'error' })
	);
	expect(new Headers(fetch.mock.calls[0]?.[1]?.headers).get('Authorization')).toBe(
		'Bearer fixture-access-token'
	);
	expect(response.headers.get('Content-Disposition')).toBe(
		'attachment; filename="subscribers.csv"'
	);
	expect(response.headers.get('Cache-Control')).toBe('no-store');
	expect(await response.text()).toContain('fixture@example.test,pending');
});
it('preserves DELETE tag bodies and upstream validation status', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValue(Response.json({ message: 'Tags are invalid' }, { status: 422 }));
	vi.stubGlobal('fetch', fetch);
	const response = await DELETE(event('DELETE', '12/tags', '{"tags":["fixture"]}'));
	expect(response.status).toBe(422);
	const body = fetch.mock.calls[0]?.[1]?.body;
	if (!(body instanceof ArrayBuffer)) throw new Error('Expected request bytes');
	expect(new TextDecoder().decode(body)).toBe('{"tags":["fixture"]}');
});
it('rejects oversized mutations before forwarding and reports transport uncertainty', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockRejectedValue(new TypeError('Fixture offline'));
	vi.stubGlobal('fetch', fetch);
	expect((await POST(event('POST', 'bulk-delete', 'x'.repeat(65537)))).status).toBe(413);
	expect(fetch).not.toHaveBeenCalled();
	const response = await GET(event('GET', 'stats'));
	expect(response.status).toBe(502);
	expect((await response.json()).message).toContain('refresh before retrying');
});

// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
vi.mock('$app/env/private', () => ({ API_BASE_URL: 'http://127.0.0.1:8281', BACKEND_URL: '' }));
const auth = vi.hoisted(() => ({
	admin: vi.fn(() => ({ token: 'fixture-token' })),
	superadmin: vi.fn(() => ({ token: 'fixture-token' }))
}));
vi.mock('../auth.js', () => ({ requireAdmin: auth.admin, requireSuperadmin: auth.superadmin }));
import { proxyConnections } from '../connection-proxy.js';
function event(method: string, body?: string, key = 'stripe') {
	const url = new URL('http://fixture.example.test/api/admin/connections?environment=sandbox');
	return {
		url,
		params: { key },
		request: new Request(url, { method, ...(body ? { body } : {}) })
	} as unknown as Parameters<typeof proxyConnections>[0];
}
afterEach(() => {
	vi.unstubAllGlobals();
	vi.clearAllMocks();
});
it('keeps environment scope, real upstream errors and private headers through disconnect', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValue(Response.json({ error: 'Fixture connection not found' }, { status: 404 }));
	vi.stubGlobal('fetch', fetch);
	const response = await proxyConnections(event('POST'), 'disconnect');
	expect(auth.superadmin).toHaveBeenCalledOnce();
	expect(auth.admin).not.toHaveBeenCalled();
	expect(fetch).toHaveBeenCalledWith(
		'http://127.0.0.1:8281/api/admin/connections/stripe/disconnect?environment=sandbox',
		expect.objectContaining({ method: 'POST', redirect: 'error', signal: expect.any(AbortSignal) })
	);
	expect(new Headers(fetch.mock.calls[0]?.[1]?.headers).get('Authorization')).toBe(
		'Bearer fixture-token'
	);
	expect(response.status).toBe(404);
	expect(await response.json()).toEqual({ error: 'Fixture connection not found' });
	expect(response.headers.get('Cache-Control')).toBe('no-store');
});
it('forwards JSON test bytes and rejects UTF-8 oversized requests before transport', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValue(Response.json({ success: false, status: 'rejected' }));
	vi.stubGlobal('fetch', fetch);
	const body = '{"credentials":{"secret_key":"fixture"},"environment":"sandbox"}';
	await proxyConnections(event('POST', body), 'test');
	const sent = fetch.mock.calls[0]?.[1]?.body;
	if (!(sent instanceof ArrayBuffer)) throw new Error('Expected bounded request bytes');
	expect(new TextDecoder().decode(sent)).toBe(body);
	fetch.mockClear();
	expect((await proxyConnections(event('POST', 'é'.repeat(32769)), 'connect')).status).toBe(413);
	expect((await proxyConnections(event('POST', body, '../../escape'), 'connect')).status).toBe(400);
	expect(fetch).not.toHaveBeenCalled();
});
it('reports service uncertainty rather than synthetic success and gates read access', async () => {
	vi.stubGlobal(
		'fetch',
		vi.fn<typeof globalThis.fetch>().mockRejectedValue(new TypeError('Fixture offline'))
	);
	const response = await proxyConnections(event('GET'));
	expect(auth.admin).toHaveBeenCalledOnce();
	expect(auth.superadmin).not.toHaveBeenCalled();
	expect(response.status).toBe(502);
	expect((await response.json()).error).toContain('refresh before retrying');
});

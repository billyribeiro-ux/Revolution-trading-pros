// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
vi.mock('$app/env/private', () => ({ API_BASE_URL: 'http://backend.test', BACKEND_URL: '' }));
const auth = vi.hoisted(() => ({ admin: vi.fn(() => ({ token: 'fixture-token' })) }));
vi.mock('../auth.js', () => ({ requireAdmin: auth.admin }));
import { proxySequences } from '../sequence-proxy.js';
function event(method: string, rest = '', body?: string) {
	const url = new URL('http://frontend.test/api/admin/email/sequences?status=paused&page=2');
	return {
		url,
		params: { rest },
		request: new Request(url, { method, ...(body ? { body } : {}) })
	} as unknown as Parameters<typeof proxySequences>[0];
}
afterEach(() => {
	vi.unstubAllGlobals();
	vi.clearAllMocks();
});
it('preserves a sequence action failure and forwards an explicit write exactly once', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValue(
			Response.json({ error: 'Reconcile this receipt before resuming' }, { status: 409 })
		);
	vi.stubGlobal('fetch', fetch);
	const response = await proxySequences(event('POST', '42/activate', '{}'));
	expect(response.status).toBe(409);
	expect(await response.json()).toEqual({ error: 'Reconcile this receipt before resuming' });
	expect(response.headers.get('Cache-Control')).toContain('no-store');
	expect(fetch).toHaveBeenCalledExactlyOnceWith(
		'http://backend.test/api/admin/crm/sequences/42/activate?status=paused&page=2',
		expect.objectContaining({
			method: 'POST',
			redirect: 'error',
			credentials: 'omit',
			headers: expect.objectContaining({ Authorization: 'Bearer fixture-token' })
		})
	);
	expect(auth.admin).toHaveBeenCalledOnce();
});
it('accepts actual UUID email identities and rejects traversal, wrong methods and oversized bytes before upstream writes', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValue(Response.json({ id: 'fixture' }));
	vi.stubGlobal('fetch', fetch);
	expect(
		(await proxySequences(event('PUT', '42/emails/34ad48b0-615b-49e8-86ee-c0d9f77206ee', '{}')))
			.status
	).toBe(200);
	expect(fetch).toHaveBeenCalledOnce();
	fetch.mockClear();
	for (const [method, path] of [
		['GET', '../settings'],
		['GET', '42/activate'],
		['POST', '42/subscribers'],
		['DELETE', '42/emails/reorder'],
		['PUT', '42/emails/not-a-uuid'],
		['POST', '42/unsupported']
	] as const)
		expect((await proxySequences(event(method, path))).status).toBe(404);
	expect((await proxySequences(event('POST', '', 'é'.repeat(4500001)))).status).toBe(413);
	expect(fetch).not.toHaveBeenCalled();
});
it('never retries an unconfirmed PUT or DELETE when the service fails', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockRejectedValue(new Error('Controlled unavailable service'));
	vi.stubGlobal('fetch', fetch);
	for (const method of ['PUT', 'DELETE'])
		expect((await proxySequences(event(method, '42', '{}'))).status).toBe(503);
	expect(fetch).toHaveBeenCalledTimes(2);
});

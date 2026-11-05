// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
vi.mock('$app/env/private', () => ({ API_BASE_URL: 'http://backend.test', BACKEND_URL: '' }));
const auth = vi.hoisted(() => ({ admin: vi.fn(() => ({ token: 'fixture-token' })) }));
vi.mock('../auth.js', () => ({ requireAdmin: auth.admin }));
import { proxyEmailCampaigns } from '../email-campaign-proxy.js';
function event(method: string, rest = '', body?: string) {
	const url = new URL('http://frontend.test/api/admin/email/campaigns?per_page=20&page=2');
	return {
		url,
		params: { rest },
		request: new Request(url, { method, ...(body ? { body } : {}) })
	} as unknown as Parameters<typeof proxyEmailCampaigns>[0];
}
afterEach(() => {
	vi.unstubAllGlobals();
	vi.clearAllMocks();
});
it('preserves uncertain submission failures and forwards a campaign write only once', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValue(
			Response.json({ error: 'Outcome unknown; reconcile first' }, { status: 409 })
		);
	vi.stubGlobal('fetch', fetch);
	const result = await proxyEmailCampaigns(event('POST', '42/send', '{}'));
	expect(result.status).toBe(409);
	expect(await result.json()).toEqual({ error: 'Outcome unknown; reconcile first' });
	expect(result.headers.get('Cache-Control')).toContain('no-store');
	expect(auth.admin).toHaveBeenCalledOnce();
	expect(fetch).toHaveBeenCalledExactlyOnceWith(
		'http://backend.test/api/admin/email/campaigns/42/send?per_page=20&page=2',
		expect.objectContaining({
			method: 'POST',
			redirect: 'error',
			credentials: 'omit',
			headers: expect.objectContaining({ Authorization: 'Bearer fixture-token' })
		})
	);
	expect(new TextDecoder().decode(fetch.mock.calls[0]?.[1]?.body as Uint8Array)).toBe('{}');
});
it('rejects traversal, unsupported actions, wrong methods and actual oversized bodies before upstream traffic', async () => {
	const fetch = vi.fn<typeof globalThis.fetch>();
	vi.stubGlobal('fetch', fetch);
	for (const [method, path] of [
		['GET', '../settings'],
		['POST', '42/report'],
		['GET', '42/send'],
		['PUT', ''],
		['POST', '42/unsupported']
	] as const)
		expect((await proxyEmailCampaigns(event(method, path))).status).toBe(404);
	expect((await proxyEmailCampaigns(event('POST', '', 'é'.repeat(4500001)))).status).toBe(413);
	expect(fetch).not.toHaveBeenCalled();
});
it('reports unavailable service without retrying a send', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockRejectedValue(new Error('Fixture unavailable'));
	vi.stubGlobal('fetch', fetch);
	expect((await proxyEmailCampaigns(event('POST', '42/send', '{}'))).status).toBe(503);
	expect(fetch).toHaveBeenCalledOnce();
});

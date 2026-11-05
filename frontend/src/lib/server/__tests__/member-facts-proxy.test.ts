// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
vi.mock('$app/env/private', () => ({ API_BASE_URL: 'http://backend.test', BACKEND_URL: '' }));
const auth = vi.hoisted(() => ({ admin: vi.fn(() => ({ token: 'fixture-token' })) }));
vi.mock('../auth.js', () => ({ requireAdmin: auth.admin }));
import { proxyMemberFacts } from '../member-facts-proxy';
const event = () =>
	({
		url: new URL(
			'http://frontend.test/api/admin/members/services?search=Actual&selected_id=17&page=2'
		)
	}) as Parameters<typeof proxyMemberFacts>[0];
afterEach(() => {
	vi.unstubAllGlobals();
	vi.clearAllMocks();
});
it('forwards the exact catalog query once with admin identity and no-store response semantics', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValue(Response.json({ schema_version: 1, services: [] }));
	vi.stubGlobal('fetch', fetch);
	const response = await proxyMemberFacts(event(), 'services');
	expect(response.status).toBe(200);
	expect(await response.json()).toEqual({ schema_version: 1, services: [] });
	expect(response.headers.get('cache-control')).toContain('no-store');
	expect(fetch).toHaveBeenCalledExactlyOnceWith(
		'http://backend.test/api/admin/members/services?search=Actual&selected_id=17&page=2',
		expect.objectContaining({
			method: 'GET',
			credentials: 'omit',
			redirect: 'error',
			headers: { Authorization: 'Bearer fixture-token', Accept: 'application/json' }
		})
	);
	expect(auth.admin).toHaveBeenCalledOnce();
});
it('preserves real validation/storage failure statuses and never substitutes successful zero facts', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValueOnce(Response.json({ message: 'Invalid plan status' }, { status: 422 }))
		.mockResolvedValueOnce(Response.json({ message: 'Storage failed' }, { status: 500 }))
		.mockRejectedValueOnce(new Error('offline'));
	vi.stubGlobal('fetch', fetch);
	for (const status of [422, 500, 503]) {
		const response = await proxyMemberFacts(event(), 'stats');
		expect(response.status).toBe(status);
		const body = await response.json();
		expect(body.overview).toBeUndefined();
		expect(typeof body.message).toBe('string');
	}
	expect(fetch).toHaveBeenCalledTimes(3);
});
it('performs no upstream read when the admin guard rejects the caller', async () => {
	const fetch = vi.fn<typeof globalThis.fetch>();
	vi.stubGlobal('fetch', fetch);
	auth.admin.mockImplementationOnce(() => {
		throw new Error('Forbidden');
	});
	await expect(proxyMemberFacts(event(), 'stats')).rejects.toThrow('Forbidden');
	expect(fetch).not.toHaveBeenCalled();
});

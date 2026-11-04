// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
vi.mock('$app/env/private', () => ({ API_BASE_URL: 'http://backend.test', BACKEND_URL: '' }));
const auth = vi.hoisted(() => ({
	admin: vi.fn(() => ({ token: 'fixture-token' })),
	superadmin: vi.fn(() => ({ token: 'fixture-token' }))
}));
vi.mock('../auth.js', () => ({ requireAdmin: auth.admin, requireSuperadmin: auth.superadmin }));
import { proxyMemberConfig } from '../member-config-proxy.js';
function event(method: string, id = '', body?: string) {
	const url = new URL('http://frontend.test/api/admin/members/tags?page=2&search=cohort');
	return {
		url,
		params: { id },
		request: new Request(url, { method, ...(body ? { body } : {}) })
	} as unknown as Parameters<typeof proxyMemberConfig>[0];
}
afterEach(() => {
	vi.unstubAllGlobals();
	vi.clearAllMocks();
});
it('forwards the removal body once with ordinary admin authorization', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValue(Response.json({ message: 'Removed' }));
	vi.stubGlobal('fetch', fetch);
	const response = await proxyMemberConfig(
		event('DELETE', '', JSON.stringify({ user_id: 12, tag_id: 34 })),
		'tags',
		'unassign'
	);
	expect(response.status).toBe(200);
	expect(fetch).toHaveBeenCalledExactlyOnceWith(
		'http://backend.test/api/admin/members/tags/unassign?page=2&search=cohort',
		expect.objectContaining({ method: 'DELETE', redirect: 'error', credentials: 'omit' })
	);
	expect(new TextDecoder().decode(fetch.mock.calls[0]?.[1]?.body as Uint8Array)).toBe(
		'{"user_id":12,"tag_id":34}'
	);
	expect(auth.admin).toHaveBeenCalledOnce();
	expect(auth.superadmin).not.toHaveBeenCalled();
});
it('enforces superadmin for deleting records and preserves actual validation failures', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValue(new Response('System segments cannot be deleted', { status: 409 }));
	vi.stubGlobal('fetch', fetch);
	const response = await proxyMemberConfig(event('DELETE', '42'), 'segments');
	expect(response.status).toBe(409);
	expect(await response.text()).toBe('System segments cannot be deleted');
	expect(response.headers.get('Cache-Control')).toContain('no-store');
	expect(auth.superadmin).toHaveBeenCalledOnce();
	expect(fetch).toHaveBeenCalledOnce();
});
it('bounds actual request bytes and rejects traversal without upstream traffic', async () => {
	const fetch = vi.fn<typeof globalThis.fetch>();
	vi.stubGlobal('fetch', fetch);
	expect((await proxyMemberConfig(event('POST', '', 'é'.repeat(131073)), 'segments')).status).toBe(
		413
	);
	expect((await proxyMemberConfig(event('GET', '../users'), 'segments')).status).toBe(404);
	expect(fetch).not.toHaveBeenCalled();
});

it('streams the actual CSV and attachment headers without replacing it with JSON', async () => {
	const fetch = vi.fn<typeof globalThis.fetch>().mockResolvedValue(
		new Response('id,name\r\n42,Fixture\r\n', {
			headers: {
				'Content-Type': 'text/csv; charset=utf-8',
				'Content-Disposition': 'attachment; filename="member-segment-42.csv"'
			}
		})
	);
	vi.stubGlobal('fetch', fetch);
	const response = await proxyMemberConfig(event('GET', '42/export'), 'segments');
	expect(response.headers.get('Content-Type')).toBe('text/csv; charset=utf-8');
	expect(response.headers.get('Content-Disposition')).toContain('member-segment-42.csv');
	expect(await response.text()).toBe('id,name\r\n42,Fixture\r\n');
	expect(auth.admin).toHaveBeenCalledOnce();
});

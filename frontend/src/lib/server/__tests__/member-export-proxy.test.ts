// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
vi.mock('$app/env/private', () => ({ API_BASE_URL: 'http://backend.test', BACKEND_URL: '' }));
const auth = vi.hoisted(() => ({ admin: vi.fn(() => ({ token: 'fixture-token' })) }));
vi.mock('../auth.js', () => ({ requireAdmin: auth.admin }));
import { proxyMemberExport } from '../member-export-proxy';
function event() {
	return {
		url: new URL(
			'http://frontend.test/api/admin/member-management/export?format=pdf&date_to=2026-11-05'
		)
	} as Parameters<typeof proxyMemberExport>[0];
}
afterEach(() => {
	vi.unstubAllGlobals();
	vi.clearAllMocks();
});
it('streams actual export bytes and headers once through the dedicated admin route', async () => {
	const fetch = vi.fn<typeof globalThis.fetch>().mockResolvedValue(
		new Response('%PDF-fixture', {
			headers: {
				'Content-Type': 'application/pdf',
				'Content-Disposition': 'attachment; filename="members.pdf"',
				'X-Export-Row-Count': '32',
				'X-Export-Sha256': 'fixture-digest'
			}
		})
	);
	vi.stubGlobal('fetch', fetch);
	const response = await proxyMemberExport(event(), true);
	expect(await response.text()).toBe('%PDF-fixture');
	expect(response.headers.get('x-export-row-count')).toBe('32');
	expect(response.headers.get('x-export-sha256')).toBe('fixture-digest');
	expect(response.headers.get('cache-control')).toContain('no-store');
	expect(fetch).toHaveBeenCalledExactlyOnceWith(
		'http://backend.test/api/admin/member-management/export?format=pdf&date_to=2026-11-05',
		expect.objectContaining({
			redirect: 'error',
			credentials: 'omit',
			headers: { Authorization: 'Bearer fixture-token' }
		})
	);
	expect(auth.admin).toHaveBeenCalledOnce();
});
it('preserves validation errors and fails truthfully when the service is unavailable', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValueOnce(Response.json({ message: 'Invalid date' }, { status: 422 }))
		.mockRejectedValueOnce(new Error('offline'));
	vi.stubGlobal('fetch', fetch);
	const rejected = await proxyMemberExport(event(), false);
	expect(rejected.status).toBe(422);
	expect(await rejected.json()).toEqual({ message: 'Invalid date' });
	expect((await proxyMemberExport(event(), true)).status).toBe(503);
	expect(fetch).toHaveBeenCalledTimes(2);
});

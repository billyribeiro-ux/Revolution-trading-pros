// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
vi.mock('$app/env/private', () => ({ API_BASE_URL: 'http://backend.test', BACKEND_URL: '' }));
const auth = vi.hoisted(() => ({
	admin: vi.fn(() => ({ token: 'fixture-token' })),
	superadmin: vi.fn(() => ({ token: 'fixture-token' }))
}));
vi.mock('../auth.js', () => ({ requireAdmin: auth.admin, requireSuperadmin: auth.superadmin }));
import { proxyEmailTemplates } from '../email-template-proxy.js';
function event(method: string, rest = '', body?: string) {
	const url = new URL(
		'http://frontend.test/api/admin/email/templates?page=2&per_page=1&search=needle'
	);
	return {
		url,
		params: { rest },
		request: new Request(url, { method, ...(body ? { body } : {}) })
	} as unknown as Parameters<typeof proxyEmailTemplates>[0];
}
afterEach(() => {
	vi.unstubAllGlobals();
	vi.clearAllMocks();
});
it('preserves filtered pagination and storage failures instead of returning an empty200', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValue(Response.json({ error: 'Fixture storage unavailable' }, { status: 503 }));
	vi.stubGlobal('fetch', fetch);
	const response = await proxyEmailTemplates(event('GET'));
	expect(response.status).toBe(503);
	expect(await response.json()).toEqual({ error: 'Fixture storage unavailable' });
	expect(response.headers.get('Cache-Control')).toContain('no-store');
	expect(fetch).toHaveBeenCalledExactlyOnceWith(
		'http://backend.test/api/admin/email/templates?page=2&per_page=1&search=needle',
		expect.objectContaining({
			redirect: 'error',
			credentials: 'omit',
			signal: expect.any(AbortSignal)
		})
	);
	expect(auth.admin).toHaveBeenCalledOnce();
});
it('preserves validation bodies and never retries a submitted mutation', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValue(new Response('Fixture invalid body', { status: 422 }));
	vi.stubGlobal('fetch', fetch);
	const response = await proxyEmailTemplates(
		event('PUT', '42', '{"body_html":"<p>Saved</p>","body_text":""}')
	);
	expect(response.status).toBe(422);
	expect(await response.text()).toBe('Fixture invalid body');
	expect(fetch).toHaveBeenCalledOnce();
	expect(new TextDecoder().decode(fetch.mock.calls[0]?.[1]?.body as Uint8Array)).toContain(
		'"body_text":""'
	);
});
it('bounds actual UTF8 bytes and rejects path traversal before provider traffic', async () => {
	const fetch = vi.fn<typeof globalThis.fetch>();
	vi.stubGlobal('fetch', fetch);
	expect((await proxyEmailTemplates(event('POST', '', 'é'.repeat(4_500_001)))).status).toBe(413);
	expect((await proxyEmailTemplates(event('GET', '../settings'))).status).toBe(404);
	expect(fetch).not.toHaveBeenCalled();
});

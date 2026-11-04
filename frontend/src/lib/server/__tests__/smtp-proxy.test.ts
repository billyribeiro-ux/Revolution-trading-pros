// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
vi.mock('$app/env/private', () => ({ API_BASE_URL: 'http://127.0.0.1:8281', BACKEND_URL: '' }));
const auth = vi.hoisted(() => ({
	admin: vi.fn(() => ({ token: 'fixture-token' })),
	superadmin: vi.fn(() => ({ token: 'fixture-token' }))
}));
vi.mock('../auth.js', () => ({ requireAdmin: auth.admin, requireSuperadmin: auth.superadmin }));
import { proxySmtp } from '../smtp-proxy.js';
function event(method: string, body?: string) {
	return {
		request: new Request('http://fixture.example.test/api/admin/email/settings', {
			method,
			...(body ? { body } : {})
		})
	} as unknown as Parameters<typeof proxySmtp>[0];
}
afterEach(() => {
	vi.unstubAllGlobals();
	vi.clearAllMocks();
});
it('preserves service failures instead of inventing empty settings, and scrubs legacy plaintext', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValueOnce(Response.json({ error: 'Fixture storage failure' }, { status: 503 }))
		.mockResolvedValueOnce(
			Response.json({
				host: 'fixture.example.test',
				password: 'private-fixture',
				has_password: true
			})
		);
	vi.stubGlobal('fetch', fetch);
	const failed = await proxySmtp(event('GET'));
	expect(failed.status).toBe(503);
	expect(await failed.json()).toEqual({ error: 'Fixture storage failure' });
	expect(failed.headers.get('Cache-Control')).toBe('no-store');
	const loaded = await proxySmtp(event('GET'));
	expect(await loaded.json()).toEqual({ host: 'fixture.example.test', has_password: true });
	expect(auth.admin).toHaveBeenCalledTimes(2);
	expect(auth.superadmin).not.toHaveBeenCalled();
});
it('gates mutation, omits legacy masked passwords, and bounds actual UTF-8 bytes before transport', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValue(
			Response.json({ success: false, message: 'SMTP rejected', email_sent: false })
		);
	vi.stubGlobal('fetch', fetch);
	const checked = await proxySmtp(
		event('POST', '{"host":"127.0.0.1","password":"••••••••"}'),
		true
	);
	expect(auth.superadmin).toHaveBeenCalledOnce();
	expect(fetch).toHaveBeenCalledExactlyOnceWith(
		'http://127.0.0.1:8281/api/admin/email/settings/test',
		expect.objectContaining({
			method: 'POST',
			body: '{"host":"127.0.0.1"}',
			redirect: 'error',
			signal: expect.any(AbortSignal)
		})
	);
	expect(await checked.json()).toMatchObject({ success: false, email_sent: false });
	fetch.mockClear();
	expect(
		(await proxySmtp(event('POST', JSON.stringify({ password: 'é'.repeat(8192) })))).status
	).toBe(413);
	expect((await proxySmtp(event('POST', 'null'))).status).toBe(400);
	expect(fetch).not.toHaveBeenCalled();
});

it('preserves a plain-text schema rejection status and does not retry a submitted save', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValue(new Response('Fixture schema rejection', { status: 422 }));
	vi.stubGlobal('fetch', fetch);
	const response = await proxySmtp(event('POST', '{"provider":"smtp"}'));
	expect(response.status).toBe(422);
	expect(await response.json()).toEqual({ success: false, error: 'Invalid SMTP settings request' });
	expect(fetch).toHaveBeenCalledOnce();
});

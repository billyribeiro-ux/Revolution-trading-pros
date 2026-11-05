// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
vi.mock('$app/env/private', () => ({ API_BASE_URL: 'http://backend.test', BACKEND_URL: '' }));
const auth = vi.hoisted(() => ({ admin: vi.fn(() => ({ token: 'fixture-token' })) }));
vi.mock('../auth.js', () => ({ requireAdmin: auth.admin }));
import { proxyCrmContacts } from '../crm-contact-proxy.js';
function event(method: string, rest = '', body?: string) {
	const url = new URL('http://frontend.test/api/admin/crm/contacts?page=2&lifecycle_stage=mql');
	return {
		url,
		params: { rest },
		request: new Request(url, {
			method,
			headers: { 'Content-Type': 'application/json' },
			...(body !== undefined ? { body } : {})
		})
	} as unknown as Parameters<typeof proxyCrmContacts>[0];
}
afterEach(() => {
	vi.unstubAllGlobals();
	vi.clearAllMocks();
});
it('preserves nullable lifecycle inputs and actual non-2xx personal-message receipts unchanged', async () => {
	const value = {
			success: false,
			data: { contact_id: '42', request_id: 'frozen', submissions: [{ status: 'unknown' }] }
		},
		transport = vi
			.fn<typeof fetch>()
			.mockImplementation(async () => Response.json(value, { status: 502 }));
	vi.stubGlobal('fetch', transport);
	const body = JSON.stringify({
		email: 'actual@example.test',
		lifecycle_stage: null,
		custom_fields: { retained: true }
	});
	const response = await proxyCrmContacts(event('POST', '', body));
	expect(response.status).toBe(502);
	expect(await response.json()).toEqual(value);
	expect(response.headers.get('Cache-Control')).toContain('no-store');
	expect(transport).toHaveBeenCalledOnce();
	const forwarded = transport.mock.calls[0]?.[1]?.body;
	expect(forwarded).toBeInstanceOf(Uint8Array);
	expect(new TextDecoder().decode(forwarded as Uint8Array)).toBe(body);
	expect(transport).toHaveBeenCalledWith(
		'http://backend.test/api/admin/crm/contacts?page=2&lifecycle_stage=mql',
		expect.objectContaining({ redirect: 'error', credentials: 'omit' })
	);
});
it('restricts identities/methods, enforces a byte limit and makes one attempt for uncertain writes', async () => {
	const transport = vi.fn<typeof fetch>().mockRejectedValue(new Error('Controlled disconnect'));
	vi.stubGlobal('fetch', transport);
	for (const [method, rest] of [
		['GET', '../settings'],
		['POST', '42/timeline'],
		['PATCH', '42'],
		['GET', '0'],
		['DELETE', '42/tags/nope']
	])
		expect((await proxyCrmContacts(event(method, rest))).status).toBe(404);
	expect((await proxyCrmContacts(event('POST', '', 'é'.repeat(1048577)))).status).toBe(413);
	expect(transport).not.toHaveBeenCalled();
	expect((await proxyCrmContacts(event('POST', '42/send-email', '{}'))).status).toBe(503);
	expect(transport).toHaveBeenCalledOnce();
});

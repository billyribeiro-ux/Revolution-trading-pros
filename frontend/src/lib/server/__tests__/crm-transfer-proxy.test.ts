// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
vi.mock('$app/env/private', () => ({ API_BASE_URL: 'http://backend.test', BACKEND_URL: '' }));
const auth = vi.hoisted(() => ({ admin: vi.fn(() => ({ token: 'fixture-token' })) }));
vi.mock('../auth.js', () => ({ requireAdmin: auth.admin }));
import { proxyCrmTransfers } from '../crm-transfer-proxy.js';
const id = '34ad48b0-615b-49e8-86ee-c0d9f77206ee';
function event(method: string, rest = '', body?: string, contentType = 'application/json') {
	const url = new URL('http://frontend.test/api/admin/crm/imports?page=2');
	return {
		url,
		params: { rest },
		request: new Request(url, {
			method,
			headers: { 'Content-Type': contentType },
			...(body !== undefined ? { body } : {})
		})
	} as unknown as Parameters<typeof proxyCrmTransfers>[0];
}
afterEach(() => {
	vi.unstubAllGlobals();
	vi.clearAllMocks();
});
it('keeps multipart boundaries and actual failure codes, with one upstream write', async () => {
	const transport = vi
		.fn<typeof fetch>()
		.mockImplementation(async () =>
			Response.json({ error: 'Source file is invalid' }, { status: 400 })
		);
	vi.stubGlobal('fetch', transport);
	const response = await proxyCrmTransfers(
		event('POST', '', '--actual-boundary--', 'multipart/form-data; boundary=actual-boundary'),
		'imports'
	);
	expect(response.status).toBe(400);
	expect(await response.json()).toEqual({ error: 'Source file is invalid' });
	expect(response.headers.get('Cache-Control')).toContain('no-store');
	expect(transport).toHaveBeenCalledExactlyOnceWith(
		'http://backend.test/api/admin/crm/imports?page=2',
		expect.objectContaining({
			method: 'POST',
			redirect: 'error',
			credentials: 'omit',
			headers: expect.objectContaining({
				'Content-Type': 'multipart/form-data; boundary=actual-boundary',
				Authorization: 'Bearer fixture-token'
			})
		})
	);
});
it('preserves real download bytes and rejects traversal, wrong methods and oversized UTF-8 uploads', async () => {
	const transport = vi.fn<typeof fetch>().mockImplementation(
		async () =>
			new Response('email\nactual@example.test\n', {
				headers: {
					'Content-Type': 'text/csv; charset=utf-8',
					'Content-Disposition': `attachment; filename="crm-contacts-${id}.csv"`
				}
			})
	);
	vi.stubGlobal('fetch', transport);
	const response = await proxyCrmTransfers(event('GET', `${id}/download`), 'exports');
	expect(await response.text()).toContain('actual@example.test');
	expect(response.headers.get('Content-Disposition')).toContain(id);
	transport.mockClear();
	for (const [method, path, direction] of [
		['GET', '../settings', 'imports'],
		['POST', `${id}/download`, 'exports'],
		['GET', `${id}/start`, 'imports'],
		['POST', 'options', 'imports']
	] as const)
		expect((await proxyCrmTransfers(event(method, path), direction)).status).toBe(404);
	expect(
		(await proxyCrmTransfers(event('POST', '', 'é'.repeat(1_081_345)), 'imports')).status
	).toBe(413);
	expect(transport).not.toHaveBeenCalled();
});
it('forwards option reads and never retries an uncertain start', async () => {
	const transport = vi.fn<typeof fetch>().mockRejectedValue(new Error('Controlled disconnect'));
	vi.stubGlobal('fetch', transport);
	expect((await proxyCrmTransfers(event('POST', `${id}/start`, '{}'), 'imports')).status).toBe(503);
	expect(transport).toHaveBeenCalledOnce();
	transport.mockImplementation(async () => Response.json({ data: [] }));
	expect((await proxyCrmTransfers(event('GET', 'options'), 'imports')).status).toBe(200);
	expect(transport).toHaveBeenCalledTimes(2);
});

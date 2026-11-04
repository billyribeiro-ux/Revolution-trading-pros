// @vitest-environment node
import { describe, expect, it, vi } from 'vite-plus/test';
vi.mock('$app/env/private', () => ({ API_BASE_URL: 'http://backend.test', BACKEND_URL: '' }));
import { campaignTracking } from '../campaign-tracking.js';

const token = 'a54ecc62-0314-4bc6-a4f2-89e318db328a';
describe('SMTP campaign tracking proxy', () => {
	it('forwards one credential-free request and preserves the stored redirect', async () => {
		const fetch = vi.fn<typeof globalThis.fetch>().mockResolvedValue(
			new Response(null, {
				status: 302,
				headers: { location: 'https://target.test/a?b=2#details' }
			})
		);
		const response = await campaignTracking({
			fetch,
			request: new Request('https://app.test/link'),
			params: { kind: 'click', token }
		});
		expect(response.status).toBe(302);
		expect(response.headers.get('location')).toBe('https://target.test/a?b=2#details');
		expect(response.headers.get('referrer-policy')).toBe('no-referrer');
		expect(fetch).toHaveBeenCalledExactlyOnceWith(
			`http://backend.test/api/newsletter/tracking/click/${token}`,
			expect.objectContaining({ credentials: 'omit', redirect: 'manual', method: 'GET' })
		);
	});
	it('preserves HEAD so previews cannot create a GET observation', async () => {
		const fetch = vi
			.fn<typeof globalThis.fetch>()
			.mockResolvedValue(new Response(null, { headers: { 'content-type': 'image/gif' } }));
		const response = await campaignTracking({
			fetch,
			request: new Request('https://app.test/link', { method: 'HEAD' }),
			params: { kind: 'open', token }
		});
		expect(response.status).toBe(200);
		expect(response.body).toBeNull();
		expect(fetch.mock.calls[0]?.[1]?.method).toBe('HEAD');
	});
	it('rejects invented targets and keeps persistence failures visible', async () => {
		const fetch = vi
			.fn<typeof globalThis.fetch>()
			.mockResolvedValueOnce(
				new Response(null, { status: 302, headers: { location: 'javascript:alert(1)' } })
			)
			.mockResolvedValueOnce(new Response(null, { status: 503 }));
		const event = {
			fetch,
			request: new Request('https://app.test/link'),
			params: { kind: 'click', token }
		};
		expect((await campaignTracking(event)).status).toBe(502);
		expect((await campaignTracking(event)).status).toBe(503);
		expect(
			(await campaignTracking({ ...event, params: { kind: 'click', token: 'invalid' } })).status
		).toBe(404);
		expect(fetch).toHaveBeenCalledTimes(2);
	});
});

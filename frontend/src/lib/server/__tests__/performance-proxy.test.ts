import { afterEach, expect, it, vi } from 'vite-plus/test';
import type { RequestHandler } from '@sveltejs/kit';
vi.mock('$app/env/private', () => ({ API_BASE_URL: 'http://127.0.0.1:8281', BACKEND_URL: '' }));
import { performanceProxy } from '../performance-proxy.js';
function event(body: string, fetch: typeof globalThis.fetch) {
	return {
		request: new Request('http://localhost/api/analytics/performance', {
			method: 'POST',
			body,
			headers: {
				'Content-Type': 'text/plain',
				Referer: 'http://localhost/fixture',
				'User-Agent': 'Fixture',
				Authorization: 'Bearer must-not-be-forwarded',
				Cookie: 'must-not-be-forwarded'
			}
		}),
		fetch
	} as Parameters<RequestHandler>[0];
}
afterEach(() => vi.restoreAllMocks());
it('waits for the durable receipt and preserves legacy beacon bytes without session credentials', async () => {
	let reply: ((response: Response) => void) | undefined;
	const fetch = vi.fn<typeof globalThis.fetch>().mockImplementation(
		() =>
			new Promise<Response>((resolve) => {
				reply = resolve;
			})
	);
	const body = JSON.stringify({ name: 'LCP', value: 1200, id: 'stable-fixture' });
	let completed = false;
	const pending = Promise.resolve(performanceProxy(event(body, fetch))).then((response) => {
		completed = true;
		return response;
	});
	await vi.waitFor(() => expect(fetch).toHaveBeenCalledTimes(1));
	expect(completed).toBe(false);
	expect(fetch.mock.calls[0]?.[0]).toBe('http://127.0.0.1:8281/api/analytics/web-vitals');
	const options = fetch.mock.calls[0]?.[1];
	const headers = new Headers(options?.headers);
	expect(headers.get('authorization')).toBeNull();
	expect(headers.get('cookie')).toBeNull();
	expect(headers.get('referer')).toBe('http://localhost/fixture');
	expect(new TextDecoder().decode(options?.body as ArrayBuffer)).toBe(body);
	reply?.(Response.json({ success: true, received: 1, persisted: 1, id: 'stable-fixture' }));
	const response = await pending;
	expect(response.status).toBe(200);
	expect(response.headers.get('cache-control')).toBe('no-store');
	expect(await response.json()).toEqual({
		success: true,
		received: 1,
		persisted: 1,
		id: 'stable-fixture'
	});
});
it('preserves validation/storage failures and reports transport failure without acknowledging storage', async () => {
	for (const status of [400, 503]) {
		const fetch = vi
			.fn<typeof globalThis.fetch>()
			.mockResolvedValue(Response.json({ success: false }, { status }));
		const response = await performanceProxy(event('{}', fetch));
		expect(response.status).toBe(status);
		expect(await response.json()).toEqual({ success: false });
	}
	const failed = vi.fn<typeof globalThis.fetch>().mockRejectedValue(new Error('unavailable'));
	const response = await performanceProxy(event('{}', failed));
	expect(response.status).toBe(502);
	expect(await response.json()).toMatchObject({ success: false });
});
it('enforces the actual UTF-8 byte limit before forwarding', async () => {
	const fetch = vi.fn<typeof globalThis.fetch>();
	const response = await performanceProxy(
		event(JSON.stringify({ value: '😀'.repeat(5000) }), fetch)
	);
	expect(response.status).toBe(413);
	expect(fetch).not.toHaveBeenCalled();
});

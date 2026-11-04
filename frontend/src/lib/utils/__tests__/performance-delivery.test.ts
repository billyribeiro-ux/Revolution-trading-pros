import { afterEach, expect, it, vi } from 'vite-plus/test';
import { deliverPerformanceObservation } from '../performance-delivery.js';
afterEach(() => {
	vi.useRealTimers();
	vi.unstubAllGlobals();
});
it('retries storage failures with identical identities and accepts a deduplicated durable receipt', async () => {
	vi.useFakeTimers();
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValueOnce(Response.json({ success: false }, { status: 503 }))
		.mockResolvedValueOnce(Response.json({ success: true, received: 1, persisted: 0 }));
	vi.stubGlobal('fetch', fetch);
	const pending = deliverPerformanceObservation('/api/analytics/web-vitals', {
		metric: 'CLS',
		value: 0
	});
	await vi.runAllTimersAsync();
	await pending;
	expect(fetch).toHaveBeenCalledTimes(2);
	expect(fetch.mock.calls[0]?.[1]?.body).toBe(fetch.mock.calls[1]?.[1]?.body);
	const sent = fetch.mock.calls[0]?.[1]?.body;
	if (typeof sent !== 'string') throw new Error('Expected a serialized metric body');
	const payload: unknown = JSON.parse(sent);
	expect(payload).toMatchObject({
		metric: 'CLS',
		value: 0,
		id: expect.any(String),
		timestamp: expect.any(Number)
	});
});
it('rejects false receipts after finite attempts and does not retry invalid metrics', async () => {
	vi.useFakeTimers();
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockImplementation(async () => Response.json({ status: 'ok' }));
	vi.stubGlobal('fetch', fetch);
	const failed = expect(
		deliverPerformanceObservation('/api/analytics/web-vitals', { metric: 'LCP', value: 1 })
	).rejects.toThrow('durable receipt');
	await vi.runAllTimersAsync();
	await failed;
	expect(fetch).toHaveBeenCalledTimes(3);
	fetch.mockClear();
	fetch.mockResolvedValue(Response.json({ success: false }, { status: 400 }));
	await expect(
		deliverPerformanceObservation('/api/analytics/web-vitals', { metric: 'LCP', value: -1 })
	).rejects.toThrow('HTTP 400');
	expect(fetch).toHaveBeenCalledTimes(1);
});

it('cancels a waiting retry when collection is stopped', async () => {
	vi.useFakeTimers();
	const controller = new AbortController();
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockImplementation(async () => Response.json({ success: false }, { status: 503 }));
	vi.stubGlobal('fetch', fetch);
	const stopped = expect(
		deliverPerformanceObservation(
			'/api/analytics/web-vitals',
			{ metric: 'LCP', value: 1 },
			{},
			controller.signal
		)
	).rejects.toMatchObject({ name: 'AbortError' });
	await vi.advanceTimersByTimeAsync(0);
	expect(fetch).toHaveBeenCalledTimes(1);
	controller.abort();
	await stopped;
	await vi.runAllTimersAsync();
	expect(fetch).toHaveBeenCalledTimes(1);
});

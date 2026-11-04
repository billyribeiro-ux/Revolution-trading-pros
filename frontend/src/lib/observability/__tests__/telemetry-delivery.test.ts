import { afterEach, expect, it, vi } from 'vite-plus/test';

vi.mock('$app/env', () => ({ browser: true }));
vi.mock('#lib/utils/logger.js', () => ({ logger: { debug: vi.fn(), error: vi.fn() } }));

afterEach(() => {
	vi.useRealTimers();
	vi.unstubAllGlobals();
});

it('retains rejected batches and metrics recorded while an acknowledgement is pending', async () => {
	vi.useFakeTimers();
	const replies: ((response: Response) => void)[] = [];
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockImplementation(() => new Promise<Response>((resolve) => replies.push(resolve)));
	vi.stubGlobal('fetch', fetch);
	const telemetry = (await import('../telemetry')).default;
	try {
		telemetry.recordMetric('first', 1, 'gauge');
		await vi.advanceTimersByTimeAsync(10000);
		expect(fetch).toHaveBeenCalledTimes(1);
		telemetry.recordMetric('during-failure', 2, 'gauge');
		replies.shift()?.(new Response('', { status: 503 }));
		await vi.advanceTimersByTimeAsync(10000);
		expect(fetch).toHaveBeenCalledTimes(2);
		const secondBody = fetch.mock.calls[1]?.[1]?.body;
		if (typeof secondBody !== 'string') throw new Error('Expected a serialized telemetry batch');
		const second: { metrics: { name: string }[] } = JSON.parse(secondBody);
		expect(second.metrics.map((metric: { name: string }) => metric.name)).toEqual([
			'first',
			'during-failure'
		]);
		telemetry.recordMetric('during-success', 3, 'gauge');
		// The interval cannot start a duplicate request while this one is pending.
		await vi.advanceTimersByTimeAsync(10000);
		expect(fetch).toHaveBeenCalledTimes(2);
		replies.shift()?.(new Response('', { status: 200 }));
		await vi.advanceTimersByTimeAsync(10000);
		expect(fetch).toHaveBeenCalledTimes(3);
		const thirdBody = fetch.mock.calls[2]?.[1]?.body;
		if (typeof thirdBody !== 'string') throw new Error('Expected a serialized telemetry batch');
		const third: { metrics: { name: string }[] } = JSON.parse(thirdBody);
		expect(third.metrics.map((metric: { name: string }) => metric.name)).toEqual([
			'during-success'
		]);
		replies.shift()?.(new Response('', { status: 200 }));
		await vi.advanceTimersByTimeAsync(0);
	} finally {
		telemetry.destroy();
	}
});

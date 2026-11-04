import { afterEach, beforeEach, describe, expect, it, vi } from 'vite-plus/test';
import { BehaviorTracker } from './tracker';
import type { BehaviorEventBatch, BehaviorTrackerConfig } from './types';

const trackers: BehaviorTracker[] = [];
const originalDNT = Object.getOwnPropertyDescriptor(navigator, 'doNotTrack');
const config: Partial<BehaviorTrackerConfig> = {
	trackScrollDepth: false,
	trackHoverIntent: false,
	trackFormBehavior: false,
	trackIdleTime: false,
	bufferTimeout: 1000
};
function tracker(options: Partial<BehaviorTrackerConfig> = {}) {
	const instance = new BehaviorTracker({ ...config, ...options });
	trackers.push(instance);
	return instance;
}
function body(
	fetch: ReturnType<typeof vi.fn<typeof globalThis.fetch>>,
	index = 0
): BehaviorEventBatch {
	const value = fetch.mock.calls[index]?.[1]?.body;
	if (typeof value !== 'string') throw new Error('Expected a serialized event batch');
	return JSON.parse(value);
}
function receipt(received: number, persisted = received) {
	return Response.json({ success: true, received, persisted });
}
beforeEach(() => {
	vi.useFakeTimers();
	localStorage.clear();
	Object.defineProperty(navigator, 'doNotTrack', { configurable: true, get: () => '0' });
});
afterEach(() => {
	for (const instance of trackers.splice(0)) instance.destroy({ flush: false });
	if (originalDNT) Object.defineProperty(navigator, 'doNotTrack', originalDNT);
	else Reflect.deleteProperty(navigator, 'doNotTrack');
	vi.restoreAllMocks();
	vi.unstubAllGlobals();
	vi.useRealTimers();
});

describe('behavior delivery', () => {
	it('replays the original saved batch after a failed upload and accepts a deduplicated receipt', async () => {
		const fetch = vi
			.fn<typeof globalThis.fetch>()
			.mockResolvedValue(new Response('', { status: 503 }));
		vi.stubGlobal('fetch', fetch);
		const first = tracker();
		await first.flush();
		const original = body(fetch);
		expect(JSON.parse(localStorage.getItem('behavior_failed_batches') || '[]')).toEqual([original]);
		first.destroy();
		await vi.advanceTimersByTimeAsync(0);
		fetch.mockReset().mockImplementation(async () => receipt(1, 0));
		const reloaded = tracker();
		await reloaded.flush();
		expect(body(fetch)).toEqual(original);
		expect(fetch).toHaveBeenCalledTimes(2);
		expect(localStorage.getItem('behavior_failed_batches')).toBeNull();
	});

	it('retains a misleading HTTP 200 until the server acknowledges the whole batch', async () => {
		const fetch = vi
			.fn<typeof globalThis.fetch>()
			.mockResolvedValue(Response.json({ success: true }));
		vi.stubGlobal('fetch', fetch);
		const instance = tracker();
		await instance.flush();
		expect(localStorage.getItem('behavior_failed_batches')).not.toBeNull();
		const original = body(fetch);
		fetch.mockImplementation(async () => receipt(1));
		await instance.flush();
		expect(body(fetch, 1)).toEqual(original);
		expect(localStorage.getItem('behavior_failed_batches')).toBeNull();
	});

	it('serializes delivery while retaining events captured during the pending request', async () => {
		let reply: (response: Response) => void = () => {
			throw new Error('No pending request');
		};
		const fetch = vi
			.fn<typeof globalThis.fetch>()
			.mockImplementationOnce(
				() =>
					new Promise<Response>((resolve) => {
						reply = resolve;
					})
			)
			.mockImplementation(async () => receipt(1));
		vi.stubGlobal('fetch', fetch);
		const instance = tracker();
		const pending = instance.flush();
		instance.trackEvent({
			event_type: 'click',
			timestamp: Date.now(),
			page_url: location.href,
			element_selector: '#during-upload'
		});
		await vi.advanceTimersByTimeAsync(1000);
		expect(fetch).toHaveBeenCalledTimes(1);
		reply(receipt(1));
		await pending;
		expect(fetch).toHaveBeenCalledTimes(2);
		expect(body(fetch, 1).events[0]?.element_selector).toBe('#during-upload');
		expect(localStorage.getItem('behavior_failed_batches')).toBeNull();
	});

	it('drops a terminal invalid batch and continues with later valid activity', async () => {
		const fetch = vi
			.fn<typeof globalThis.fetch>()
			.mockResolvedValueOnce(new Response('', { status: 503 }));
		vi.stubGlobal('fetch', fetch);
		const instance = tracker();
		await instance.flush();
		instance.trackEvent({ event_type: 'click', timestamp: Date.now(), page_url: location.href });
		fetch
			.mockResolvedValueOnce(new Response('', { status: 422 }))
			.mockResolvedValueOnce(receipt(1));
		await instance.flush();
		expect(fetch).toHaveBeenCalledTimes(3);
		expect(body(fetch, 2).events[0]?.event_type).toBe('click');
		expect(localStorage.getItem('behavior_failed_batches')).toBeNull();
	});

	it('aborts and discards pending delivery on consent withdrawal without restoring it later', async () => {
		let reply: (response: Response) => void = () => {
			throw new Error('No pending request');
		};
		const fetch = vi.fn<typeof globalThis.fetch>().mockImplementation(
			() =>
				new Promise<Response>((resolve) => {
					reply = resolve;
				})
		);
		vi.stubGlobal('fetch', fetch);
		const instance = tracker({ trackHoverIntent: true, trackIdleTime: true });
		const pending = instance.flush();
		const signal = fetch.mock.calls[0]?.[1]?.signal;
		instance.destroy({ flush: false });
		expect(signal?.aborted).toBe(true);
		document.body.dispatchEvent(new MouseEvent('click', { bubbles: true }));
		document.body.dispatchEvent(new MouseEvent('mouseover', { bubbles: true }));
		instance.trackEvent({ event_type: 'click', timestamp: Date.now(), page_url: location.href });
		reply(new Response('', { status: 503 }));
		await pending;
		await vi.advanceTimersByTimeAsync(60000);
		await instance.flush();
		expect(fetch).toHaveBeenCalledTimes(1);
		expect(localStorage.getItem('behavior_failed_batches')).toBeNull();
	});

	it('honors DNT and zero sampling without persisting an identifier', async () => {
		const fetch = vi.fn<typeof globalThis.fetch>();
		vi.stubGlobal('fetch', fetch);
		Object.defineProperty(navigator, 'doNotTrack', { configurable: true, get: () => '1' });
		await tracker().flush();
		Object.defineProperty(navigator, 'doNotTrack', { configurable: true, get: () => '0' });
		await tracker({ sampleRate: 0 }).flush();
		expect(fetch).not.toHaveBeenCalled();
		expect(localStorage.length).toBe(0);
	});

	it('applies the session limit across flushes including the initial page view', async () => {
		const fetch = vi.fn<typeof globalThis.fetch>().mockImplementation(async () => receipt(1));
		vi.stubGlobal('fetch', fetch);
		const instance = tracker({ maxEventsPerSession: 2 });
		await instance.flush();
		instance.trackEvent({ event_type: 'click', timestamp: Date.now(), page_url: location.href });
		await instance.flush();
		instance.trackEvent({ event_type: 'click', timestamp: Date.now(), page_url: location.href });
		await instance.flush();
		expect(fetch).toHaveBeenCalledTimes(2);
	});
});

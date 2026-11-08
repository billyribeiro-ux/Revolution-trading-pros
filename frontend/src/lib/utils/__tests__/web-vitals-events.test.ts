import { afterEach, expect, it, vi } from 'vite-plus/test';
import { initWebVitals } from '../webVitals.js';
const listeners = vi.spyOn(window, 'addEventListener');
afterEach(() => {
	for (const [event, listener, options] of listeners.mock.calls)
		window.removeEventListener(event, listener as EventListener, options);
	listeners.mockClear();
	vi.restoreAllMocks();
	vi.unstubAllGlobals();
});
class Observer {
	static supportedEntryTypes = [
		'largest-contentful-paint',
		'first-input',
		'layout-shift',
		'paint',
		'navigation',
		'event'
	];
	static byType = new Map<string, Observer>();
	disconnected = false;
	constructor(private callback: PerformanceObserverCallback) {}
	observe(options: PerformanceObserverInit) {
		Observer.byType.set(options.type ?? '', this);
	}
	disconnect() {
		this.disconnected = true;
	}
	emit(entries: PerformanceEntry[]) {
		this.callback(
			{ getEntries: () => entries } as PerformanceObserverEntryList,
			this as unknown as PerformanceObserver
		);
	}
}
it('finalizes one measurement across both hide events and records a measured zero CLS', async () => {
	history.replaceState({}, '', '/marketing?private_token=secret#private-fragment');
	vi.stubGlobal('PerformanceObserver', Observer);
	vi.stubGlobal('performance', { getEntriesByType: () => [] });
	vi.spyOn(document, 'visibilityState', 'get').mockReturnValue('hidden');
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockImplementation(async () => Response.json({ success: true, received: 1, persisted: 1 }));
	vi.stubGlobal('fetch', fetch);
	const cleanup = initWebVitals({ sampleRate: 1 });
	Observer.byType.get('largest-contentful-paint')?.emit([{ startTime: 123 } as PerformanceEntry]);
	window.dispatchEvent(new Event('pagehide'));
	window.dispatchEvent(new Event('visibilitychange'));
	await vi.waitFor(() => expect(fetch).toHaveBeenCalledTimes(2));
	const metrics = fetch.mock.calls.map(([, options]) => {
		if (typeof options?.body !== 'string') throw new Error('Expected serialized observation');
		return JSON.parse(options.body) as {
			metric: string;
			value: number;
			id: string;
			url: string;
			userAgent?: string;
			connection?: string;
			deviceMemory?: number;
		};
	});
	expect(metrics.every((metric) => metric.url === `${location.origin}/marketing`)).toBe(true);
	expect(
		metrics.every(
			(metric) => !('userAgent' in metric || 'connection' in metric || 'deviceMemory' in metric)
		)
	).toBe(true);
	expect(metrics.filter((metric) => metric.metric === 'LCP')).toEqual([
		expect.objectContaining({ value: 123, id: expect.any(String) })
	]);
	expect(metrics.filter((metric) => metric.metric === 'CLS')).toEqual([
		expect.objectContaining({ value: 0, id: expect.any(String) })
	]);
	expect(metrics.some((metric) => metric.metric === 'INP')).toBe(false);
	cleanup();
	expect([...Observer.byType.values()].every((observer) => observer.disconnected)).toBe(true);
});

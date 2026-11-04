import { afterEach, expect, it, vi } from 'vite-plus/test';
const fixture = vi.hoisted(() => ({
	listener: null as ((state: { analytics: boolean }) => void) | null,
	stop: vi.fn(),
	unsubscribe: vi.fn(),
	init: vi.fn<(options: { signal?: AbortSignal }) => () => void>()
}));
vi.mock('../store.svelte.js', () => ({
	consentStore: {
		subscribe: (listener: (state: { analytics: boolean }) => void) => {
			fixture.listener = listener;
			listener({ analytics: false });
			return fixture.unsubscribe;
		}
	}
}));
vi.mock('#lib/utils/webVitals.js', () => ({ initWebVitals: fixture.init }));
vi.mock('#lib/utils/logger.js', () => ({ logger: { error: vi.fn() } }));
import {
	initConsentAwarePerformanceTracking,
	cleanupPerformanceIntegration
} from '../performance-integration.js';
afterEach(() => {
	cleanupPerformanceIntegration();
	vi.clearAllMocks();
});
it('starts only with consent, stops and aborts on withdrawal, and releases its subscription', () => {
	fixture.init.mockReturnValue(fixture.stop);
	const cleanup = initConsentAwarePerformanceTracking();
	expect(fixture.init).not.toHaveBeenCalled();
	fixture.listener?.({ analytics: true });
	expect(fixture.init).toHaveBeenCalledTimes(1);
	const first = fixture.init.mock.calls[0]?.[0].signal;
	expect(first?.aborted).toBe(false);
	fixture.listener?.({ analytics: true });
	expect(fixture.init).toHaveBeenCalledTimes(1);
	fixture.listener?.({ analytics: false });
	expect(first?.aborted).toBe(true);
	expect(fixture.stop).toHaveBeenCalledTimes(1);
	fixture.listener?.({ analytics: true });
	expect(fixture.init).toHaveBeenCalledTimes(2);
	cleanup();
	expect(fixture.init.mock.calls[1]?.[0].signal?.aborted).toBe(true);
	expect(fixture.unsubscribe).toHaveBeenCalledTimes(1);
	expect(fixture.stop).toHaveBeenCalledTimes(2);
});

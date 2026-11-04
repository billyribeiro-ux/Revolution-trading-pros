/** Wire first-party browser measurements to the existing analytics preference. */
import { consentStore } from './store.svelte.js';
import { initWebVitals } from '#lib/utils/webVitals.js';
import { logger } from '#lib/utils/logger.js';

let unsubscribe: (() => void) | null = null;
let stop: (() => void) | null = null;
let controller: AbortController | null = null;
function stopCollection(): void {
	controller?.abort();
	controller = null;
	stop?.();
	stop = null;
}
export function cleanupPerformanceIntegration(): void {
	unsubscribe?.();
	unsubscribe = null;
	stopCollection();
}
export function initConsentAwarePerformanceTracking(): () => void {
	if (typeof window === 'undefined') return () => {};
	if (unsubscribe) return cleanupPerformanceIntegration;
	unsubscribe = consentStore.subscribe((consent) => {
		if (!consent.analytics) {
			stopCollection();
			return;
		}
		if (stop) return;
		controller = new AbortController();
		try {
			stop = initWebVitals({ signal: controller.signal });
		} catch (error) {
			stopCollection();
			logger.error('[Performance] Observation initialization failed', { error });
		}
	});
	return cleanupPerformanceIntegration;
}

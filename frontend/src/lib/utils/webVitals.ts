/**
 * Web Vitals Monitoring - Svelte 5 / SvelteKit
 * Captures Core Web Vitals and reports to analytics endpoint
 *
 * @version 1.0.0 - December 2024
 *
 * Metrics tracked:
 * - LCP (Largest Contentful Paint) - target < 2.5s
 * - FID (First Input Delay) - target < 100ms
 * - CLS (Cumulative Layout Shift) - target < 0.1
 * - FCP (First Contentful Paint) - target < 1.8s
 * - TTFB (Time to First Byte) - target < 800ms
 * - INP (Interaction to Next Paint) - target < 200ms
 */

import { deliverPerformanceObservation } from './performance-delivery.js';

interface WebVitalMetric {
	name: 'LCP' | 'FID' | 'CLS' | 'FCP' | 'TTFB' | 'INP';
	value: number;
	rating: 'good' | 'needs-improvement' | 'poor';
	delta: number;
	id: string;
	navigationType?: string;
}

/**
 * `layout-shift` PerformanceEntry — not in lib.dom's typings; model the two
 * fields the CLS observer reads.
 */
interface LayoutShiftEntry extends PerformanceEntry {
	value: number;
	hadRecentInput: boolean;
}

interface WebVitalsOptions {
	/** Stops observers and pending retries when consent or the app lifecycle ends. */
	signal?: AbortSignal;
	/** Analytics endpoint URL */
	endpoint?: string;
	/** Enable console logging in development */
	debug?: boolean;
	/** Custom headers for analytics requests */
	headers?: Record<string, string>;
	/** Sample rate (0-1) for reporting */
	sampleRate?: number;
	/** Custom callback for each metric */
	onMetric?: (metric: WebVitalMetric) => void;
}

const DEFAULT_OPTIONS: WebVitalsOptions = {
	endpoint: '/api/analytics/web-vitals',
	debug: false,
	sampleRate: 1
};

// Thresholds for rating metrics
const THRESHOLDS = {
	LCP: { good: 2500, poor: 4000 },
	FID: { good: 100, poor: 300 },
	CLS: { good: 0.1, poor: 0.25 },
	FCP: { good: 1800, poor: 3000 },
	TTFB: { good: 800, poor: 1800 },
	INP: { good: 200, poor: 500 }
} as const;

/**
 * Get rating for a metric value
 */
function getRating(
	name: WebVitalMetric['name'],
	value: number
): 'good' | 'needs-improvement' | 'poor' {
	const threshold = THRESHOLDS[name];
	if (value <= threshold.good) return 'good';
	if (value <= threshold.poor) return 'needs-improvement';
	return 'poor';
}

/**
 * Generate unique ID for metrics
 */
function generateId(): string {
	return `v1-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
}

/**
 * Report metric to analytics endpoint
 */
async function reportMetric(metric: WebVitalMetric, options: WebVitalsOptions): Promise<void> {
	if (options.signal?.aborted) return;
	// Apply sample rate
	if (Math.random() > (options.sampleRate ?? 1)) return;

	// Debug logging
	if (options.debug) {
		const color =
			metric.rating === 'good'
				? '#22c55e'
				: metric.rating === 'needs-improvement'
					? '#f59e0b'
					: '#ef4444';
		console.info(
			`%c[Web Vitals] ${metric.name}: ${metric.value.toFixed(2)} (${metric.rating})`,
			`color: ${color}; font-weight: bold;`
		);
	}

	// Custom callback
	if (options.onMetric) {
		options.onMetric(metric);
	}

	// Send to analytics endpoint
	if (options.endpoint) {
		try {
			await deliverPerformanceObservation(
				options.endpoint,
				{
					metric: metric.name,
					value: metric.value,
					id: metric.id,
					url: window.location.origin + window.location.pathname,
					timestamp: Date.now(),
					navigationType: metric.navigationType
				},
				options.headers,
				options.signal
			);
		} catch (error) {
			if (options.debug) {
				console.error('[Web Vitals] Failed to report metric:', error);
			}
		}
	}
}

/**
 * Observe Largest Contentful Paint (LCP)
 */
function reportMetricInBackground(metric: WebVitalMetric, options: WebVitalsOptions): void {
	void reportMetric(metric, options).catch((error: unknown) => {
		console.error('[Web Vitals] Metric callback failed:', error);
	});
}

function observeLCP(options: WebVitalsOptions): () => void {
	if (
		!('PerformanceObserver' in window) ||
		!PerformanceObserver.supportedEntryTypes.includes('largest-contentful-paint')
	)
		return () => {};

	let lcpValue = 0;
	let lastReported: number | undefined;
	const measurementId = generateId();

	const observer = new PerformanceObserver((list) => {
		const entries = list.getEntries();
		const lastEntry = entries[entries.length - 1];
		if (lastEntry) {
			lcpValue = lastEntry.startTime;
		}
	});

	observer.observe({ type: 'largest-contentful-paint', buffered: true });

	// Report on page hide or visibility change
	const reportLCP = () => {
		if (lcpValue > 0 && lcpValue !== lastReported) {
			lastReported = lcpValue;
			const metric: WebVitalMetric = {
				name: 'LCP',
				value: lcpValue,
				rating: getRating('LCP', lcpValue),
				delta: lcpValue,
				id: measurementId,
				navigationType: getNavigationType()
			};
			reportMetricInBackground(metric, options);
			observer.disconnect();
		}
	};

	// Multiple event listeners for reliability
	const onVisibility = () => {
		if (document.visibilityState === 'hidden') reportLCP();
	};
	addEventListener('visibilitychange', onVisibility);
	addEventListener('pagehide', reportLCP);
	return () => {
		observer.disconnect();
		removeEventListener('visibilitychange', onVisibility);
		removeEventListener('pagehide', reportLCP);
	};
}

/**
 * Observe First Input Delay (FID)
 */
function observeFID(options: WebVitalsOptions): () => void {
	if (
		!('PerformanceObserver' in window) ||
		!PerformanceObserver.supportedEntryTypes.includes('first-input')
	)
		return () => {};

	const observer = new PerformanceObserver((list) => {
		const entries = list.getEntries();
		for (const entry of entries) {
			const fidEntry = entry as PerformanceEventTiming;
			const value = fidEntry.processingStart - fidEntry.startTime;
			const metric: WebVitalMetric = {
				name: 'FID',
				value,
				rating: getRating('FID', value),
				delta: value,
				id: generateId(),
				navigationType: getNavigationType()
			};
			reportMetricInBackground(metric, options);
		}
	});

	observer.observe({ type: 'first-input', buffered: true });
	return () => observer.disconnect();
}

/**
 * Observe Cumulative Layout Shift (CLS)
 */
function observeCLS(options: WebVitalsOptions): () => void {
	if (
		!('PerformanceObserver' in window) ||
		!PerformanceObserver.supportedEntryTypes.includes('layout-shift')
	)
		return () => {};

	let clsValue = 0;
	let lastReported: number | undefined;
	const measurementId = generateId();
	let sessionValue = 0;
	let sessionEntries: PerformanceEntry[] = [];

	const observer = new PerformanceObserver((list) => {
		for (const entry of list.getEntries()) {
			const layoutShift = entry as LayoutShiftEntry;
			// Only count layout shifts without recent input
			if (!layoutShift.hadRecentInput) {
				const firstEntry = sessionEntries[0];
				const lastEntry = sessionEntries[sessionEntries.length - 1];

				// Start a new session if gap > 1s or total > 5s
				if (
					sessionValue &&
					(layoutShift.startTime - lastEntry?.startTime > 1000 ||
						layoutShift.startTime - firstEntry?.startTime > 5000)
				) {
					if (sessionValue > clsValue) {
						clsValue = sessionValue;
					}
					sessionValue = 0;
					sessionEntries = [];
				}

				sessionValue += layoutShift.value;
				sessionEntries.push(entry);
			}
		}
	});

	observer.observe({ type: 'layout-shift', buffered: true });

	// Report on page hide
	const reportCLS = () => {
		if (sessionValue > clsValue) {
			clsValue = sessionValue;
		}
		if (clsValue !== lastReported) {
			lastReported = clsValue;
			const metric: WebVitalMetric = {
				name: 'CLS',
				value: clsValue,
				rating: getRating('CLS', clsValue),
				delta: clsValue,
				id: measurementId,
				navigationType: getNavigationType()
			};
			reportMetricInBackground(metric, options);
		}
	};

	const onVisibility = () => {
		if (document.visibilityState === 'hidden') reportCLS();
	};
	addEventListener('visibilitychange', onVisibility);
	addEventListener('pagehide', reportCLS);
	return () => {
		observer.disconnect();
		removeEventListener('visibilitychange', onVisibility);
		removeEventListener('pagehide', reportCLS);
	};
}

/**
 * Observe First Contentful Paint (FCP)
 */
function observeFCP(options: WebVitalsOptions): () => void {
	if (
		!('PerformanceObserver' in window) ||
		!PerformanceObserver.supportedEntryTypes.includes('paint')
	)
		return () => {};

	const observer = new PerformanceObserver((list) => {
		for (const entry of list.getEntries()) {
			if (entry.name === 'first-contentful-paint') {
				const value = entry.startTime;
				const metric: WebVitalMetric = {
					name: 'FCP',
					value,
					rating: getRating('FCP', value),
					delta: value,
					id: generateId(),
					navigationType: getNavigationType()
				};
				reportMetricInBackground(metric, options);
				observer.disconnect();
			}
		}
	});

	observer.observe({ type: 'paint', buffered: true });
	return () => observer.disconnect();
}

/**
 * Observe Time to First Byte (TTFB)
 */
function observeTTFB(options: WebVitalsOptions): () => void {
	if (
		!('PerformanceObserver' in window) ||
		!PerformanceObserver.supportedEntryTypes.includes('navigation')
	)
		return () => {};

	const observer = new PerformanceObserver((list) => {
		for (const entry of list.getEntries()) {
			const navEntry = entry as PerformanceNavigationTiming;
			const value = navEntry.responseStart - navEntry.requestStart;
			if (value > 0) {
				const metric: WebVitalMetric = {
					name: 'TTFB',
					value,
					rating: getRating('TTFB', value),
					delta: value,
					id: generateId(),
					navigationType: getNavigationType()
				};
				reportMetricInBackground(metric, options);
				observer.disconnect();
			}
		}
	});

	observer.observe({ type: 'navigation', buffered: true });
	return () => observer.disconnect();
}

/**
 * Observe Interaction to Next Paint (INP)
 */
function observeINP(options: WebVitalsOptions): () => void {
	if (
		!('PerformanceObserver' in window) ||
		!PerformanceObserver.supportedEntryTypes.includes('event')
	)
		return () => {};

	let maxINP = 0;
	let lastReported: number | undefined;
	const measurementId = generateId();
	const interactions: number[] = [];

	const observer = new PerformanceObserver((list) => {
		for (const entry of list.getEntries()) {
			const eventEntry = entry as PerformanceEventTiming;
			// Only measure discrete events
			if (
				eventEntry.interactionId &&
				(eventEntry.entryType === 'event' || eventEntry.entryType === 'first-input')
			) {
				const duration = eventEntry.duration;
				interactions.push(duration);

				// INP is typically the 98th percentile
				if (interactions.length >= 50) {
					interactions.sort((a, b) => b - a);
					maxINP = interactions[Math.floor(interactions.length * 0.02)] || maxINP;
				} else {
					maxINP = Math.max(maxINP, duration);
				}
			}
		}
	});

	observer.observe({
		type: 'event',
		buffered: true,
		durationThreshold: 40
	} as PerformanceObserverInit);

	// Report on page hide
	const reportINP = () => {
		if (maxINP > 0 && maxINP !== lastReported) {
			lastReported = maxINP;
			const metric: WebVitalMetric = {
				name: 'INP',
				value: maxINP,
				rating: getRating('INP', maxINP),
				delta: maxINP,
				id: measurementId,
				navigationType: getNavigationType()
			};
			reportMetricInBackground(metric, options);
		}
	};

	const onVisibility = () => {
		if (document.visibilityState === 'hidden') reportINP();
	};
	addEventListener('visibilitychange', onVisibility);
	addEventListener('pagehide', reportINP);
	return () => {
		observer.disconnect();
		removeEventListener('visibilitychange', onVisibility);
		removeEventListener('pagehide', reportINP);
	};
}

/**
 * Get navigation type
 */
function getNavigationType(): string {
	if (typeof window === 'undefined') return 'unknown';

	const nav = performance.getEntriesByType('navigation')[0] as
		| PerformanceNavigationTiming
		| undefined;
	if (nav) return nav.type;

	// Fallback for older browsers
	return 'navigate';
}

/**
 * Initialize Web Vitals monitoring
 *
 * @example
 * ```ts
 * import { initWebVitals } from '#lib/utils/webVitals.js';
 *
 * // In +layout.svelte or app initialization
 * if (browser) {
 *   initWebVitals({
 *     debug: import.meta.env.DEV,
 *     endpoint: '/api/analytics/web-vitals',
 *     sampleRate: 0.1, // 10% sampling in production
 *   });
 * }
 * ```
 */
export function initWebVitals(userOptions: WebVitalsOptions = {}): () => void {
	if (typeof window === 'undefined' || userOptions.signal?.aborted) return () => {};
	const options = { ...DEFAULT_OPTIONS, ...userOptions };
	const stops: (() => void)[] = [];
	const cleanup = () => {
		for (const stop of stops.splice(0)) stop();
		options.signal?.removeEventListener('abort', cleanup);
	};
	try {
		for (const observe of [observeLCP, observeFID, observeCLS, observeFCP, observeTTFB, observeINP])
			stops.push(observe(options));
	} catch (error) {
		cleanup();
		throw error;
	}
	options.signal?.addEventListener('abort', cleanup, { once: true });
	if (options.debug) console.info('[Web Vitals] Monitoring initialized');
	return cleanup;
}

/**
 * Get current Web Vitals thresholds
 */
export function getThresholds(): typeof THRESHOLDS {
	return THRESHOLDS;
}

/**
 * Check if a metric value is considered good
 */
export function isGood(name: WebVitalMetric['name'], value: number): boolean {
	return value <= THRESHOLDS[name].good;
}

export type { WebVitalMetric, WebVitalsOptions };

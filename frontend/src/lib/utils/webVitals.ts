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

interface WebVitalMetric {
	name: 'LCP' | 'FID' | 'CLS' | 'FCP' | 'TTFB' | 'INP';
	value: number;
	rating: 'good' | 'needs-improvement' | 'poor';
	delta: number;
	id: string;
	navigationType?: string;
}

interface WebVitalsOptions {
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
	sampleRate: 1,
};

// Thresholds for rating metrics
const THRESHOLDS = {
	LCP: { good: 2500, poor: 4000 },
	FID: { good: 100, poor: 300 },
	CLS: { good: 0.1, poor: 0.25 },
	FCP: { good: 1800, poor: 3000 },
	TTFB: { good: 800, poor: 1800 },
	INP: { good: 200, poor: 500 },
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
async function reportMetric(
	metric: WebVitalMetric,
	options: WebVitalsOptions
): Promise<void> {
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
		console.log(
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
			await fetch(options.endpoint, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					...options.headers,
				},
				body: JSON.stringify({
					metric: metric.name,
					value: metric.value,
					rating: metric.rating,
					delta: metric.delta,
					id: metric.id,
					url: window.location.href,
					timestamp: Date.now(),
					navigationType: metric.navigationType,
					userAgent: navigator.userAgent,
					connection: (navigator as any).connection?.effectiveType,
					deviceMemory: (navigator as any).deviceMemory,
				}),
				// Use keepalive for page unload scenarios
				keepalive: true,
			});
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
function observeLCP(options: WebVitalsOptions): void {
	if (!('PerformanceObserver' in window)) return;

	let lcpValue = 0;
	let lcpEntry: PerformanceEntry | null = null;

	const observer = new PerformanceObserver((list) => {
		const entries = list.getEntries();
		const lastEntry = entries[entries.length - 1];
		if (lastEntry) {
			lcpValue = (lastEntry as any).startTime;
			lcpEntry = lastEntry;
		}
	});

	observer.observe({ type: 'largest-contentful-paint', buffered: true });

	// Report on page hide or visibility change
	const reportLCP = () => {
		if (lcpValue > 0) {
			const metric: WebVitalMetric = {
				name: 'LCP',
				value: lcpValue,
				rating: getRating('LCP', lcpValue),
				delta: lcpValue,
				id: generateId(),
				navigationType: getNavigationType(),
			};
			reportMetric(metric, options);
			observer.disconnect();
		}
	};

	// Multiple event listeners for reliability
	addEventListener('visibilitychange', () => {
		if (document.visibilityState === 'hidden') reportLCP();
	});
	addEventListener('pagehide', reportLCP);
}

/**
 * Observe First Input Delay (FID)
 */
function observeFID(options: WebVitalsOptions): void {
	if (!('PerformanceObserver' in window)) return;

	const observer = new PerformanceObserver((list) => {
		const entries = list.getEntries();
		for (const entry of entries) {
			const fidEntry = entry as any;
			const value = fidEntry.processingStart - fidEntry.startTime;
			const metric: WebVitalMetric = {
				name: 'FID',
				value,
				rating: getRating('FID', value),
				delta: value,
				id: generateId(),
				navigationType: getNavigationType(),
			};
			reportMetric(metric, options);
		}
	});

	observer.observe({ type: 'first-input', buffered: true });
}

/**
 * Observe Cumulative Layout Shift (CLS)
 */
function observeCLS(options: WebVitalsOptions): void {
	if (!('PerformanceObserver' in window)) return;

	let clsValue = 0;
	let sessionValue = 0;
	let sessionEntries: PerformanceEntry[] = [];

	const observer = new PerformanceObserver((list) => {
		for (const entry of list.getEntries()) {
			const layoutShift = entry as any;
			// Only count layout shifts without recent input
			if (!layoutShift.hadRecentInput) {
				const firstEntry = sessionEntries[0] as any;
				const lastEntry = sessionEntries[sessionEntries.length - 1] as any;

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
		if (clsValue > 0) {
			const metric: WebVitalMetric = {
				name: 'CLS',
				value: clsValue,
				rating: getRating('CLS', clsValue),
				delta: clsValue,
				id: generateId(),
				navigationType: getNavigationType(),
			};
			reportMetric(metric, options);
		}
	};

	addEventListener('visibilitychange', () => {
		if (document.visibilityState === 'hidden') reportCLS();
	});
	addEventListener('pagehide', reportCLS);
}

/**
 * Observe First Contentful Paint (FCP)
 */
function observeFCP(options: WebVitalsOptions): void {
	if (!('PerformanceObserver' in window)) return;

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
					navigationType: getNavigationType(),
				};
				reportMetric(metric, options);
				observer.disconnect();
			}
		}
	});

	observer.observe({ type: 'paint', buffered: true });
}

/**
 * Observe Time to First Byte (TTFB)
 */
function observeTTFB(options: WebVitalsOptions): void {
	if (!('PerformanceObserver' in window)) return;

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
					navigationType: getNavigationType(),
				};
				reportMetric(metric, options);
				observer.disconnect();
			}
		}
	});

	observer.observe({ type: 'navigation', buffered: true });
}

/**
 * Observe Interaction to Next Paint (INP)
 */
function observeINP(options: WebVitalsOptions): void {
	if (!('PerformanceObserver' in window)) return;

	let maxINP = 0;
	const interactions: number[] = [];

	const observer = new PerformanceObserver((list) => {
		for (const entry of list.getEntries()) {
			const eventEntry = entry as any;
			// Only measure discrete events
			if (
				eventEntry.interactionId &&
				(eventEntry.entryType === 'event' ||
					eventEntry.entryType === 'first-input')
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

	observer.observe({ type: 'event', buffered: true, durationThreshold: 40 });

	// Report on page hide
	const reportINP = () => {
		if (maxINP > 0) {
			const metric: WebVitalMetric = {
				name: 'INP',
				value: maxINP,
				rating: getRating('INP', maxINP),
				delta: maxINP,
				id: generateId(),
				navigationType: getNavigationType(),
			};
			reportMetric(metric, options);
		}
	};

	addEventListener('visibilitychange', () => {
		if (document.visibilityState === 'hidden') reportINP();
	});
	addEventListener('pagehide', reportINP);
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
 * import { initWebVitals } from '$lib/utils/webVitals';
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
export function initWebVitals(userOptions: WebVitalsOptions = {}): void {
	if (typeof window === 'undefined') return;

	const options = { ...DEFAULT_OPTIONS, ...userOptions };

	// Start observing all metrics
	observeLCP(options);
	observeFID(options);
	observeCLS(options);
	observeFCP(options);
	observeTTFB(options);
	observeINP(options);

	if (options.debug) {
		console.log('[Web Vitals] Monitoring initialized');
	}
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

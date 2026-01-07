/**
 * Performance Monitoring - Google November 2025 Web Vitals Tracking
 * ══════════════════════════════════════════════════════════════════════════════
 * Tracks Core Web Vitals: LCP, INP (replaced FID March 2024), CLS, TTFB, FCP
 * Non-blocking, production-ready performance monitoring
 *
 * Updated for November 2025 Google Core Web Vitals:
 * - INP (Interaction to Next Paint) replaced FID as official CWV
 * - Enhanced reporting for AI-driven search ranking signals
 * ══════════════════════════════════════════════════════════════════════════════
 */

// ICT 11+ Principal Engineer: Import from centralized config - single source of truth
import { API_BASE_URL, API_ENDPOINTS } from '$lib/api/config';

export interface PerformanceMetric {
	name: string;
	value: number;
	rating: 'good' | 'needs-improvement' | 'poor';
	delta?: number;
	id?: string;
}

/**
 * Report performance metrics (can be sent to analytics)
 */
function reportMetric(metric: PerformanceMetric): void {
	// Log to console in development
	if (import.meta.env.DEV) {
		console.log(`[Performance] ${metric.name}:`, {
			value: Math.round(metric.value),
			rating: metric.rating
		});
	}

	// Send to analytics in production
	if (import.meta.env.PROD && typeof window !== 'undefined') {
		// Example: Send to Google Analytics
		if ('gtag' in window) {
			(window as any).gtag('event', metric.name, {
				value: Math.round(metric.value),
				metric_rating: metric.rating,
				metric_delta: metric.delta ? Math.round(metric.delta) : undefined,
				metric_id: metric.id
			});
		}

		// Send to custom analytics endpoint on Fly.io backend
		// ICT 11+: Use Blob with application/json Content-Type for sendBeacon
		const blob = new Blob([JSON.stringify(metric)], { type: 'application/json' });
		navigator.sendBeacon?.(`${API_BASE_URL}${API_ENDPOINTS.analytics.performance}`, blob);
	}
}

/**
 * Get rating based on thresholds
 */
function getRating(value: number, thresholds: [number, number]): 'good' | 'needs-improvement' | 'poor' {
	if (value <= thresholds[0]) return 'good';
	if (value <= thresholds[1]) return 'needs-improvement';
	return 'poor';
}

/**
 * Largest Contentful Paint (LCP)
 * Good: < 2.5s, Needs Improvement: < 4s, Poor: >= 4s
 */
export function measureLCP(): void {
	if (typeof window === 'undefined' || !('PerformanceObserver' in window)) return;

	try {
		let reported = false;
		const observer = new PerformanceObserver((list) => {
			if (reported) return;
			
			const entries = list.getEntries();
			const lastEntry = entries[entries.length - 1] as any;

			const metric: PerformanceMetric = {
				name: 'LCP',
				value: lastEntry.renderTime || lastEntry.loadTime,
				rating: getRating(lastEntry.renderTime || lastEntry.loadTime, [2500, 4000]),
				id: lastEntry.id
			};

			reportMetric(metric);
		});

		observer.observe({ type: 'largest-contentful-paint', buffered: true });
		
		// Disconnect after page load settles
		setTimeout(() => {
			reported = true;
			observer.disconnect();
		}, 10000);
	} catch (error) {
		console.error('LCP measurement failed:', error);
	}
}

/**
 * Interaction to Next Paint (INP) - Replaced FID as Core Web Vital in March 2024
 * Good: < 200ms, Needs Improvement: < 500ms, Poor: >= 500ms
 *
 * INP measures responsiveness to ALL user interactions, not just the first one.
 * It captures the worst interaction latency throughout the page lifecycle.
 *
 * @returns Cleanup function to remove event listeners (prevents memory leaks in SPAs)
 */
export function measureINP(): (() => void) | undefined {
	if (typeof window === 'undefined' || !('PerformanceObserver' in window)) return;

	try {
		let worstINP = 0;
		let reported = false;
		let timeoutId: ReturnType<typeof setTimeout> | null = null;

		const observer = new PerformanceObserver((list) => {
			const entries = list.getEntries();
			entries.forEach((entry: any) => {
				// Calculate interaction duration (input delay + processing + presentation delay)
				const interactionDuration = entry.duration;
				if (interactionDuration > worstINP) {
					worstINP = interactionDuration;
				}
			});
		});

		observer.observe({ type: 'event', buffered: true, durationThreshold: 16 } as PerformanceObserverInit);

		// Report INP on page visibility change (user leaving)
		const reportINP = () => {
			if (!reported && worstINP > 0) {
				reported = true;
				const metric: PerformanceMetric = {
					name: 'INP',
					value: worstINP,
					rating: getRating(worstINP, [200, 500])
				};
				reportMetric(metric);
				observer.disconnect();
			}
		};

		// Named handler for cleanup (prevents memory leaks)
		const visibilityHandler = () => {
			if (document.visibilityState === 'hidden') {
				reportINP();
			}
		};

		// Report when page is hidden (most accurate)
		document.addEventListener('visibilitychange', visibilityHandler);

		// Fallback: report after 30 seconds of page load
		timeoutId = setTimeout(() => {
			reportINP();
		}, 30000);

		// Return cleanup function for SPA navigation
		return () => {
			document.removeEventListener('visibilitychange', visibilityHandler);
			if (timeoutId) clearTimeout(timeoutId);
			observer.disconnect();
		};

	} catch (error) {
		console.error('INP measurement failed:', error);
		return undefined;
	}
}

/**
 * First Input Delay (FID) - DEPRECATED: Replaced by INP in March 2024
 * Kept for backwards compatibility and legacy analytics
 * Good: < 100ms, Needs Improvement: < 300ms, Poor: >= 300ms
 * @deprecated Use measureINP() instead - INP is the official Core Web Vital since March 2024
 */
export function measureFID(): void {
	if (typeof window === 'undefined' || !('PerformanceObserver' in window)) return;

	try {
		let reported = false;
		const observer = new PerformanceObserver((list) => {
			if (reported) return;
			reported = true;

			const entries = list.getEntries();
			const entry = entries[0] as any;
			if (!entry) return;

			const metric: PerformanceMetric = {
				name: 'FID',
				value: entry.processingStart - entry.startTime,
				rating: getRating(entry.processingStart - entry.startTime, [100, 300])
			};

			reportMetric(metric);
			observer.disconnect();
		});

		observer.observe({ type: 'first-input', buffered: true });
	} catch (error) {
		console.error('FID measurement failed:', error);
	}
}

/**
 * Cumulative Layout Shift (CLS)
 * Good: < 0.1, Needs Improvement: < 0.25, Poor: >= 0.25
 *
 * @returns Cleanup function to remove event listeners (prevents memory leaks in SPAs)
 */
export function measureCLS(): (() => void) | undefined {
	if (typeof window === 'undefined' || !('PerformanceObserver' in window)) return;

	try {
		let clsValue = 0;
		let reported = false;
		let timeoutId: ReturnType<typeof setTimeout> | null = null;

		const observer = new PerformanceObserver((list) => {
			const entries = list.getEntries();
			entries.forEach((entry: any) => {
				if (!entry.hadRecentInput) {
					clsValue += entry.value;
				}
			});
		});

		observer.observe({ type: 'layout-shift', buffered: true });

		// Named handler for proper cleanup (prevents memory leaks)
		const visibilityHandler = () => {
			if (document.visibilityState === 'hidden' && !reported) {
				reported = true;
				const metric: PerformanceMetric = {
					name: 'CLS',
					value: clsValue,
					rating: getRating(clsValue, [0.1, 0.25])
				};
				reportMetric(metric);
				observer.disconnect();
			}
		};

		// Report CLS once after initial load settles (5 seconds)
		timeoutId = setTimeout(() => {
			if (!reported) {
				reported = true;
				const metric: PerformanceMetric = {
					name: 'CLS',
					value: clsValue,
					rating: getRating(clsValue, [0.1, 0.25])
				};
				reportMetric(metric);
				observer.disconnect();
			}
		}, 5000);

		// Also report on page hide
		document.addEventListener('visibilitychange', visibilityHandler);

		// Return cleanup function for SPA navigation
		return () => {
			document.removeEventListener('visibilitychange', visibilityHandler);
			if (timeoutId) clearTimeout(timeoutId);
			observer.disconnect();
		};
	} catch (error) {
		console.error('CLS measurement failed:', error);
		return undefined;
	}
}

/**
 * First Contentful Paint (FCP)
 * Good: < 1.8s, Needs Improvement: < 3s, Poor: >= 3s
 */
export function measureFCP(): void {
	if (typeof window === 'undefined' || !('PerformanceObserver' in window)) return;

	try {
		let reported = false;
		const observer = new PerformanceObserver((list) => {
			if (reported) return;
			
			const entries = list.getEntries();
			// Find the first-contentful-paint entry
			const fcpEntry = entries.find(e => e.name === 'first-contentful-paint');
			if (!fcpEntry) return;
			
			reported = true;
			const metric: PerformanceMetric = {
				name: 'FCP',
				value: fcpEntry.startTime,
				rating: getRating(fcpEntry.startTime, [1800, 3000])
			};

			reportMetric(metric);
			observer.disconnect();
		});

		observer.observe({ type: 'paint', buffered: true });
	} catch (error) {
		console.error('FCP measurement failed:', error);
	}
}

/**
 * Time to First Byte (TTFB)
 * Good: < 800ms, Needs Improvement: < 1800ms, Poor: >= 1800ms
 */
export function measureTTFB(): void {
	if (typeof window === 'undefined' || !('performance' in window)) return;

	try {
		const navigationEntry = performance.getEntriesByType('navigation')[0] as PerformanceNavigationTiming;
		
		if (navigationEntry) {
			const ttfb = navigationEntry.responseStart - navigationEntry.requestStart;

			const metric: PerformanceMetric = {
				name: 'TTFB',
				value: ttfb,
				rating: getRating(ttfb, [800, 1800])
			};

			reportMetric(metric);
		}
	} catch (error) {
		console.error('TTFB measurement failed:', error);
	}
}

// Store cleanup functions for SPA navigation
let performanceCleanupFns: Array<() => void> = [];

/**
 * Initialize all performance monitoring
 * Call this once on app mount
 *
 * @returns Cleanup function to remove all listeners (call on component destroy in SPAs)
 */
export function initPerformanceMonitoring(): () => void {
	if (typeof window === 'undefined') return () => {};

	// Wait for page load to avoid blocking
	if (document.readyState === 'complete') {
		startMonitoring();
	} else {
		const loadHandler = () => startMonitoring();
		window.addEventListener('load', loadHandler, { once: true });
	}

	// Return master cleanup function
	return () => {
		performanceCleanupFns.forEach((cleanup) => cleanup());
		performanceCleanupFns = [];
	};
}

function startMonitoring(): void {
	// Clear any previous cleanup functions (in case of re-initialization)
	performanceCleanupFns.forEach((cleanup) => cleanup());
	performanceCleanupFns = [];

	// Measure all Core Web Vitals (November 2025 standard)
	measureLCP();   // Largest Contentful Paint

	// Collect cleanup functions from metrics that return them
	const inpCleanup = measureINP();   // Interaction to Next Paint (replaced FID in March 2024)
	if (inpCleanup) performanceCleanupFns.push(inpCleanup);

	const clsCleanup = measureCLS();   // Cumulative Layout Shift
	if (clsCleanup) performanceCleanupFns.push(clsCleanup);

	measureFCP();   // First Contentful Paint
	measureTTFB();  // Time to First Byte

	// Legacy FID for backwards compatibility with older analytics
	measureFID();

	// Log bundle size in development
	if (import.meta.env.DEV) {
		logBundleSize();
	}
}

/**
 * Log bundle size for monitoring
 */
function logBundleSize(): void {
	if (typeof window === 'undefined' || !('performance' in window)) return;

	try {
		const resources = performance.getEntriesByType('resource') as PerformanceResourceTiming[];
		let totalSize = 0;
		let jsSize = 0;
		let cssSize = 0;

		resources.forEach((resource) => {
			const size = resource.transferSize || 0;
			totalSize += size;

			if (resource.name.endsWith('.js')) {
				jsSize += size;
			} else if (resource.name.endsWith('.css')) {
				cssSize += size;
			}
		});

		console.log('[Performance] Bundle Size:', {
			total: `${(totalSize / 1024).toFixed(2)} KB`,
			js: `${(jsSize / 1024).toFixed(2)} KB`,
			css: `${(cssSize / 1024).toFixed(2)} KB`
		});
	} catch (error) {
		console.error('Bundle size logging failed:', error);
	}
}

/**
 * Measure custom performance marks
 */
export function measureCustomMetric(name: string, startMark: string, endMark: string): void {
	if (typeof window === 'undefined' || !('performance' in window)) return;

	try {
		performance.measure(name, startMark, endMark);
		const measure = performance.getEntriesByName(name)[0];

		if (measure) {
			console.log(`[Performance] ${name}:`, `${measure.duration.toFixed(2)}ms`);
		}
	} catch (error) {
		console.error(`Custom metric measurement failed for ${name}:`, error);
	}
}

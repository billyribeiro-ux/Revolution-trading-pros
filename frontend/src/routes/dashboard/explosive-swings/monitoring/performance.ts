/**
 * Performance monitoring for Explosive Swings
 * Tracks key metrics for A+++ grade observability
 */

import { browser } from '$app/environment';
import { logger } from '$lib/utils/logger';

export interface PerformanceMetrics {
	// Page load metrics
	pageLoadTime: number;
	firstContentfulPaint: number;
	largestContentfulPaint: number;
	timeToInteractive: number;

	// API metrics
	alertsFetchTime: number;
	tradesFetchTime: number;
	statsFetchTime: number;

	// User interaction metrics
	filterChangeTime: number;
	paginationTime: number;
	modalOpenTime: number;
}

export interface PerformanceThresholds {
	good: number;
	needsImprovement: number;
}

export const PERFORMANCE_THRESHOLDS: Record<string, PerformanceThresholds> = {
	pageLoadTime: { good: 1000, needsImprovement: 2500 },
	firstContentfulPaint: { good: 1800, needsImprovement: 3000 },
	largestContentfulPaint: { good: 2500, needsImprovement: 4000 },
	timeToInteractive: { good: 3800, needsImprovement: 7300 },
	alertsFetchTime: { good: 200, needsImprovement: 500 },
	tradesFetchTime: { good: 200, needsImprovement: 500 },
	statsFetchTime: { good: 150, needsImprovement: 300 },
	filterChangeTime: { good: 100, needsImprovement: 300 },
	paginationTime: { good: 150, needsImprovement: 400 },
	modalOpenTime: { good: 50, needsImprovement: 150 }
};

class PerformanceMonitor {
	private static instance: PerformanceMonitor;
	private metrics: Map<string, number[]> = new Map();
	private observers: PerformanceObserver[] = [];
	private webVitals: Map<string, number> = new Map();

	private constructor() {
		if (browser) {
			this.initializeWebVitalsObservers();
		}
	}

	static getInstance(): PerformanceMonitor {
		if (!PerformanceMonitor.instance) {
			PerformanceMonitor.instance = new PerformanceMonitor();
		}
		return PerformanceMonitor.instance;
	}

	/**
	 * Initialize Web Vitals observers for Core Web Vitals metrics
	 */
	private initializeWebVitalsObservers(): void {
		if (!browser || typeof PerformanceObserver === 'undefined') return;

		try {
			// Largest Contentful Paint (LCP)
			const lcpObserver = new PerformanceObserver((list) => {
				const entries = list.getEntries();
				const lastEntry = entries[entries.length - 1] as PerformanceEntry & {
					renderTime?: number;
					loadTime?: number;
				};
				const lcp = lastEntry.renderTime || lastEntry.loadTime || lastEntry.startTime;
				this.webVitals.set('LCP', lcp);
			});
			lcpObserver.observe({ entryTypes: ['largest-contentful-paint'] });
			this.observers.push(lcpObserver);

			// First Input Delay (FID) / Interaction to Next Paint (INP)
			const fidObserver = new PerformanceObserver((list) => {
				for (const entry of list.getEntries() as (PerformanceEntry & {
					processingStart: number;
				})[]) {
					const fid = entry.processingStart - entry.startTime;
					this.webVitals.set('FID', fid);
				}
			});
			fidObserver.observe({ entryTypes: ['first-input'] });
			this.observers.push(fidObserver);

			// Cumulative Layout Shift (CLS)
			let clsValue = 0;
			const clsObserver = new PerformanceObserver((list) => {
				for (const entry of list.getEntries() as (PerformanceEntry & {
					hadRecentInput: boolean;
					value: number;
				})[]) {
					if (!entry.hadRecentInput) {
						clsValue += entry.value;
						this.webVitals.set('CLS', clsValue);
					}
				}
			});
			clsObserver.observe({ entryTypes: ['layout-shift'] });
			this.observers.push(clsObserver);

			// First Contentful Paint (FCP)
			const paintObserver = new PerformanceObserver((list) => {
				for (const entry of list.getEntries()) {
					if (entry.name === 'first-contentful-paint') {
						this.webVitals.set('FCP', entry.startTime);
					}
				}
			});
			paintObserver.observe({ entryTypes: ['paint'] });
			this.observers.push(paintObserver);
		} catch (error) {
			// Graceful degradation: Log observer initialization failures
			// This is expected in older browsers or environments without PerformanceObserver support
			logger.debug('[PerformanceMonitor] Some observers not supported:', error);
		}
	}

	/**
	 * Start a performance mark for measuring duration
	 */
	startMark(name: string): void {
		if (!browser) return;
		try {
			performance.mark(`${name}-start`);
		} catch {
			// Ignore if Performance API not available
		}
	}

	/**
	 * End a performance mark and return the duration
	 */
	endMark(name: string): number {
		if (!browser) return 0;

		try {
			performance.mark(`${name}-end`);
			performance.measure(name, `${name}-start`, `${name}-end`);

			const measure = performance.getEntriesByName(name, 'measure')[0];
			const duration = measure?.duration ?? 0;

			// Store for aggregation
			if (!this.metrics.has(name)) {
				this.metrics.set(name, []);
			}
			this.metrics.get(name)!.push(duration);

			// Cleanup
			performance.clearMarks(`${name}-start`);
			performance.clearMarks(`${name}-end`);
			performance.clearMeasures(name);

			// Log slow operations in development
			if (import.meta.env.DEV) {
				const threshold = PERFORMANCE_THRESHOLDS[name];
				if (threshold && duration > threshold.needsImprovement) {
					console.warn(
						`[PerformanceMonitor] Slow operation: ${name} took ${duration.toFixed(2)}ms`
					);
				}
			}

			return duration;
		} catch {
			return 0;
		}
	}

	/**
	 * Measure an async operation's duration
	 */
	async measureAsync<T>(name: string, fn: () => Promise<T>): Promise<T> {
		this.startMark(name);
		try {
			return await fn();
		} finally {
			this.endMark(name);
		}
	}

	/**
	 * Measure a sync operation's duration
	 */
	measureSync<T>(name: string, fn: () => T): T {
		this.startMark(name);
		try {
			return fn();
		} finally {
			this.endMark(name);
		}
	}

	/**
	 * Get the average time for a specific metric
	 */
	getAverageTime(name: string): number {
		const times = this.metrics.get(name);
		if (!times || times.length === 0) return 0;
		return times.reduce((a, b) => a + b, 0) / times.length;
	}

	/**
	 * Get the P95 time for a specific metric
	 */
	getP95Time(name: string): number {
		const times = this.metrics
			.get(name)
			?.slice()
			.sort((a, b) => a - b);
		if (!times || times.length === 0) return 0;
		const index = Math.floor(times.length * 0.95);
		return times[Math.min(index, times.length - 1)];
	}

	/**
	 * Get the P99 time for a specific metric
	 */
	getP99Time(name: string): number {
		const times = this.metrics
			.get(name)
			?.slice()
			.sort((a, b) => a - b);
		if (!times || times.length === 0) return 0;
		const index = Math.floor(times.length * 0.99);
		return times[Math.min(index, times.length - 1)];
	}

	/**
	 * Get the minimum time for a specific metric
	 */
	getMinTime(name: string): number {
		const times = this.metrics.get(name);
		if (!times || times.length === 0) return 0;
		return Math.min(...times);
	}

	/**
	 * Get the maximum time for a specific metric
	 */
	getMaxTime(name: string): number {
		const times = this.metrics.get(name);
		if (!times || times.length === 0) return 0;
		return Math.max(...times);
	}

	/**
	 * Get the count of measurements for a specific metric
	 */
	getCount(name: string): number {
		return this.metrics.get(name)?.length ?? 0;
	}

	/**
	 * Get a Web Vital metric value
	 */
	private getWebVital(name: string): number {
		return this.webVitals.get(name) ?? 0;
	}

	/**
	 * Report all collected performance metrics
	 */
	reportMetrics(): PerformanceMetrics {
		return {
			pageLoadTime: this.getAverageTime('page-load'),
			firstContentfulPaint: this.getWebVital('FCP'),
			largestContentfulPaint: this.getWebVital('LCP'),
			timeToInteractive: this.getWebVital('TTI'),
			alertsFetchTime: this.getAverageTime('fetch-alerts'),
			tradesFetchTime: this.getAverageTime('fetch-trades'),
			statsFetchTime: this.getAverageTime('fetch-stats'),
			filterChangeTime: this.getAverageTime('filter-change'),
			paginationTime: this.getAverageTime('pagination'),
			modalOpenTime: this.getAverageTime('modal-open')
		};
	}

	/**
	 * Get detailed metrics for a specific operation
	 */
	getDetailedMetrics(name: string): {
		average: number;
		p95: number;
		p99: number;
		min: number;
		max: number;
		count: number;
	} {
		return {
			average: this.getAverageTime(name),
			p95: this.getP95Time(name),
			p99: this.getP99Time(name),
			min: this.getMinTime(name),
			max: this.getMaxTime(name),
			count: this.getCount(name)
		};
	}

	/**
	 * Get performance rating for a metric
	 */
	getPerformanceRating(name: string, value?: number): 'good' | 'needs-improvement' | 'poor' {
		const actualValue = value ?? this.getAverageTime(name);
		const threshold = PERFORMANCE_THRESHOLDS[name];

		if (!threshold) return 'good';

		if (actualValue <= threshold.good) return 'good';
		if (actualValue <= threshold.needsImprovement) return 'needs-improvement';
		return 'poor';
	}

	/**
	 * Clear all stored metrics
	 */
	clearMetrics(): void {
		this.metrics.clear();
	}

	/**
	 * Clear metrics for a specific operation
	 */
	clearMetric(name: string): void {
		this.metrics.delete(name);
	}

	/**
	 * Export all metrics as JSON for logging/reporting
	 */
	exportMetrics(): Record<string, unknown> {
		const allMetrics: Record<string, unknown> = {};

		for (const [name, _times] of this.metrics) {
			allMetrics[name] = this.getDetailedMetrics(name);
		}

		// Add Web Vitals
		allMetrics['webVitals'] = {
			FCP: this.getWebVital('FCP'),
			LCP: this.getWebVital('LCP'),
			FID: this.getWebVital('FID'),
			CLS: this.getWebVital('CLS')
		};

		return allMetrics;
	}

	/**
	 * Cleanup observers on destroy
	 */
	destroy(): void {
		for (const observer of this.observers) {
			observer.disconnect();
		}
		this.observers = [];
		this.metrics.clear();
		this.webVitals.clear();
	}
}

// Export singleton instance
export const performanceMonitor = PerformanceMonitor.getInstance();

// Convenience exports
export const startMark = (name: string) => performanceMonitor.startMark(name);
export const endMark = (name: string) => performanceMonitor.endMark(name);
export const measureAsync = <T>(name: string, fn: () => Promise<T>) =>
	performanceMonitor.measureAsync(name, fn);
export const measureSync = <T>(name: string, fn: () => T) =>
	performanceMonitor.measureSync(name, fn);
export const reportMetrics = () => performanceMonitor.reportMetrics();
export const getDetailedMetrics = (name: string) => performanceMonitor.getDetailedMetrics(name);
export const getPerformanceRating = (name: string, value?: number) =>
	performanceMonitor.getPerformanceRating(name, value);

export default performanceMonitor;

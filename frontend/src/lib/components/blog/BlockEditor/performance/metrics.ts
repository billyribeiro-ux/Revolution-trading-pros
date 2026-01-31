/**
 * Revolution Trading Pros - Web Vitals & Performance Metrics
 * ===============================================================================
 * Enterprise-grade performance monitoring for the Block Editor
 *
 * Tracks Core Web Vitals (LCP, FID, CLS, INP, TTFB) and editor-specific metrics
 * using the PerformanceObserver API and web-vitals library.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */

import { browser } from '$app/environment';
import { onCLS, onLCP, onINP, onTTFB, onFCP } from 'web-vitals';
import type { Metric } from 'web-vitals';

// =============================================================================
// Types
// =============================================================================

// Note: FID is deprecated in favor of INP as of Core Web Vitals 2024
// FCP (First Contentful Paint) is included as an additional useful metric
export type WebVitalName = 'LCP' | 'FCP' | 'CLS' | 'INP' | 'TTFB';

export interface WebVitalMetric {
	name: WebVitalName;
	value: number;
	rating: 'good' | 'needs-improvement' | 'poor';
	delta: number;
	entries: PerformanceEntry[];
	navigationType: string;
	timestamp: number;
}

export interface EditorMetric {
	name: string;
	values: number[];
	count: number;
	min: number;
	max: number;
	sum: number;
	lastUpdated: number;
}

export interface BlockRenderMetric {
	blockType: string;
	renderTime: number;
	timestamp: number;
	blockId: string;
}

export interface OperationMetric {
	operation: string;
	duration: number;
	success: boolean;
	timestamp: number;
	metadata?: Record<string, unknown>;
}

export interface MemoryMetrics {
	usedJSHeapSize: number;
	totalJSHeapSize: number;
	jsHeapSizeLimit: number;
	usagePercentage: number;
	timestamp: number;
}

export interface PercentileResult {
	p50: number;
	p75: number;
	p95: number;
	p99: number;
}

export interface MetricsSnapshot {
	webVitals: Record<WebVitalName, WebVitalMetric | null>;
	editorMetrics: Record<string, EditorMetric>;
	blockRenderMetrics: BlockRenderMetric[];
	operationMetrics: OperationMetric[];
	memoryMetrics: MemoryMetrics | null;
	totalBlockCount: number;
	fps: number;
	timestamp: number;
}

// =============================================================================
// Constants
// =============================================================================

const WEB_VITAL_THRESHOLDS: Record<WebVitalName, { good: number; poor: number }> = {
	LCP: { good: 2500, poor: 4000 },
	FCP: { good: 1800, poor: 3000 },
	CLS: { good: 0.1, poor: 0.25 },
	INP: { good: 200, poor: 500 },
	TTFB: { good: 800, poor: 1800 }
};

const EDITOR_METRIC_NAMES = {
	BLOCK_RENDER: 'block_render',
	DRAG_DROP_LATENCY: 'drag_drop_latency',
	KEYSTROKE_LATENCY: 'keystroke_latency',
	SAVE_OPERATION: 'save_operation',
	AI_RESPONSE: 'ai_response',
	AUTOSAVE: 'autosave',
	UNDO_REDO: 'undo_redo',
	BLOCK_INSERT: 'block_insert',
	BLOCK_DELETE: 'block_delete',
	BLOCK_MOVE: 'block_move',
	SELECTION_CHANGE: 'selection_change',
	PREVIEW_RENDER: 'preview_render'
} as const;

const MAX_STORED_VALUES = 1000;
const MAX_BLOCK_RENDER_HISTORY = 500;
const MAX_OPERATION_HISTORY = 200;

// =============================================================================
// Performance Metrics Store
// =============================================================================

class PerformanceMetricsStore {
	private webVitals: Map<WebVitalName, WebVitalMetric> = new Map();
	private editorMetrics: Map<string, EditorMetric> = new Map();
	private blockRenderMetrics: BlockRenderMetric[] = [];
	private operationMetrics: OperationMetric[] = [];
	private memoryMetrics: MemoryMetrics | null = null;
	private totalBlockCount: number = 0;
	private fps: number = 60;
	private fpsFrames: number[] = [];
	private lastFrameTime: number = 0;
	private observers: Set<(snapshot: MetricsSnapshot) => void> = new Set();
	private isInitialized: boolean = false;
	private longTaskObserver: PerformanceObserver | null = null;
	private layoutShiftObserver: PerformanceObserver | null = null;
	private fpsRafId: number | null = null;

	/**
	 * Initialize performance monitoring
	 */
	initialize(): void {
		if (!browser || this.isInitialized) return;

		this.isInitialized = true;

		// Initialize Core Web Vitals tracking
		this.initWebVitals();

		// Initialize PerformanceObserver for long tasks
		this.initLongTaskObserver();

		// Initialize layout shift observer
		this.initLayoutShiftObserver();

		// Start FPS monitoring
		this.startFPSMonitoring();

		// Start memory monitoring
		this.startMemoryMonitoring();

		console.debug('[PerformanceMetrics] Initialized');
	}

	/**
	 * Initialize Core Web Vitals tracking using web-vitals library
	 * Note: FID is deprecated, using FCP (First Contentful Paint) instead
	 */
	private initWebVitals(): void {
		const handleMetric = (metric: Metric, name: WebVitalName) => {
			const webVitalMetric: WebVitalMetric = {
				name,
				value: metric.value,
				rating: metric.rating,
				delta: metric.delta,
				entries: metric.entries,
				navigationType: metric.navigationType,
				timestamp: Date.now()
			};

			this.webVitals.set(name, webVitalMetric);
			this.notifyObservers();
		};

		onLCP((metric: Metric) => handleMetric(metric, 'LCP'));
		onFCP((metric: Metric) => handleMetric(metric, 'FCP'));
		onCLS((metric: Metric) => handleMetric(metric, 'CLS'));
		onINP((metric: Metric) => handleMetric(metric, 'INP'));
		onTTFB((metric: Metric) => handleMetric(metric, 'TTFB'));
	}

	/**
	 * Initialize long task observer for detecting jank
	 */
	private initLongTaskObserver(): void {
		if (!('PerformanceObserver' in window)) return;

		try {
			this.longTaskObserver = new PerformanceObserver((list) => {
				for (const entry of list.getEntries()) {
					if (entry.duration > 50) {
						this.recordEditorMetric('long_task', entry.duration);
					}
				}
			});

			this.longTaskObserver.observe({ entryTypes: ['longtask'] });
		} catch (e) {
			console.debug('[PerformanceMetrics] Long task observer not supported');
		}
	}

	/**
	 * Initialize layout shift observer
	 */
	private initLayoutShiftObserver(): void {
		if (!('PerformanceObserver' in window)) return;

		try {
			this.layoutShiftObserver = new PerformanceObserver((list) => {
				for (const entry of list.getEntries()) {
					const layoutShiftEntry = entry as PerformanceEntry & {
						hadRecentInput: boolean;
						value: number
					};
					if (!layoutShiftEntry.hadRecentInput) {
						this.recordEditorMetric('layout_shift', layoutShiftEntry.value);
					}
				}
			});

			this.layoutShiftObserver.observe({
				type: 'layout-shift',
				buffered: true
			});
		} catch (e) {
			console.debug('[PerformanceMetrics] Layout shift observer not supported');
		}
	}

	/**
	 * Start FPS monitoring using requestAnimationFrame
	 */
	private startFPSMonitoring(): void {
		this.lastFrameTime = performance.now();

		const measureFPS = () => {
			const now = performance.now();
			const delta = now - this.lastFrameTime;
			this.lastFrameTime = now;

			if (delta > 0) {
				this.fpsFrames.push(1000 / delta);

				// Keep only last 60 frames for averaging
				if (this.fpsFrames.length > 60) {
					this.fpsFrames.shift();
				}

				// Calculate average FPS
				const sum = this.fpsFrames.reduce((a, b) => a + b, 0);
				this.fps = Math.round(sum / this.fpsFrames.length);
			}

			this.fpsRafId = requestAnimationFrame(measureFPS);
		};

		this.fpsRafId = requestAnimationFrame(measureFPS);
	}

	/**
	 * Start memory monitoring
	 */
	private startMemoryMonitoring(): void {
		const measureMemory = () => {
			if ('memory' in performance) {
				const memory = (performance as Performance & {
					memory: {
						usedJSHeapSize: number;
						totalJSHeapSize: number;
						jsHeapSizeLimit: number;
					}
				}).memory;

				this.memoryMetrics = {
					usedJSHeapSize: memory.usedJSHeapSize,
					totalJSHeapSize: memory.totalJSHeapSize,
					jsHeapSizeLimit: memory.jsHeapSizeLimit,
					usagePercentage: (memory.usedJSHeapSize / memory.jsHeapSizeLimit) * 100,
					timestamp: Date.now()
				};
			}
		};

		// Initial measurement
		measureMemory();

		// Update every 5 seconds
		setInterval(measureMemory, 5000);
	}

	/**
	 * Record an editor-specific metric
	 */
	recordEditorMetric(name: string, value: number): void {
		let metric = this.editorMetrics.get(name);

		if (!metric) {
			metric = {
				name,
				values: [],
				count: 0,
				min: Infinity,
				max: -Infinity,
				sum: 0,
				lastUpdated: Date.now()
			};
			this.editorMetrics.set(name, metric);
		}

		metric.values.push(value);
		metric.count++;
		metric.sum += value;
		metric.min = Math.min(metric.min, value);
		metric.max = Math.max(metric.max, value);
		metric.lastUpdated = Date.now();

		// Trim values array if too large
		if (metric.values.length > MAX_STORED_VALUES) {
			metric.values = metric.values.slice(-MAX_STORED_VALUES);
		}

		this.notifyObservers();
	}

	/**
	 * Record block render time
	 */
	recordBlockRender(blockType: string, renderTime: number, blockId: string): void {
		const metric: BlockRenderMetric = {
			blockType,
			renderTime,
			blockId,
			timestamp: Date.now()
		};

		this.blockRenderMetrics.push(metric);

		// Trim history
		if (this.blockRenderMetrics.length > MAX_BLOCK_RENDER_HISTORY) {
			this.blockRenderMetrics = this.blockRenderMetrics.slice(-MAX_BLOCK_RENDER_HISTORY);
		}

		// Also record as editor metric
		this.recordEditorMetric(`${EDITOR_METRIC_NAMES.BLOCK_RENDER}_${blockType}`, renderTime);
		this.recordEditorMetric(EDITOR_METRIC_NAMES.BLOCK_RENDER, renderTime);

		this.notifyObservers();
	}

	/**
	 * Record an operation metric
	 */
	recordOperation(
		operation: string,
		duration: number,
		success: boolean = true,
		metadata?: Record<string, unknown>
	): void {
		const metric: OperationMetric = {
			operation,
			duration,
			success,
			timestamp: Date.now(),
			metadata
		};

		this.operationMetrics.push(metric);

		// Trim history
		if (this.operationMetrics.length > MAX_OPERATION_HISTORY) {
			this.operationMetrics = this.operationMetrics.slice(-MAX_OPERATION_HISTORY);
		}

		// Also record as editor metric
		this.recordEditorMetric(operation, duration);

		this.notifyObservers();
	}

	/**
	 * Update total block count
	 */
	setBlockCount(count: number): void {
		this.totalBlockCount = count;
		this.notifyObservers();
	}

	/**
	 * Calculate percentiles for a metric
	 */
	getPercentiles(metricName: string): PercentileResult | null {
		const metric = this.editorMetrics.get(metricName);
		if (!metric || metric.values.length === 0) return null;

		const sorted = [...metric.values].sort((a, b) => a - b);
		const len = sorted.length;

		return {
			p50: sorted[Math.floor(len * 0.5)] ?? 0,
			p75: sorted[Math.floor(len * 0.75)] ?? 0,
			p95: sorted[Math.floor(len * 0.95)] ?? 0,
			p99: sorted[Math.floor(len * 0.99)] ?? 0
		};
	}

	/**
	 * Get average value for a metric
	 */
	getAverage(metricName: string): number | null {
		const metric = this.editorMetrics.get(metricName);
		if (!metric || metric.count === 0) return null;
		return metric.sum / metric.count;
	}

	/**
	 * Get block render stats by type
	 */
	getBlockRenderStats(): Record<string, {
		avgRenderTime: number;
		count: number;
		percentiles: PercentileResult
	}> {
		const stats: Record<string, {
			avgRenderTime: number;
			count: number;
			percentiles: PercentileResult
		}> = {};

		const byType = new Map<string, number[]>();

		for (const metric of this.blockRenderMetrics) {
			const existing = byType.get(metric.blockType) || [];
			existing.push(metric.renderTime);
			byType.set(metric.blockType, existing);
		}

		for (const [type, times] of byType) {
			const sorted = [...times].sort((a, b) => a - b);
			const len = sorted.length;
			const sum = times.reduce((a, b) => a + b, 0);

			stats[type] = {
				avgRenderTime: sum / len,
				count: len,
				percentiles: {
					p50: sorted[Math.floor(len * 0.5)] ?? 0,
					p75: sorted[Math.floor(len * 0.75)] ?? 0,
					p95: sorted[Math.floor(len * 0.95)] ?? 0,
					p99: sorted[Math.floor(len * 0.99)] ?? 0
				}
			};
		}

		return stats;
	}

	/**
	 * Get a snapshot of all metrics
	 */
	getSnapshot(): MetricsSnapshot {
		const webVitalsRecord: Record<WebVitalName, WebVitalMetric | null> = {
			LCP: this.webVitals.get('LCP') ?? null,
			FCP: this.webVitals.get('FCP') ?? null,
			CLS: this.webVitals.get('CLS') ?? null,
			INP: this.webVitals.get('INP') ?? null,
			TTFB: this.webVitals.get('TTFB') ?? null
		};

		const editorMetricsRecord: Record<string, EditorMetric> = {};
		for (const [key, value] of this.editorMetrics) {
			editorMetricsRecord[key] = value;
		}

		return {
			webVitals: webVitalsRecord,
			editorMetrics: editorMetricsRecord,
			blockRenderMetrics: [...this.blockRenderMetrics],
			operationMetrics: [...this.operationMetrics],
			memoryMetrics: this.memoryMetrics,
			totalBlockCount: this.totalBlockCount,
			fps: this.fps,
			timestamp: Date.now()
		};
	}

	/**
	 * Subscribe to metric updates
	 */
	subscribe(callback: (snapshot: MetricsSnapshot) => void): () => void {
		this.observers.add(callback);

		// Immediately call with current snapshot
		callback(this.getSnapshot());

		return () => {
			this.observers.delete(callback);
		};
	}

	/**
	 * Notify all observers of metric changes
	 */
	private notifyObservers(): void {
		const snapshot = this.getSnapshot();
		for (const observer of this.observers) {
			observer(snapshot);
		}
	}

	/**
	 * Reset all metrics
	 */
	reset(): void {
		this.editorMetrics.clear();
		this.blockRenderMetrics = [];
		this.operationMetrics = [];
		this.notifyObservers();
	}

	/**
	 * Cleanup observers and intervals
	 */
	destroy(): void {
		if (this.longTaskObserver) {
			this.longTaskObserver.disconnect();
		}
		if (this.layoutShiftObserver) {
			this.layoutShiftObserver.disconnect();
		}
		if (this.fpsRafId !== null) {
			cancelAnimationFrame(this.fpsRafId);
		}
		this.observers.clear();
		this.isInitialized = false;
	}
}

// =============================================================================
// Singleton Instance
// =============================================================================

const metricsStore = new PerformanceMetricsStore();

// =============================================================================
// Exported Functions
// =============================================================================

/**
 * Initialize performance monitoring
 */
export function initializeMetrics(): void {
	metricsStore.initialize();
}

/**
 * Measure block render time
 */
export function measureBlockRender(
	blockType: string,
	blockId: string
): () => void {
	const startTime = performance.now();

	return () => {
		const renderTime = performance.now() - startTime;
		metricsStore.recordBlockRender(blockType, renderTime, blockId);
	};
}

/**
 * Measure an operation
 */
export function measureOperation(
	operation: string,
	metadata?: Record<string, unknown>
): { end: (success?: boolean) => void } {
	const startTime = performance.now();

	return {
		end: (success: boolean = true) => {
			const duration = performance.now() - startTime;
			metricsStore.recordOperation(operation, duration, success, metadata);
		}
	};
}

/**
 * Measure an async operation
 */
export async function measureAsync<T>(
	operation: string,
	fn: () => Promise<T>,
	metadata?: Record<string, unknown>
): Promise<T> {
	const startTime = performance.now();
	let success = true;

	try {
		const result = await fn();
		return result;
	} catch (error) {
		success = false;
		throw error;
	} finally {
		const duration = performance.now() - startTime;
		metricsStore.recordOperation(operation, duration, success, metadata);
	}
}

/**
 * Record drag-drop latency
 */
export function recordDragDropLatency(latency: number): void {
	metricsStore.recordEditorMetric(EDITOR_METRIC_NAMES.DRAG_DROP_LATENCY, latency);
}

/**
 * Record keystroke latency
 */
export function recordKeystrokeLatency(latency: number): void {
	metricsStore.recordEditorMetric(EDITOR_METRIC_NAMES.KEYSTROKE_LATENCY, latency);
}

/**
 * Record save operation time
 */
export function recordSaveOperation(duration: number, success: boolean = true): void {
	metricsStore.recordOperation(EDITOR_METRIC_NAMES.SAVE_OPERATION, duration, success);
}

/**
 * Record AI response time
 */
export function recordAIResponse(duration: number, success: boolean = true): void {
	metricsStore.recordOperation(EDITOR_METRIC_NAMES.AI_RESPONSE, duration, success);
}

/**
 * Update total block count
 */
export function setBlockCount(count: number): void {
	metricsStore.setBlockCount(count);
}

/**
 * Get all metrics
 */
export function getMetrics(): MetricsSnapshot {
	return metricsStore.getSnapshot();
}

/**
 * Get percentiles for a specific metric
 */
export function getPercentiles(metricName: string): PercentileResult | null {
	return metricsStore.getPercentiles(metricName);
}

/**
 * Get block render statistics by type
 */
export function getBlockRenderStats(): Record<string, {
	avgRenderTime: number;
	count: number;
	percentiles: PercentileResult
}> {
	return metricsStore.getBlockRenderStats();
}

/**
 * Subscribe to metric updates
 */
export function subscribeToMetrics(
	callback: (snapshot: MetricsSnapshot) => void
): () => void {
	return metricsStore.subscribe(callback);
}

/**
 * Reset all metrics
 */
export function resetMetrics(): void {
	metricsStore.reset();
}

/**
 * Destroy metrics store
 */
export function destroyMetrics(): void {
	metricsStore.destroy();
}

/**
 * Get rating for a Web Vital value
 */
export function getWebVitalRating(
	name: WebVitalName,
	value: number
): 'good' | 'needs-improvement' | 'poor' {
	const thresholds = WEB_VITAL_THRESHOLDS[name];
	if (value <= thresholds.good) return 'good';
	if (value <= thresholds.poor) return 'needs-improvement';
	return 'poor';
}

/**
 * Get thresholds for a Web Vital
 */
export function getWebVitalThresholds(name: WebVitalName): { good: number; poor: number } {
	return WEB_VITAL_THRESHOLDS[name];
}

// =============================================================================
// Export Constants
// =============================================================================

export { EDITOR_METRIC_NAMES, WEB_VITAL_THRESHOLDS };

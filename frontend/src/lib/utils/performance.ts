/**
 * Performance Monitoring Utilities
 * ══════════════════════════════════════════════════════════════════════════════
 * Comprehensive performance monitoring, optimization utilities, and memory management
 * for high-performance web applications.
 *
 * Features:
 * - PerformanceMonitor class for tracking metrics and FPS
 * - Debounce and throttle with TypeScript generics
 * - Lazy loading with caching
 * - Virtual scroll calculations
 * - Image optimization utilities
 * - Memory management with WeakRef/FinalizationRegistry
 * - Scheduler utilities for idle/animation frame work
 * - Network-aware feature loading
 * ══════════════════════════════════════════════════════════════════════════════
 */

/// <reference lib="dom" />
/// <reference lib="dom.iterable" />

// ============================================================================
// Types and Interfaces
// ============================================================================

export interface PerformanceMetricEntry {
	name: string;
	value: number;
	timestamp: number;
	metadata?: Record<string, unknown>;
}

export interface MetricStats {
	name: string;
	count: number;
	total: number;
	average: number;
	min: number;
	max: number;
	latest: number;
}

export interface PerformanceReport {
	metrics: MetricStats[];
	fps: number;
	memory: MemoryMetrics | null;
	degraded: boolean;
	timestamp: number;
}

export interface MemoryMetrics {
	usedJSHeapSize: number;
	totalJSHeapSize: number;
	jsHeapSizeLimit: number;
	usagePercentage: number;
}

export interface VirtualScrollIndices {
	startIndex: number;
	endIndex: number;
	visibleCount: number;
	offsetY: number;
}

export interface NetworkInfo {
	effectiveType: '4g' | '3g' | '2g' | 'slow-2g' | 'unknown';
	downlink: number;
	rtt: number;
	saveData: boolean;
	online: boolean;
}

export interface ImageOptimizationOptions {
	baseUrl?: string;
	format?: 'webp' | 'avif' | 'jpeg' | 'png' | 'auto';
	quality?: number;
	fit?: 'cover' | 'contain' | 'fill' | 'inside' | 'outside';
}

// ============================================================================
// PerformanceMonitor Class
// ============================================================================

/**
 * Performance monitoring class for tracking metrics, FPS, and memory usage
 */
export class PerformanceMonitor {
	private marks: Map<string, number> = new Map();
	private metrics: Map<string, PerformanceMetricEntry[]> = new Map();
	private fpsHistory: number[] = [];
	private lastFrameTime: number = 0;
	private frameCount: number = 0;
	private fpsUpdateInterval: number = 1000;
	private currentFps: number = 60;
	private isMonitoringFps: boolean = false;
	private rafId: number | null = null;

	// Performance thresholds for degradation detection
	private thresholds = {
		fps: 30,
		memoryUsagePercent: 80,
		avgMetricMs: 100
	};

	constructor(options?: { thresholds?: Partial<typeof PerformanceMonitor.prototype.thresholds> }) {
		if (options?.thresholds) {
			this.thresholds = { ...this.thresholds, ...options.thresholds };
		}
	}

	/**
	 * Mark the start of a performance measurement
	 */
	markStart(name: string): void {
		this.marks.set(name, performance.now());
	}

	/**
	 * Mark the end of a performance measurement and record the duration
	 */
	markEnd(name: string, metadata?: Record<string, unknown>): number | null {
		const startTime = this.marks.get(name);
		if (startTime === undefined) {
			console.warn(`[PerformanceMonitor] No start mark found for: ${name}`);
			return null;
		}

		const duration = performance.now() - startTime;
		this.marks.delete(name);
		this.recordMetric(name, duration, metadata);
		return duration;
	}

	/**
	 * Record a metric value directly
	 */
	recordMetric(name: string, value: number, metadata?: Record<string, unknown>): void {
		const entry: PerformanceMetricEntry = {
			name,
			value,
			timestamp: Date.now(),
			metadata
		};

		if (!this.metrics.has(name)) {
			this.metrics.set(name, []);
		}

		const entries = this.metrics.get(name)!;
		entries.push(entry);

		// Keep only the last 100 entries per metric to prevent memory bloat
		if (entries.length > 100) {
			entries.shift();
		}
	}

	/**
	 * Get all recorded metrics for a specific name
	 */
	getMetrics(name: string): PerformanceMetricEntry[] {
		return this.metrics.get(name) || [];
	}

	/**
	 * Get average metrics for all or specific metric names
	 */
	getAverageMetrics(names?: string[]): MetricStats[] {
		const targetNames = names || Array.from(this.metrics.keys());
		const stats: MetricStats[] = [];

		for (const name of targetNames) {
			const entries = this.metrics.get(name);
			if (!entries || entries.length === 0) continue;

			const values = entries.map((e) => e.value);
			const total = values.reduce((sum, v) => sum + v, 0);
			const average = total / values.length;
			const min = Math.min(...values);
			const max = Math.max(...values);
			const latest = values[values.length - 1];

			stats.push({
				name,
				count: entries.length,
				total,
				average,
				min,
				max,
				latest
			});
		}

		return stats;
	}

	/**
	 * Check if performance is degraded based on thresholds
	 */
	isPerformanceDegraded(): boolean {
		// Check FPS
		if (this.currentFps < this.thresholds.fps) {
			return true;
		}

		// Check memory usage
		const memory = this.getMemoryMetrics();
		if (memory && memory.usagePercentage > this.thresholds.memoryUsagePercent) {
			return true;
		}

		// Check average metric times
		const allStats = this.getAverageMetrics();
		for (const stat of allStats) {
			if (stat.average > this.thresholds.avgMetricMs) {
				return true;
			}
		}

		return false;
	}

	/**
	 * Update FPS calculation (call this in animation frame loop)
	 */
	updateFPS(timestamp?: number): void {
		const now = timestamp || performance.now();

		if (this.lastFrameTime === 0) {
			this.lastFrameTime = now;
			return;
		}

		this.frameCount++;
		const elapsed = now - this.lastFrameTime;

		if (elapsed >= this.fpsUpdateInterval) {
			this.currentFps = Math.round((this.frameCount * 1000) / elapsed);
			this.fpsHistory.push(this.currentFps);

			// Keep only last 60 FPS samples
			if (this.fpsHistory.length > 60) {
				this.fpsHistory.shift();
			}

			this.frameCount = 0;
			this.lastFrameTime = now;
		}
	}

	/**
	 * Start automatic FPS monitoring
	 */
	startFPSMonitoring(): void {
		if (this.isMonitoringFps) return;
		this.isMonitoringFps = true;

		const loop = (timestamp: number) => {
			this.updateFPS(timestamp);
			if (this.isMonitoringFps) {
				this.rafId = requestAnimationFrame(loop);
			}
		};

		this.rafId = requestAnimationFrame(loop);
	}

	/**
	 * Stop automatic FPS monitoring
	 */
	stopFPSMonitoring(): void {
		this.isMonitoringFps = false;
		if (this.rafId !== null) {
			cancelAnimationFrame(this.rafId);
			this.rafId = null;
		}
	}

	/**
	 * Get current FPS and history
	 */
	getFPS(): { current: number; average: number; history: number[] } {
		const average =
			this.fpsHistory.length > 0
				? this.fpsHistory.reduce((sum, fps) => sum + fps, 0) / this.fpsHistory.length
				: this.currentFps;

		return {
			current: this.currentFps,
			average: Math.round(average),
			history: [...this.fpsHistory]
		};
	}

	/**
	 * Get memory metrics if available
	 */
	getMemoryMetrics(): MemoryMetrics | null {
		if (typeof window === 'undefined') return null;

		// Memory API is non-standard but available in Chrome
		const perfMemory = (performance as any).memory;
		if (!perfMemory) return null;

		return {
			usedJSHeapSize: perfMemory.usedJSHeapSize,
			totalJSHeapSize: perfMemory.totalJSHeapSize,
			jsHeapSizeLimit: perfMemory.jsHeapSizeLimit,
			usagePercentage: (perfMemory.usedJSHeapSize / perfMemory.jsHeapSizeLimit) * 100
		};
	}

	/**
	 * Generate a comprehensive performance report
	 */
	getReport(): PerformanceReport {
		return {
			metrics: this.getAverageMetrics(),
			fps: this.currentFps,
			memory: this.getMemoryMetrics(),
			degraded: this.isPerformanceDegraded(),
			timestamp: Date.now()
		};
	}

	/**
	 * Clear all recorded metrics and reset state
	 */
	clear(): void {
		this.marks.clear();
		this.metrics.clear();
		this.fpsHistory = [];
		this.frameCount = 0;
		this.lastFrameTime = 0;
		this.currentFps = 60;
	}
}

// ============================================================================
// Debounce and Throttle Utilities
// ============================================================================

/**
 * Debounce function with proper TypeScript generics
 * Delays execution until after a specified wait time has elapsed since the last call
 */
export function debounce<T extends (...args: any[]) => any>(
	fn: T,
	wait: number,
	options?: { leading?: boolean; trailing?: boolean; maxWait?: number }
): T & { cancel: () => void; flush: () => ReturnType<T> | undefined } {
	const { leading = false, trailing = true, maxWait } = options || {};

	let timeoutId: ReturnType<typeof setTimeout> | null = null;
	let maxTimeoutId: ReturnType<typeof setTimeout> | null = null;
	let lastCallTime: number | undefined;
	let lastInvokeTime: number = 0;
	let lastArgs: Parameters<T> | undefined;
	let lastThis: any;
	let result: ReturnType<T> | undefined;

	const invokeFunc = (time: number): ReturnType<T> => {
		const args = lastArgs!;
		const thisArg = lastThis;

		lastArgs = undefined;
		lastThis = undefined;
		lastInvokeTime = time;
		result = fn.apply(thisArg, args);
		return result as ReturnType<T>;
	};

	const shouldInvoke = (time: number): boolean => {
		const timeSinceLastCall = lastCallTime === undefined ? 0 : time - lastCallTime;
		const timeSinceLastInvoke = time - lastInvokeTime;

		return (
			lastCallTime === undefined ||
			timeSinceLastCall >= wait ||
			timeSinceLastCall < 0 ||
			(maxWait !== undefined && timeSinceLastInvoke >= maxWait)
		);
	};

	const trailingEdge = (time: number): ReturnType<T> | undefined => {
		timeoutId = null;

		if (trailing && lastArgs) {
			return invokeFunc(time);
		}
		lastArgs = undefined;
		lastThis = undefined;
		return result;
	};

	const timerExpired = (): void => {
		const time = Date.now();
		if (shouldInvoke(time)) {
			trailingEdge(time);
			return;
		}
		// Restart timer
		const timeSinceLastCall = time - (lastCallTime || 0);
		const timeSinceLastInvoke = time - lastInvokeTime;
		const timeWaiting = wait - timeSinceLastCall;
		const maxTimeWaiting = maxWait !== undefined ? maxWait - timeSinceLastInvoke : timeWaiting;

		timeoutId = setTimeout(timerExpired, Math.min(timeWaiting, maxTimeWaiting));
	};

	const leadingEdge = (time: number): ReturnType<T> | undefined => {
		lastInvokeTime = time;
		timeoutId = setTimeout(timerExpired, wait);

		if (maxWait !== undefined) {
			maxTimeoutId = setTimeout(() => {
				if (timeoutId) {
					trailingEdge(Date.now());
				}
			}, maxWait);
		}

		return leading ? invokeFunc(time) : result;
	};

	const debounced = function (this: any, ...args: Parameters<T>): ReturnType<T> | undefined {
		const time = Date.now();
		const isInvoking = shouldInvoke(time);

		lastArgs = args;
		lastThis = this;
		lastCallTime = time;

		if (isInvoking) {
			if (timeoutId === null) {
				return leadingEdge(time);
			}
			if (maxWait !== undefined) {
				timeoutId = setTimeout(timerExpired, wait);
				return invokeFunc(time);
			}
		}

		if (timeoutId === null) {
			timeoutId = setTimeout(timerExpired, wait);
		}

		return result;
	} as T & { cancel: () => void; flush: () => ReturnType<T> | undefined };

	debounced.cancel = (): void => {
		if (timeoutId) {
			clearTimeout(timeoutId);
			timeoutId = null;
		}
		if (maxTimeoutId) {
			clearTimeout(maxTimeoutId);
			maxTimeoutId = null;
		}
		lastInvokeTime = 0;
		lastCallTime = undefined;
		lastArgs = undefined;
		lastThis = undefined;
	};

	debounced.flush = (): ReturnType<T> | undefined => {
		if (timeoutId === null) {
			return result;
		}
		return trailingEdge(Date.now());
	};

	return debounced;
}

/**
 * Throttle function with proper TypeScript generics
 * Limits execution to at most once per specified interval
 */
export function throttle<T extends (...args: any[]) => any>(
	fn: T,
	limit: number,
	options?: { leading?: boolean; trailing?: boolean }
): T & { cancel: () => void } {
	const { leading = true, trailing = true } = options || {};

	let lastCallTime: number = 0;
	let timeoutId: ReturnType<typeof setTimeout> | null = null;
	let lastArgs: Parameters<T> | undefined;
	let lastThis: any;
	let result: ReturnType<T>;

	const invokeFunc = (): ReturnType<T> => {
		const args = lastArgs!;
		const thisArg = lastThis;
		lastArgs = undefined;
		lastThis = undefined;
		lastCallTime = Date.now();
		result = fn.apply(thisArg, args);
		return result;
	};

	const throttled = function (this: any, ...args: Parameters<T>): ReturnType<T> {
		const now = Date.now();
		const remaining = limit - (now - lastCallTime);

		lastArgs = args;
		lastThis = this;

		if (remaining <= 0 || remaining > limit) {
			if (timeoutId) {
				clearTimeout(timeoutId);
				timeoutId = null;
			}
			if (leading || lastCallTime !== 0) {
				return invokeFunc();
			}
		}

		if (timeoutId === null && trailing) {
			timeoutId = setTimeout(
				() => {
					timeoutId = null;
					if (trailing && lastArgs) {
						invokeFunc();
					}
				},
				remaining > 0 ? remaining : limit
			);
		}

		return result;
	} as T & { cancel: () => void };

	throttled.cancel = (): void => {
		if (timeoutId) {
			clearTimeout(timeoutId);
			timeoutId = null;
		}
		lastCallTime = 0;
		lastArgs = undefined;
		lastThis = undefined;
	};

	return throttled;
}

// ============================================================================
// Lazy Loading Utilities
// ============================================================================

const lazyLoadCache = new Map<string, Promise<any>>();

/**
 * Lazy load a module or resource with caching
 */
export async function lazyLoad<T>(
	key: string,
	loader: () => Promise<T>,
	options?: { force?: boolean; timeout?: number }
): Promise<T> {
	const { force = false, timeout = 30000 } = options || {};

	// Return cached promise if available and not forcing refresh
	if (!force && lazyLoadCache.has(key)) {
		return lazyLoadCache.get(key) as Promise<T>;
	}

	// Create loading promise with timeout
	const loadPromise = new Promise<T>((resolve, reject) => {
		const timeoutId = setTimeout(() => {
			lazyLoadCache.delete(key);
			reject(new Error(`Lazy load timeout for: ${key}`));
		}, timeout);

		loader()
			.then((result) => {
				clearTimeout(timeoutId);
				resolve(result);
			})
			.catch((error) => {
				clearTimeout(timeoutId);
				lazyLoadCache.delete(key);
				reject(error);
			});
	});

	lazyLoadCache.set(key, loadPromise);
	return loadPromise;
}

/**
 * Clear the lazy load cache
 */
export function clearLazyLoadCache(key?: string): void {
	if (key) {
		lazyLoadCache.delete(key);
	} else {
		lazyLoadCache.clear();
	}
}

// ============================================================================
// Virtual Scroll Utilities
// ============================================================================

/**
 * Calculate indices for virtual scrolling
 */
export function calculateVirtualScrollIndices(
	scrollTop: number,
	containerHeight: number,
	itemHeight: number,
	totalItems: number,
	overscan: number = 3
): VirtualScrollIndices {
	// Calculate visible range
	const visibleCount = Math.ceil(containerHeight / itemHeight);
	const startIndex = Math.floor(scrollTop / itemHeight);

	// Apply overscan for smoother scrolling
	const overscanStart = Math.max(0, startIndex - overscan);
	const overscanEnd = Math.min(totalItems - 1, startIndex + visibleCount + overscan);

	// Calculate offset for positioning
	const offsetY = overscanStart * itemHeight;

	return {
		startIndex: overscanStart,
		endIndex: overscanEnd,
		visibleCount: overscanEnd - overscanStart + 1,
		offsetY
	};
}

/**
 * Calculate dynamic virtual scroll indices for variable height items
 */
export function calculateDynamicVirtualScrollIndices(
	scrollTop: number,
	containerHeight: number,
	itemHeights: number[],
	overscan: number = 3
): VirtualScrollIndices & { heights: number[] } {
	const totalItems = itemHeights.length;
	if (totalItems === 0) {
		return { startIndex: 0, endIndex: 0, visibleCount: 0, offsetY: 0, heights: [] };
	}

	// Build cumulative heights for binary search
	const cumulativeHeights: number[] = [0];
	for (let i = 0; i < totalItems; i++) {
		cumulativeHeights.push(cumulativeHeights[i] + itemHeights[i]);
	}

	// Binary search for start index
	let startIndex = 0;
	let low = 0;
	let high = totalItems - 1;

	while (low <= high) {
		const mid = Math.floor((low + high) / 2);
		if (cumulativeHeights[mid] <= scrollTop) {
			startIndex = mid;
			low = mid + 1;
		} else {
			high = mid - 1;
		}
	}

	// Find end index
	let endIndex = startIndex;
	const targetBottom = scrollTop + containerHeight;

	while (endIndex < totalItems && cumulativeHeights[endIndex] < targetBottom) {
		endIndex++;
	}

	// Apply overscan
	const overscanStart = Math.max(0, startIndex - overscan);
	const overscanEnd = Math.min(totalItems - 1, endIndex + overscan);

	return {
		startIndex: overscanStart,
		endIndex: overscanEnd,
		visibleCount: overscanEnd - overscanStart + 1,
		offsetY: cumulativeHeights[overscanStart],
		heights: itemHeights.slice(overscanStart, overscanEnd + 1)
	};
}

// ============================================================================
// Image Optimization Utilities
// ============================================================================

/**
 * Get optimized image URL with transformations
 */
export function getOptimizedImageUrl(
	src: string,
	width: number,
	height?: number,
	options?: ImageOptimizationOptions
): string {
	const { baseUrl, format = 'auto', quality = 80, fit = 'cover' } = options || {};

	// Handle relative URLs
	const fullSrc = src.startsWith('http') ? src : `${baseUrl || ''}${src}`;

	// If using a CDN that supports transformations (e.g., Cloudinary, imgix, Cloudflare)
	// This is a generic implementation - customize for your CDN
	const url = new URL(
		fullSrc,
		typeof window !== 'undefined' ? window.location.origin : 'http://localhost'
	);

	url.searchParams.set('w', String(width));
	if (height) {
		url.searchParams.set('h', String(height));
	}
	url.searchParams.set('q', String(quality));
	url.searchParams.set('fit', fit);

	if (format !== 'auto') {
		url.searchParams.set('fm', format);
	} else {
		// Auto-detect best format based on browser support
		if (typeof window !== 'undefined') {
			const supportsAvif = document
				.createElement('canvas')
				.toDataURL('image/avif')
				.startsWith('data:image/avif');
			const supportsWebp = document
				.createElement('canvas')
				.toDataURL('image/webp')
				.startsWith('data:image/webp');

			if (supportsAvif) {
				url.searchParams.set('fm', 'avif');
			} else if (supportsWebp) {
				url.searchParams.set('fm', 'webp');
			}
		}
	}

	return url.toString();
}

/**
 * Generate srcset for responsive images
 */
export function generateSrcSet(
	src: string,
	widths: number[] = [320, 640, 768, 1024, 1280, 1536, 1920],
	options?: ImageOptimizationOptions
): string {
	return widths
		.map((width) => {
			const url = getOptimizedImageUrl(src, width, undefined, options);
			return `${url} ${width}w`;
		})
		.join(', ');
}

/**
 * Generate sizes attribute for responsive images
 */
export function generateSizes(breakpoints: { maxWidth: number; size: string }[]): string {
	const sorted = [...breakpoints].sort((a, b) => a.maxWidth - b.maxWidth);
	const sizes = sorted.map((bp) => `(max-width: ${bp.maxWidth}px) ${bp.size}`);
	sizes.push('100vw'); // Default fallback
	return sizes.join(', ');
}

/**
 * Preload an image and return a promise
 */
export function preloadImage(src: string): Promise<HTMLImageElement> {
	return new Promise((resolve, reject) => {
		const img = new Image();

		img.onload = () => resolve(img);
		img.onerror = (_error) => reject(new Error(`Failed to preload image: ${src}`));

		// Start loading
		img.src = src;

		// Handle already cached images
		if (img.complete) {
			resolve(img);
		}
	});
}

/**
 * Preload multiple images with concurrency control
 */
export async function preloadImages(
	sources: string[],
	concurrency: number = 4
): Promise<Map<string, HTMLImageElement | Error>> {
	const results = new Map<string, HTMLImageElement | Error>();
	const queue = [...sources];

	const worker = async () => {
		while (queue.length > 0) {
			const src = queue.shift()!;
			try {
				const img = await preloadImage(src);
				results.set(src, img);
			} catch (error) {
				results.set(src, error as Error);
			}
		}
	};

	const workers = Array(Math.min(concurrency, sources.length))
		.fill(null)
		.map(() => worker());

	await Promise.all(workers);
	return results;
}

// ============================================================================
// Memory Manager Class
// ============================================================================

type CleanupCallback = (heldValue: any) => void;

// Check if FinalizationRegistry is available (not in Cloudflare Workers)
const hasFinalizationRegistry = typeof FinalizationRegistry !== 'undefined';
const hasWeakRef = typeof WeakRef !== 'undefined';

/**
 * Memory manager using WeakRef and FinalizationRegistry for automatic cleanup
 * Falls back to simple Map-based storage when these APIs are unavailable (e.g., Cloudflare Workers)
 */
export class MemoryManager<T extends object> {
	private registry: FinalizationRegistry<any> | null = null;
	private refs: Map<string, WeakRef<T>> = new Map();
	private strongRefs: Map<string, T> = new Map(); // Fallback for environments without WeakRef
	private cleanupCallbacks: Map<string, CleanupCallback> = new Map();

	constructor() {
		// FinalizationRegistry calls cleanup when objects are garbage collected
		// Only create if available (not in Cloudflare Workers)
		if (hasFinalizationRegistry) {
			this.registry = new FinalizationRegistry((heldValue: { key: string; value: any }) => {
				this.refs.delete(heldValue.key);

				// Call custom cleanup if registered
				const cleanup = this.cleanupCallbacks.get(heldValue.key);
				if (cleanup) {
					cleanup(heldValue.value);
					this.cleanupCallbacks.delete(heldValue.key);
				}
			});
		}
	}

	/**
	 * Register an object to be tracked
	 */
	register(key: string, object: T, heldValue?: any, cleanup?: CleanupCallback): void {
		// Clean up existing ref if present
		if (hasWeakRef) {
			const existingRef = this.refs.get(key);
			if (existingRef) {
				this.unregister(key);
			}

			// Create weak reference
			const ref = new WeakRef(object);
			this.refs.set(key, ref);

			// Register with finalization registry if available
			if (this.registry) {
				this.registry.register(object, { key, value: heldValue || key }, object);
			}
		} else {
			// Fallback: use strong reference (no automatic GC cleanup)
			this.strongRefs.set(key, object);
		}

		// Store cleanup callback if provided
		if (cleanup) {
			this.cleanupCallbacks.set(key, cleanup);
		}
	}

	/**
	 * Get a registered object (may return undefined if garbage collected)
	 */
	get(key: string): T | undefined {
		if (hasWeakRef) {
			const ref = this.refs.get(key);
			return ref?.deref();
		} else {
			return this.strongRefs.get(key);
		}
	}

	/**
	 * Check if an object is still alive
	 */
	has(key: string): boolean {
		if (hasWeakRef) {
			const ref = this.refs.get(key);
			return ref !== undefined && ref.deref() !== undefined;
		} else {
			return this.strongRefs.has(key);
		}
	}

	/**
	 * Manually unregister an object
	 */
	unregister(key: string): boolean {
		if (hasWeakRef) {
			const ref = this.refs.get(key);
			if (!ref) return false;

			const object = ref.deref();
			if (object && this.registry) {
				this.registry.unregister(object);
			}

			this.refs.delete(key);
		} else {
			if (!this.strongRefs.has(key)) return false;
			this.strongRefs.delete(key);
		}

		this.cleanupCallbacks.delete(key);
		return true;
	}

	/**
	 * Get all active keys
	 */
	keys(): string[] {
		if (hasWeakRef) {
			const activeKeys: string[] = [];
			for (const [key, ref] of this.refs.entries()) {
				if (ref.deref() !== undefined) {
					activeKeys.push(key);
				}
			}
			return activeKeys;
		} else {
			return Array.from(this.strongRefs.keys());
		}
	}

	/**
	 * Get count of alive references
	 */
	size(): number {
		if (hasWeakRef) {
			let count = 0;
			for (const ref of this.refs.values()) {
				if (ref.deref() !== undefined) {
					count++;
				}
			}
			return count;
		} else {
			return this.strongRefs.size;
		}
	}

	/**
	 * Clean up dead references (optional manual cleanup)
	 */
	prune(): number {
		if (!hasWeakRef) {
			return 0; // No pruning needed for strong refs
		}

		let pruned = 0;
		for (const [key, ref] of this.refs.entries()) {
			if (ref.deref() === undefined) {
				this.refs.delete(key);
				this.cleanupCallbacks.delete(key);
				pruned++;
			}
		}
		return pruned;
	}

	/**
	 * Clear all references
	 */
	clear(): void {
		if (hasWeakRef) {
			for (const [_key, ref] of this.refs.entries()) {
				const object = ref.deref();
				if (object && this.registry) {
					this.registry.unregister(object);
				}
			}
			this.refs.clear();
		} else {
			this.strongRefs.clear();
		}
		this.cleanupCallbacks.clear();
	}
}

// ============================================================================
// Scheduler Utilities
// ============================================================================

// Type for requestIdleCallback which may not be available in all environments
// Using built-in IdleRequestCallback and IdleDeadline types from lib.dom.d.ts
type IdleDeadlineCompat = IdleDeadline;

/**
 * Schedule work during idle time using requestIdleCallback
 * Falls back to setTimeout if not available
 */
export function scheduleIdleWork<T>(
	work: (deadline: IdleDeadlineCompat) => T,
	options?: { timeout?: number }
): Promise<T> {
	return new Promise((resolve, reject) => {
		const { timeout = 1000 } = options || {};

		if (typeof window !== 'undefined' && 'requestIdleCallback' in window) {
			window.requestIdleCallback!(
				(deadline) => {
					try {
						const result = work(deadline);
						resolve(result);
					} catch (error) {
						reject(error);
					}
				},
				{ timeout }
			);
		} else {
			// Fallback for browsers without requestIdleCallback
			setTimeout(() => {
				try {
					// Simulate deadline object
					const mockDeadline: IdleDeadline = {
						didTimeout: false,
						timeRemaining: () => 50 // Conservative estimate
					};
					const result = work(mockDeadline);
					resolve(result);
				} catch (error) {
					reject(error);
				}
			}, 1);
		}
	});
}

/**
 * Schedule work for next animation frame using requestAnimationFrame
 */
export function scheduleWork<T>(work: (timestamp: number) => T): Promise<T> {
	return new Promise((resolve, reject) => {
		if (typeof window !== 'undefined' && 'requestAnimationFrame' in window) {
			requestAnimationFrame((timestamp) => {
				try {
					const result = work(timestamp);
					resolve(result);
				} catch (error) {
					reject(error);
				}
			});
		} else {
			// Fallback for SSR or environments without RAF
			setTimeout(() => {
				try {
					const result = work(performance.now());
					resolve(result);
				} catch (error) {
					reject(error);
				}
			}, 16); // ~60fps
		}
	});
}

/**
 * Schedule a series of work chunks during idle time
 */
export async function scheduleChunkedWork<T, R>(
	items: T[],
	processor: (item: T, index: number) => R,
	options?: { chunkSize?: number; timeout?: number }
): Promise<R[]> {
	const { chunkSize = 10, timeout = 1000 } = options || {};
	const results: R[] = [];
	let index = 0;

	while (index < items.length) {
		await scheduleIdleWork(
			(deadline) => {
				while (index < items.length && (deadline.timeRemaining() > 0 || deadline.didTimeout)) {
					const endIndex = Math.min(index + chunkSize, items.length);
					for (let i = index; i < endIndex; i++) {
						results.push(processor(items[i], i));
					}
					index = endIndex;

					if (deadline.timeRemaining() <= 0) break;
				}
			},
			{ timeout }
		);
	}

	return results;
}

// ============================================================================
// Network-Aware Loading
// ============================================================================

/**
 * Get network information if available
 */
export function getNetworkInfo(): NetworkInfo {
	const defaultInfo: NetworkInfo = {
		effectiveType: 'unknown',
		downlink: 10,
		rtt: 100,
		saveData: false,
		online: true
	};

	if (typeof navigator === 'undefined') {
		return defaultInfo;
	}

	// Check online status
	defaultInfo.online = navigator.onLine;

	// Check for Network Information API
	const connection =
		(navigator as any).connection ||
		(navigator as any).mozConnection ||
		(navigator as any).webkitConnection;

	if (!connection) {
		return defaultInfo;
	}

	return {
		effectiveType: connection.effectiveType || 'unknown',
		downlink: connection.downlink || 10,
		rtt: connection.rtt || 100,
		saveData: connection.saveData || false,
		online: navigator.onLine
	};
}

/**
 * Determine if a feature should be loaded based on network conditions
 */
export function shouldLoadFeature(
	featureType: 'video' | 'image' | 'animation' | 'heavy' | 'light',
	options?: { respectSaveData?: boolean; minConnectionType?: NetworkInfo['effectiveType'] }
): boolean {
	const { respectSaveData = true, minConnectionType = '3g' } = options || {};
	const network = getNetworkInfo();

	// Always deny if offline
	if (!network.online) {
		return featureType === 'light';
	}

	// Respect save data mode
	if (respectSaveData && network.saveData) {
		return featureType === 'light';
	}

	// Connection type priority
	const connectionPriority: Record<NetworkInfo['effectiveType'], number> = {
		'4g': 4,
		'3g': 3,
		'2g': 2,
		'slow-2g': 1,
		unknown: 3 // Assume decent connection if unknown
	};

	const currentPriority = connectionPriority[network.effectiveType];
	const minPriority = connectionPriority[minConnectionType];

	// Feature requirements
	const featureRequirements: Record<typeof featureType, number> = {
		video: 4, // Only on 4g
		heavy: 3, // 3g or better
		animation: 3, // 3g or better
		image: 2, // 2g or better
		light: 1 // Always load
	};

	const requiredPriority = featureRequirements[featureType];

	return currentPriority >= Math.max(requiredPriority, minPriority);
}

/**
 * Get recommended quality settings based on network
 */
export function getQualityForNetwork(): {
	imageQuality: number;
	videoQuality: 'auto' | '1080p' | '720p' | '480p' | '360p';
	enableAnimations: boolean;
	prefetchEnabled: boolean;
} {
	const network = getNetworkInfo();

	if (network.saveData) {
		return {
			imageQuality: 50,
			videoQuality: '360p',
			enableAnimations: false,
			prefetchEnabled: false
		};
	}

	switch (network.effectiveType) {
		case '4g':
			return {
				imageQuality: 85,
				videoQuality: 'auto',
				enableAnimations: true,
				prefetchEnabled: true
			};
		case '3g':
			return {
				imageQuality: 70,
				videoQuality: '720p',
				enableAnimations: true,
				prefetchEnabled: false
			};
		case '2g':
			return {
				imageQuality: 50,
				videoQuality: '480p',
				enableAnimations: false,
				prefetchEnabled: false
			};
		case 'slow-2g':
			return {
				imageQuality: 40,
				videoQuality: '360p',
				enableAnimations: false,
				prefetchEnabled: false
			};
		default:
			return {
				imageQuality: 80,
				videoQuality: 'auto',
				enableAnimations: true,
				prefetchEnabled: true
			};
	}
}

// ============================================================================
// Global Instances
// ============================================================================

/**
 * Global performance monitor instance
 */
export const perfMonitor = new PerformanceMonitor();

/**
 * Global memory manager instance
 */
export const memoryManager = new MemoryManager<object>();

/**
 * Initialize performance monitoring
 * Starts FPS tracking and sets up performance observers
 */
export function initPerformanceMonitoring(): void {
	if (typeof window === 'undefined') return;

	// Start FPS monitoring
	perfMonitor.startFPSMonitoring();

	// Log initialization
	console.debug('[Performance] Monitoring initialized');
}

// ============================================================================
// Legacy Web Vitals Support (for backwards compatibility)
// ============================================================================

// Import from centralized config - single source of truth
import { API_ENDPOINTS } from '$lib/api/config';

export interface LegacyPerformanceMetric {
	name: string;
	value: number;
	rating: 'good' | 'needs-improvement' | 'poor';
	delta?: number;
	id?: string;
}

/**
 * Report legacy performance metric (for backwards compatibility)
 */
export function reportLegacyMetric(metric: LegacyPerformanceMetric): void {
	if (import.meta.env.DEV) {
		console.log(`[Performance] ${metric.name}:`, {
			value: Math.round(metric.value),
			rating: metric.rating
		});
	}

	if (import.meta.env.PROD && typeof window !== 'undefined' && typeof navigator !== 'undefined') {
		if ('gtag' in window) {
			(window as any).gtag('event', metric.name, {
				value: Math.round(metric.value),
				metric_rating: metric.rating,
				metric_delta: metric.delta ? Math.round(metric.delta) : undefined,
				metric_id: metric.id
			});
		}

		if ('sendBeacon' in navigator && typeof navigator.sendBeacon === 'function') {
			const blob = new Blob([JSON.stringify(metric)], { type: 'text/plain' });
			navigator.sendBeacon(API_ENDPOINTS.analytics.performance, blob);
		}
	}
}

export function getRating(
	value: number,
	thresholds: [number, number]
): 'good' | 'needs-improvement' | 'poor' {
	if (value <= thresholds[0]) return 'good';
	if (value <= thresholds[1]) return 'needs-improvement';
	return 'poor';
}

/**
 * Measure custom performance marks (legacy)
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

// Export types for external use
export type { IdleDeadlineCompat, CleanupCallback };

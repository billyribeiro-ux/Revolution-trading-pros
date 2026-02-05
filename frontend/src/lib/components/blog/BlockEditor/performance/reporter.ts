/**
 * Revolution Trading Pros - Performance Metrics Reporter
 * ===============================================================================
 * Batches and sends performance metrics to analytics endpoint
 *
 * Features:
 * - Batches metrics every 30 seconds
 * - Respects user privacy settings
 * - Fallback to console in development mode
 * - Automatic retry with exponential backoff
 * - Session and user context
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */

import { browser } from '$app/environment';
import { dev } from '$app/environment';
import {
	subscribeToMetrics,
	type MetricsSnapshot,
	type WebVitalMetric,
	getPercentiles
} from './metrics';

// =============================================================================
// Types
// =============================================================================

export interface ReporterConfig {
	/** Endpoint to send metrics to */
	endpoint: string;
	/** Interval in milliseconds between sends (default: 30000) */
	batchInterval: number;
	/** Maximum number of retries on failure */
	maxRetries: number;
	/** Base delay for exponential backoff (ms) */
	baseRetryDelay: number;
	/** Whether to respect privacy settings */
	respectPrivacy: boolean;
	/** Minimum metrics count before sending */
	minMetricsCount: number;
	/** Enable verbose logging */
	verbose: boolean;
}

export interface MetricsPayload {
	sessionId: string;
	userId: string | null;
	postId: string | null;
	timestamp: string;
	url: string;
	userAgent: string;
	viewport: { width: number; height: number };
	devicePixelRatio: number;
	connection: ConnectionInfo | null;
	webVitals: WebVitalsPayload;
	editorMetrics: EditorMetricsPayload;
	blockRenderStats: BlockRenderStatsPayload;
	operationStats: OperationStatsPayload;
	memoryUsage: MemoryUsagePayload | null;
	totalBlockCount: number;
	avgFps: number;
	sessionDuration: number;
}

export interface WebVitalsPayload {
	lcp: WebVitalData | null;
	fcp: WebVitalData | null;
	cls: WebVitalData | null;
	inp: WebVitalData | null;
	ttfb: WebVitalData | null;
}

export interface WebVitalData {
	value: number;
	rating: 'good' | 'needs-improvement' | 'poor';
	delta: number;
}

export interface EditorMetricsPayload {
	[key: string]: {
		count: number;
		avg: number;
		min: number;
		max: number;
		p50: number;
		p75: number;
		p95: number;
		p99: number;
	};
}

export interface BlockRenderStatsPayload {
	[blockType: string]: {
		count: number;
		avgRenderTime: number;
		p50: number;
		p75: number;
		p95: number;
	};
}

export interface OperationStatsPayload {
	[operation: string]: {
		count: number;
		successCount: number;
		failureCount: number;
		avgDuration: number;
		p95Duration: number;
	};
}

export interface MemoryUsagePayload {
	usedMB: number;
	totalMB: number;
	limitMB: number;
	usagePercentage: number;
}

export interface ConnectionInfo {
	effectiveType: string;
	downlink: number;
	rtt: number;
	saveData: boolean;
}

export interface PrivacySettings {
	analyticsEnabled: boolean;
	performanceTrackingEnabled: boolean;
}

// =============================================================================
// Constants
// =============================================================================

const DEFAULT_CONFIG: ReporterConfig = {
	endpoint: '/api/analytics/editor-performance',
	batchInterval: 30000, // 30 seconds
	maxRetries: 3,
	baseRetryDelay: 1000,
	respectPrivacy: true,
	minMetricsCount: 1,
	verbose: false
};

const STORAGE_KEYS = {
	SESSION_ID: 'rtp_perf_session_id',
	SESSION_START: 'rtp_perf_session_start',
	PENDING_METRICS: 'rtp_pending_perf_metrics'
};

// =============================================================================
// Performance Reporter Class
// =============================================================================

class PerformanceReporter {
	private config: ReporterConfig;
	private sessionId: string;
	private sessionStart: number;
	private userId: string | null = null;
	private postId: string | null = null;
	private batchInterval: ReturnType<typeof setInterval> | null = null;
	private pendingPayloads: MetricsPayload[] = [];
	private isOnline: boolean = true;
	private unsubscribeMetrics: (() => void) | null = null;
	private lastSnapshot: MetricsSnapshot | null = null;
	private retryCount: number = 0;

	constructor(config: Partial<ReporterConfig> = {}) {
		this.config = { ...DEFAULT_CONFIG, ...config };
		this.sessionId = this.getOrCreateSessionId();
		this.sessionStart = this.getSessionStart();

		if (browser) {
			this.initOnlineListener();
			this.loadPendingPayloads();
		}
	}

	/**
	 * Initialize the reporter
	 */
	initialize(): void {
		if (!browser) return;

		// Subscribe to metrics updates
		this.unsubscribeMetrics = subscribeToMetrics((snapshot) => {
			this.lastSnapshot = snapshot;
		});

		// Start batch interval
		this.startBatchInterval();

		// Send pending metrics on visibility change
		document.addEventListener('visibilitychange', this.handleVisibilityChange.bind(this));

		// Send metrics before page unload
		window.addEventListener('beforeunload', this.handleBeforeUnload.bind(this));

		this.log('Performance reporter initialized');
	}

	/**
	 * Set user context
	 */
	setUserContext(userId: string | null): void {
		this.userId = userId;
		this.log(`User context set: ${userId || 'anonymous'}`);
	}

	/**
	 * Set post context
	 */
	setPostContext(postId: string | null): void {
		this.postId = postId;
		this.log(`Post context set: ${postId || 'none'}`);
	}

	/**
	 * Get or create session ID
	 */
	private getOrCreateSessionId(): string {
		if (!browser) return crypto.randomUUID();

		let sessionId = sessionStorage.getItem(STORAGE_KEYS.SESSION_ID);
		if (!sessionId) {
			sessionId = crypto.randomUUID();
			sessionStorage.setItem(STORAGE_KEYS.SESSION_ID, sessionId);
			sessionStorage.setItem(STORAGE_KEYS.SESSION_START, Date.now().toString());
		}
		return sessionId;
	}

	/**
	 * Get session start time
	 */
	private getSessionStart(): number {
		if (!browser) return Date.now();

		const stored = sessionStorage.getItem(STORAGE_KEYS.SESSION_START);
		return stored ? parseInt(stored, 10) : Date.now();
	}

	/**
	 * Initialize online/offline listener
	 */
	private initOnlineListener(): void {
		this.isOnline = navigator.onLine;

		window.addEventListener('online', () => {
			this.isOnline = true;
			this.log('Connection restored, flushing pending metrics');
			this.flushPendingPayloads();
		});

		window.addEventListener('offline', () => {
			this.isOnline = false;
			this.log('Connection lost, metrics will be queued');
		});
	}

	/**
	 * Load pending payloads from storage
	 */
	private loadPendingPayloads(): void {
		try {
			const stored = localStorage.getItem(STORAGE_KEYS.PENDING_METRICS);
			if (stored) {
				this.pendingPayloads = JSON.parse(stored);
				this.log(`Loaded ${this.pendingPayloads.length} pending payloads`);
			}
		} catch (e) {
			console.error('[PerformanceReporter] Failed to load pending payloads:', e);
		}
	}

	/**
	 * Save pending payloads to storage
	 */
	private savePendingPayloads(): void {
		try {
			// Keep only last 50 payloads to avoid storage overflow
			const toSave = this.pendingPayloads.slice(-50);
			localStorage.setItem(STORAGE_KEYS.PENDING_METRICS, JSON.stringify(toSave));
		} catch (e) {
			console.error('[PerformanceReporter] Failed to save pending payloads:', e);
		}
	}

	/**
	 * Start batch interval
	 */
	private startBatchInterval(): void {
		if (this.batchInterval) {
			clearInterval(this.batchInterval);
		}

		this.batchInterval = setInterval(() => {
			this.sendMetrics();
		}, this.config.batchInterval);
	}

	/**
	 * Handle visibility change
	 */
	private handleVisibilityChange(): void {
		if (document.visibilityState === 'hidden') {
			this.sendMetrics(true);
		}
	}

	/**
	 * Handle before unload
	 */
	private handleBeforeUnload(): void {
		this.sendMetrics(true);
	}

	/**
	 * Check if metrics collection is allowed
	 */
	private isCollectionAllowed(): boolean {
		if (!this.config.respectPrivacy) return true;

		try {
			// Check consent settings
			const consent = localStorage.getItem('rtp_consent_state');
			if (consent) {
				const parsed = JSON.parse(consent);
				if (!parsed.analytics || !parsed.preferences) {
					return false;
				}
			}

			// Check Do Not Track
			if (navigator.doNotTrack === '1') {
				return false;
			}

			// Check Global Privacy Control
			if ((navigator as Navigator & { globalPrivacyControl?: boolean }).globalPrivacyControl) {
				return false;
			}
		} catch (e) {
			// If we can't determine, default to allowing
		}

		return true;
	}

	/**
	 * Get connection info
	 */
	private getConnectionInfo(): ConnectionInfo | null {
		const connection = (navigator as Navigator & {
			connection?: {
				effectiveType: string;
				downlink: number;
				rtt: number;
				saveData: boolean;
			}
		}).connection;

		if (!connection) return null;

		return {
			effectiveType: connection.effectiveType,
			downlink: connection.downlink,
			rtt: connection.rtt,
			saveData: connection.saveData
		};
	}

	/**
	 * Build metrics payload
	 */
	private buildPayload(): MetricsPayload | null {
		if (!this.lastSnapshot) return null;

		const snapshot = this.lastSnapshot;

		// Build Web Vitals payload
		const webVitals: WebVitalsPayload = {
			lcp: this.formatWebVital(snapshot.webVitals.LCP),
			fcp: this.formatWebVital(snapshot.webVitals.FCP),
			cls: this.formatWebVital(snapshot.webVitals.CLS),
			inp: this.formatWebVital(snapshot.webVitals.INP),
			ttfb: this.formatWebVital(snapshot.webVitals.TTFB)
		};

		// Build editor metrics payload
		const editorMetrics: EditorMetricsPayload = {};
		for (const [key, metric] of Object.entries(snapshot.editorMetrics)) {
			const percentiles = getPercentiles(key);
			editorMetrics[key] = {
				count: metric.count,
				avg: metric.count > 0 ? metric.sum / metric.count : 0,
				min: metric.min === Infinity ? 0 : metric.min,
				max: metric.max === -Infinity ? 0 : metric.max,
				p50: percentiles?.p50 ?? 0,
				p75: percentiles?.p75 ?? 0,
				p95: percentiles?.p95 ?? 0,
				p99: percentiles?.p99 ?? 0
			};
		}

		// Build block render stats
		const blockRenderStats: BlockRenderStatsPayload = {};
		const renderByType = new Map<string, number[]>();

		for (const metric of snapshot.blockRenderMetrics) {
			const existing = renderByType.get(metric.blockType) || [];
			existing.push(metric.renderTime);
			renderByType.set(metric.blockType, existing);
		}

		for (const [type, times] of renderByType) {
			const sorted = [...times].sort((a, b) => a - b);
			const len = sorted.length;
			const sum = times.reduce((a, b) => a + b, 0);

			blockRenderStats[type] = {
				count: len,
				avgRenderTime: sum / len,
				p50: sorted[Math.floor(len * 0.5)] ?? 0,
				p75: sorted[Math.floor(len * 0.75)] ?? 0,
				p95: sorted[Math.floor(len * 0.95)] ?? 0
			};
		}

		// Build operation stats
		const operationStats: OperationStatsPayload = {};
		const opsByName = new Map<string, { durations: number[]; successes: number; failures: number }>();

		for (const metric of snapshot.operationMetrics) {
			let existing = opsByName.get(metric.operation);
			if (!existing) {
				existing = { durations: [], successes: 0, failures: 0 };
				opsByName.set(metric.operation, existing);
			}
			existing.durations.push(metric.duration);
			if (metric.success) {
				existing.successes++;
			} else {
				existing.failures++;
			}
		}

		for (const [op, data] of opsByName) {
			const sorted = [...data.durations].sort((a, b) => a - b);
			const len = sorted.length;
			const sum = data.durations.reduce((a, b) => a + b, 0);

			operationStats[op] = {
				count: len,
				successCount: data.successes,
				failureCount: data.failures,
				avgDuration: sum / len,
				p95Duration: sorted[Math.floor(len * 0.95)] ?? 0
			};
		}

		// Build memory usage payload
		let memoryUsage: MemoryUsagePayload | null = null;
		if (snapshot.memoryMetrics) {
			const m = snapshot.memoryMetrics;
			memoryUsage = {
				usedMB: m.usedJSHeapSize / (1024 * 1024),
				totalMB: m.totalJSHeapSize / (1024 * 1024),
				limitMB: m.jsHeapSizeLimit / (1024 * 1024),
				usagePercentage: m.usagePercentage
			};
		}

		return {
			sessionId: this.sessionId,
			userId: this.userId,
			postId: this.postId,
			timestamp: new Date().toISOString(),
			url: window.location.href,
			userAgent: navigator.userAgent,
			viewport: {
				width: window.innerWidth,
				height: window.innerHeight
			},
			devicePixelRatio: window.devicePixelRatio,
			connection: this.getConnectionInfo(),
			webVitals,
			editorMetrics,
			blockRenderStats,
			operationStats,
			memoryUsage,
			totalBlockCount: snapshot.totalBlockCount,
			avgFps: snapshot.fps,
			sessionDuration: Date.now() - this.sessionStart
		};
	}

	/**
	 * Format a Web Vital metric
	 */
	private formatWebVital(metric: WebVitalMetric | null): WebVitalData | null {
		if (!metric) return null;

		return {
			value: metric.value,
			rating: metric.rating,
			delta: metric.delta
		};
	}

	/**
	 * Send metrics to analytics endpoint
	 */
	async sendMetrics(useBeacon: boolean = false): Promise<void> {
		// Check privacy settings
		if (!this.isCollectionAllowed()) {
			this.log('Metrics collection disabled by privacy settings');
			return;
		}

		const payload = this.buildPayload();
		if (!payload) {
			this.log('No metrics to send');
			return;
		}

		// Check minimum metrics count
		const metricsCount = Object.keys(payload.editorMetrics).length;
		if (metricsCount < this.config.minMetricsCount) {
			this.log(`Skipping send: only ${metricsCount} metrics collected`);
			return;
		}

		// In development mode, log to console instead
		if (dev) {
			this.logDevMetrics(payload);
			return;
		}

		// Queue payload
		this.pendingPayloads.push(payload);
		this.savePendingPayloads();

		// Send if online
		if (this.isOnline) {
			await this.flushPendingPayloads(useBeacon);
		}
	}

	/**
	 * Flush pending payloads
	 */
	private async flushPendingPayloads(useBeacon: boolean = false): Promise<void> {
		if (this.pendingPayloads.length === 0) return;

		const payloadsToSend = [...this.pendingPayloads];
		this.pendingPayloads = [];

		for (const payload of payloadsToSend) {
			const success = await this.sendSinglePayload(payload, useBeacon);
			if (!success) {
				// Re-queue failed payload
				this.pendingPayloads.push(payload);
			}
		}

		this.savePendingPayloads();
	}

	/**
	 * Send a single payload
	 */
	private async sendSinglePayload(
		payload: MetricsPayload,
		useBeacon: boolean = false
	): Promise<boolean> {
		const data = JSON.stringify(payload);

		// Use sendBeacon for reliability on page unload
		if (useBeacon && navigator.sendBeacon) {
			const blob = new Blob([data], { type: 'application/json' });
			const sent = navigator.sendBeacon(this.config.endpoint, blob);
			this.log(`Sent metrics via beacon: ${sent}`);
			return sent;
		}

		// Use fetch for normal sends
		try {
			const response = await fetch(this.config.endpoint, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: data,
				keepalive: true
			});

			if (response.ok) {
				this.retryCount = 0;
				this.log('Metrics sent successfully');
				return true;
			}

			throw new Error(`HTTP ${response.status}: ${response.statusText}`);
		} catch (error) {
			console.error('[PerformanceReporter] Failed to send metrics:', error);

			// Retry with exponential backoff
			if (this.retryCount < this.config.maxRetries) {
				this.retryCount++;
				const delay = this.config.baseRetryDelay * Math.pow(2, this.retryCount - 1);
				this.log(`Retrying in ${delay}ms (attempt ${this.retryCount})`);

				await new Promise((resolve) => setTimeout(resolve, delay));
				return this.sendSinglePayload(payload, useBeacon);
			}

			return false;
		}
	}

	/**
	 * Log metrics in development mode
	 */
	private logDevMetrics(payload: MetricsPayload): void {
		console.groupCollapsed(
			'%c[Performance Metrics]',
			'color: #8b5cf6; font-weight: bold;'
		);

		// Web Vitals
		console.group('Web Vitals');
		const vitals = payload.webVitals;
		if (vitals.lcp) console.log(`LCP: ${vitals.lcp.value.toFixed(0)}ms (${vitals.lcp.rating})`);
		if (vitals.fcp) console.log(`FCP: ${vitals.fcp.value.toFixed(0)}ms (${vitals.fcp.rating})`);
		if (vitals.cls) console.log(`CLS: ${vitals.cls.value.toFixed(3)} (${vitals.cls.rating})`);
		if (vitals.inp) console.log(`INP: ${vitals.inp.value.toFixed(0)}ms (${vitals.inp.rating})`);
		if (vitals.ttfb) console.log(`TTFB: ${vitals.ttfb.value.toFixed(0)}ms (${vitals.ttfb.rating})`);
		console.groupEnd();

		// Editor Metrics
		if (Object.keys(payload.editorMetrics).length > 0) {
			console.group('Editor Metrics');
			for (const [key, metric] of Object.entries(payload.editorMetrics)) {
				console.log(
					`${key}: avg=${metric.avg.toFixed(2)}ms, p95=${metric.p95.toFixed(2)}ms (n=${metric.count})`
				);
			}
			console.groupEnd();
		}

		// Block Render Stats
		if (Object.keys(payload.blockRenderStats).length > 0) {
			console.group('Block Render Stats');
			for (const [type, stats] of Object.entries(payload.blockRenderStats)) {
				console.log(
					`${type}: avg=${stats.avgRenderTime.toFixed(2)}ms, p95=${stats.p95.toFixed(2)}ms (n=${stats.count})`
				);
			}
			console.groupEnd();
		}

		// Memory & FPS
		console.group('System');
		console.log(`FPS: ${payload.avgFps}`);
		console.log(`Block Count: ${payload.totalBlockCount}`);
		if (payload.memoryUsage) {
			console.log(
				`Memory: ${payload.memoryUsage.usedMB.toFixed(1)}MB / ${payload.memoryUsage.limitMB.toFixed(0)}MB (${payload.memoryUsage.usagePercentage.toFixed(1)}%)`
			);
		}
		console.groupEnd();

		console.groupEnd();
	}

	/**
	 * Log helper
	 */
	private log(message: string): void {
		if (this.config.verbose || dev) {
			console.debug(`[PerformanceReporter] ${message}`);
		}
	}

	/**
	 * Force send metrics immediately
	 */
	async flush(): Promise<void> {
		await this.sendMetrics();
	}

	/**
	 * Update configuration
	 */
	updateConfig(config: Partial<ReporterConfig>): void {
		this.config = { ...this.config, ...config };

		// Restart batch interval if changed
		if (config.batchInterval) {
			this.startBatchInterval();
		}
	}

	/**
	 * Destroy the reporter
	 */
	destroy(): void {
		if (this.batchInterval) {
			clearInterval(this.batchInterval);
		}

		if (this.unsubscribeMetrics) {
			this.unsubscribeMetrics();
		}

		document.removeEventListener('visibilitychange', this.handleVisibilityChange.bind(this));
		window.removeEventListener('beforeunload', this.handleBeforeUnload.bind(this));

		// Final flush
		this.sendMetrics(true);

		this.log('Performance reporter destroyed');
	}
}

// =============================================================================
// Singleton Instance
// =============================================================================

let reporterInstance: PerformanceReporter | null = null;

// =============================================================================
// Exported Functions
// =============================================================================

/**
 * Initialize the performance reporter
 */
export function initializeReporter(config?: Partial<ReporterConfig>): void {
	if (!browser) return;

	if (reporterInstance) {
		reporterInstance.destroy();
	}

	reporterInstance = new PerformanceReporter(config);
	reporterInstance.initialize();
}

/**
 * Set user context for metrics
 */
export function setReporterUserContext(userId: string | null): void {
	reporterInstance?.setUserContext(userId);
}

/**
 * Set post context for metrics
 */
export function setReporterPostContext(postId: string | null): void {
	reporterInstance?.setPostContext(postId);
}

/**
 * Force flush pending metrics
 */
export async function flushMetrics(): Promise<void> {
	await reporterInstance?.flush();
}

/**
 * Update reporter configuration
 */
export function updateReporterConfig(config: Partial<ReporterConfig>): void {
	reporterInstance?.updateConfig(config);
}

/**
 * Destroy the reporter
 */
export function destroyReporter(): void {
	reporterInstance?.destroy();
	reporterInstance = null;
}

/**
 * Get current reporter instance (for testing)
 */
export function getReporterInstance(): PerformanceReporter | null {
	return reporterInstance;
}

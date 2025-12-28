/**
 * Enterprise Observability - OpenTelemetry Integration
 * Google L7+ Principal Engineer Level
 *
 * Features:
 * - Distributed tracing
 * - Metrics collection
 * - Structured logging
 * - Error tracking
 * - Performance monitoring
 */

import { browser } from '$app/environment';

// ═══════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════

export interface Span {
	name: string;
	startTime: number;
	endTime?: number;
	attributes: Record<string, any>;
	events: SpanEvent[];
	status: SpanStatus;
	traceId: string;
	spanId: string;
	parentSpanId?: string;
}

export interface SpanEvent {
	name: string;
	timestamp: number;
	attributes: Record<string, any>;
}

export interface SpanStatus {
	code: 'OK' | 'ERROR' | 'UNSET';
	message?: string;
}

export interface Metric {
	name: string;
	value: number;
	type: 'counter' | 'gauge' | 'histogram';
	labels: Record<string, string>;
	timestamp: number;
}

export interface LogEntry {
	level: 'debug' | 'info' | 'warn' | 'error' | 'fatal';
	message: string;
	timestamp: number;
	attributes: Record<string, any>;
	traceId?: string;
	spanId?: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// Telemetry Service
// ═══════════════════════════════════════════════════════════════════════════

class TelemetryService {
	private static instance: TelemetryService;
	private spans: Map<string, Span> = new Map();
	private metrics: Metric[] = [];
	private logs: LogEntry[] = [];
	private currentTraceId: string | null = null;
	private flushInterval: number | null = null;
	private config = {
		endpoint: import.meta.env['VITE_TELEMETRY_ENDPOINT'] || 'http://localhost:4318',
		serviceName: 'revolution-trading-frontend',
		serviceVersion: '2.0.0',
		environment: import.meta.env.MODE || 'development',
		sampleRate: 1.0, // 100% sampling in dev, adjust for prod
		flushIntervalMs: 10000, // 10 seconds
		maxBatchSize: 100
	};

	private constructor() {
		if (browser) {
			this.initialize();
		}
	}

	static getInstance(): TelemetryService {
		if (!TelemetryService.instance) {
			TelemetryService.instance = new TelemetryService();
		}
		return TelemetryService.instance;
	}

	private initialize(): void {
		// Start periodic flush
		this.flushInterval = window.setInterval(() => {
			this.flush();
		}, this.config.flushIntervalMs);

		// Flush on page unload
		window.addEventListener('beforeunload', () => {
			this.flush();
		});

		// Capture unhandled errors
		window.addEventListener('error', (event) => {
			this.recordError(event.error, {
				message: event.message,
				filename: event.filename,
				lineno: event.lineno,
				colno: event.colno
			});
		});

		// Capture unhandled promise rejections
		window.addEventListener('unhandledrejection', (event) => {
			this.recordError(event.reason, {
				type: 'unhandledrejection',
				promise: event.promise
			});
		});

		console.debug('[Telemetry] Initialized', this.config);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Tracing
	// ═══════════════════════════════════════════════════════════════════════════

	startSpan(name: string, attributes: Record<string, any> = {}): string {
		const spanId = this.generateId();
		const traceId = this.currentTraceId || this.generateId();
		this.currentTraceId = traceId;

		const span: Span = {
			name,
			startTime: performance.now(),
			attributes: {
				...attributes,
				'service.name': this.config.serviceName,
				'service.version': this.config.serviceVersion,
				'deployment.environment': this.config.environment
			},
			events: [],
			status: { code: 'UNSET' },
			traceId,
			spanId
		};

		this.spans.set(spanId, span);
		return spanId;
	}

	endSpan(spanId: string, status: SpanStatus = { code: 'OK' }): void {
		const span = this.spans.get(spanId);
		if (!span) {
			console.warn('[Telemetry] Span not found:', spanId);
			return;
		}

		span.endTime = performance.now();
		span.status = status;

		// Calculate duration
		const duration = span.endTime - span.startTime;
		span.attributes['duration_ms'] = duration;

		// Record as metric
		this.recordMetric('span_duration_ms', duration, 'histogram', {
			span_name: span.name,
			status: status.code
		});

		console.debug('[Telemetry] Span ended:', {
			name: span.name,
			duration: `${duration.toFixed(2)}ms`,
			status: status.code
		});
	}

	addSpanEvent(spanId: string, name: string, attributes: Record<string, any> = {}): void {
		const span = this.spans.get(spanId);
		if (!span) return;

		span.events.push({
			name,
			timestamp: performance.now(),
			attributes
		});
	}

	setSpanAttribute(spanId: string, key: string, value: any): void {
		const span = this.spans.get(spanId);
		if (!span) return;

		span.attributes[key] = value;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Metrics
	// ═══════════════════════════════════════════════════════════════════════════

	recordMetric(
		name: string,
		value: number,
		type: 'counter' | 'gauge' | 'histogram' = 'counter',
		labels: Record<string, string> = {}
	): void {
		this.metrics.push({
			name,
			value,
			type,
			labels: {
				...labels,
				service: this.config.serviceName,
				environment: this.config.environment
			},
			timestamp: Date.now()
		});

		// Auto-flush if batch is full
		if (this.metrics.length >= this.config.maxBatchSize) {
			this.flush();
		}
	}

	incrementCounter(name: string, labels: Record<string, string> = {}): void {
		this.recordMetric(name, 1, 'counter', labels);
	}

	setGauge(name: string, value: number, labels: Record<string, string> = {}): void {
		this.recordMetric(name, value, 'gauge', labels);
	}

	recordHistogram(name: string, value: number, labels: Record<string, string> = {}): void {
		this.recordMetric(name, value, 'histogram', labels);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Logging
	// ═══════════════════════════════════════════════════════════════════════════

	log(level: LogEntry['level'], message: string, attributes: Record<string, any> = {}): void {
		const entry: LogEntry = {
			level,
			message,
			timestamp: Date.now(),
			...(this.currentTraceId && { traceId: this.currentTraceId }),
			attributes: {
				...attributes,
				service: this.config.serviceName,
				environment: this.config.environment,
				userAgent: navigator.userAgent,
				url: window.location.href
			}
		};

		this.logs.push(entry);

		// Console output
		const consoleMethod = level === 'fatal' ? 'error' : level;
		console[consoleMethod](`[${level.toUpperCase()}]`, message, attributes);

		// Auto-flush on error/fatal
		if (level === 'error' || level === 'fatal') {
			this.flush();
		}
	}

	debug(message: string, attributes?: Record<string, any>): void {
		this.log('debug', message, attributes);
	}

	info(message: string, attributes?: Record<string, any>): void {
		this.log('info', message, attributes);
	}

	warn(message: string, attributes?: Record<string, any>): void {
		this.log('warn', message, attributes);
	}

	error(message: string, attributes?: Record<string, any>): void {
		this.log('error', message, attributes);
	}

	fatal(message: string, attributes?: Record<string, any>): void {
		this.log('fatal', message, attributes);
	}

	recordError(error: Error | any, context: Record<string, any> = {}): void {
		const errorInfo = {
			name: error?.name || 'Error',
			message: error?.message || String(error),
			stack: error?.stack,
			...context
		};

		this.error('Unhandled error', errorInfo);

		// Record error metric
		this.incrementCounter('errors_total', {
			error_type: errorInfo.name
		});
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Performance Monitoring
	// ═══════════════════════════════════════════════════════════════════════════

	recordPageLoad(): void {
		if (!browser || !window.performance) return;

		const navigation = performance.getEntriesByType('navigation')[0] as PerformanceNavigationTiming;
		if (!navigation) return;

		const metrics = {
			dns_lookup: navigation.domainLookupEnd - navigation.domainLookupStart,
			tcp_connection: navigation.connectEnd - navigation.connectStart,
			tls_negotiation:
				navigation.secureConnectionStart > 0
					? navigation.connectEnd - navigation.secureConnectionStart
					: 0,
			request_time: navigation.responseStart - navigation.requestStart,
			response_time: navigation.responseEnd - navigation.responseStart,
			dom_processing: navigation.domComplete - navigation.domInteractive,
			dom_content_loaded:
				navigation.domContentLoadedEventEnd - navigation.domContentLoadedEventStart,
			load_complete: navigation.loadEventEnd - navigation.loadEventStart,
			total_time: navigation.loadEventEnd - navigation.fetchStart
		};

		Object.entries(metrics).forEach(([name, value]) => {
			this.recordHistogram(`page_load_${name}_ms`, value, {
				page: window.location.pathname
			});
		});

		this.info('Page load metrics recorded', metrics);
	}

	recordWebVitals(): void {
		if (!browser) return;

		// Largest Contentful Paint (LCP)
		new PerformanceObserver((list) => {
			const entries = list.getEntries();
			const lastEntry = entries[entries.length - 1] as any;
			this.recordHistogram('web_vitals_lcp_ms', lastEntry.renderTime || lastEntry.loadTime, {
				page: window.location.pathname
			});
		}).observe({ entryTypes: ['largest-contentful-paint'] });

		// First Input Delay (FID)
		new PerformanceObserver((list) => {
			const entries = list.getEntries();
			entries.forEach((entry: any) => {
				this.recordHistogram('web_vitals_fid_ms', entry.processingStart - entry.startTime, {
					page: window.location.pathname
				});
			});
		}).observe({ entryTypes: ['first-input'] });

		// Cumulative Layout Shift (CLS)
		let clsValue = 0;
		new PerformanceObserver((list) => {
			for (const entry of list.getEntries() as any[]) {
				if (!entry.hadRecentInput) {
					clsValue += entry.value;
				}
			}
			this.recordHistogram('web_vitals_cls', clsValue, {
				page: window.location.pathname
			});
		}).observe({ entryTypes: ['layout-shift'] });
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Data Export
	// ═══════════════════════════════════════════════════════════════════════════

	private async flush(): Promise<void> {
		if (!browser) return;

		const spans = Array.from(this.spans.values()).filter((s) => s.endTime);
		const metrics = [...this.metrics];
		const logs = [...this.logs];

		if (spans.length === 0 && metrics.length === 0 && logs.length === 0) {
			return;
		}

		try {
			// Send to telemetry backend
			await this.sendTelemetry({
				spans,
				metrics,
				logs,
				resource: {
					'service.name': this.config.serviceName,
					'service.version': this.config.serviceVersion,
					'deployment.environment': this.config.environment
				}
			});

			// Clear sent data
			spans.forEach((span) => this.spans.delete(span.spanId));
			this.metrics = [];
			this.logs = [];

			console.debug('[Telemetry] Flushed', {
				spans: spans.length,
				metrics: metrics.length,
				logs: logs.length
			});
		} catch (error) {
			console.error('[Telemetry] Failed to flush:', error);
		}
	}

	private async sendTelemetry(data: any): Promise<void> {
		// In production, send to OpenTelemetry collector
		// For now, log to console in development
		if (this.config.environment === 'development') {
			console.debug('[Telemetry] Data:', data);
			return;
		}

		try {
			await fetch(`${this.config.endpoint}/v1/traces`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(data),
				keepalive: true
			});
		} catch (error) {
			// Silently fail to avoid affecting user experience
			console.error('[Telemetry] Send failed:', error);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Utilities
	// ═══════════════════════════════════════════════════════════════════════════

	private generateId(): string {
		return `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
	}

	getCurrentTraceId(): string | null {
		return this.currentTraceId;
	}

	setTraceId(traceId: string): void {
		this.currentTraceId = traceId;
	}

	clearTraceId(): void {
		this.currentTraceId = null;
	}

	destroy(): void {
		if (this.flushInterval) {
			clearInterval(this.flushInterval);
		}
		this.flush();
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Convenience Functions
// ═══════════════════════════════════════════════════════════════════════════

const telemetry = TelemetryService.getInstance();

export const startSpan = (name: string, attributes?: Record<string, any>) =>
	telemetry.startSpan(name, attributes);

export const endSpan = (spanId: string, status?: SpanStatus) => telemetry.endSpan(spanId, status);

export const addSpanEvent = (spanId: string, name: string, attributes?: Record<string, any>) =>
	telemetry.addSpanEvent(spanId, name, attributes);

export const recordMetric = (
	name: string,
	value: number,
	type?: 'counter' | 'gauge' | 'histogram',
	labels?: Record<string, string>
) => telemetry.recordMetric(name, value, type, labels);

export const incrementCounter = (name: string, labels?: Record<string, string>) =>
	telemetry.incrementCounter(name, labels);

export const setGauge = (name: string, value: number, labels?: Record<string, string>) =>
	telemetry.setGauge(name, value, labels);

export const recordHistogram = (name: string, value: number, labels?: Record<string, string>) =>
	telemetry.recordHistogram(name, value, labels);

export const debug = (message: string, attributes?: Record<string, any>) =>
	telemetry.debug(message, attributes);

export const info = (message: string, attributes?: Record<string, any>) =>
	telemetry.info(message, attributes);

export const warn = (message: string, attributes?: Record<string, any>) =>
	telemetry.warn(message, attributes);

export const error = (message: string, attributes?: Record<string, any>) =>
	telemetry.error(message, attributes);

export const fatal = (message: string, attributes?: Record<string, any>) =>
	telemetry.fatal(message, attributes);

export const recordError = (err: Error | any, context?: Record<string, any>) =>
	telemetry.recordError(err, context);

export const recordPageLoad = () => telemetry.recordPageLoad();

export const recordWebVitals = () => telemetry.recordWebVitals();

// ═══════════════════════════════════════════════════════════════════════════
// Decorators & Higher-Order Functions
// ═══════════════════════════════════════════════════════════════════════════

export function traced<T extends (...args: any[]) => any>(name: string, fn: T): T {
	return ((...args: any[]) => {
		const spanId = startSpan(name, {
			'function.name': fn.name,
			'function.args': JSON.stringify(args)
		});

		try {
			const result = fn(...args);

			// Handle promises
			if (result instanceof Promise) {
				return result
					.then((value) => {
						endSpan(spanId, { code: 'OK' });
						return value;
					})
					.catch((err) => {
						endSpan(spanId, {
							code: 'ERROR',
							message: err.message
						});
						throw err;
					});
			}

			endSpan(spanId, { code: 'OK' });
			return result;
		} catch (err: any) {
			endSpan(spanId, {
				code: 'ERROR',
				message: err.message
			});
			throw err;
		}
	}) as T;
}

export default telemetry;

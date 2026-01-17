/**
 * Request Tracing - Apple ICT9+ Observability
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Implements distributed tracing with:
 * - Unique trace IDs for request correlation
 * - Span IDs for nested request tracking
 * - Timing data for performance monitoring
 * - Context propagation across async boundaries
 *
 * W3C Trace Context compliant header propagation
 */

import { browser } from '$app/environment';
import type { RequestContext, RequestPriority, RequestMetrics } from './types';

// ═══════════════════════════════════════════════════════════════════════════════
// Constants
// ═══════════════════════════════════════════════════════════════════════════════

const TRACE_ID_LENGTH = 32;
const SPAN_ID_LENGTH = 16;
const TRACE_HEADER = 'X-Trace-ID';
const SPAN_HEADER = 'X-Span-ID';
const PARENT_SPAN_HEADER = 'X-Parent-Span-ID';
const REQUEST_ID_HEADER = 'X-Request-ID';

// ═══════════════════════════════════════════════════════════════════════════════
// ID Generation
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Generate a cryptographically secure random hex string
 * Uses crypto.getRandomValues when available, falls back to Math.random
 */
function generateSecureHex(length: number): string {
	if (browser && window.crypto && window.crypto.getRandomValues) {
		const bytes = new Uint8Array(length / 2);
		window.crypto.getRandomValues(bytes);
		return Array.from(bytes)
			.map((b) => b.toString(16).padStart(2, '0'))
			.join('');
	}

	// Fallback for SSR or older browsers
	let result = '';
	for (let i = 0; i < length; i++) {
		result += Math.floor(Math.random() * 16).toString(16);
	}
	return result;
}

/**
 * Generate a unique trace ID (32 hex characters)
 * Format: <timestamp-hex><random-hex>
 */
export function generateTraceId(): string {
	const timestamp = Date.now().toString(16).padStart(12, '0');
	const random = generateSecureHex(TRACE_ID_LENGTH - 12);
	return `${timestamp}${random}`;
}

/**
 * Generate a unique span ID (16 hex characters)
 */
export function generateSpanId(): string {
	return generateSecureHex(SPAN_ID_LENGTH);
}

/**
 * Generate a human-readable request ID
 * Format: req_<timestamp>_<random>
 */
export function generateRequestId(): string {
	const timestamp = Date.now().toString(36);
	const random = generateSecureHex(8);
	return `req_${timestamp}_${random}`;
}

// ═══════════════════════════════════════════════════════════════════════════════
// Context Management
// ═══════════════════════════════════════════════════════════════════════════════

/** Current active trace context for nested requests */
let activeContext: RequestContext | null = null;

/**
 * Create a new request context
 */
export function createRequestContext(
	options: {
		priority?: RequestPriority;
		parentContext?: RequestContext;
		userId?: number;
		sessionId?: string;
		metadata?: Record<string, unknown>;
	} = {}
): RequestContext {
	const parentContext = options.parentContext ?? activeContext;

	const parentSpanId = parentContext?.spanId;
	const userId = options.userId ?? parentContext?.userId;
	const sessionId = options.sessionId ?? parentContext?.sessionId;

	return {
		traceId: parentContext?.traceId ?? generateTraceId(),
		spanId: generateSpanId(),
		...(parentSpanId !== undefined && { parentSpanId }),
		startTime: performance.now(),
		priority: options.priority ?? 'normal',
		isRetry: false,
		retryAttempt: 0,
		...(userId !== undefined && { userId }),
		...(sessionId !== undefined && { sessionId }),
		metadata: {
			...parentContext?.metadata,
			...options.metadata
		}
	};
}

/**
 * Create a child context for nested requests
 */
export function createChildContext(parentContext: RequestContext): RequestContext {
	return {
		...parentContext,
		spanId: generateSpanId(),
		parentSpanId: parentContext.spanId,
		startTime: performance.now(),
		isRetry: false,
		retryAttempt: 0,
		metadata: { ...parentContext.metadata }
	};
}

/**
 * Create a retry context
 */
export function createRetryContext(
	originalContext: RequestContext,
	attempt: number
): RequestContext {
	return {
		...originalContext,
		spanId: generateSpanId(),
		parentSpanId: originalContext.spanId,
		startTime: performance.now(),
		isRetry: true,
		retryAttempt: attempt,
		metadata: {
			...originalContext.metadata,
			originalSpanId: originalContext.spanId
		}
	};
}

/**
 * Set the active context for nested request correlation
 */
export function setActiveContext(context: RequestContext | null): void {
	activeContext = context;
}

/**
 * Get the current active context
 */
export function getActiveContext(): RequestContext | null {
	return activeContext;
}

/**
 * Run a function with a specific context as active
 */
export async function withContext<T>(context: RequestContext, fn: () => Promise<T>): Promise<T> {
	const previousContext = activeContext;
	activeContext = context;
	try {
		return await fn();
	} finally {
		activeContext = previousContext;
	}
}

// ═══════════════════════════════════════════════════════════════════════════════
// Header Propagation
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Get trace headers for outgoing requests
 */
export function getTraceHeaders(context: RequestContext): Record<string, string> {
	const headers: Record<string, string> = {
		[TRACE_HEADER]: context.traceId,
		[SPAN_HEADER]: context.spanId,
		[REQUEST_ID_HEADER]: generateRequestId()
	};

	if (context.parentSpanId) {
		headers[PARENT_SPAN_HEADER] = context.parentSpanId;
	}

	// W3C Trace Context format for interoperability
	// traceparent: version-traceId-spanId-flags
	headers['traceparent'] = `00-${context.traceId}-${context.spanId}-01`;

	return headers;
}

/**
 * Parse trace context from incoming headers
 */
export function parseTraceHeaders(headers: Record<string, string>): Partial<RequestContext> | null {
	const traceId = headers[TRACE_HEADER.toLowerCase()] || headers[TRACE_HEADER];
	const spanId = headers[SPAN_HEADER.toLowerCase()] || headers[SPAN_HEADER];
	const parentSpanId = headers[PARENT_SPAN_HEADER.toLowerCase()] || headers[PARENT_SPAN_HEADER];

	// Try W3C traceparent header
	const traceparent = headers['traceparent'];
	if (traceparent) {
		const parts = traceparent.split('-');
		if (parts.length >= 3 && parts[1] && parts[2]) {
			return {
				traceId: parts[1],
				spanId: parts[2],
				...(parentSpanId !== undefined && { parentSpanId })
			};
		}
	}

	if (traceId) {
		return {
			traceId,
			...(spanId !== undefined && { spanId }),
			...(parentSpanId !== undefined && { parentSpanId })
		};
	}

	return null;
}

// ═══════════════════════════════════════════════════════════════════════════════
// Timing & Metrics
// ═══════════════════════════════════════════════════════════════════════════════

/** Response time samples for percentile calculation */
const responseTimeSamples: number[] = [];
const MAX_SAMPLES = 1000;

/**
 * Calculate request duration from context
 */
export function getRequestDuration(context: RequestContext): number {
	return performance.now() - context.startTime;
}

/**
 * Record a response time sample
 */
export function recordResponseTime(duration: number): void {
	responseTimeSamples.push(duration);

	// Keep samples bounded
	if (responseTimeSamples.length > MAX_SAMPLES) {
		responseTimeSamples.shift();
	}
}

/**
 * Calculate percentile from samples
 */
function calculatePercentile(percentile: number): number {
	if (responseTimeSamples.length === 0) return 0;

	const sorted = [...responseTimeSamples].sort((a, b) => a - b);
	const index = Math.ceil((percentile / 100) * sorted.length) - 1;
	return sorted[Math.max(0, index)] ?? 0;
}

/**
 * Get current metrics
 */
export function getTracingMetrics(): Pick<
	RequestMetrics,
	'avgResponseTime' | 'p50ResponseTime' | 'p95ResponseTime' | 'p99ResponseTime'
> {
	const samples = responseTimeSamples;
	const avg = samples.length > 0 ? samples.reduce((a, b) => a + b, 0) / samples.length : 0;

	return {
		avgResponseTime: Math.round(avg),
		p50ResponseTime: Math.round(calculatePercentile(50)),
		p95ResponseTime: Math.round(calculatePercentile(95)),
		p99ResponseTime: Math.round(calculatePercentile(99))
	};
}

// ═══════════════════════════════════════════════════════════════════════════════
// Logging
// ═══════════════════════════════════════════════════════════════════════════════

export type LogLevel = 'debug' | 'info' | 'warn' | 'error';

interface LogEntry {
	level: LogLevel;
	message: string;
	context?: RequestContext;
	data?: Record<string, unknown>;
	timestamp: string;
}

/** Log buffer for batched shipping */
const logBuffer: LogEntry[] = [];
const MAX_LOG_BUFFER = 100;

/**
 * Create a structured log entry
 */
export function createLogEntry(
	level: LogLevel,
	message: string,
	context?: RequestContext,
	data?: Record<string, unknown>
): LogEntry {
	return {
		level,
		message,
		...(context && {
			context: {
				...context,
				// Sanitize metadata to prevent sensitive data leakage
				metadata: sanitizeLogData(context.metadata)
			}
		}),
		...(data && { data: sanitizeLogData(data) }),
		timestamp: new Date().toISOString()
	};
}

/**
 * Sanitize data for logging (remove sensitive fields)
 */
function sanitizeLogData(data: Record<string, unknown>): Record<string, unknown> {
	const sensitiveKeys = [
		'password',
		'token',
		'secret',
		'key',
		'auth',
		'credential',
		'ssn',
		'credit',
		'card'
	];

	const sanitized: Record<string, unknown> = {};

	for (const [key, value] of Object.entries(data)) {
		const lowerKey = key.toLowerCase();
		const isSensitive = sensitiveKeys.some((k) => lowerKey.includes(k));

		if (isSensitive) {
			sanitized[key] = '[REDACTED]';
		} else if (typeof value === 'object' && value !== null) {
			sanitized[key] = sanitizeLogData(value as Record<string, unknown>);
		} else {
			sanitized[key] = value;
		}
	}

	return sanitized;
}

/**
 * Log with context
 */
export function log(
	level: LogLevel,
	message: string,
	context?: RequestContext,
	data?: Record<string, unknown>
): void {
	const entry = createLogEntry(level, message, context, data);

	// Console output in development
	if (import.meta.env.DEV) {
		const prefix = context ? `[${context.traceId.slice(0, 8)}]` : '';
		const consoleMethod = level === 'error' ? 'error' : level === 'warn' ? 'warn' : 'debug';
		console[consoleMethod](`${prefix} ${message}`, entry);
	}

	// Buffer for batched shipping in production
	logBuffer.push(entry);
	if (logBuffer.length > MAX_LOG_BUFFER) {
		logBuffer.shift();
	}
}

/**
 * Get and clear log buffer
 */
export function flushLogs(): LogEntry[] {
	const logs = [...logBuffer];
	logBuffer.length = 0;
	return logs;
}

// ═══════════════════════════════════════════════════════════════════════════════
// Trace Storage (for debugging)
// ═══════════════════════════════════════════════════════════════════════════════

export interface TraceRecord {
	context: RequestContext;
	endpoint: string;
	method: string;
	status?: number;
	duration?: number;
	error?: string;
	timestamp: string;
}

/** Recent traces for debugging */
const recentTraces: TraceRecord[] = [];
const MAX_TRACES = 50;

/**
 * Record a trace
 */
export function recordTrace(
	context: RequestContext,
	endpoint: string,
	method: string,
	status?: number,
	error?: string
): void {
	const trace: TraceRecord = {
		context,
		endpoint,
		method,
		...(status !== undefined && { status }),
		duration: getRequestDuration(context),
		...(error !== undefined && { error }),
		timestamp: new Date().toISOString()
	};

	recentTraces.unshift(trace);
	if (recentTraces.length > MAX_TRACES) {
		recentTraces.pop();
	}
}

/**
 * Get recent traces (for debugging)
 */
export function getRecentTraces(): TraceRecord[] {
	return [...recentTraces];
}

/**
 * Find traces by trace ID
 */
export function findTracesByTraceId(traceId: string): TraceRecord[] {
	return recentTraces.filter((t) => t.context.traceId === traceId);
}

/**
 * Clear all traces
 */
export function clearTraces(): void {
	recentTraces.length = 0;
}

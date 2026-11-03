/**
 * Enterprise API Types - Apple ICT9+ Type Safety
 * ═══════════════════════════════════════════════════════════════════════════════
 */

// ═══════════════════════════════════════════════════════════════════════════════
// Request Types
// ═══════════════════════════════════════════════════════════════════════════════

export type HttpMethod = 'GET' | 'POST' | 'PUT' | 'PATCH' | 'DELETE' | 'HEAD' | 'OPTIONS';

export type RequestPriority = 'critical' | 'high' | 'normal' | 'low' | 'background';

export interface RequestContext {
	/** Unique trace ID for request correlation */
	traceId: string;
	/** Span ID for distributed tracing */
	spanId: string;
	/** Parent span ID if this is a child request */
	parentSpanId?: string;
	/** Request start timestamp */
	startTime: number;
	/** Request priority level */
	priority: RequestPriority;
	/** Whether this is a retry attempt */
	isRetry: boolean;
	/** Current retry attempt number */
	retryAttempt: number;
	/** User ID if authenticated */
	userId?: number;
	/** Session ID for correlation */
	sessionId?: string;
	/** Custom metadata */
	metadata: Record<string, unknown>;
}

export interface EnterpriseRequestConfig {
	/** HTTP method */
	method?: HttpMethod;
	/** Request headers */
	headers?: Record<string, string>;
	/** Request body */
	body?: unknown;
	/** URL query parameters */
	params?: Record<string, string | number | boolean | null | undefined>;
	/** Request timeout in milliseconds */
	timeout?: number;
	/** Request priority */
	priority?: RequestPriority;
	/** Retry configuration */
	retry?: RetryConfig;
	/** Cache configuration */
	cache?: CacheConfig;
	/** CSRF configuration */
	csrf?: CsrfConfig;
	/** Whether to include credentials */
	credentials?: RequestCredentials;
	/** Whether to skip token refresh on 401 */
	skipTokenRefresh?: boolean;
	/** Custom transform function for response */
	transform?: <T>(data: unknown) => T;
	/** Signal for request cancellation */
	signal?: AbortSignal;
	/** Custom metadata for tracing */
	metadata?: Record<string, unknown>;
}

// ═══════════════════════════════════════════════════════════════════════════════
// Retry Types
// ═══════════════════════════════════════════════════════════════════════════════

export type BackoffStrategy = 'linear' | 'exponential' | 'fibonacci' | 'decorrelated-jitter';

export interface RetryConfig {
	/** Maximum number of retry attempts */
	maxAttempts?: number;
	/** Base delay between retries in milliseconds */
	baseDelay?: number;
	/** Maximum delay between retries in milliseconds */
	maxDelay?: number;
	/** Backoff strategy */
	backoff?: BackoffStrategy;
	/** HTTP status codes that should trigger a retry */
	retryableStatuses?: number[];
	/** Custom condition to determine if request should be retried */
	retryCondition?: (error: EnterpriseApiError, attempt: number) => boolean;
	/** Callback invoked before each retry */
	onRetry?: (error: EnterpriseApiError, attempt: number) => void;
}

// ═══════════════════════════════════════════════════════════════════════════════
// Cache Types
// ═══════════════════════════════════════════════════════════════════════════════

export type CacheStrategy =
	| 'cache-first'
	| 'network-first'
	| 'stale-while-revalidate'
	| 'cache-only'
	| 'network-only';

export interface CacheConfig {
	/** Cache strategy */
	strategy?: CacheStrategy;
	/** Cache TTL in milliseconds */
	ttl?: number;
	/** Custom cache key */
	key?: string;
	/** Cache key patterns to invalidate on mutation */
	invalidate?: string[];
	/** Whether to persist cache to storage */
	persist?: boolean;
	/** Tags for grouped invalidation */
	tags?: string[];
}

export interface CacheEntry<T = unknown> {
	/** Cached data */
	data: T;
	/** Cache timestamp */
	timestamp: number;
	/** Cache expiry timestamp */
	expiry: number;
	/** ETag for conditional requests */
	etag?: string;
	/** Cache tags */
	tags: string[];
}

// ═══════════════════════════════════════════════════════════════════════════════
// CSRF Types
// ═══════════════════════════════════════════════════════════════════════════════

export interface CsrfConfig {
	/** Whether CSRF protection is enabled */
	enabled?: boolean;
	/** CSRF token header name */
	headerName?: string;
	/** Cookie name for double-submit pattern */
	cookieName?: string;
}

// ═══════════════════════════════════════════════════════════════════════════════
// Response Types
// ═══════════════════════════════════════════════════════════════════════════════

export interface EnterpriseResponse<T = unknown> {
	/** Response data */
	data: T;
	/** HTTP status code */
	status: number;
	/** HTTP status text */
	statusText: string;
	/** Response headers */
	headers: Record<string, string>;
	/** Request context for tracing */
	context: RequestContext;
	/** Response metadata */
	meta?: ResponseMeta;
}

export interface ResponseMeta {
	/** Total count for paginated responses */
	total?: number;
	/** Current page */
	page?: number;
	/** Items per page */
	perPage?: number;
	/** Total pages */
	totalPages?: number;
	/** Whether response was served from cache */
	cached?: boolean;
	/** Server execution time */
	executionTime?: number;
	/** API version */
	version?: string;
}

// ═══════════════════════════════════════════════════════════════════════════════
// Error Types
// ═══════════════════════════════════════════════════════════════════════════════

export type ErrorSeverity = 'critical' | 'error' | 'warning' | 'info';

export type ErrorCategory =
	| 'authentication'
	| 'authorization'
	| 'validation'
	| 'network'
	| 'timeout'
	| 'rate_limit'
	| 'server'
	| 'client'
	| 'unknown';

export interface EnterpriseApiError extends Error {
	/** Error code for programmatic handling */
	code: string;
	/** HTTP status code */
	status: number;
	/** Error category for routing */
	category: ErrorCategory;
	/** Error severity for alerting */
	severity: ErrorSeverity;
	/** Validation errors by field */
	validationErrors?: Record<string, string[]>;
	/** Request context for debugging */
	context?: RequestContext;
	/** Retry-After header value in seconds */
	retryAfter?: number;
	/** Whether error is retryable */
	isRetryable: boolean;
	/** Original error if wrapped */
	cause?: Error;
	/** Timestamp of error */
	timestamp: string;
}

// ═══════════════════════════════════════════════════════════════════════════════
// Interceptor Types
// ═══════════════════════════════════════════════════════════════════════════════

export type RequestInterceptor = (
	config: EnterpriseRequestConfig,
	context: RequestContext
) => EnterpriseRequestConfig | Promise<EnterpriseRequestConfig>;

export type ResponseInterceptor = <T>(
	response: EnterpriseResponse<T>,
	context: RequestContext
) => EnterpriseResponse<T> | Promise<EnterpriseResponse<T>>;

export type ErrorInterceptor = (
	error: EnterpriseApiError,
	context: RequestContext
) => EnterpriseApiError | Promise<EnterpriseApiError>;

export interface InterceptorChain {
	request: RequestInterceptor[];
	response: ResponseInterceptor[];
	error: ErrorInterceptor[];
}

// ═══════════════════════════════════════════════════════════════════════════════
// Metrics Types
// ═══════════════════════════════════════════════════════════════════════════════

export interface RequestMetrics {
	/** Total requests made */
	totalRequests: number;
	/** Successful requests (2xx) */
	successfulRequests: number;
	/** Failed requests */
	failedRequests: number;
	/** Requests currently in flight */
	activeRequests: number;
	/** Average response time in ms */
	avgResponseTime: number;
	/** P50 response time */
	p50ResponseTime: number;
	/** P95 response time */
	p95ResponseTime: number;
	/** P99 response time */
	p99ResponseTime: number;
	/** Cache hit rate (0-1) */
	cacheHitRate: number;
	/** Error rate (0-1) */
	errorRate: number;
	/** Retry rate (0-1) */
	retryRate: number;
	/** Rate limit hits */
	rateLimitHits: number;
}

// ═══════════════════════════════════════════════════════════════════════════════
// Circuit Breaker Types
// ═══════════════════════════════════════════════════════════════════════════════

export type CircuitState = 'closed' | 'open' | 'half-open';

export interface CircuitBreakerConfig {
	/** Failure threshold before opening circuit */
	failureThreshold: number;
	/** Success threshold before closing circuit from half-open */
	successThreshold: number;
	/** Timeout before attempting half-open state (ms) */
	timeout: number;
	/** Volume threshold - minimum requests before circuit can trip */
	volumeThreshold: number;
	/** Time window for failure rate calculation (ms) */
	windowSize: number;
}

export interface CircuitBreakerState {
	state: CircuitState;
	failures: number;
	successes: number;
	lastFailureTime?: number;
	lastStateChange: number;
}

// ═══════════════════════════════════════════════════════════════════════════════
// Event Types
// ═══════════════════════════════════════════════════════════════════════════════

export type ApiEventType =
	| 'request:start'
	| 'request:success'
	| 'request:error'
	| 'request:retry'
	| 'token:refresh'
	| 'token:expired'
	| 'circuit:open'
	| 'circuit:close'
	| 'circuit:half-open'
	| 'cache:hit'
	| 'cache:miss'
	| 'rate:limited';

export interface ApiEvent<T = unknown> {
	type: ApiEventType;
	timestamp: number;
	context?: RequestContext;
	data?: T;
}

export type ApiEventHandler<T = unknown> = (event: ApiEvent<T>) => void;

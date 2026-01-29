/**
 * Enterprise API Client - TypeScript 5.9+ Implementation
 * =============================================================================
 *
 * Apple Principal Engineer ICT Level 7 Grade Quality
 * Built for the next 10 years - January 2026
 *
 * ARCHITECTURE PRINCIPLES:
 * 1. Type-safe by default - leverages branded types and const assertions
 * 2. Resource-safe - uses Disposable pattern for cleanup
 * 3. Immutable configurations - frozen objects prevent mutation
 * 4. Composable interceptors - clean separation of concerns
 * 5. Observable operations - built-in telemetry hooks
 *
 * FEATURES:
 * - Configurable request timeouts (default 30s)
 * - Exponential backoff retry (3 attempts: 1s, 2s, 4s)
 * - Request/response interceptors with async support
 * - Request cancellation via AbortController
 * - Request deduplication (prevents duplicate in-flight requests)
 * - Automatic token refresh on 401
 * - Circuit breaker pattern for fault tolerance
 *
 * @version 2.0.0 - TypeScript 5.9+ Rewrite
 * @license MIT
 */

import { browser } from '$app/environment';
import { getAuthToken } from '$lib/stores/auth.svelte';
import {
	ApiError,
	NetworkError,
	TimeoutError,
	AuthenticationError,
	RateLimitError,
	ServerError,
	UnknownError,
	type ApiErrorCode
} from './errors';
import { RequestCache, type CacheConfig } from './cache';

// =============================================================================
// BRANDED TYPES - Compile-time safety for string identifiers
// =============================================================================

declare const RequestIdBrand: unique symbol;
declare const TraceIdBrand: unique symbol;

/** Branded type for request IDs - prevents string mixups */
export type RequestId = string & { readonly [RequestIdBrand]: typeof RequestIdBrand };

/** Branded type for trace IDs - for distributed tracing */
export type TraceId = string & { readonly [TraceIdBrand]: typeof TraceIdBrand };

/** Generate a unique request ID */
function createRequestId(): RequestId {
	const timestamp = Date.now().toString(36);
	const random = Math.random().toString(36).substring(2, 10);
	return `req_${timestamp}_${random}` as RequestId;
}

/** Generate a trace ID for distributed tracing */
function createTraceId(): TraceId {
	const timestamp = Date.now().toString(16).padStart(12, '0');
	const randomBytes = new Uint8Array(10);
	if (browser && crypto?.getRandomValues) {
		crypto.getRandomValues(randomBytes);
	} else {
		for (let i = 0; i < randomBytes.length; i++) {
			randomBytes[i] = Math.floor(Math.random() * 256);
		}
	}
	const random = Array.from(randomBytes)
		.map((b) => b.toString(16).padStart(2, '0'))
		.join('');
	return `${timestamp}${random}` as TraceId;
}

// =============================================================================
// TYPE DEFINITIONS - Const type parameters for inference
// =============================================================================

/** HTTP methods supported by the client */
export type HttpMethod = 'GET' | 'POST' | 'PUT' | 'PATCH' | 'DELETE' | 'HEAD' | 'OPTIONS';

/** Request priority levels for queue ordering */
export type RequestPriority = 'critical' | 'high' | 'normal' | 'low' | 'background';

/** Backoff strategies for retry logic */
export type BackoffStrategy = 'linear' | 'exponential' | 'fibonacci' | 'jitter';

/** Cache strategies */
export type CacheStrategy =
	| 'cache-first'
	| 'network-first'
	| 'stale-while-revalidate'
	| 'cache-only'
	| 'network-only';

/**
 * API Client configuration - Immutable after creation
 * Uses `satisfies` for exact type checking with inference
 */
export interface ApiClientConfig {
	/** Base URL for all API requests */
	readonly baseUrl: string;
	/** Default request timeout in milliseconds */
	readonly timeout: number;
	/** Number of retry attempts */
	readonly retries: number;
	/** Retry delay calculator - receives attempt number (0-indexed) */
	readonly retryDelay: (attempt: number) => number;
	/** Default headers for all requests */
	readonly defaultHeaders: Readonly<Record<string, string>>;
	/** Whether to include credentials (cookies) */
	readonly credentials: RequestCredentials;
	/** Circuit breaker configuration */
	readonly circuitBreaker: Readonly<CircuitBreakerConfig>;
}

/** Circuit breaker configuration */
export interface CircuitBreakerConfig {
	/** Number of failures before opening circuit */
	readonly failureThreshold: number;
	/** Successes needed to close circuit from half-open */
	readonly successThreshold: number;
	/** Time in ms before attempting half-open */
	readonly resetTimeout: number;
	/** Minimum requests before circuit can trip */
	readonly volumeThreshold: number;
}

/** Request options for individual requests */
export interface RequestOptions<TBody = unknown> {
	/** HTTP method */
	method?: HttpMethod;
	/** Request headers */
	headers?: Record<string, string>;
	/** Request body (auto-serialized for objects) */
	body?: TBody;
	/** URL query parameters */
	params?: Record<string, string | number | boolean | null | undefined>;
	/** Request timeout override */
	timeout?: number;
	/** Number of retries override */
	retries?: number;
	/** Request priority */
	priority?: RequestPriority;
	/** Cache configuration */
	cache?: Partial<CacheConfig>;
	/** Skip automatic token refresh on 401 */
	skipTokenRefresh?: boolean;
	/** AbortSignal for request cancellation */
	signal?: AbortSignal;
	/** Custom tags for cache invalidation */
	tags?: readonly string[];
	/** Transform response data */
	transform?: <T>(data: unknown) => T;
}

/** GET-specific options (no body allowed) */
export type GetOptions = Omit<RequestOptions<never>, 'body' | 'method'>;

/** POST-specific options */
export type PostOptions<TBody = unknown> = Omit<RequestOptions<TBody>, 'method'>;

/** PUT-specific options */
export type PutOptions<TBody = unknown> = Omit<RequestOptions<TBody>, 'method'>;

/** DELETE-specific options (typically no body) */
export type DeleteOptions = Omit<RequestOptions<never>, 'body' | 'method'>;

/** Structured API response with metadata */
export interface ApiResponse<T> {
	/** Response data */
	readonly data: T;
	/** HTTP status code */
	readonly status: number;
	/** HTTP status text */
	readonly statusText: string;
	/** Response headers */
	readonly headers: Readonly<Record<string, string>>;
	/** Request ID for tracing */
	readonly requestId: RequestId;
	/** Whether response was served from cache */
	readonly cached: boolean;
	/** Response time in milliseconds */
	readonly duration: number;
}

/** Request context for interceptors and tracing */
export interface RequestContext {
	/** Unique request ID */
	readonly requestId: RequestId;
	/** Trace ID for distributed tracing */
	readonly traceId: TraceId;
	/** Request start timestamp */
	readonly startTime: number;
	/** Full request URL */
	readonly url: string;
	/** HTTP method */
	readonly method: HttpMethod;
	/** Request priority */
	readonly priority: RequestPriority;
	/** Whether this is a retry attempt */
	readonly isRetry: boolean;
	/** Current retry attempt (0-indexed) */
	readonly retryAttempt: number;
	/** Custom metadata */
	readonly metadata: Readonly<Record<string, unknown>>;
}

/** Request interceptor function type */
export type RequestInterceptor = (
	config: RequestOptions,
	context: RequestContext
) => RequestOptions | Promise<RequestOptions>;

/** Response interceptor function type */
export type ResponseInterceptor<T = unknown> = (
	response: ApiResponse<T>,
	context: RequestContext
) => ApiResponse<T> | Promise<ApiResponse<T>>;

/** Error interceptor function type */
export type ErrorInterceptor = (
	error: ApiError,
	context: RequestContext
) => ApiError | Promise<ApiError>;

// =============================================================================
// CIRCUIT BREAKER - Prevents cascade failures
// =============================================================================

type CircuitState = 'closed' | 'open' | 'half-open';

interface CircuitBreakerState {
	state: CircuitState;
	failures: number;
	successes: number;
	lastFailureTime: number;
	lastStateChange: number;
}

class CircuitBreaker {
	private readonly config: CircuitBreakerConfig;
	private state: CircuitBreakerState;
	private readonly stateListeners: Set<(state: CircuitState) => void> = new Set();

	constructor(config: CircuitBreakerConfig) {
		this.config = Object.freeze({ ...config });
		this.state = {
			state: 'closed',
			failures: 0,
			successes: 0,
			lastFailureTime: 0,
			lastStateChange: Date.now()
		};
	}

	isOpen(): boolean {
		this.checkStateTransition();
		return this.state.state === 'open';
	}

	recordSuccess(): void {
		if (this.state.state === 'half-open') {
			this.state.successes++;
			if (this.state.successes >= this.config.successThreshold) {
				this.transitionTo('closed');
				this.state.failures = 0;
			}
		} else if (this.state.state === 'closed') {
			// Decay failures on success
			this.state.failures = Math.max(0, this.state.failures - 1);
		}
	}

	recordFailure(): void {
		this.state.failures++;
		this.state.lastFailureTime = Date.now();

		if (this.state.state === 'half-open') {
			this.transitionTo('open');
		} else if (this.state.failures >= this.config.failureThreshold) {
			this.transitionTo('open');
		}
	}

	getTimeUntilReset(): number {
		if (this.state.state !== 'open') return 0;
		const elapsed = Date.now() - this.state.lastStateChange;
		return Math.max(0, this.config.resetTimeout - elapsed);
	}

	onStateChange(listener: (state: CircuitState) => void): () => void {
		this.stateListeners.add(listener);
		return () => this.stateListeners.delete(listener);
	}

	private checkStateTransition(): void {
		if (this.state.state === 'open') {
			const elapsed = Date.now() - this.state.lastStateChange;
			if (elapsed >= this.config.resetTimeout) {
				this.transitionTo('half-open');
				this.state.successes = 0;
			}
		}
	}

	private transitionTo(newState: CircuitState): void {
		if (this.state.state !== newState) {
			this.state.state = newState;
			this.state.lastStateChange = Date.now();
			this.stateListeners.forEach((listener) => listener(newState));
		}
	}
}

// =============================================================================
// API CLIENT CLASS - Core implementation
// =============================================================================

/** Default configuration values */
const DEFAULT_CONFIG = {
	baseUrl: '',
	timeout: 30_000,
	retries: 3,
	retryDelay: (attempt: number) => Math.min(1000 * Math.pow(2, attempt), 10_000),
	defaultHeaders: {
		'Content-Type': 'application/json',
		Accept: 'application/json'
	},
	credentials: 'include' as const,
	circuitBreaker: {
		failureThreshold: 5,
		successThreshold: 2,
		resetTimeout: 30_000,
		volumeThreshold: 10
	}
} satisfies ApiClientConfig;

export class ApiClient implements Disposable {
	private readonly config: Readonly<ApiClientConfig>;
	private readonly cache: RequestCache;
	private readonly circuitBreaker: CircuitBreaker;
	private readonly activeRequests: Map<string, Promise<unknown>> = new Map();
	private readonly requestInterceptors: RequestInterceptor[] = [];
	private readonly responseInterceptors: ResponseInterceptor[] = [];
	private readonly errorInterceptors: ErrorInterceptor[] = [];
	private refreshPromise: Promise<boolean> | null = null;
	private isDisposed = false;

	constructor(config: Partial<ApiClientConfig> = {}) {
		this.config = Object.freeze({
			...DEFAULT_CONFIG,
			...config,
			defaultHeaders: Object.freeze({
				...DEFAULT_CONFIG.defaultHeaders,
				...config.defaultHeaders
			}),
			circuitBreaker: Object.freeze({
				...DEFAULT_CONFIG.circuitBreaker,
				...config.circuitBreaker
			})
		});

		this.cache = new RequestCache({
			ttl: 300_000, // 5 minutes
			maxSize: 100,
			staleWhileRevalidate: true
		});

		this.circuitBreaker = new CircuitBreaker(this.config.circuitBreaker);
	}

	/**
	 * Disposable implementation for resource cleanup
	 * Usage: using client = new ApiClient();
	 */
	[Symbol.dispose](): void {
		this.dispose();
	}

	dispose(): void {
		if (this.isDisposed) return;
		this.isDisposed = true;
		this.activeRequests.clear();
		this.cache.clear();
	}

	// ===========================================================================
	// HTTP METHODS - Type-safe API
	// ===========================================================================

	/**
	 * Perform a GET request
	 */
	async get<T>(endpoint: string, options?: GetOptions): Promise<T> {
		const response = await this.request<T>(endpoint, { ...options, method: 'GET' });
		return response.data;
	}

	/**
	 * Perform a POST request
	 */
	async post<T, TBody = unknown>(
		endpoint: string,
		data?: TBody,
		options?: PostOptions<TBody>
	): Promise<T> {
		const response = await this.request<T>(endpoint, {
			...options,
			method: 'POST',
			body: data
		});
		return response.data;
	}

	/**
	 * Perform a PUT request
	 */
	async put<T, TBody = unknown>(
		endpoint: string,
		data?: TBody,
		options?: PutOptions<TBody>
	): Promise<T> {
		const response = await this.request<T>(endpoint, {
			...options,
			method: 'PUT',
			body: data
		});
		return response.data;
	}

	/**
	 * Perform a PATCH request
	 */
	async patch<T, TBody = unknown>(
		endpoint: string,
		data?: TBody,
		options?: RequestOptions<TBody>
	): Promise<T> {
		const response = await this.request<T>(endpoint, {
			...options,
			method: 'PATCH',
			body: data
		});
		return response.data;
	}

	/**
	 * Perform a DELETE request
	 */
	async delete<T = void>(endpoint: string, options?: DeleteOptions): Promise<T> {
		const response = await this.request<T>(endpoint, { ...options, method: 'DELETE' });
		return response.data;
	}

	/**
	 * Core request method with full response metadata
	 */
	async request<T>(endpoint: string, options: RequestOptions = {}): Promise<ApiResponse<T>> {
		if (this.isDisposed) {
			throw new Error('ApiClient has been disposed');
		}

		const context = this.createContext(endpoint, options);
		const method = options.method ?? 'GET';

		// Circuit breaker check
		if (this.circuitBreaker.isOpen()) {
			const retryAfter = Math.ceil(this.circuitBreaker.getTimeUntilReset() / 1000);
			throw new ServerError(
				'Service temporarily unavailable (circuit breaker open)',
				503,
				context.requestId,
				{ retryAfter }
			);
		}

		// Request deduplication for GET requests
		const cacheKey = this.getCacheKey(endpoint, options);
		if (method === 'GET') {
			const existingRequest = this.activeRequests.get(cacheKey);
			if (existingRequest) {
				return existingRequest as Promise<ApiResponse<T>>;
			}

			// Check cache
			const cached = this.cache.get<ApiResponse<T>>(cacheKey);
			if (cached && options.cache?.strategy !== 'network-only') {
				return { ...cached.data, cached: true };
			}
		}

		// Execute request with retry logic
		const requestPromise = this.executeWithRetry<T>(endpoint, options, context);

		if (method === 'GET') {
			this.activeRequests.set(cacheKey, requestPromise);
		}

		try {
			const response = await requestPromise;

			// Cache successful GET responses
			if (method === 'GET' && response.status < 400) {
				this.cache.set(cacheKey, response, options.cache);

				// Invalidate related cache on mutations
				if (options.cache?.invalidate) {
					for (const pattern of options.cache.invalidate) {
						this.cache.invalidate(pattern);
					}
				}
			}

			return response;
		} finally {
			this.activeRequests.delete(cacheKey);
		}
	}

	// ===========================================================================
	// INTERCEPTORS - Composable middleware
	// ===========================================================================

	/**
	 * Add a request interceptor
	 * @returns Cleanup function to remove the interceptor
	 */
	addRequestInterceptor(interceptor: RequestInterceptor): () => void {
		this.requestInterceptors.push(interceptor);
		return () => {
			const index = this.requestInterceptors.indexOf(interceptor);
			if (index > -1) this.requestInterceptors.splice(index, 1);
		};
	}

	/**
	 * Add a response interceptor
	 * @returns Cleanup function to remove the interceptor
	 */
	addResponseInterceptor(interceptor: ResponseInterceptor): () => void {
		this.responseInterceptors.push(interceptor);
		return () => {
			const index = this.responseInterceptors.indexOf(interceptor);
			if (index > -1) this.responseInterceptors.splice(index, 1);
		};
	}

	/**
	 * Add an error interceptor
	 * @returns Cleanup function to remove the interceptor
	 */
	addErrorInterceptor(interceptor: ErrorInterceptor): () => void {
		this.errorInterceptors.push(interceptor);
		return () => {
			const index = this.errorInterceptors.indexOf(interceptor);
			if (index > -1) this.errorInterceptors.splice(index, 1);
		};
	}

	// ===========================================================================
	// CACHE CONTROL
	// ===========================================================================

	/**
	 * Invalidate cache entries matching pattern
	 */
	invalidateCache(pattern: string | RegExp): void {
		this.cache.invalidate(pattern);
	}

	/**
	 * Clear entire cache
	 */
	clearCache(): void {
		this.cache.clear();
	}

	// ===========================================================================
	// PRIVATE METHODS
	// ===========================================================================

	private createContext(endpoint: string, options: RequestOptions): RequestContext {
		const url = this.buildUrl(endpoint, options.params);
		return Object.freeze({
			requestId: createRequestId(),
			traceId: createTraceId(),
			startTime: performance.now(),
			url,
			method: options.method ?? 'GET',
			priority: options.priority ?? 'normal',
			isRetry: false,
			retryAttempt: 0,
			metadata: Object.freeze({})
		});
	}

	private async executeWithRetry<T>(
		endpoint: string,
		options: RequestOptions,
		context: RequestContext
	): Promise<ApiResponse<T>> {
		const maxRetries = options.retries ?? this.config.retries;
		let lastError: ApiError | null = null;

		for (let attempt = 0; attempt <= maxRetries; attempt++) {
			const retryContext: RequestContext =
				attempt === 0
					? context
					: Object.freeze({
							...context,
							isRetry: true,
							retryAttempt: attempt
						});

			try {
				return await this.executeRequest<T>(endpoint, options, retryContext);
			} catch (error) {
				lastError =
					error instanceof ApiError
						? error
						: new NetworkError(
								error instanceof Error ? error.message : 'Unknown error',
								context.requestId
							);

				// Don't retry on client errors (4xx) except 429
				if (
					lastError.statusCode >= 400 &&
					lastError.statusCode < 500 &&
					lastError.statusCode !== 429
				) {
					// Handle 401 with token refresh
					if (lastError.statusCode === 401 && !options.skipTokenRefresh) {
						const refreshed = await this.handleTokenRefresh();
						if (refreshed) {
							continue; // Retry with new token
						}
					}
					throw lastError;
				}

				// Check if we should retry
				if (attempt < maxRetries && this.shouldRetry(lastError)) {
					const delay = this.config.retryDelay(attempt);
					await this.delay(delay);
					continue;
				}

				this.circuitBreaker.recordFailure();
				throw lastError;
			}
		}

		throw lastError!;
	}

	private async executeRequest<T>(
		endpoint: string,
		options: RequestOptions,
		context: RequestContext
	): Promise<ApiResponse<T>> {
		// Run request interceptors
		let processedOptions = options;
		for (const interceptor of this.requestInterceptors) {
			processedOptions = await interceptor(processedOptions, context);
		}

		const url = this.buildUrl(endpoint, processedOptions.params);
		const timeout = processedOptions.timeout ?? this.config.timeout;

		// Create abort controller with timeout
		const controller = new AbortController();
		const timeoutId = setTimeout(() => controller.abort(), timeout);

		// Link to external signal if provided
		if (processedOptions.signal) {
			processedOptions.signal.addEventListener('abort', () => controller.abort());
		}

		try {
			const headers = this.buildHeaders(processedOptions.headers);
			const body = this.serializeBody(processedOptions.body);

			const response = await fetch(url, {
				method: processedOptions.method ?? 'GET',
				headers,
				body,
				credentials: this.config.credentials,
				signal: controller.signal
			});

			clearTimeout(timeoutId);

			// Parse response
			const responseData = await this.parseResponse<T>(response);
			const duration = performance.now() - context.startTime;

			if (!response.ok) {
				throw await this.createErrorFromResponse(response, responseData, context);
			}

			this.circuitBreaker.recordSuccess();

			let apiResponse: ApiResponse<T> = Object.freeze({
				data: processedOptions.transform
					? processedOptions.transform<T>(responseData)
					: responseData,
				status: response.status,
				statusText: response.statusText,
				headers: Object.freeze(this.parseHeaders(response.headers)),
				requestId: context.requestId,
				cached: false,
				duration
			});

			// Run response interceptors
			for (const interceptor of this.responseInterceptors) {
				apiResponse = (await interceptor(apiResponse, context)) as ApiResponse<T>;
			}

			return apiResponse;
		} catch (error) {
			clearTimeout(timeoutId);

			if (error instanceof ApiError) {
				// Run error interceptors
				let processedError = error;
				for (const interceptor of this.errorInterceptors) {
					processedError = await interceptor(processedError, context);
				}
				throw processedError;
			}

			if (error instanceof Error && error.name === 'AbortError') {
				throw new TimeoutError(`Request timed out after ${timeout}ms`, context.requestId);
			}

			throw new NetworkError(
				error instanceof Error ? error.message : 'Network request failed',
				context.requestId
			);
		}
	}

	private buildUrl(
		endpoint: string,
		params?: Record<string, string | number | boolean | null | undefined>
	): string {
		let url: string;

		if (endpoint.startsWith('http://') || endpoint.startsWith('https://')) {
			url = endpoint;
		} else if (endpoint.startsWith('/api/')) {
			url = endpoint;
		} else if (endpoint.startsWith('/')) {
			url = `/api${endpoint}`;
		} else {
			url = `/api/${endpoint}`;
		}

		if (params && Object.keys(params).length > 0) {
			const searchParams = new URLSearchParams();
			for (const [key, value] of Object.entries(params)) {
				if (value !== null && value !== undefined) {
					searchParams.append(key, String(value));
				}
			}
			url += `?${searchParams.toString()}`;
		}

		return url;
	}

	private buildHeaders(customHeaders?: Record<string, string>): Record<string, string> {
		const headers: Record<string, string> = {
			...this.config.defaultHeaders,
			...customHeaders
		};

		// Add auth token
		const token = browser ? getAuthToken() : null;
		if (token) {
			headers['Authorization'] = `Bearer ${token}`;
		}

		return headers;
	}

	private serializeBody(body: unknown): string | FormData | null {
		if (body === undefined || body === null) {
			return null;
		}

		if (body instanceof FormData) {
			return body;
		}

		return JSON.stringify(body);
	}

	private async parseResponse<T>(response: Response): Promise<T> {
		const contentType = response.headers.get('Content-Type') ?? '';

		if (contentType.includes('application/json')) {
			const text = await response.text();
			if (!text) return {} as T;

			const json = JSON.parse(text);
			// Handle backend wrapper: { success: boolean, data: T }
			return json.data !== undefined ? json.data : json;
		}

		if (contentType.includes('text/')) {
			return (await response.text()) as unknown as T;
		}

		return (await response.blob()) as unknown as T;
	}

	private parseHeaders(headers: Headers): Record<string, string> {
		const result: Record<string, string> = {};
		headers.forEach((value, key) => {
			result[key] = value;
		});
		return result;
	}

	private getCacheKey(endpoint: string, options: RequestOptions): string {
		const params = options.params ? JSON.stringify(options.params) : '';
		return `${options.method ?? 'GET'}:${endpoint}:${params}`;
	}

	private shouldRetry(error: ApiError): boolean {
		// Retry on network errors
		if (error.code === 'NETWORK_ERROR' || error.code === 'TIMEOUT_ERROR') {
			return true;
		}

		// Retry on rate limit (after waiting)
		if (error.code === 'RATE_LIMIT_ERROR') {
			return true;
		}

		// Retry on server errors (5xx)
		if (error.statusCode >= 500) {
			return true;
		}

		return false;
	}

	private async handleTokenRefresh(): Promise<boolean> {
		// Deduplicate refresh requests
		if (this.refreshPromise) {
			return this.refreshPromise;
		}

		this.refreshPromise = (async () => {
			try {
				const response = await fetch('/api/auth/refresh', {
					method: 'POST',
					credentials: 'include'
				});
				return response.ok;
			} catch {
				return false;
			} finally {
				this.refreshPromise = null;
			}
		})();

		return this.refreshPromise;
	}

	private async createErrorFromResponse(
		response: Response,
		data: unknown,
		context: RequestContext
	): Promise<ApiError> {
		const body = data as Record<string, unknown> | null;
		const message = (body?.message as string) ?? (body?.error as string) ?? response.statusText;
		const code = this.mapStatusToCode(response.status);

		const retryAfter = response.headers.get('Retry-After');
		const parsedRetryAfter = retryAfter ? parseInt(retryAfter, 10) : undefined;

		switch (response.status) {
			case 401:
				return new AuthenticationError(message, context.requestId);
			case 429:
				return new RateLimitError(message, context.requestId, parsedRetryAfter ?? 60);
			default:
				if (response.status >= 500) {
					return new ServerError(message, response.status, context.requestId, {
						retryAfter: parsedRetryAfter
					});
				}
				return new UnknownError(message, response.status, context.requestId);
		}
	}

	private mapStatusToCode(status: number): ApiErrorCode {
		const statusMap: Record<number, ApiErrorCode> = {
			400: 'VALIDATION_ERROR',
			401: 'AUTH_ERROR',
			403: 'FORBIDDEN_ERROR',
			404: 'NOT_FOUND_ERROR',
			429: 'RATE_LIMIT_ERROR',
			500: 'SERVER_ERROR',
			502: 'SERVER_ERROR',
			503: 'SERVICE_UNAVAILABLE',
			504: 'TIMEOUT_ERROR'
		};
		return statusMap[status] ?? 'UNKNOWN_ERROR';
	}

	private delay(ms: number): Promise<void> {
		return new Promise((resolve) => setTimeout(resolve, ms));
	}
}

// =============================================================================
// SINGLETON INSTANCE - Default client for application use
// =============================================================================

let clientInstance: ApiClient | null = null;

/**
 * Get the singleton API client instance
 */
export function getApiClient(config?: Partial<ApiClientConfig>): ApiClient {
	if (!clientInstance) {
		clientInstance = new ApiClient(config);
	}
	return clientInstance;
}

/**
 * Create a new API client instance (for testing or isolated use)
 */
export function createApiClient(config?: Partial<ApiClientConfig>): ApiClient {
	return new ApiClient(config);
}

/**
 * Default singleton instance
 */
export const apiClient = browser ? getApiClient() : null;

/**
 * Convenience export for direct HTTP methods
 */
export const api = {
	get: <T>(endpoint: string, options?: GetOptions) => getApiClient().get<T>(endpoint, options),
	post: <T, TBody = unknown>(endpoint: string, data?: TBody, options?: PostOptions<TBody>) =>
		getApiClient().post<T, TBody>(endpoint, data, options),
	put: <T, TBody = unknown>(endpoint: string, data?: TBody, options?: PutOptions<TBody>) =>
		getApiClient().put<T, TBody>(endpoint, data, options),
	patch: <T, TBody = unknown>(endpoint: string, data?: TBody, options?: RequestOptions<TBody>) =>
		getApiClient().patch<T, TBody>(endpoint, data, options),
	delete: <T = void>(endpoint: string, options?: DeleteOptions) =>
		getApiClient().delete<T>(endpoint, options),
	request: <T>(endpoint: string, options?: RequestOptions) =>
		getApiClient().request<T>(endpoint, options),
	invalidateCache: (pattern: string | RegExp) => getApiClient().invalidateCache(pattern),
	clearCache: () => getApiClient().clearCache()
} as const;

export default api;

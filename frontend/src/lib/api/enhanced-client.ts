/**
 * Enhanced API Client - Enterprise Grade
 * Google L7+ Principal Engineer Level
 *
 * Features:
 * - Circuit breaker integration
 * - Automatic retries with exponential backoff
 * - Distributed tracing
 * - Request/response interceptors
 * - Caching with TTL
 * - Rate limiting
 * - Request deduplication
 * - Comprehensive error handling
 */

import { browser } from '$app/environment';
import { getAuthToken } from '$lib/stores/auth.svelte';
import {
	getCircuitBreaker,
	type CircuitBreaker as _CircuitBreaker
} from '../resilience/circuit-breaker';
import { retryNetworkRequest, withIdempotency, generateIdempotencyKey } from '../resilience/retry';
import {
	startSpan,
	endSpan,
	addSpanEvent,
	recordMetric,
	incrementCounter,
	error as logError,
	info
} from '../observability/telemetry';
import type { JsonValue } from './_types';

// ═══════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════

export interface RequestConfig extends Omit<RequestInit, 'cache'> {
	// Circuit breaker
	useCircuitBreaker?: boolean;
	circuitBreakerName?: string;

	// Retry
	retry?: boolean;
	maxRetries?: number;

	// Caching
	useCache?: boolean;
	cacheTTL?: number;

	// Tracing
	trace?: boolean;

	// Idempotency
	idempotent?: boolean;
	idempotencyKey?: string;

	// Timeout
	timeout?: number;

	// Rate limiting
	rateLimit?: {
		maxRequests: number;
		windowMs: number;
	};
}

/**
 * R13-A: `T` defaults to `unknown` (not `any`). Matches `client.svelte.ts`'s
 * typed-envelope precedent — callers must commit to an explicit shape
 * (`apiClient.get<UserList>(…)`) or narrow `response.data` before use. Pre-R13-A
 * the default `any` silently poisoned every untyped consumer's property access.
 */
export interface ApiResponse<T = unknown> {
	data: T;
	status: number;
	statusText: string;
	headers: Headers;
	cached: boolean;
	duration: number;
}

export interface CacheEntry<T = unknown> {
	data: T;
	timestamp: number;
	ttl: number;
}

export interface RateLimitState {
	requests: number[];
	maxRequests: number;
	windowMs: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// Enhanced API Client
// ═══════════════════════════════════════════════════════════════════════════

export class EnhancedApiClient {
	private baseURL: string;
	private defaultConfig: RequestConfig;
	private cache = new Map<string, CacheEntry>();
	private rateLimits = new Map<string, RateLimitState>();
	private requestInterceptors: Array<
		(config: RequestConfig) => RequestConfig | Promise<RequestConfig>
	> = [];
	private responseInterceptors: Array<
		(response: ApiResponse) => ApiResponse | Promise<ApiResponse>
	> = [];
	private errorInterceptors: Array<(error: Error) => Error | Promise<Error>> = [];

	constructor(baseURL: string, defaultConfig: RequestConfig = {}) {
		this.baseURL = baseURL;
		this.defaultConfig = {
			useCircuitBreaker: true,
			retry: true,
			maxRetries: 3,
			useCache: false,
			cacheTTL: 300000, // 5 minutes
			trace: true,
			idempotent: false,
			timeout: 30000, // 30 seconds
			...defaultConfig
		};
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// HTTP Methods
	// ═══════════════════════════════════════════════════════════════════════════

	async get<T = unknown>(url: string, config: RequestConfig = {}): Promise<ApiResponse<T>> {
		return this.request<T>(url, {
			...config,
			method: 'GET',
			useCache: config.useCache !== undefined ? config.useCache : true
		});
	}

	async post<T = unknown>(
		url: string,
		data?: unknown,
		config: RequestConfig = {}
	): Promise<ApiResponse<T>> {
		return this.request<T>(url, {
			...config,
			method: 'POST',
			...(data ? { body: JSON.stringify(data) } : {}),
			idempotent: config.idempotent !== undefined ? config.idempotent : true
		});
	}

	async put<T = unknown>(
		url: string,
		data?: unknown,
		config: RequestConfig = {}
	): Promise<ApiResponse<T>> {
		return this.request<T>(url, {
			...config,
			method: 'PUT',
			...(data ? { body: JSON.stringify(data) } : {}),
			idempotent: config.idempotent !== undefined ? config.idempotent : true
		});
	}

	async patch<T = unknown>(
		url: string,
		data?: unknown,
		config: RequestConfig = {}
	): Promise<ApiResponse<T>> {
		return this.request<T>(url, {
			...config,
			method: 'PATCH',
			...(data ? { body: JSON.stringify(data) } : {})
		});
	}

	async delete<T = unknown>(url: string, config: RequestConfig = {}): Promise<ApiResponse<T>> {
		return this.request<T>(url, {
			...config,
			method: 'DELETE',
			idempotent: config.idempotent !== undefined ? config.idempotent : true
		});
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Core Request Method
	// ═══════════════════════════════════════════════════════════════════════════

	async request<T = unknown>(url: string, config: RequestConfig = {}): Promise<ApiResponse<T>> {
		const fullConfig = { ...this.defaultConfig, ...config };
		const fullURL = url.startsWith('http') ? url : `${this.baseURL}${url}`;
		const method = fullConfig.method || 'GET';

		// Start tracing
		let spanId: string | undefined;
		if (fullConfig.trace) {
			spanId = startSpan(`HTTP ${method} ${url}`, {
				'http.method': method,
				'http.url': fullURL,
				'http.target': url
			});
		}

		const startTime = performance.now();

		try {
			// Run request interceptors
			let interceptedConfig = fullConfig;
			for (const interceptor of this.requestInterceptors) {
				interceptedConfig = await interceptor(interceptedConfig);
			}

			// Check cache
			if (method === 'GET' && interceptedConfig.useCache) {
				const cached = this.getFromCache<T>(fullURL);
				if (cached) {
					if (spanId) {
						addSpanEvent(spanId, 'cache_hit', { url: fullURL });
						endSpan(spanId, { code: 'OK' });
					}

					incrementCounter('api_cache_hit_total', {
						endpoint: url,
						method
					});

					info('Cache hit', { url: fullURL });

					return {
						data: cached,
						status: 200,
						statusText: 'OK',
						headers: new Headers(),
						cached: true,
						duration: performance.now() - startTime
					};
				}
			}

			// Check rate limit
			if (interceptedConfig.rateLimit) {
				this.checkRateLimit(fullURL, interceptedConfig.rateLimit);
			}

			// Execute request with resilience patterns
			const response = await this.executeRequest<T>(fullURL, interceptedConfig, spanId);

			// Cache successful GET requests
			if (
				method === 'GET' &&
				interceptedConfig.useCache &&
				response.status >= 200 &&
				response.status < 300
			) {
				this.setCache(
					fullURL,
					response.data,
					interceptedConfig.cacheTTL ?? this.defaultConfig.cacheTTL ?? 300000
				);
			}

			// Run response interceptors
			// R13-A: interceptors are stored as `(ApiResponse) => ApiResponse`
			// without a generic — they may mutate `data` arbitrarily. The cast
			// preserves `<T>` at the public-method boundary; identical to the
			// `client.svelte.ts:transform` pattern (see line 697).
			let interceptedResponse: ApiResponse<T> = response;
			for (const interceptor of this.responseInterceptors) {
				interceptedResponse = (await interceptor(interceptedResponse)) as ApiResponse<T>;
			}

			// Record metrics
			const duration = performance.now() - startTime;
			recordMetric('api_request_duration_ms', duration, 'histogram', {
				endpoint: url,
				method,
				status: String(response.status)
			});
			incrementCounter('api_request_total', {
				endpoint: url,
				method,
				status: String(response.status)
			});

			if (spanId) {
				addSpanEvent(spanId, 'request_complete', {
					status: response.status,
					duration: `${duration.toFixed(2)}ms`
				});
				endSpan(spanId, { code: 'OK' });
			}

			return interceptedResponse;
		} catch (error: unknown) {
			// R13-A: narrow once at the catch boundary. Pre-R13-A `error: any`
			// silently allowed reading `.name`/`.message`/`.stack` off a thrown
			// scalar — TS now forces us to commit to the `Error`-shaped path.
			const err = error instanceof Error ? error : new Error(String(error));
			let interceptedError: Error = err;
			for (const interceptor of this.errorInterceptors) {
				interceptedError = await interceptor(interceptedError);
			}

			// Record error metrics
			incrementCounter('api_request_error_total', {
				endpoint: url,
				method,
				error_type: err.name
			});

			if (spanId) {
				addSpanEvent(spanId, 'request_error', {
					error: err.message,
					error_type: err.name
				});
				endSpan(spanId, {
					code: 'ERROR',
					message: err.message
				});
			}

			logError('API request failed', {
				url: fullURL,
				method,
				error: err.message,
				stack: err.stack
			});

			throw interceptedError;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Request Execution with Resilience
	// ═══════════════════════════════════════════════════════════════════════════

	private async executeRequest<T>(
		url: string,
		config: RequestConfig,
		spanId?: string
	): Promise<ApiResponse<T>> {
		const timeoutMs = config.timeout ?? this.defaultConfig.timeout ?? 30000;
		const executeFetch = async (): Promise<ApiResponse<T>> => {
			// Add timeout
			const controller = new AbortController();
			const timeoutId = setTimeout(() => controller.abort(), timeoutMs);

			try {
				const response = await fetch(url, {
					...config,
					signal: controller.signal,
					headers: {
						'Content-Type': 'application/json',
						Accept: 'application/json',
						...this.getAuthHeaders(),
						...config.headers
					}
				});

				clearTimeout(timeoutId);

				if (!response.ok) {
					const errBody = (await response.json().catch(() => null)) as JsonValue | null;
					throw new ApiError(
						`HTTP ${response.status}: ${response.statusText}`,
						response.status,
						response.statusText,
						errBody ?? undefined
					);
				}

				const data = await response.json();

				return {
					data,
					status: response.status,
					statusText: response.statusText,
					headers: response.headers,
					cached: false,
					duration: 0 // Will be set by caller
				};
			} catch (error: unknown) {
				clearTimeout(timeoutId);

				// R13-A: narrow at the catch boundary. The only branch this code
				// actually inspects is `error.name === 'AbortError'` (DOMException
				// from `signal.abort()`); rethrow everything else untouched.
				if (error instanceof Error && error.name === 'AbortError') {
					throw new ApiError(`Request timeout after ${timeoutMs}ms`, 408, 'Request Timeout');
				}

				throw error;
			}
		};

		// Wrap with idempotency if needed
		let requestFn = executeFetch;
		if (config.idempotent) {
			const key =
				config.idempotencyKey || generateIdempotencyKey(config.method || 'GET', url, config.body);
			requestFn = () => withIdempotency(key, executeFetch);
		}

		// Wrap with retry if needed
		if (config.retry) {
			const retryFn = requestFn;
			requestFn = () => retryNetworkRequest(retryFn, config.maxRetries);
		}

		// Wrap with circuit breaker if needed
		if (config.useCircuitBreaker) {
			const circuitName = config.circuitBreakerName || `api_${url}`;
			const breaker = getCircuitBreaker(circuitName, {
				name: circuitName,
				failureThreshold: 5,
				...(config.timeout !== undefined && { timeout: config.timeout }),
				resetTimeout: 60000
			});

			if (spanId) {
				addSpanEvent(spanId, 'circuit_breaker_check', {
					circuit: circuitName,
					state: breaker.getState()
				});
			}

			return breaker.execute(requestFn);
		}

		return requestFn();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Caching
	// ═══════════════════════════════════════════════════════════════════════════

	private getFromCache<T>(key: string): T | null {
		const entry = this.cache.get(key);
		if (!entry) return null;

		const now = Date.now();
		if (now - entry.timestamp > entry.ttl) {
			this.cache.delete(key);
			return null;
		}

		// R13-A: `CacheEntry.data` is `unknown` post-tightening; re-establish
		// `<T>` at the caller boundary. Same trade-off as `client.svelte.ts`'s
		// `getFromCache<T>` (line 1283: `return cached.data as T`).
		return entry.data as T;
	}

	private setCache<T>(key: string, data: T, ttl: number): void {
		this.cache.set(key, {
			data,
			timestamp: Date.now(),
			ttl
		});

		// Clean old cache entries periodically
		if (this.cache.size > 100) {
			this.cleanCache();
		}
	}

	private cleanCache(): void {
		const now = Date.now();
		for (const [key, entry] of this.cache.entries()) {
			if (now - entry.timestamp > entry.ttl) {
				this.cache.delete(key);
			}
		}
	}

	clearCache(pattern?: string): void {
		if (pattern) {
			for (const key of this.cache.keys()) {
				if (key.includes(pattern)) {
					this.cache.delete(key);
				}
			}
		} else {
			this.cache.clear();
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Rate Limiting
	// ═══════════════════════════════════════════════════════════════════════════

	private checkRateLimit(key: string, config: { maxRequests: number; windowMs: number }): void {
		const now = Date.now();
		let state = this.rateLimits.get(key);

		if (!state) {
			state = {
				requests: [],
				maxRequests: config.maxRequests,
				windowMs: config.windowMs
			};
			this.rateLimits.set(key, state);
		}

		// Remove old requests outside window
		state.requests = state.requests.filter((time) => now - time < state.windowMs);

		// Check if limit exceeded
		if (state.requests.length >= state.maxRequests) {
			const oldestRequest = state.requests[0];
			if (oldestRequest === undefined) {
				throw new RateLimitError('Rate limit exceeded', 429, Date.now() + state.windowMs);
			}
			const resetTime = oldestRequest + state.windowMs;
			const waitTime = resetTime - now;

			throw new RateLimitError(
				`Rate limit exceeded. Try again in ${Math.ceil(waitTime / 1000)}s`,
				429,
				resetTime
			);
		}

		// Add current request
		state.requests.push(now);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Interceptors
	// ═══════════════════════════════════════════════════════════════════════════

	addRequestInterceptor(
		interceptor: (config: RequestConfig) => RequestConfig | Promise<RequestConfig>
	): void {
		this.requestInterceptors.push(interceptor);
	}

	addResponseInterceptor(
		interceptor: (response: ApiResponse) => ApiResponse | Promise<ApiResponse>
	): void {
		this.responseInterceptors.push(interceptor);
	}

	addErrorInterceptor(interceptor: (error: Error) => Error | Promise<Error>): void {
		this.errorInterceptors.push(interceptor);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Utilities
	// ═══════════════════════════════════════════════════════════════════════════

	private getAuthHeaders(): Record<string, string> {
		if (!browser) return {};

		// Use secure auth store token (memory-only, not localStorage)
		const token = getAuthToken();
		if (!token) return {};

		return {
			Authorization: `Bearer ${token}`
		};
	}

	setBaseURL(url: string): void {
		this.baseURL = url;
	}

	getBaseURL(): string {
		return this.baseURL;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Custom Errors
// ═══════════════════════════════════════════════════════════════════════════

export class ApiError extends Error {
	/**
	 * R13-A: `data` typed as `JsonValue` (parsed JSON body of the failed
	 * response) — matches the precedent in `config.ts:ApiError.data`. Callers
	 * must narrow before property access; pre-R13-A `any` silently passed
	 * through every malformed shape.
	 */
	constructor(
		message: string,
		public status: number,
		public statusText: string,
		public data?: JsonValue
	) {
		super(message);
		this.name = 'ApiError';
	}
}

export class RateLimitError extends Error {
	constructor(
		message: string,
		public status: number,
		public resetTime: number
	) {
		super(message);
		this.name = 'RateLimitError';
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Default Instance
// ═══════════════════════════════════════════════════════════════════════════

// ICT 11+ CORB Fix: Use same-origin endpoints to prevent CORB
// Empty base URL means all requests go to same-origin SvelteKit proxy routes
const API_BASE_URL = '';

export const apiClient = new EnhancedApiClient(API_BASE_URL);

// Add default error interceptor for auth errors
// R13-A: typed as `Error` to match the interceptor signature; narrow to
// `ApiError` to read `.status` instead of poisoning with `any`.
apiClient.addErrorInterceptor(async (error: Error) => {
	if (error instanceof ApiError && error.status === 401) {
		// Handle unauthorized - redirect to login
		if (browser) {
			window.location.href = '/login';
		}
	}
	return error;
});

export default apiClient;

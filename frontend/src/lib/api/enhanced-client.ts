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

export interface ApiResponse<T = any> {
	data: T;
	status: number;
	statusText: string;
	headers: Headers;
	cached: boolean;
	duration: number;
}

export interface CacheEntry<T = any> {
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

	async get<T = any>(url: string, config: RequestConfig = {}): Promise<ApiResponse<T>> {
		return this.request<T>(url, {
			...config,
			method: 'GET',
			useCache: config.useCache !== undefined ? config.useCache : true
		});
	}

	async post<T = any>(
		url: string,
		data?: any,
		config: RequestConfig = {}
	): Promise<ApiResponse<T>> {
		return this.request<T>(url, {
			...config,
			method: 'POST',
			...(data && { body: JSON.stringify(data) }),
			idempotent: config.idempotent !== undefined ? config.idempotent : true
		});
	}

	async put<T = any>(url: string, data?: any, config: RequestConfig = {}): Promise<ApiResponse<T>> {
		return this.request<T>(url, {
			...config,
			method: 'PUT',
			...(data && { body: JSON.stringify(data) }),
			idempotent: config.idempotent !== undefined ? config.idempotent : true
		});
	}

	async patch<T = any>(
		url: string,
		data?: any,
		config: RequestConfig = {}
	): Promise<ApiResponse<T>> {
		return this.request<T>(url, {
			...config,
			method: 'PATCH',
			...(data && { body: JSON.stringify(data) })
		});
	}

	async delete<T = any>(url: string, config: RequestConfig = {}): Promise<ApiResponse<T>> {
		return this.request<T>(url, {
			...config,
			method: 'DELETE',
			idempotent: config.idempotent !== undefined ? config.idempotent : true
		});
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Core Request Method
	// ═══════════════════════════════════════════════════════════════════════════

	async request<T = any>(url: string, config: RequestConfig = {}): Promise<ApiResponse<T>> {
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
				this.setCache(fullURL, response.data, interceptedConfig.cacheTTL!);
			}

			// Run response interceptors
			let interceptedResponse = response;
			for (const interceptor of this.responseInterceptors) {
				interceptedResponse = await interceptor(interceptedResponse);
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
		} catch (error: any) {
			// Run error interceptors
			let interceptedError = error;
			for (const interceptor of this.errorInterceptors) {
				interceptedError = await interceptor(interceptedError);
			}

			// Record error metrics
			incrementCounter('api_request_error_total', {
				endpoint: url,
				method,
				error_type: error.name
			});

			if (spanId) {
				addSpanEvent(spanId, 'request_error', {
					error: error.message,
					error_type: error.name
				});
				endSpan(spanId, {
					code: 'ERROR',
					message: error.message
				});
			}

			logError('API request failed', {
				url: fullURL,
				method,
				error: error.message,
				stack: error.stack
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
		const executeFetch = async (): Promise<ApiResponse<T>> => {
			// Add timeout
			const controller = new AbortController();
			const timeoutId = setTimeout(() => controller.abort(), config.timeout!);

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
					throw new ApiError(
						`HTTP ${response.status}: ${response.statusText}`,
						response.status,
						response.statusText,
						await response.json().catch(() => null)
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
			} catch (error: any) {
				clearTimeout(timeoutId);

				if (error.name === 'AbortError') {
					throw new ApiError(`Request timeout after ${config.timeout}ms`, 408, 'Request Timeout');
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

		return entry.data;
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
	constructor(
		message: string,
		public status: number,
		public statusText: string,
		public data?: any
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
apiClient.addErrorInterceptor(async (error: any) => {
	if (error.status === 401) {
		// Handle unauthorized - redirect to login
		if (browser) {
			window.location.href = '/login';
		}
	}
	return error;
});

export default apiClient;

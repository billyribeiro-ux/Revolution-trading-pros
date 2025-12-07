/**
 * Enterprise API Client - Apple ICT9+ Production-Grade HTTP Client
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * A fully-featured, production-hardened API client with:
 *
 * 1. Request Pipeline: Interceptor-based request/response processing
 * 2. Automatic Retry: Configurable retry with exponential backoff
 * 3. Circuit Breaker: Prevents cascade failures
 * 4. Rate Limiting: Token bucket with burst support
 * 5. Caching: Multi-strategy caching with invalidation
 * 6. Tracing: Distributed tracing with correlation IDs
 * 7. CSRF Protection: Double-submit cookie pattern
 * 8. Error Handling: Normalized errors with categorization
 *
 * @version 1.0.0
 * @license MIT
 */

import { browser } from '$app/environment';
import { writable, derived, type Readable } from 'svelte/store';

import type {
	HttpMethod,
	EnterpriseRequestConfig,
	EnterpriseResponse,
	EnterpriseApiError,
	RequestContext,
	RetryConfig,
	CacheConfig,
	CacheEntry,
	CircuitBreakerConfig,
	CircuitBreakerState,
	CircuitState,
	RequestMetrics,
	ApiEvent,
	ApiEventHandler,
	ApiEventType,
	BackoffStrategy
} from './types';

import {
	createRequestContext,
	createRetryContext,
	getRequestDuration,
	recordResponseTime,
	getTracingMetrics,
	recordTrace,
	log
} from './tracing';

import {
	createErrorFromResponse,
	createNetworkError,
	createApiError,
	ErrorCodes,
	isApiError
} from './errors';

import {
	createDefaultInterceptorChain,
	executeRequestInterceptors,
	executeResponseInterceptors,
	executeErrorInterceptors,
	type InterceptorChain
} from './interceptor';

// ═══════════════════════════════════════════════════════════════════════════════
// Configuration
// ═══════════════════════════════════════════════════════════════════════════════

export interface EnterpriseClientConfig {
	/** Base URL for API requests */
	baseUrl: string;
	/** Default request timeout in milliseconds */
	timeout: number;
	/** Default retry configuration */
	retry: RetryConfig;
	/** Default cache configuration */
	cache: CacheConfig;
	/** Circuit breaker configuration */
	circuitBreaker: CircuitBreakerConfig;
	/** Rate limit (requests per second) */
	rateLimit: number;
	/** Whether to include credentials */
	credentials: RequestCredentials;
}

const defaultConfig: EnterpriseClientConfig = {
	baseUrl: import.meta.env.VITE_API_URL || '',
	timeout: 30000,
	retry: {
		maxAttempts: 3,
		baseDelay: 1000,
		maxDelay: 10000,
		backoff: 'exponential',
		retryableStatuses: [408, 429, 500, 502, 503, 504]
	},
	cache: {
		strategy: 'network-first',
		ttl: 300000, // 5 minutes
		persist: false
	},
	circuitBreaker: {
		failureThreshold: 5,
		successThreshold: 2,
		timeout: 30000,
		volumeThreshold: 10,
		windowSize: 60000
	},
	rateLimit: 100,
	credentials: 'include'
};

// ═══════════════════════════════════════════════════════════════════════════════
// Enterprise API Client
// ═══════════════════════════════════════════════════════════════════════════════

export class EnterpriseClient {
	private config: EnterpriseClientConfig;
	private interceptors: InterceptorChain;
	private cache: Map<string, CacheEntry> = new Map();
	private circuitState: CircuitBreakerState;
	private rateLimitTokens: number;
	private rateLimitLastRefill: number;
	private activeRequests: Map<string, Promise<unknown>> = new Map();
	private eventHandlers: Map<ApiEventType, Set<ApiEventHandler>> = new Map();

	// Metrics
	private _metrics: RequestMetrics = {
		totalRequests: 0,
		successfulRequests: 0,
		failedRequests: 0,
		activeRequests: 0,
		avgResponseTime: 0,
		p50ResponseTime: 0,
		p95ResponseTime: 0,
		p99ResponseTime: 0,
		cacheHitRate: 0,
		errorRate: 0,
		retryRate: 0,
		rateLimitHits: 0
	};

	// Stores
	private metricsStore = writable(this._metrics);
	private loadingStore = writable(false);
	private errorStore = writable<EnterpriseApiError | null>(null);

	public readonly metrics: Readable<RequestMetrics> = this.metricsStore;
	public readonly loading: Readable<boolean> = this.loadingStore;
	public readonly error: Readable<EnterpriseApiError | null> = this.errorStore;

	constructor(config: Partial<EnterpriseClientConfig> = {}) {
		this.config = { ...defaultConfig, ...config };
		this.interceptors = createDefaultInterceptorChain();
		this.circuitState = {
			state: 'closed',
			failures: 0,
			successes: 0,
			lastStateChange: Date.now()
		};
		this.rateLimitTokens = this.config.rateLimit;
		this.rateLimitLastRefill = Date.now();

		// Start periodic cache cleanup
		if (browser) {
			setInterval(() => this.cleanupCache(), 60000);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// HTTP Methods
	// ═══════════════════════════════════════════════════════════════════════════

	async get<T>(endpoint: string, config?: Partial<EnterpriseRequestConfig>): Promise<T> {
		return this.request<T>(endpoint, { ...config, method: 'GET' });
	}

	async post<T>(
		endpoint: string,
		body?: unknown,
		config?: Partial<EnterpriseRequestConfig>
	): Promise<T> {
		return this.request<T>(endpoint, { ...config, method: 'POST', body });
	}

	async put<T>(
		endpoint: string,
		body?: unknown,
		config?: Partial<EnterpriseRequestConfig>
	): Promise<T> {
		return this.request<T>(endpoint, { ...config, method: 'PUT', body });
	}

	async patch<T>(
		endpoint: string,
		body?: unknown,
		config?: Partial<EnterpriseRequestConfig>
	): Promise<T> {
		return this.request<T>(endpoint, { ...config, method: 'PATCH', body });
	}

	async delete<T = void>(endpoint: string, config?: Partial<EnterpriseRequestConfig>): Promise<T> {
		return this.request<T>(endpoint, { ...config, method: 'DELETE' });
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Core Request Method
	// ═══════════════════════════════════════════════════════════════════════════

	async request<T>(endpoint: string, config: Partial<EnterpriseRequestConfig> = {}): Promise<T> {
		const context = createRequestContext({
			priority: config.priority,
			metadata: config.metadata
		});

		const fullConfig: EnterpriseRequestConfig = {
			method: 'GET',
			timeout: this.config.timeout,
			credentials: this.config.credentials,
			retry: { ...this.config.retry, ...config.retry },
			cache: { ...this.config.cache, ...config.cache },
			...config
		};

		// Check circuit breaker
		if (this.isCircuitOpen()) {
			const error = createApiError({
				message: 'Circuit breaker is open',
				code: ErrorCodes.CIRCUIT_OPEN,
				status: 503,
				context
			});
			this.emit('circuit:open', { endpoint });
			throw error;
		}

		// Check rate limit
		if (!this.acquireRateLimitToken()) {
			this._metrics.rateLimitHits++;
			this.updateMetrics();
			this.emit('rate:limited', { endpoint });

			// Wait and retry
			await this.waitForRateLimitToken();
		}

		// Generate cache key
		const cacheKey = this.getCacheKey(endpoint, fullConfig);

		// Check for cache (GET only)
		if (fullConfig.method === 'GET' && fullConfig.cache?.strategy !== 'network-only') {
			const cached = this.getFromCache<T>(cacheKey, fullConfig.cache);
			if (cached !== null) {
				this.emit('cache:hit', { endpoint, cacheKey });
				return cached;
			}
			this.emit('cache:miss', { endpoint, cacheKey });
		}

		// Check for duplicate request
		const existingRequest = this.activeRequests.get(cacheKey);
		if (existingRequest && fullConfig.method === 'GET') {
			log('debug', 'Deduplicating request', context, { endpoint });
			return existingRequest as Promise<T>;
		}

		// Execute request
		const requestPromise = this.executeRequest<T>(endpoint, fullConfig, context);
		this.activeRequests.set(cacheKey, requestPromise);

		try {
			const result = await requestPromise;
			this.activeRequests.delete(cacheKey);
			return result;
		} catch (error) {
			this.activeRequests.delete(cacheKey);
			throw error;
		}
	}

	private async executeRequest<T>(
		endpoint: string,
		config: EnterpriseRequestConfig,
		context: RequestContext
	): Promise<T> {
		this._metrics.totalRequests++;
		this._metrics.activeRequests++;
		this.loadingStore.set(true);
		this.emit('request:start', { endpoint, context });

		try {
			// Execute request interceptors
			const processedConfig = await executeRequestInterceptors(
				this.interceptors,
				config,
				context
			);

			// Build URL
			const url = this.buildUrl(endpoint, processedConfig.params);

			// Build request options
			const requestInit: RequestInit = {
				method: processedConfig.method,
				headers: processedConfig.headers,
				credentials: processedConfig.credentials,
				signal: this.createAbortSignal(processedConfig.timeout, processedConfig.signal)
			};

			// Add body for mutating methods
			if (
				processedConfig.body !== undefined &&
				['POST', 'PUT', 'PATCH'].includes(processedConfig.method || 'GET')
			) {
				if (processedConfig.body instanceof FormData) {
					requestInit.body = processedConfig.body;
				} else {
					requestInit.body = JSON.stringify(processedConfig.body);
				}
			}

			// Execute fetch with retry
			const response = await this.fetchWithRetry(url, requestInit, processedConfig, context);

			// Parse response
			const data = await this.parseResponse<T>(response);

			// Build enterprise response
			const enterpriseResponse: EnterpriseResponse<T> = {
				data,
				status: response.status,
				statusText: response.statusText,
				headers: this.parseHeaders(response.headers),
				context,
				meta: {
					executionTime: getRequestDuration(context)
				}
			};

			// Execute response interceptors
			const processedResponse = await executeResponseInterceptors(
				this.interceptors,
				enterpriseResponse,
				context
			);

			// Update metrics
			const duration = getRequestDuration(context);
			recordResponseTime(duration);
			this._metrics.successfulRequests++;
			this.recordCircuitSuccess();

			// Cache successful GET responses
			if (config.method === 'GET' && config.cache?.strategy !== 'network-only') {
				this.setCache(this.getCacheKey(endpoint, config), processedResponse.data, config.cache);
			}

			// Record trace
			recordTrace(context, endpoint, config.method || 'GET', response.status);

			this.emit('request:success', { endpoint, context, duration });

			// Transform if needed
			if (config.transform) {
				return config.transform<T>(processedResponse.data);
			}

			return processedResponse.data;
		} catch (error) {
			// Handle error
			let apiError: EnterpriseApiError;

			if (isApiError(error)) {
				apiError = error;
			} else if (error instanceof Error) {
				apiError = createNetworkError(error, context);
			} else {
				apiError = createApiError({
					message: 'Unknown error occurred',
					code: ErrorCodes.UNKNOWN,
					status: 0,
					context
				});
			}

			// Execute error interceptors
			const processedError = await executeErrorInterceptors(this.interceptors, apiError, context);

			// Check if we should retry (token refreshed)
			if (processedError.context?.metadata?.shouldRetry && !context.metadata?.retriedAfterRefresh) {
				const retryContext = createRetryContext(context, 0);
				retryContext.metadata.retriedAfterRefresh = true;
				return this.executeRequest<T>(endpoint, config, retryContext);
			}

			// Update metrics
			this._metrics.failedRequests++;
			this.recordCircuitFailure();

			// Record trace
			recordTrace(context, endpoint, config.method || 'GET', processedError.status, processedError.message);

			this.errorStore.set(processedError);
			this.emit('request:error', { endpoint, context, error: processedError });

			throw processedError;
		} finally {
			this._metrics.activeRequests--;
			this.updateMetrics();
			this.loadingStore.set(this._metrics.activeRequests > 0);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Retry Logic
	// ═══════════════════════════════════════════════════════════════════════════

	private async fetchWithRetry(
		url: string,
		init: RequestInit,
		config: EnterpriseRequestConfig,
		context: RequestContext
	): Promise<Response> {
		const maxAttempts = config.retry?.maxAttempts ?? this.config.retry.maxAttempts ?? 3;
		let lastError: Error | null = null;

		for (let attempt = 0; attempt < maxAttempts; attempt++) {
			try {
				const response = await fetch(url, init);

				// Check if response indicates an error that should be retried
				if (!response.ok) {
					const shouldRetry = this.shouldRetryResponse(response, config.retry, attempt);

					if (shouldRetry) {
						const delay = this.calculateRetryDelay(attempt, config.retry, response);
						log('debug', `Retrying request (attempt ${attempt + 1})`, context, { delay });

						config.retry?.onRetry?.(
							await createErrorFromResponse(response.clone(), context),
							attempt + 1
						);

						this._metrics.retryRate =
							(this._metrics.retryRate * this._metrics.totalRequests + 1) /
							(this._metrics.totalRequests + 1);

						this.emit('request:retry', { attempt: attempt + 1, delay });
						await this.delay(delay);
						continue;
					}

					// Not retryable, throw error
					throw await createErrorFromResponse(response, context);
				}

				return response;
			} catch (error) {
				lastError = error as Error;

				// Check if error is retryable
				if (this.shouldRetryError(error, config.retry, attempt, maxAttempts)) {
					const delay = this.calculateRetryDelay(attempt, config.retry);
					log('debug', `Retrying after error (attempt ${attempt + 1})`, context, {
						error: (error as Error).message,
						delay
					});

					this.emit('request:retry', { attempt: attempt + 1, delay });
					await this.delay(delay);
					continue;
				}

				throw error;
			}
		}

		throw lastError || new Error('Request failed after max retries');
	}

	private shouldRetryResponse(
		response: Response,
		retry?: RetryConfig,
		attempt?: number
	): boolean {
		const retryableStatuses = retry?.retryableStatuses ?? this.config.retry.retryableStatuses ?? [];
		return retryableStatuses.includes(response.status);
	}

	private shouldRetryError(
		error: unknown,
		retry?: RetryConfig,
		attempt: number,
		maxAttempts: number
	): boolean {
		if (attempt >= maxAttempts - 1) return false;

		// Custom retry condition
		if (retry?.retryCondition && isApiError(error)) {
			return retry.retryCondition(error, attempt);
		}

		// Network errors are retryable
		if (error instanceof TypeError || (error as Error).name === 'AbortError') {
			return true;
		}

		// Check if error is marked as retryable
		if (isApiError(error)) {
			return error.isRetryable;
		}

		return false;
	}

	private calculateRetryDelay(
		attempt: number,
		retry?: RetryConfig,
		response?: Response
	): number {
		// Check Retry-After header
		if (response) {
			const retryAfter = response.headers.get('Retry-After');
			if (retryAfter) {
				const seconds = parseInt(retryAfter, 10);
				if (!isNaN(seconds)) {
					return seconds * 1000;
				}
			}
		}

		const baseDelay = retry?.baseDelay ?? this.config.retry.baseDelay ?? 1000;
		const maxDelay = retry?.maxDelay ?? this.config.retry.maxDelay ?? 10000;
		const backoff = retry?.backoff ?? this.config.retry.backoff ?? 'exponential';

		let delay: number;

		switch (backoff) {
			case 'linear':
				delay = baseDelay * (attempt + 1);
				break;
			case 'exponential':
				delay = baseDelay * Math.pow(2, attempt);
				break;
			case 'fibonacci':
				delay = baseDelay * this.fibonacci(attempt + 2);
				break;
			case 'decorrelated-jitter':
				// AWS-style decorrelated jitter
				delay = Math.min(maxDelay, Math.random() * baseDelay * Math.pow(2, attempt));
				break;
			default:
				delay = baseDelay * Math.pow(2, attempt);
		}

		// Add jitter (±10%)
		const jitter = delay * 0.1 * (Math.random() * 2 - 1);
		delay = Math.min(maxDelay, delay + jitter);

		return Math.round(delay);
	}

	private fibonacci(n: number): number {
		if (n <= 1) return n;
		let a = 0, b = 1;
		for (let i = 2; i <= n; i++) {
			[a, b] = [b, a + b];
		}
		return b;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Circuit Breaker
	// ═══════════════════════════════════════════════════════════════════════════

	private isCircuitOpen(): boolean {
		if (this.circuitState.state === 'closed') {
			return false;
		}

		if (this.circuitState.state === 'open') {
			// Check if timeout has passed
			const timeInOpen = Date.now() - this.circuitState.lastStateChange;
			if (timeInOpen >= this.config.circuitBreaker.timeout) {
				// Transition to half-open
				this.circuitState.state = 'half-open';
				this.circuitState.successes = 0;
				this.circuitState.lastStateChange = Date.now();
				this.emit('circuit:half-open', {});
				return false;
			}
			return true;
		}

		// half-open state
		return false;
	}

	private recordCircuitSuccess(): void {
		if (this.circuitState.state === 'half-open') {
			this.circuitState.successes++;
			if (this.circuitState.successes >= this.config.circuitBreaker.successThreshold) {
				this.circuitState.state = 'closed';
				this.circuitState.failures = 0;
				this.circuitState.lastStateChange = Date.now();
				this.emit('circuit:close', {});
			}
		} else if (this.circuitState.state === 'closed') {
			// Reset failures on success in closed state
			this.circuitState.failures = Math.max(0, this.circuitState.failures - 1);
		}
	}

	private recordCircuitFailure(): void {
		this.circuitState.failures++;
		this.circuitState.lastFailureTime = Date.now();

		if (this.circuitState.state === 'half-open') {
			// Single failure in half-open returns to open
			this.circuitState.state = 'open';
			this.circuitState.lastStateChange = Date.now();
			this.emit('circuit:open', { reason: 'half-open-failure' });
		} else if (this.circuitState.failures >= this.config.circuitBreaker.failureThreshold) {
			this.circuitState.state = 'open';
			this.circuitState.lastStateChange = Date.now();
			this.emit('circuit:open', { reason: 'threshold-exceeded' });
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Rate Limiting
	// ═══════════════════════════════════════════════════════════════════════════

	private acquireRateLimitToken(): boolean {
		this.refillRateLimitTokens();

		if (this.rateLimitTokens > 0) {
			this.rateLimitTokens--;
			return true;
		}

		return false;
	}

	private refillRateLimitTokens(): void {
		const now = Date.now();
		const elapsed = now - this.rateLimitLastRefill;
		const tokensToAdd = (elapsed / 1000) * this.config.rateLimit;

		this.rateLimitTokens = Math.min(this.config.rateLimit, this.rateLimitTokens + tokensToAdd);
		this.rateLimitLastRefill = now;
	}

	private async waitForRateLimitToken(): Promise<void> {
		const waitTime = (1 / this.config.rateLimit) * 1000;
		await this.delay(waitTime);
		this.refillRateLimitTokens();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Caching
	// ═══════════════════════════════════════════════════════════════════════════

	private getCacheKey(endpoint: string, config: EnterpriseRequestConfig): string {
		if (config.cache?.key) {
			return config.cache.key;
		}

		const params = config.params ? JSON.stringify(config.params) : '';
		return `${config.method || 'GET'}:${endpoint}:${params}`;
	}

	private getFromCache<T>(key: string, config?: CacheConfig): T | null {
		const entry = this.cache.get(key);

		if (!entry) {
			return null;
		}

		const now = Date.now();

		// Check if expired
		if (now > entry.expiry) {
			this.cache.delete(key);
			return null;
		}

		// Update cache hit metrics
		const totalCacheAttempts = this._metrics.totalRequests;
		const currentHits = this._metrics.cacheHitRate * totalCacheAttempts;
		this._metrics.cacheHitRate = (currentHits + 1) / (totalCacheAttempts + 1);

		return entry.data as T;
	}

	private setCache(key: string, data: unknown, config?: CacheConfig): void {
		const ttl = config?.ttl ?? this.config.cache.ttl ?? 300000;

		const entry: CacheEntry = {
			data,
			timestamp: Date.now(),
			expiry: Date.now() + ttl,
			tags: config?.tags ?? []
		};

		this.cache.set(key, entry);
	}

	private cleanupCache(): void {
		const now = Date.now();
		for (const [key, entry] of this.cache) {
			if (now > entry.expiry) {
				this.cache.delete(key);
			}
		}
	}

	/**
	 * Invalidate cache entries by key pattern or tags
	 */
	invalidateCache(patterns?: string[], tags?: string[]): void {
		for (const [key, entry] of this.cache) {
			// Check patterns
			if (patterns?.some((pattern) => key.includes(pattern) || new RegExp(pattern).test(key))) {
				this.cache.delete(key);
				continue;
			}

			// Check tags
			if (tags?.some((tag) => entry.tags.includes(tag))) {
				this.cache.delete(key);
			}
		}
	}

	/**
	 * Clear entire cache
	 */
	clearCache(): void {
		this.cache.clear();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Utilities
	// ═══════════════════════════════════════════════════════════════════════════

	private buildUrl(endpoint: string, params?: Record<string, unknown>): string {
		const base = endpoint.startsWith('http') ? '' : this.config.baseUrl;
		let url = `${base}${endpoint}`;

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

	private createAbortSignal(timeout?: number, existingSignal?: AbortSignal): AbortSignal {
		const controller = new AbortController();

		// Set timeout
		if (timeout) {
			setTimeout(() => controller.abort(), timeout);
		}

		// Link to existing signal
		if (existingSignal) {
			existingSignal.addEventListener('abort', () => controller.abort());
		}

		return controller.signal;
	}

	private async parseResponse<T>(response: Response): Promise<T> {
		const contentType = response.headers.get('Content-Type') || '';

		if (contentType.includes('application/json')) {
			const text = await response.text();
			return text ? JSON.parse(text) : null;
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

	private delay(ms: number): Promise<void> {
		return new Promise((resolve) => setTimeout(resolve, ms));
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Events
	// ═══════════════════════════════════════════════════════════════════════════

	on<T>(event: ApiEventType, handler: ApiEventHandler<T>): () => void {
		if (!this.eventHandlers.has(event)) {
			this.eventHandlers.set(event, new Set());
		}
		this.eventHandlers.get(event)!.add(handler as ApiEventHandler);

		return () => {
			this.eventHandlers.get(event)?.delete(handler as ApiEventHandler);
		};
	}

	private emit<T>(type: ApiEventType, data: T): void {
		const event: ApiEvent<T> = {
			type,
			timestamp: Date.now(),
			data
		};

		this.eventHandlers.get(type)?.forEach((handler) => {
			try {
				handler(event);
			} catch (e) {
				console.error('Event handler error:', e);
			}
		});
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Metrics
	// ═══════════════════════════════════════════════════════════════════════════

	private updateMetrics(): void {
		const tracingMetrics = getTracingMetrics();

		this._metrics = {
			...this._metrics,
			...tracingMetrics,
			errorRate:
				this._metrics.totalRequests > 0
					? this._metrics.failedRequests / this._metrics.totalRequests
					: 0
		};

		this.metricsStore.set(this._metrics);
	}

	getMetrics(): RequestMetrics {
		return { ...this._metrics };
	}

	getCircuitState(): CircuitBreakerState {
		return { ...this.circuitState };
	}
}

// ═══════════════════════════════════════════════════════════════════════════════
// Singleton Export
// ═══════════════════════════════════════════════════════════════════════════════

let clientInstance: EnterpriseClient | null = null;

/**
 * Get the singleton enterprise client instance
 */
export function getEnterpriseClient(config?: Partial<EnterpriseClientConfig>): EnterpriseClient {
	if (!clientInstance) {
		clientInstance = new EnterpriseClient(config);
	}
	return clientInstance;
}

/**
 * Create a new client instance (for testing or isolated use)
 */
export function createEnterpriseClient(config?: Partial<EnterpriseClientConfig>): EnterpriseClient {
	return new EnterpriseClient(config);
}

// Default export
export const enterpriseClient = browser ? getEnterpriseClient() : null;

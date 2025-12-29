/**
 * Retry Pattern with Exponential Backoff
 * Google L7+ Principal Engineer Level
 *
 * Features:
 * - Exponential backoff
 * - Jitter for thundering herd prevention
 * - Configurable retry strategies
 * - Idempotency support
 * - Metrics collection
 */

import { recordMetric, incrementCounter, warn } from '../observability/telemetry';

// ═══════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════

export interface RetryConfig {
	maxAttempts: number;
	initialDelay: number;
	maxDelay: number;
	backoffMultiplier: number;
	jitter: boolean;
	retryableErrors?: string[];
	retryableStatusCodes?: number[];
	onRetry?: (attempt: number, error: Error) => void;
}

export interface RetryResult<T> {
	success: boolean;
	data?: T;
	error?: Error;
	attempts: number;
	totalDuration: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// Retry Implementation
// ═══════════════════════════════════════════════════════════════════════════

export class RetryPolicy {
	private config: RetryConfig;

	constructor(config: Partial<RetryConfig> = {}) {
		this.config = {
			maxAttempts: config.maxAttempts || 3,
			initialDelay: config.initialDelay || 1000,
			maxDelay: config.maxDelay || 30000,
			backoffMultiplier: config.backoffMultiplier || 2,
			jitter: config.jitter !== undefined ? config.jitter : true,
			retryableErrors: config.retryableErrors || [
				'NetworkError',
				'TimeoutError',
				'ConnectionError',
				'ECONNREFUSED',
				'ETIMEDOUT',
				'ENOTFOUND'
			],
			retryableStatusCodes: config.retryableStatusCodes || [
				408, // Request Timeout
				429, // Too Many Requests
				500, // Internal Server Error
				502, // Bad Gateway
				503, // Service Unavailable
				504 // Gateway Timeout
			],
			...(config.onRetry && { onRetry: config.onRetry })
		};
	}

	/**
	 * Execute function with retry logic
	 */
	async execute<T>(fn: () => Promise<T>, context?: string): Promise<T> {
		const startTime = performance.now();
		let lastError: Error | undefined;
		let attempt = 0;

		while (attempt < this.config.maxAttempts) {
			attempt++;

			try {
				const result = await fn();

				// Record success metrics
				const duration = performance.now() - startTime;
				recordMetric('retry_success_total', 1, 'counter', {
					context: context || 'unknown',
					attempts: String(attempt)
				});
				recordMetric('retry_duration_ms', duration, 'histogram', {
					context: context || 'unknown',
					result: 'success'
				});

				if (attempt > 1) {
					warn(`Retry succeeded after ${attempt} attempts`, {
						context,
						duration: `${duration.toFixed(2)}ms`
					});
				}

				return result;
			} catch (error: any) {
				lastError = error;

				// Check if error is retryable
				if (!this.isRetryable(error)) {
					incrementCounter('retry_non_retryable_error_total', {
						context: context || 'unknown',
						error_type: error.name
					});
					throw error;
				}

				// Check if we have more attempts
				if (attempt >= this.config.maxAttempts) {
					const duration = performance.now() - startTime;
					incrementCounter('retry_exhausted_total', {
						context: context || 'unknown',
						attempts: String(attempt)
					});
					recordMetric('retry_duration_ms', duration, 'histogram', {
						context: context || 'unknown',
						result: 'failure'
					});
					throw new RetryExhaustedError(
						`Max retry attempts (${this.config.maxAttempts}) exceeded`,
						attempt,
						lastError
					);
				}

				// Calculate delay
				const delay = this.calculateDelay(attempt);

				// Call retry callback
				if (this.config.onRetry) {
					this.config.onRetry(attempt, error);
				}

				warn(`Retry attempt ${attempt}/${this.config.maxAttempts} failed, retrying in ${delay}ms`, {
					context,
					error: error.message
				});

				incrementCounter('retry_attempt_total', {
					context: context || 'unknown',
					attempt: String(attempt)
				});

				// Wait before retry
				await this.sleep(delay);
			}
		}

		// This should never be reached, but TypeScript needs it
		throw lastError || new Error('Retry failed');
	}

	/**
	 * Check if error is retryable
	 */
	private isRetryable(error: any): boolean {
		// Check error name/type
		if (this.config.retryableErrors) {
			const errorName = error.name || error.constructor?.name || '';
			const errorMessage = error.message || '';

			for (const retryableError of this.config.retryableErrors) {
				if (errorName.includes(retryableError) || errorMessage.includes(retryableError)) {
					return true;
				}
			}
		}

		// Check HTTP status code
		if (error.status && this.config.retryableStatusCodes) {
			return this.config.retryableStatusCodes.includes(error.status);
		}

		// Check response status
		if (error.response?.status && this.config.retryableStatusCodes) {
			return this.config.retryableStatusCodes.includes(error.response.status);
		}

		return false;
	}

	/**
	 * Calculate delay with exponential backoff and jitter
	 */
	private calculateDelay(attempt: number): number {
		// Exponential backoff: initialDelay * (backoffMultiplier ^ (attempt - 1))
		let delay = this.config.initialDelay * Math.pow(this.config.backoffMultiplier, attempt - 1);

		// Cap at maxDelay
		delay = Math.min(delay, this.config.maxDelay);

		// Add jitter to prevent thundering herd
		if (this.config.jitter) {
			const jitterAmount = delay * 0.1; // 10% jitter
			delay = delay + (Math.random() * jitterAmount * 2 - jitterAmount);
		}

		return Math.floor(delay);
	}

	/**
	 * Sleep for specified duration
	 */
	private sleep(ms: number): Promise<void> {
		return new Promise((resolve) => setTimeout(resolve, ms));
	}

	/**
	 * Update configuration
	 */
	configure(config: Partial<RetryConfig>): void {
		this.config = { ...this.config, ...config };
	}

	/**
	 * Get current configuration
	 */
	getConfig(): RetryConfig {
		return { ...this.config };
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Custom Errors
// ═══════════════════════════════════════════════════════════════════════════

export class RetryExhaustedError extends Error {
	constructor(
		message: string,
		public attempts: number,
		public lastError?: Error
	) {
		super(message);
		this.name = 'RetryExhaustedError';
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Convenience Functions
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Retry a function with default configuration
 */
export async function retry<T>(fn: () => Promise<T>, config?: Partial<RetryConfig>): Promise<T> {
	const policy = new RetryPolicy(config);
	return policy.execute(fn);
}

/**
 * Retry with exponential backoff (common pattern)
 */
export async function retryWithBackoff<T>(
	fn: () => Promise<T>,
	maxAttempts: number = 3,
	initialDelay: number = 1000
): Promise<T> {
	return retry(fn, {
		maxAttempts,
		initialDelay,
		backoffMultiplier: 2,
		jitter: true
	});
}

/**
 * Retry for network requests
 */
export async function retryNetworkRequest<T>(
	fn: () => Promise<T>,
	maxAttempts: number = 3
): Promise<T> {
	return retry(fn, {
		maxAttempts,
		initialDelay: 1000,
		maxDelay: 10000,
		backoffMultiplier: 2,
		jitter: true,
		retryableStatusCodes: [408, 429, 500, 502, 503, 504]
	});
}

/**
 * Retry with custom error handling
 */
export async function retryWithHandler<T>(
	fn: () => Promise<T>,
	onRetry: (attempt: number, error: Error) => void,
	maxAttempts: number = 3
): Promise<T> {
	return retry(fn, {
		maxAttempts,
		onRetry
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// Idempotency Helper
// ═══════════════════════════════════════════════════════════════════════════

const idempotencyKeys = new Map<string, Promise<any>>();

/**
 * Execute function with idempotency guarantee
 * Same key will return the same promise (deduplication)
 */
export async function withIdempotency<T>(
	key: string,
	fn: () => Promise<T>,
	ttl: number = 60000 // 1 minute
): Promise<T> {
	// Check if request is already in flight
	if (idempotencyKeys.has(key)) {
		warn('Idempotent request already in flight, returning existing promise', { key });
		return idempotencyKeys.get(key)!;
	}

	// Execute and cache promise
	const promise = fn();
	idempotencyKeys.set(key, promise);

	// Clean up after TTL
	setTimeout(() => {
		idempotencyKeys.delete(key);
	}, ttl);

	try {
		const result = await promise;
		return result;
	} catch (error) {
		// Remove from cache on error
		idempotencyKeys.delete(key);
		throw error;
	}
}

/**
 * Generate idempotency key from request details
 */
export function generateIdempotencyKey(method: string, url: string, body?: any): string {
	const bodyStr = body ? JSON.stringify(body) : '';
	const combined = `${method}:${url}:${bodyStr}`;

	// Simple hash function
	let hash = 0;
	for (let i = 0; i < combined.length; i++) {
		const char = combined.charCodeAt(i);
		hash = (hash << 5) - hash + char;
		hash = hash & hash; // Convert to 32bit integer
	}

	return `idem_${Math.abs(hash).toString(36)}`;
}

export default RetryPolicy;

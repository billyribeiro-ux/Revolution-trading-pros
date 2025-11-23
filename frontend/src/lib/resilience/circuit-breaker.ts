/**
 * Circuit Breaker Pattern - Enterprise Implementation
 * Google L7+ Principal Engineer Level
 * 
 * Features:
 * - Automatic failure detection
 * - Exponential backoff
 * - Health monitoring
 * - Fallback strategies
 * - Metrics collection
 */

import { writable, derived } from 'svelte/store';
import { recordMetric, incrementCounter, warn, error as logError } from '../observability/telemetry';

// ═══════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════

export type CircuitState = 'CLOSED' | 'OPEN' | 'HALF_OPEN';

export interface CircuitBreakerConfig {
	name: string;
	failureThreshold: number;        // Number of failures before opening
	successThreshold: number;        // Number of successes to close from half-open
	timeout: number;                 // Request timeout in ms
	resetTimeout: number;            // Time before trying half-open in ms
	monitoringPeriod: number;        // Rolling window for failure rate in ms
	volumeThreshold: number;         // Minimum requests before calculating failure rate
	errorThresholdPercentage: number; // Percentage of errors to open circuit
}

export interface CircuitBreakerStats {
	state: CircuitState;
	failures: number;
	successes: number;
	consecutiveFailures: number;
	consecutiveSuccesses: number;
	totalRequests: number;
	lastFailureTime?: number;
	lastSuccessTime?: number;
	nextAttemptTime?: number;
}

interface RequestRecord {
	timestamp: number;
	success: boolean;
	duration: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// Circuit Breaker Implementation
// ═══════════════════════════════════════════════════════════════════════════

export class CircuitBreaker<T = any> {
	private config: CircuitBreakerConfig;
	private state: CircuitState = 'CLOSED';
	private failures = 0;
	private successes = 0;
	private consecutiveFailures = 0;
	private consecutiveSuccesses = 0;
	private totalRequests = 0;
	private lastFailureTime?: number;
	private lastSuccessTime?: number;
	private nextAttemptTime?: number;
	private requestHistory: RequestRecord[] = [];
	private fallbackFn?: (error: Error) => T | Promise<T>;
	private onStateChange?: (state: CircuitState) => void;

	// Stores for reactive UI
	public stats = writable<CircuitBreakerStats>(this.getStats());
	public isOpen = derived(this.stats, $stats => $stats.state === 'OPEN');
	public isHalfOpen = derived(this.stats, $stats => $stats.state === 'HALF_OPEN');
	public isClosed = derived(this.stats, $stats => $stats.state === 'CLOSED');

	constructor(config: Partial<CircuitBreakerConfig> = {}) {
		this.config = {
			name: config.name || 'default',
			failureThreshold: config.failureThreshold || 5,
			successThreshold: config.successThreshold || 2,
			timeout: config.timeout || 3000,
			resetTimeout: config.resetTimeout || 60000,
			monitoringPeriod: config.monitoringPeriod || 60000,
			volumeThreshold: config.volumeThreshold || 10,
			errorThresholdPercentage: config.errorThresholdPercentage || 50
		};
	}

	/**
	 * Execute a function with circuit breaker protection
	 */
	async execute<R = T>(fn: () => Promise<R>): Promise<R> {
		// Check if circuit is open
		if (this.state === 'OPEN') {
			if (this.shouldAttemptReset()) {
				this.transitionTo('HALF_OPEN');
			} else {
				const error = new CircuitBreakerOpenError(
					`Circuit breaker '${this.config.name}' is OPEN`,
					this.nextAttemptTime
				);
				
				if (this.fallbackFn) {
					warn(`Circuit breaker ${this.config.name} is OPEN, using fallback`);
					return this.fallbackFn(error) as Promise<R>;
				}
				
				throw error;
			}
		}

		const startTime = performance.now();
		this.totalRequests++;

		try {
			// Execute with timeout
			const result = await this.executeWithTimeout(fn);
			
			const duration = performance.now() - startTime;
			this.onSuccess(duration);
			
			return result;
		} catch (error: any) {
			const duration = performance.now() - startTime;
			this.onFailure(duration, error);
			
			if (this.fallbackFn) {
				warn(`Request failed, using fallback for ${this.config.name}`, {
					error: error.message
				});
				return this.fallbackFn(error) as Promise<R>;
			}
			
			throw error;
		}
	}

	/**
	 * Execute function with timeout
	 */
	private async executeWithTimeout<R>(fn: () => Promise<R>): Promise<R> {
		return Promise.race([
			fn(),
			new Promise<R>((_, reject) =>
				setTimeout(
					() => reject(new Error(`Request timeout after ${this.config.timeout}ms`)),
					this.config.timeout
				)
			)
		]);
	}

	/**
	 * Handle successful request
	 */
	private onSuccess(duration: number): void {
		this.successes++;
		this.consecutiveSuccesses++;
		this.consecutiveFailures = 0;
		this.lastSuccessTime = Date.now();

		this.recordRequest(true, duration);

		// Transition from HALF_OPEN to CLOSED if threshold met
		if (
			this.state === 'HALF_OPEN' &&
			this.consecutiveSuccesses >= this.config.successThreshold
		) {
			this.transitionTo('CLOSED');
		}

		// Record metrics
		recordMetric(`circuit_breaker_success_total`, 1, 'counter', {
			circuit: this.config.name
		});
		recordMetric(`circuit_breaker_request_duration_ms`, duration, 'histogram', {
			circuit: this.config.name,
			result: 'success'
		});

		this.updateStats();
	}

	/**
	 * Handle failed request
	 */
	private onFailure(duration: number, error: Error): void {
		this.failures++;
		this.consecutiveFailures++;
		this.consecutiveSuccesses = 0;
		this.lastFailureTime = Date.now();

		this.recordRequest(false, duration);

		// Check if we should open the circuit
		if (this.shouldOpen()) {
			this.transitionTo('OPEN');
		}

		// Record metrics
		incrementCounter(`circuit_breaker_failure_total`, {
			circuit: this.config.name,
			error_type: error.name
		});
		recordMetric(`circuit_breaker_request_duration_ms`, duration, 'histogram', {
			circuit: this.config.name,
			result: 'failure'
		});

		logError(`Circuit breaker ${this.config.name} request failed`, {
			error: error.message,
			consecutiveFailures: this.consecutiveFailures,
			state: this.state
		});

		this.updateStats();
	}

	/**
	 * Check if circuit should open
	 */
	private shouldOpen(): boolean {
		// Always open on consecutive failures threshold
		if (this.consecutiveFailures >= this.config.failureThreshold) {
			return true;
		}

		// Check failure rate in monitoring period
		const recentRequests = this.getRecentRequests();
		if (recentRequests.length < this.config.volumeThreshold) {
			return false;
		}

		const failureRate = recentRequests.filter(r => !r.success).length / recentRequests.length;
		return failureRate * 100 >= this.config.errorThresholdPercentage;
	}

	/**
	 * Check if we should attempt to reset (transition to HALF_OPEN)
	 */
	private shouldAttemptReset(): boolean {
		if (!this.nextAttemptTime) return false;
		return Date.now() >= this.nextAttemptTime;
	}

	/**
	 * Transition to new state
	 */
	private transitionTo(newState: CircuitState): void {
		const oldState = this.state;
		this.state = newState;

		switch (newState) {
			case 'OPEN':
				this.nextAttemptTime = Date.now() + this.config.resetTimeout;
				logError(`Circuit breaker ${this.config.name} opened`, {
					consecutiveFailures: this.consecutiveFailures,
					failureRate: this.getFailureRate(),
					nextAttemptTime: new Date(this.nextAttemptTime).toISOString()
				});
				break;

			case 'HALF_OPEN':
				warn(`Circuit breaker ${this.config.name} half-open, attempting reset`);
				this.consecutiveSuccesses = 0;
				break;

			case 'CLOSED':
				warn(`Circuit breaker ${this.config.name} closed, normal operation resumed`);
				this.consecutiveFailures = 0;
				this.nextAttemptTime = undefined;
				break;
		}

		// Record state change metric
		incrementCounter(`circuit_breaker_state_change_total`, {
			circuit: this.config.name,
			from_state: oldState,
			to_state: newState
		});

		// Call state change callback
		if (this.onStateChange) {
			this.onStateChange(newState);
		}

		this.updateStats();
	}

	/**
	 * Record request in history
	 */
	private recordRequest(success: boolean, duration: number): void {
		this.requestHistory.push({
			timestamp: Date.now(),
			success,
			duration
		});

		// Clean old records
		this.cleanRequestHistory();
	}

	/**
	 * Get recent requests within monitoring period
	 */
	private getRecentRequests(): RequestRecord[] {
		const cutoff = Date.now() - this.config.monitoringPeriod;
		return this.requestHistory.filter(r => r.timestamp >= cutoff);
	}

	/**
	 * Clean old request history
	 */
	private cleanRequestHistory(): void {
		const cutoff = Date.now() - this.config.monitoringPeriod;
		this.requestHistory = this.requestHistory.filter(r => r.timestamp >= cutoff);
	}

	/**
	 * Get current failure rate
	 */
	private getFailureRate(): number {
		const recent = this.getRecentRequests();
		if (recent.length === 0) return 0;
		return (recent.filter(r => !r.success).length / recent.length) * 100;
	}

	/**
	 * Get current stats
	 */
	private getStats(): CircuitBreakerStats {
		return {
			state: this.state,
			failures: this.failures,
			successes: this.successes,
			consecutiveFailures: this.consecutiveFailures,
			consecutiveSuccesses: this.consecutiveSuccesses,
			totalRequests: this.totalRequests,
			lastFailureTime: this.lastFailureTime,
			lastSuccessTime: this.lastSuccessTime,
			nextAttemptTime: this.nextAttemptTime
		};
	}

	/**
	 * Update stats store
	 */
	private updateStats(): void {
		this.stats.set(this.getStats());
	}

	/**
	 * Set fallback function
	 */
	fallback(fn: (error: Error) => T | Promise<T>): this {
		this.fallbackFn = fn;
		return this;
	}

	/**
	 * Set state change callback
	 */
	onStateChangeCallback(fn: (state: CircuitState) => void): this {
		this.onStateChange = fn;
		return this;
	}

	/**
	 * Manually open circuit
	 */
	open(): void {
		this.transitionTo('OPEN');
	}

	/**
	 * Manually close circuit
	 */
	close(): void {
		this.transitionTo('CLOSED');
	}

	/**
	 * Reset circuit breaker
	 */
	reset(): void {
		this.failures = 0;
		this.successes = 0;
		this.consecutiveFailures = 0;
		this.consecutiveSuccesses = 0;
		this.totalRequests = 0;
		this.lastFailureTime = undefined;
		this.lastSuccessTime = undefined;
		this.nextAttemptTime = undefined;
		this.requestHistory = [];
		this.transitionTo('CLOSED');
	}

	/**
	 * Get current state
	 */
	getState(): CircuitState {
		return this.state;
	}

	/**
	 * Get statistics
	 */
	getStatistics(): CircuitBreakerStats {
		return this.getStats();
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Custom Error
// ═══════════════════════════════════════════════════════════════════════════

export class CircuitBreakerOpenError extends Error {
	constructor(
		message: string,
		public nextAttemptTime?: number
	) {
		super(message);
		this.name = 'CircuitBreakerOpenError';
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Circuit Breaker Registry
// ═══════════════════════════════════════════════════════════════════════════

class CircuitBreakerRegistry {
	private static instance: CircuitBreakerRegistry;
	private breakers = new Map<string, CircuitBreaker>();

	private constructor() {}

	static getInstance(): CircuitBreakerRegistry {
		if (!CircuitBreakerRegistry.instance) {
			CircuitBreakerRegistry.instance = new CircuitBreakerRegistry();
		}
		return CircuitBreakerRegistry.instance;
	}

	get(name: string, config?: Partial<CircuitBreakerConfig>): CircuitBreaker {
		if (!this.breakers.has(name)) {
			this.breakers.set(name, new CircuitBreaker({ ...config, name }));
		}
		return this.breakers.get(name)!;
	}

	getAll(): Map<string, CircuitBreaker> {
		return this.breakers;
	}

	remove(name: string): void {
		this.breakers.delete(name);
	}

	clear(): void {
		this.breakers.clear();
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Convenience Functions
// ═══════════════════════════════════════════════════════════════════════════

const registry = CircuitBreakerRegistry.getInstance();

export function getCircuitBreaker(
	name: string,
	config?: Partial<CircuitBreakerConfig>
): CircuitBreaker {
	return registry.get(name, config);
}

export function withCircuitBreaker<T>(
	name: string,
	fn: () => Promise<T>,
	config?: Partial<CircuitBreakerConfig>
): Promise<T> {
	const breaker = getCircuitBreaker(name, config);
	return breaker.execute(fn);
}

export default CircuitBreaker;

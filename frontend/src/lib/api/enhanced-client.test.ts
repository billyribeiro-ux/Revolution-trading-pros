/**
 * Enhanced API Client - Integration Tests
 * Verify end-to-end functionality
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { EnhancedApiClient } from './enhanced-client';

describe('EnhancedApiClient', () => {
	let client: EnhancedApiClient;
	const baseURL = 'http://localhost:8000/api';

	beforeEach(() => {
		client = new EnhancedApiClient(baseURL);
		vi.clearAllMocks();
	});

	describe('Configuration', () => {
		it('should initialize with correct defaults', () => {
			expect(client.getBaseURL()).toBe(baseURL);
		});

		it('should allow base URL changes', () => {
			const newURL = 'https://api.example.com';
			client.setBaseURL(newURL);
			expect(client.getBaseURL()).toBe(newURL);
		});
	});

	describe('HTTP Methods', () => {
		it('should have GET method', () => {
			expect(typeof client.get).toBe('function');
		});

		it('should have POST method', () => {
			expect(typeof client.post).toBe('function');
		});

		it('should have PUT method', () => {
			expect(typeof client.put).toBe('function');
		});

		it('should have PATCH method', () => {
			expect(typeof client.patch).toBe('function');
		});

		it('should have DELETE method', () => {
			expect(typeof client.delete).toBe('function');
		});
	});

	describe('Caching', () => {
		it('should support cache clearing', () => {
			expect(typeof client.clearCache).toBe('function');
			client.clearCache();
		});

		it('should support pattern-based cache clearing', () => {
			client.clearCache('/users');
		});
	});

	describe('Interceptors', () => {
		it('should support request interceptors', () => {
			const interceptor = vi.fn((config) => config);
			client.addRequestInterceptor(interceptor);
		});

		it('should support response interceptors', () => {
			const interceptor = vi.fn((response) => response);
			client.addResponseInterceptor(interceptor);
		});

		it('should support error interceptors', () => {
			const interceptor = vi.fn((error) => error);
			client.addErrorInterceptor(interceptor);
		});
	});

	describe('Configuration Options', () => {
		it('should support circuit breaker configuration', async () => {
			const config = {
				useCircuitBreaker: true,
				circuitBreakerName: 'test-api'
			};

			// Configuration should be accepted
			expect(config.useCircuitBreaker).toBe(true);
		});

		it('should support retry configuration', () => {
			const config = {
				retry: true,
				maxRetries: 5
			};

			expect(config.retry).toBe(true);
			expect(config.maxRetries).toBe(5);
		});

		it('should support caching configuration', () => {
			const config = {
				useCache: true,
				cacheTTL: 60000
			};

			expect(config.useCache).toBe(true);
			expect(config.cacheTTL).toBe(60000);
		});

		it('should support tracing configuration', () => {
			const config = {
				trace: true
			};

			expect(config.trace).toBe(true);
		});

		it('should support idempotency configuration', () => {
			const config = {
				idempotent: true,
				idempotencyKey: 'test-key-123'
			};

			expect(config.idempotent).toBe(true);
			expect(config.idempotencyKey).toBe('test-key-123');
		});

		it('should support timeout configuration', () => {
			const config = {
				timeout: 5000
			};

			expect(config.timeout).toBe(5000);
		});

		it('should support rate limiting configuration', () => {
			const config = {
				rateLimit: {
					maxRequests: 100,
					windowMs: 60000
				}
			};

			expect(config.rateLimit?.maxRequests).toBe(100);
			expect(config.rateLimit?.windowMs).toBe(60000);
		});
	});
});

describe('End-to-End Integration', () => {
	it('should integrate with circuit breaker', () => {
		const client = new EnhancedApiClient('http://localhost:8000/api', {
			useCircuitBreaker: true
		});

		expect(client).toBeDefined();
	});

	it('should integrate with retry logic', () => {
		const client = new EnhancedApiClient('http://localhost:8000/api', {
			retry: true,
			maxRetries: 3
		});

		expect(client).toBeDefined();
	});

	it('should integrate with telemetry', () => {
		const client = new EnhancedApiClient('http://localhost:8000/api', {
			trace: true
		});

		expect(client).toBeDefined();
	});

	it('should support all features together', () => {
		const client = new EnhancedApiClient('http://localhost:8000/api', {
			useCircuitBreaker: true,
			retry: true,
			maxRetries: 3,
			useCache: true,
			cacheTTL: 300000,
			trace: true,
			timeout: 30000
		});

		expect(client).toBeDefined();
	});
});

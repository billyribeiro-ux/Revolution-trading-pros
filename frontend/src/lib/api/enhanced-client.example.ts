/**
 * Enhanced API Client - Usage Examples
 * Demonstrates end-to-end integration with backend
 */

import { apiClient } from './enhanced-client';

// ═══════════════════════════════════════════════════════════════════════════
// Example 1: Simple GET Request with Caching
// ═══════════════════════════════════════════════════════════════════════════

export async function getUsers() {
	try {
		const response = await apiClient.get('/users', {
			useCache: true,
			cacheTTL: 300000 // 5 minutes
		});

		return response.data;
	} catch (error) {
		console.error('Failed to fetch users:', error);
		throw error;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Example 2: POST Request with Circuit Breaker and Retry
// ═══════════════════════════════════════════════════════════════════════════

export async function createOrder(orderData: any) {
	try {
		const response = await apiClient.post('/orders', orderData, {
			useCircuitBreaker: true,
			circuitBreakerName: 'orders-api',
			retry: true,
			maxRetries: 3,
			idempotent: true,
			timeout: 10000
		});

		return response.data;
	} catch (error) {
		console.error('Failed to create order:', error);
		throw error;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Example 3: Payment Processing with All Features
// ═══════════════════════════════════════════════════════════════════════════

export async function processPayment(paymentData: any) {
	try {
		const response = await apiClient.post('/payments/charge', paymentData, {
			// Circuit breaker for payment service
			useCircuitBreaker: true,
			circuitBreakerName: 'payment-service',

			// Retry with exponential backoff
			retry: true,
			maxRetries: 3,

			// Ensure idempotency
			idempotent: true,
			idempotencyKey: `payment-${paymentData.orderId}-${Date.now()}`,

			// Distributed tracing
			trace: true,

			// Timeout after 30 seconds
			timeout: 30000,

			// Rate limiting
			rateLimit: {
				maxRequests: 10,
				windowMs: 60000 // 10 requests per minute
			}
		});

		return response.data;
	} catch (error) {
		console.error('Payment processing failed:', error);
		throw error;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Example 4: Subscription Management
// ═══════════════════════════════════════════════════════════════════════════

export async function getSubscriptions() {
	const response = await apiClient.get('/my/subscriptions', {
		useCache: true,
		cacheTTL: 60000, // 1 minute
		useCircuitBreaker: true,
		retry: true
	});

	return response.data;
}

export async function createSubscription(subscriptionData: any) {
	const response = await apiClient.post('/my/subscriptions', subscriptionData, {
		useCircuitBreaker: true,
		retry: true,
		idempotent: true,
		trace: true
	});

	return response.data;
}

export async function cancelSubscription(subscriptionId: string, reason: string) {
	const response = await apiClient.post(
		`/my/subscriptions/${subscriptionId}/cancel`,
		{ reason },
		{
			useCircuitBreaker: true,
			retry: true,
			idempotent: true
		}
	);

	return response.data;
}

// ═══════════════════════════════════════════════════════════════════════════
// Example 5: Form Submission
// ═══════════════════════════════════════════════════════════════════════════

export async function submitForm(slug: string, formData: any) {
	const response = await apiClient.post(`/forms/${slug}/submit`, formData, {
		retry: true,
		maxRetries: 2,
		idempotent: true,
		trace: true
	});

	return response.data;
}

// ═══════════════════════════════════════════════════════════════════════════
// Example 6: Using Request Interceptors
// ═══════════════════════════════════════════════════════════════════════════

// Add custom headers to all requests
apiClient.addRequestInterceptor(async (config) => {
	return {
		...config,
		headers: {
			...config.headers,
			'X-App-Version': '2.0.0',
			'X-Request-ID': crypto.randomUUID()
		}
	};
});

// ═══════════════════════════════════════════════════════════════════════════
// Example 7: Using Response Interceptors
// ═══════════════════════════════════════════════════════════════════════════

// Log all successful responses
apiClient.addResponseInterceptor(async (response) => {
	console.log(`API Response: ${response.status} - ${response.duration}ms`);
	return response;
});

// ═══════════════════════════════════════════════════════════════════════════
// Example 8: Using Error Interceptors
// ═══════════════════════════════════════════════════════════════════════════

// Handle specific error codes
apiClient.addErrorInterceptor(async (error: any) => {
	if (error.status === 429) {
		// Rate limited - show user-friendly message
		console.warn('Rate limit exceeded. Please try again later.');
	}

	if (error.status === 503) {
		// Service unavailable - circuit breaker likely open
		console.warn('Service temporarily unavailable. Please try again.');
	}

	return error;
});

// ═══════════════════════════════════════════════════════════════════════════
// Example 9: Cache Management
// ═══════════════════════════════════════════════════════════════════════════

export function clearUserCache() {
	// Clear all cached user-related requests
	apiClient.clearCache('/users');
}

export function clearAllCache() {
	// Clear entire cache
	apiClient.clearCache();
}

// ═══════════════════════════════════════════════════════════════════════════
// Example 10: Backend Integration - Health Check
// ═══════════════════════════════════════════════════════════════════════════

export async function checkBackendHealth() {
	try {
		const response = await apiClient.get('/health', {
			useCache: false,
			timeout: 5000
		});

		return {
			healthy: response.status === 200,
			data: response.data
		};
	} catch (error) {
		return {
			healthy: false,
			error
		};
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Example 11: Batch Requests
// ═══════════════════════════════════════════════════════════════════════════

export async function fetchDashboardData() {
	try {
		// Execute multiple requests in parallel
		const [users, subscriptions, orders, metrics] = await Promise.all([
			apiClient.get('/users', { useCache: true }),
			apiClient.get('/my/subscriptions', { useCache: true }),
			apiClient.get('/orders', { useCache: true }),
			apiClient.get('/my/subscriptions/metrics', { useCache: true })
		]);

		return {
			users: users.data,
			subscriptions: subscriptions.data,
			orders: orders.data,
			metrics: metrics.data
		};
	} catch (error) {
		console.error('Failed to fetch dashboard data:', error);
		throw error;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Example 12: File Upload
// ═══════════════════════════════════════════════════════════════════════════

export async function uploadFile(file: File) {
	const formData = new FormData();
	formData.append('file', file);

	const response = await apiClient.post('/upload', formData, {
		headers: {
			// Let browser set Content-Type with boundary
		},
		retry: false, // Don't retry file uploads
		timeout: 60000 // 1 minute for large files
	});

	return response.data;
}

// ═══════════════════════════════════════════════════════════════════════════
// Example 13: Polling with Circuit Breaker
// ═══════════════════════════════════════════════════════════════════════════

export async function pollJobStatus(jobId: string): Promise<any> {
	const maxAttempts = 30;
	const pollInterval = 2000; // 2 seconds

	for (let attempt = 0; attempt < maxAttempts; attempt++) {
		try {
			const response = await apiClient.get(`/jobs/${jobId}`, {
				useCache: false,
				useCircuitBreaker: true,
				retry: false // Don't retry polls
			});

			if (response.data.status === 'completed') {
				return response.data;
			}

			if (response.data.status === 'failed') {
				throw new Error('Job failed');
			}

			// Wait before next poll
			await new Promise((resolve) => setTimeout(resolve, pollInterval));
		} catch (error) {
			console.error(`Poll attempt ${attempt + 1} failed:`, error);

			if (attempt === maxAttempts - 1) {
				throw error;
			}
		}
	}

	throw new Error('Job polling timeout');
}

// ═══════════════════════════════════════════════════════════════════════════
// Example 14: Real-world Integration - E-commerce Checkout
// ═══════════════════════════════════════════════════════════════════════════

export async function completeCheckout(checkoutData: {
	cartId: string;
	paymentMethod: any;
	shippingAddress: any;
	billingAddress: any;
}) {
	try {
		// Step 1: Validate cart
		await apiClient.get(`/cart/${checkoutData.cartId}`, {
			useCache: false,
			useCircuitBreaker: true
		});

		// Step 2: Create order
		const order = await apiClient.post(
			'/orders',
			{
				cartId: checkoutData.cartId,
				shippingAddress: checkoutData.shippingAddress,
				billingAddress: checkoutData.billingAddress
			},
			{
				useCircuitBreaker: true,
				retry: true,
				idempotent: true,
				trace: true
			}
		);

		// Step 3: Process payment
		const payment = await apiClient.post(
			'/payments/charge',
			{
				orderId: order.data.id,
				amount: order.data.total,
				paymentMethod: checkoutData.paymentMethod
			},
			{
				useCircuitBreaker: true,
				circuitBreakerName: 'payment-service',
				retry: true,
				maxRetries: 3,
				idempotent: true,
				idempotencyKey: `payment-${order.data.id}`,
				timeout: 30000
			}
		);

		// Step 4: Clear cart cache
		apiClient.clearCache(`/cart/${checkoutData.cartId}`);

		return {
			order: order.data,
			payment: payment.data
		};
	} catch (error) {
		console.error('Checkout failed:', error);
		throw error;
	}
}

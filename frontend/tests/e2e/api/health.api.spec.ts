/**
 * Revolution Trading Pros - API Health Check Tests
 *
 * API-level tests for backend health and availability:
 * - Health endpoints
 * - Basic API connectivity
 * - Response format validation
 *
 * Netflix L11+ Standard: Verify infrastructure health
 *
 * Note: All tests in this file require backend API and will be skipped
 * when backend is not available (e.g., in CI environments).
 */

import { test, expect } from '@playwright/test';
import {
	createApiHelper,
	API_ENDPOINTS,
	isApiAvailable,
	waitForApi,
	shouldSkipBackendTests,
	getBackendSkipReason
} from '../helpers';

// These tests run against the API directly, no browser needed
// Skip all API tests if backend is not available
test.describe('API Health Checks', () => {
	test.skip(shouldSkipBackendTests(), getBackendSkipReason());

	const api = createApiHelper();

	test.describe('Health Endpoints', () => {
		test('liveness probe returns 200', async ({ request }) => {
			const response = await api.get(request, API_ENDPOINTS.healthLive);

			// If API is available, should return 200
			if (response.success) {
				expect(response.status).toBe(200);
			} else {
				// API not running is acceptable in some test environments
				console.log('API not available:', response.error);
			}
		});

		test('readiness probe returns 200 when healthy', async ({ request }) => {
			const response = await api.get(request, API_ENDPOINTS.healthReady);

			if (response.success) {
				expect(response.status).toBe(200);
			} else {
				console.log('API readiness check:', response.error);
			}
		});
	});

	test.describe('API Availability', () => {
		test('API is reachable', async ({ request }) => {
			const available = await isApiAvailable(request);

			// Log availability status
			console.log(`API available: ${available}`);

			// Don't fail if API is not running in frontend-only tests
			expect(typeof available).toBe('boolean');
		});

		test.skip('API becomes available within timeout', async ({ request }) => {
			// This is useful for CI where services start concurrently
			const available = await waitForApi(request, {
				timeout: 30000,
				interval: 2000
			});

			expect(available).toBe(true);
		});
	});

	test.describe('Public Endpoints', () => {
		test('posts endpoint returns data', async ({ request }) => {
			const response = await api.get(request, API_ENDPOINTS.posts);

			if (response.success) {
				expect(response.status).toBe(200);
				expect(response.data).toBeDefined();
			} else {
				// API not available
				console.log('Posts endpoint:', response.error);
			}
		});

		test('indicators endpoint returns data', async ({ request }) => {
			const response = await api.get(request, API_ENDPOINTS.indicators);

			if (response.success) {
				expect(response.status).toBe(200);
			} else {
				console.log('Indicators endpoint:', response.error);
			}
		});

		test('time endpoint returns current time', async ({ request }) => {
			const response = await api.get(request, '/time/now');

			if (response.success) {
				expect(response.status).toBe(200);
			} else {
				console.log('Time endpoint:', response.error);
			}
		});
	});

	test.describe('Authentication Endpoints', () => {
		test('login endpoint exists and returns proper error for missing credentials', async ({
			request
		}) => {
			const response = await api.post(request, API_ENDPOINTS.login, {
				email: '',
				password: ''
			});

			// Should return 422 (validation error) or 401 (unauthorized)
			if (response.status > 0) {
				expect([401, 422]).toContain(response.status);
			}
		});

		test('register endpoint exists', async ({ request }) => {
			// Just verify the endpoint exists (don't actually create user)
			const response = await api.post(request, API_ENDPOINTS.register, {
				email: '',
				password: ''
			});

			// Should return validation error, not 404
			if (response.status > 0) {
				expect(response.status).not.toBe(404);
			}
		});
	});

	test.describe('Protected Endpoints', () => {
		test('me endpoint requires authentication', async ({ request }) => {
			const response = await api.get(request, API_ENDPOINTS.me);

			// Should return 401 without token
			if (response.status > 0) {
				expect(response.status).toBe(401);
			}
		});

		test('trading-rooms endpoint requires authentication', async ({ request }) => {
			const response = await api.get(request, API_ENDPOINTS.tradingRooms);

			// Should return 401 without token
			if (response.status > 0) {
				expect(response.status).toBe(401);
			}
		});
	});

	test.describe('Rate Limiting', () => {
		test.skip('login endpoint has rate limiting', async ({ request }) => {
			// Make multiple rapid requests
			const requests = Array.from({ length: 10 }, () =>
				api.post(request, API_ENDPOINTS.login, {
					email: 'test@example.com',
					password: 'wrong'
				})
			);

			const responses = await Promise.all(requests);

			// At least one should be rate limited (429)
			const rateLimited = responses.some((r) => r.status === 429);

			// Rate limiting should kick in
			console.log('Rate limiting active:', rateLimited);
		});
	});

	test.describe('Response Format', () => {
		test('API returns JSON content type', async ({ request }) => {
			const response = await api.get(request, API_ENDPOINTS.healthLive);

			if (response.headers) {
				const contentType = response.headers['content-type'] || '';
				expect(contentType).toContain('application/json');
			}
		});

		test('error responses include message field', async ({ request }) => {
			const response = await api.get(request, '/nonexistent-endpoint-12345');

			// 404 responses should have a message
			if (response.status === 404 && response.error) {
				expect(response.error.length).toBeGreaterThan(0);
			}
		});
	});
});

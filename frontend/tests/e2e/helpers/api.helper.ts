/// <reference types="node" />
/**
 * Revolution Trading Pros - API Helper
 *
 * Provides utilities for API interactions during E2E tests:
 * - Direct API calls for setup/teardown
 * - Response validation
 * - Error handling
 * - Rate limiting awareness
 *
 * Netflix L11+ Pattern: Decouple API testing from UI
 */

import { type Page, type APIRequestContext, expect } from '@playwright/test';

/** API Response wrapper */
export interface ApiResponse<T = unknown> {
	success: boolean;
	status: number;
	data?: T;
	error?: string;
	headers?: Record<string, string>;
}

/** Standard API endpoints */
export const API_ENDPOINTS = {
	// Auth
	login: '/login',
	register: '/register',
	logout: '/logout',
	me: '/me',
	forgotPassword: '/forgot-password',
	resetPassword: '/reset-password',

	// Health
	healthLive: '/health/live',
	healthReady: '/health/ready',

	// Posts/Blog
	posts: '/posts',
	postBySlug: (slug: string) => `/posts/${slug}`,

	// Trading Rooms
	tradingRooms: '/trading-rooms',
	tradingRoomBySlug: (slug: string) => `/trading-rooms/${slug}`,

	// Subscriptions
	mySubscriptions: '/my/subscriptions',

	// Cart
	cart: '/cart',
	checkout: '/cart/checkout',

	// Indicators
	indicators: '/indicators',
	indicatorBySlug: (slug: string) => `/indicators/${slug}`,

	// Admin
	adminUsers: '/admin/users',
	adminPosts: '/admin/posts',
	adminProducts: '/admin/products'
} as const;

/**
 * Creates an API helper instance with base URL configuration
 */
export function createApiHelper(baseUrl?: string) {
	const apiUrl = baseUrl || process.env.E2E_API_URL || 'https://revolution-trading-pros-api.fly.dev/api';

	return {
		baseUrl: apiUrl,

		/**
		 * Makes a GET request
		 */
		async get<T>(
			request: APIRequestContext,
			endpoint: string,
			options?: { token?: string; params?: Record<string, string> }
		): Promise<ApiResponse<T>> {
			return this.request<T>(request, 'GET', endpoint, undefined, options);
		},

		/**
		 * Makes a POST request
		 */
		async post<T>(
			request: APIRequestContext,
			endpoint: string,
			data?: unknown,
			options?: { token?: string }
		): Promise<ApiResponse<T>> {
			return this.request<T>(request, 'POST', endpoint, data, options);
		},

		/**
		 * Makes a PUT request
		 */
		async put<T>(
			request: APIRequestContext,
			endpoint: string,
			data?: unknown,
			options?: { token?: string }
		): Promise<ApiResponse<T>> {
			return this.request<T>(request, 'PUT', endpoint, data, options);
		},

		/**
		 * Makes a DELETE request
		 */
		async delete<T>(
			request: APIRequestContext,
			endpoint: string,
			options?: { token?: string }
		): Promise<ApiResponse<T>> {
			return this.request<T>(request, 'DELETE', endpoint, undefined, options);
		},

		/**
		 * Generic request handler
		 */
		async request<T>(
			request: APIRequestContext,
			method: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH',
			endpoint: string,
			data?: unknown,
			options?: { token?: string; params?: Record<string, string> }
		): Promise<ApiResponse<T>> {
			const url = new URL(endpoint.startsWith('/') ? endpoint : `/${endpoint}`, apiUrl);

			if (options?.params) {
				Object.entries(options.params).forEach(([key, value]) => {
					url.searchParams.set(key, value);
				});
			}

			const headers: Record<string, string> = {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			};

			if (options?.token) {
				headers.Authorization = `Bearer ${options.token}`;
			}

			try {
				const response = await request.fetch(url.toString(), {
					method,
					headers,
					data: data ? JSON.stringify(data) : undefined
				});

				const responseHeaders: Record<string, string> = response.headers();

				if (response.ok()) {
					const responseData = await response.json().catch(() => null);
					return {
						success: true,
						status: response.status(),
						data: responseData as T,
						headers: responseHeaders
					};
				}

				const errorData = await response.json().catch(() => ({ message: 'Unknown error' }));
				return {
					success: false,
					status: response.status(),
					error: errorData.message || `Request failed with status ${response.status()}`,
					headers: responseHeaders
				};
			} catch (error) {
				return {
					success: false,
					status: 0,
					error: error instanceof Error ? error.message : 'Network error'
				};
			}
		}
	};
}

/**
 * Checks if the API is available
 */
export async function isApiAvailable(request: APIRequestContext): Promise<boolean> {
	const api = createApiHelper();

	try {
		const response = await api.get(request, API_ENDPOINTS.healthLive);
		return response.success || response.status === 200;
	} catch {
		return false;
	}
}

/**
 * Waits for API to become available (useful for CI)
 */
export async function waitForApi(
	request: APIRequestContext,
	options: { timeout?: number; interval?: number } = {}
): Promise<boolean> {
	const timeout = options.timeout || 30000;
	const interval = options.interval || 1000;
	const startTime = Date.now();

	while (Date.now() - startTime < timeout) {
		if (await isApiAvailable(request)) {
			return true;
		}
		await new Promise((resolve) => setTimeout(resolve, interval));
	}

	return false;
}

/**
 * Gets the current authenticated user's info
 */
export async function getCurrentUser(
	request: APIRequestContext,
	token: string
): Promise<ApiResponse<{ id: number; email: string; name: string }>> {
	const api = createApiHelper();
	return api.get(request, API_ENDPOINTS.me, { token });
}

/**
 * Creates a test data seed request (for test environments only)
 */
export async function seedTestData(
	request: APIRequestContext,
	seedType: 'user' | 'posts' | 'products' | 'orders',
	token: string
): Promise<ApiResponse> {
	const api = createApiHelper();
	return api.post(request, `/test/seed/${seedType}`, {}, { token });
}

/**
 * Cleans up test data (for test environments only)
 */
export async function cleanupTestData(
	request: APIRequestContext,
	cleanupType: 'user' | 'all',
	token: string
): Promise<ApiResponse> {
	const api = createApiHelper();
	return api.post(request, `/test/cleanup/${cleanupType}`, {}, { token });
}

/**
 * Gets posts from the API
 */
export async function getPosts(
	request: APIRequestContext,
	options?: { page?: number; limit?: number }
): Promise<ApiResponse<{ data: Array<{ id: number; title: string; slug: string }> }>> {
	const api = createApiHelper();
	return api.get(request, API_ENDPOINTS.posts, {
		params: {
			page: String(options?.page || 1),
			limit: String(options?.limit || 10)
		}
	});
}

/**
 * Gets trading rooms from the API
 */
export async function getTradingRooms(
	request: APIRequestContext,
	token?: string
): Promise<
	ApiResponse<{ data: Array<{ id: number; name: string; slug: string; is_active: boolean }> }>
> {
	const api = createApiHelper();
	return api.get(request, API_ENDPOINTS.tradingRooms, { token });
}

/**
 * Asserts API response is successful
 */
export function expectApiSuccess<T>(response: ApiResponse<T>): asserts response is ApiResponse<T> & {
	success: true;
	data: T;
} {
	expect(response.success).toBe(true);
	expect(response.data).toBeDefined();
}

/**
 * Asserts API response status code
 */
export function expectApiStatus(response: ApiResponse, expectedStatus: number): void {
	expect(response.status).toBe(expectedStatus);
}

/**
 * API Configuration
 * Central configuration for API endpoints and base URLs
 * Enhanced with better type safety and error handling
 */

import { getAuthToken } from '$lib/stores/auth';

// ICT11+ Pattern: Use relative URLs in development to leverage Vite proxy
// This prevents CORS issues by routing all API calls through the same origin
// In production, use the full API URL from environment variables
const isDev = import.meta.env.DEV;

// ICT11+ PRODUCTION FIX: Hardcode the production API URL since Cloudflare Pages
// secrets are not available via import.meta.env on server-side
// The VITE_API_URL secret is only for client-side code
const PRODUCTION_API_URL = 'https://revolution-trading-pros-api.billy-ribeiro.workers.dev';

// Get API base URL - empty string in dev (uses Vite proxy), full URL in production
export const API_BASE_URL = isDev 
	? '' 
	: (import.meta.env['VITE_API_URL'] || PRODUCTION_API_URL);

export const CDN_URL = import.meta.env['VITE_CDN_URL'] || 'http://localhost:8000/storage';
export const WS_URL = import.meta.env['VITE_WS_URL'] || 'ws://localhost:8000';

/**
 * API Version
 */
export const API_VERSION = 'v1';

/**
 * API endpoints with type-safe builders
 */
export const API_ENDPOINTS = {
	// Authentication
	auth: {
		login: '/api/auth/login',
		register: '/api/auth/register',
		logout: '/api/auth/logout',
		refresh: '/api/auth/refresh',
		me: '/api/auth/me',
		forgotPassword: '/api/auth/forgot-password',
		resetPassword: '/api/auth/reset-password'
	},

	// Posts
	posts: {
		list: '/api/posts',
		single: (slug: string) => `/api/posts/${slug}`,
		create: '/api/posts',
		update: (id: number) => `/api/posts/${id}`,
		delete: (id: number) => `/api/posts/${id}`,
		like: (id: number) => `/api/posts/${id}/like`,
		unlike: (id: number) => `/api/posts/${id}/unlike`
	},

	// Products/Indicators
	products: {
		list: '/api/products',
		single: (slug: string) => `/api/products/${slug}`,
		categories: '/api/products/categories'
	},

	indicators: {
		list: '/api/indicators',
		single: (slug: string) => `/api/indicators/${slug}`,
		download: (slug: string, downloadId: string) =>
			`/api/indicators/${slug}/download/${downloadId}`,
		purchased: '/api/indicators/purchased'
	},

	// Memberships
	memberships: {
		plans: '/api/memberships/plans',
		current: '/api/memberships/current',
		subscribe: '/api/memberships/subscribe',
		cancel: '/api/memberships/cancel',
		update: '/api/memberships/update'
	},

	// User
	user: {
		profile: '/api/user/profile',
		update: '/api/user/profile',
		memberships: '/api/user/memberships',
		products: '/api/user/products',
		notifications: '/api/user/notifications',
		preferences: '/api/user/preferences'
	},

	// Newsletter
	newsletter: {
		subscribe: '/api/newsletter/subscribe',
		unsubscribe: '/api/newsletter/unsubscribe'
	},

	// Cart/Checkout
	cart: {
		checkout: '/api/cart/checkout',
		add: '/api/cart/add',
		remove: '/api/cart/remove',
		clear: '/api/cart/clear'
	},

	// Trading Room
	tradingRoom: {
		sessions: '/api/trading-room/sessions',
		join: (id: string) => `/api/trading-room/sessions/${id}/join`,
		leave: (id: string) => `/api/trading-room/sessions/${id}/leave`,
		signals: '/api/trading-room/signals'
	},

	// Analytics
	analytics: {
		dashboard: '/api/analytics/dashboard',
		performance: '/api/analytics/performance',
		trades: '/api/analytics/trades'
	},

	// Admin
	admin: {
		users: '/api/admin/users',
		stats: '/api/admin/stats',
		settings: '/api/admin/settings'
	},

	// Upload
	upload: '/api/upload',

	// Search
	search: '/api/search',

	// Time
	time: {
		now: '/api/time/now'
	},

	// Timers
	timers: {
		events: '/api/timers/events'
	}
} as const;

/**
 * Request configuration defaults
 */
export const REQUEST_CONFIG = {
	timeout: 30000, // 30 seconds
	retries: 3,
	retryDelay: 1000, // 1 second
	headers: {
		'Content-Type': 'application/json',
		Accept: 'application/json'
	}
} as const;

/**
 * Error messages
 */
export const ERROR_MESSAGES = {
	NETWORK_ERROR: 'Network error. Please check your connection.',
	TIMEOUT_ERROR: 'Request timed out. Please try again.',
	UNAUTHORIZED: 'You are not authorized. Please login.',
	FORBIDDEN: 'You do not have permission to access this resource.',
	NOT_FOUND: 'The requested resource was not found.',
	SERVER_ERROR: 'Server error. Please try again later.',
	VALIDATION_ERROR: 'Please check your input and try again.'
} as const;

/**
 * Enhanced fetch wrapper with error handling and retries
 * Accepts optional fetch function for SSR compatibility
 */
export async function apiFetch<T>(
	endpoint: string,
	options: RequestInit & {
		params?: Record<string, any>;
		retries?: number;
		retryDelay?: number;
		fetch?: typeof fetch; // Optional fetch function for SSR
	} = {}
): Promise<T> {
	// Use provided fetch or fall back to global fetch
	const fetchFn = options.fetch || (typeof window !== 'undefined' ? window.fetch : fetch);

	// Build URL with params
	let url = `${API_BASE_URL}${endpoint}`;

	if (options.params) {
		const queryString = new URLSearchParams(options.params).toString();
		url += `?${queryString}`;
	}

	// Get auth token from secure auth store (memory-only, not localStorage)
	const token = typeof window !== 'undefined' ? getAuthToken() : null;

	// Merge headers
	const headers: Record<string, string> = {
		...REQUEST_CONFIG.headers,
		...(options.headers as Record<string, string> | undefined)
	};

	if (token) {
		headers['Authorization'] = `Bearer ${token}`;
	}

	// Setup retry logic
	const maxRetries = options.retries ?? REQUEST_CONFIG.retries;
	const retryDelay = options.retryDelay ?? REQUEST_CONFIG.retryDelay;
	let lastError: Error | null = null;

	for (let attempt = 0; attempt < maxRetries; attempt++) {
		try {
			// Create abort controller for timeout
			const controller = new AbortController();
			const timeoutId = setTimeout(() => controller.abort(), REQUEST_CONFIG.timeout);

			const response = await fetchFn(url, {
				...options,
				headers,
				signal: controller.signal
			});

			clearTimeout(timeoutId);

			// Handle error responses
			if (!response.ok) {
				const errorData = await response.json().catch(() => ({}));

				// Don't retry on client errors (except 429)
				if (response.status >= 400 && response.status < 500 && response.status !== 429) {
					throw new ApiError(
						errorData.message || getErrorMessage(response.status),
						response.status,
						errorData
					);
				}

				// Retry on server errors
				throw new ApiError(
					errorData.message || getErrorMessage(response.status),
					response.status,
					errorData
				);
			}

			// Success - return parsed response
			return await response.json();
		} catch (error) {
			lastError = error as Error;

			// Don't retry on abort or client errors
			if (error instanceof Error) {
				if (error.name === 'AbortError') {
					throw new ApiError(ERROR_MESSAGES.TIMEOUT_ERROR, 0);
				}

				if (
					error instanceof ApiError &&
					error.status >= 400 &&
					error.status < 500 &&
					error.status !== 429
				) {
					throw error;
				}
			}

			// Wait before retry (exponential backoff)
			if (attempt < maxRetries - 1) {
				await new Promise((resolve) => setTimeout(resolve, retryDelay * Math.pow(2, attempt)));
			}
		}
	}

	// All retries failed
	throw lastError || new ApiError(ERROR_MESSAGES.NETWORK_ERROR, 0);
}

/**
 * Custom API Error class
 */
export class ApiError extends Error {
	constructor(
		message: string,
		public status: number,
		public data?: any
	) {
		super(message);
		this.name = 'ApiError';
	}
}

/**
 * Get error message by status code
 */
function getErrorMessage(status: number): string {
	switch (status) {
		case 401:
			return ERROR_MESSAGES.UNAUTHORIZED;
		case 403:
			return ERROR_MESSAGES.FORBIDDEN;
		case 404:
			return ERROR_MESSAGES.NOT_FOUND;
		case 422:
			return ERROR_MESSAGES.VALIDATION_ERROR;
		case 500:
		case 502:
		case 503:
			return ERROR_MESSAGES.SERVER_ERROR;
		default:
			return `API Error: ${status}`;
	}
}

/**
 * Convenience methods
 */
export const api = {
	get: <T = any>(endpoint: string, params?: Record<string, any>) =>
		apiFetch<T>(endpoint, params ? { method: 'GET', params } : { method: 'GET' }),

	post: <T = any>(endpoint: string, data?: any, params?: Record<string, any>) =>
		apiFetch<T>(endpoint, {
			method: 'POST',
			...(data ? { body: JSON.stringify(data) } : {}),
			...(params ? { params } : {})
		}),

	put: <T = any>(endpoint: string, data?: any, params?: Record<string, any>) =>
		apiFetch<T>(endpoint, {
			method: 'PUT',
			...(data ? { body: JSON.stringify(data) } : {}),
			...(params ? { params } : {})
		}),

	patch: <T = any>(endpoint: string, data?: any, params?: Record<string, any>) =>
		apiFetch<T>(endpoint, {
			method: 'PATCH',
			...(data ? { body: JSON.stringify(data) } : {}),
			...(params ? { params } : {})
		}),

	delete: <T = any>(endpoint: string, params?: Record<string, any>) =>
		apiFetch<T>(endpoint, params ? { method: 'DELETE', params } : { method: 'DELETE' })
};

/**
 * Build full URL for external use
 */
export function buildApiUrl(endpoint: string, params?: Record<string, any>): string {
	let url = `${API_BASE_URL}${endpoint}`;

	if (params) {
		const queryString = new URLSearchParams(params).toString();
		url += `?${queryString}`;
	}

	return url;
}

/**
 * Build CDN URL for assets
 */
export function buildCdnUrl(path: string): string {
	if (path.startsWith('http')) {
		return path;
	}
	return `${CDN_URL}${path.startsWith('/') ? path : '/' + path}`;
}

/**
 * Check if error is an API error
 */
export function isApiError(error: any): error is ApiError {
	return error instanceof ApiError;
}

/**
 * Extract validation errors from API error
 */
export function getValidationErrors(error: any): Record<string, string[]> {
	if (isApiError(error) && error.data?.errors) {
		return error.data.errors;
	}
	return {};
}

export default api;

/**
 * API Configuration
 * Central configuration for API endpoints and base URLs
 * Enhanced with better type safety and error handling
 */

import { getAuthToken } from '$lib/stores/auth.svelte';

// ICT11+ PRODUCTION DEPLOYMENT: Always use deployed URLs
// Backend is deployed on Fly.io (Rust + Axum)
const PRODUCTION_API_URL = 'https://revolution-trading-pros-api.fly.dev';
const PRODUCTION_CDN_URL = 'https://pub-2e5bd1b702b440bd888a0fc47f3493ae.r2.dev';
const PRODUCTION_WS_URL = 'wss://revolution-trading-pros-api.fly.dev';

// Always use production URLs - no localhost
export const API_BASE_URL = import.meta.env['VITE_API_URL'] || PRODUCTION_API_URL;

export const CDN_URL = import.meta.env['VITE_CDN_URL'] || PRODUCTION_CDN_URL;

export const WS_URL = import.meta.env['VITE_WS_URL'] || PRODUCTION_WS_URL;

// Deprecated: Use API_BASE_URL instead (kept for backward compatibility)
export const BACKEND_URL = API_BASE_URL;

export const ML_API_URL = import.meta.env['VITE_ML_API'] || `${PRODUCTION_API_URL}/ml`;

/**
 * API Version
 */
export const API_VERSION = 'v1';

/**
 * API endpoints with type-safe builders
 * Updated for Rust API - December 2025
 */
/**
 * ICT 11+ Principal Engineer Grade
 * Single source of truth for all API endpoints
 * All services MUST use these constants - no hardcoded strings
 */
export const API_ENDPOINTS = {
	// Authentication - Rust API routes
	auth: {
		login: '/api/auth/login',
		loginMfa: '/api/auth/login/mfa',
		loginBiometric: '/api/auth/login/biometric',
		register: '/api/auth/register',
		logout: '/api/logout', // ICT7 FIX: Matches frontend proxy at /api/logout/+server.ts
		refresh: '/api/auth/refresh',
		forgotPassword: '/api/auth/forgot-password',
		resetPassword: '/api/auth/reset-password',
		verifyEmail: (token: string) => `/api/auth/verify-email/${token}`,
		resendVerification: '/api/auth/resend-verification',
		emailVerificationNotification: '/api/auth/email/verification-notification'
	},

	// Current User (me) - Rust API routes
	// ICT7 FIX: Backend has /me under /auth router (api/src/routes/auth.rs:1033)
	me: {
		profile: '/api/auth/me',
		update: '/api/auth/me',
		password: '/api/auth/me/password',
		memberships: '/api/auth/me/memberships',
		products: '/api/auth/me/products',
		indicators: '/api/auth/me/indicators',
		sessions: '/api/auth/me/sessions',
		session: (id: string) => `/api/auth/me/sessions/${id}`,
		logoutAll: '/api/auth/me/sessions/logout-all',
		securityEvents: '/api/auth/me/security-events',
		mfa: {
			enable: '/api/auth/me/mfa/enable',
			verify: '/api/auth/me/mfa/verify',
			disable: '/api/auth/me/mfa/disable'
		}
	},

	// Posts/Blog - Rust API
	posts: {
		list: '/api/posts',
		single: (slug: string) => `/api/posts/${slug}`,
		create: '/api/posts',
		update: (id: number) => `/api/posts/${id}`,
		delete: (id: number) => `/api/posts/${id}`
	},

	// Products - Rust API
	products: {
		list: '/api/products',
		single: (slug: string) => `/api/products/${slug}`,
		my: '/api/products/my',
		create: '/api/products',
		update: (slug: string) => `/api/products/${slug}`,
		delete: (slug: string) => `/api/products/${slug}`
	},

	// Indicators - Rust API
	indicators: {
		list: '/api/indicators',
		single: (slug: string) => `/api/indicators/${slug}`,
		my: '/api/indicators/my',
		download: (id: number) => `/api/indicators/${id}/download`,
		create: '/api/indicators'
	},

	// Courses - Rust API
	courses: {
		list: '/api/courses',
		single: (slug: string) => `/api/courses/${slug}`,
		lessons: (id: string) => `/api/courses/${id}/lessons`,
		create: '/api/courses'
	},

	// Subscriptions/Memberships - Rust API
	subscriptions: {
		plans: '/api/subscriptions/plans',
		planBySlug: (slug: string) => `/api/subscriptions/plans/${slug}`,
		my: '/api/subscriptions/my',
		active: '/api/subscriptions/my/active',
		create: '/api/subscriptions',
		cancel: (id: number) => `/api/subscriptions/${id}/cancel`,
		metrics: '/api/subscriptions/metrics'
	},

	// User - Rust API
	user: {
		profile: '/api/users/me',
		update: '/api/users/me',
		products: '/api/products/my',
		indicators: '/api/indicators/my',
		subscriptions: '/api/subscriptions/my'
	},

	// Newsletter - Rust API
	newsletter: {
		subscribe: '/api/newsletter/subscribe',
		confirm: '/api/newsletter/confirm',
		unsubscribe: '/api/newsletter/unsubscribe',
		subscribers: '/api/newsletter/subscribers',
		stats: '/api/newsletter/stats'
	},

	// Checkout - Rust API
	checkout: {
		create: '/api/checkout',
		calculateTax: '/api/checkout/calculate-tax',
		orders: '/api/checkout/orders',
		order: (orderNumber: string) => `/api/checkout/orders/${orderNumber}`
	},

	// Videos - Rust API
	videos: {
		list: '/api/videos',
		single: (id: number) => `/api/videos/${id}`,
		track: (id: number) => `/api/videos/${id}/track`
	},

	// Articles - Content endpoints (to be implemented)
	articles: {
		list: '/api/articles',
		byRoom: (slug: string) => `/api/articles/${slug}`,
		single: (id: number) => `/api/articles/item/${id}`,
		dailyVideos: (slug: string) => `/api/articles/${slug}/daily-videos`,
		chatroomArchives: (slug: string) => `/api/articles/${slug}/chatroom-archives`
	},

	// Analytics - Rust API
	analytics: {
		track: '/api/analytics/track',
		reading: '/api/analytics/reading',
		performance: '/api/analytics/performance',
		overview: '/api/analytics/overview'
	},

	// Contacts/CRM - Rust API
	contacts: {
		list: '/api/contacts',
		single: (id: number) => `/api/contacts/${id}`,
		create: '/api/contacts',
		update: (id: number) => `/api/contacts/${id}`,
		delete: (id: number) => `/api/contacts/${id}`,
		stats: '/api/contacts/stats'
	},

	// Admin - Rust API
	admin: {
		dashboard: '/api/admin/dashboard',
		users: '/api/admin/users',
		userStats: '/api/admin/users/stats',
		user: (id: number) => `/api/admin/users/${id}`,
		banUser: (id: number) => `/api/admin/users/${id}/ban`,
		unbanUser: (id: number) => `/api/admin/users/${id}/unban`,
		coupons: '/api/admin/coupons',
		coupon: (id: number) => `/api/admin/coupons/${id}`,
		validateCoupon: (code: string) => `/api/admin/coupons/validate/${code}`,
		settings: '/api/admin/settings',
		setting: (key: string) => `/api/admin/settings/${key}`
	},

	// Search - Rust API
	search: '/api/search',

	// Time - Server time sync
	time: {
		now: '/api/time/now'
	},

	// Timers - Timer events tracking
	timers: {
		events: '/api/timers/events'
	},

	// Health - Note: health endpoints are at root level, not under /api
	health: '/health'
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

	// Build URL with params - handle both absolute and relative endpoints
	// If endpoint already starts with http, use it as-is
	// If endpoint starts with /api, prepend API_BASE_URL
	// Otherwise, prepend API_BASE_URL and /api
	let url: string;
	if (endpoint.startsWith('http://') || endpoint.startsWith('https://')) {
		url = endpoint;
	} else if (endpoint.startsWith('/api/')) {
		url = `${API_BASE_URL}${endpoint}`;
	} else if (endpoint.startsWith('/')) {
		url = `${API_BASE_URL}/api${endpoint}`;
	} else {
		url = `${API_BASE_URL}/api/${endpoint}`;
	}

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
				credentials: 'include', // ICT 11+ CORB Fix: Required when backend CORS has allow_credentials(true)
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

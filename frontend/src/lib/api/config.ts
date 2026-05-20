/**
 * API Configuration
 * Central configuration for API endpoints and base URLs
 * Enhanced with better type safety and error handling
 */

import { getAuthToken } from '$lib/stores/auth.svelte';
import type { JsonValue, QueryParams } from './_types';

// ICT11+ PRODUCTION DEPLOYMENT: Always use deployed URLs
// Backend is deployed on Fly.io (Rust + Axum)
const PRODUCTION_API_URL = 'http://localhost:8080';
const PRODUCTION_CDN_URL = 'https://pub-2e5bd1b702b440bd888a0fc47f3493ae.r2.dev';
const PRODUCTION_WS_URL = 'ws://localhost:8080';

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
		setting: (key: string) => `/api/admin/settings/${key}`,
		// Media Library - ICT 7+ Principal Engineer
		media: {
			list: '/api/admin/media',
			files: '/api/admin/media/files',
			upload: '/api/admin/media/upload',
			presignedUpload: '/api/admin/media/presigned-upload',
			confirmUpload: '/api/admin/media/confirm-upload',
			statistics: '/api/admin/media/statistics',
			bulkDelete: '/api/admin/media/bulk-delete',
			bulkUpdate: '/api/admin/media/bulk-update',
			single: (id: number) => `/api/admin/media/${id}`,
			update: (id: number) => `/api/admin/media/${id}`,
			delete: (id: number) => `/api/admin/media/${id}`
		},
		// Bunny.net Video Uploads - ICT 7+ Principal Engineer
		bunny: {
			createVideo: '/api/admin/bunny/create-video',
			videoStatus: (guid: string) => `/api/admin/bunny/video-status/${guid}`,
			uploads: '/api/admin/bunny/uploads'
		}
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
	health: '/health',

	// Export - ICT 7+ Phase 4: CSV/PDF exports for trading rooms
	export: {
		alertsCsv: (roomSlug: string) => `/api/export/${roomSlug}/alerts.csv`,
		tradesCsv: (roomSlug: string) => `/api/export/${roomSlug}/trades.csv`,
		performance: (roomSlug: string) => `/api/export/${roomSlug}/performance`
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
export async function apiFetch<T = unknown>(
	endpoint: string,
	options: RequestInit & {
		params?: QueryParams;
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
	//
	// FIX-2026-04-26 (cross-cutting audit §D): admin pages MUST go through the
	// SvelteKit `+server.ts` proxy at `/api/admin/*` so the proxy can attach
	// `rtp_access_token` from the HttpOnly cookie. If we let `apiFetch` route
	// `/admin/...` calls to the prod backend directly, we bypass the proxy
	// (skipping auth, validation, and `last_verified_at` request-time logic)
	// and rely on the in-memory `getAuthToken()` instead — which is empty on
	// SSR and on first paint. Short-circuit `/admin/*` to same-origin.
	let url: string;
	if (endpoint.startsWith('http://') || endpoint.startsWith('https://')) {
		url = endpoint;
	} else if (endpoint.startsWith('/api/')) {
		url = `${API_BASE_URL}${endpoint}`;
	} else if (endpoint.startsWith('/admin/') || endpoint.startsWith('admin/')) {
		// Same-origin: hits the SK proxy at /api/admin/* (cookie-authed).
		const path = endpoint.startsWith('/') ? endpoint : `/${endpoint}`;
		url = `/api${path}`;
	} else if (endpoint.startsWith('/')) {
		url = `${API_BASE_URL}/api${endpoint}`;
	} else {
		url = `${API_BASE_URL}/api/${endpoint}`;
	}

	if (options.params) {
		// LATENT BUG FIX (R12-A): `new URLSearchParams({ search: undefined })`
		// coerces to the literal string `search=undefined` in the URL — every
		// caller that passes `{ search: searchQuery || undefined, ... }` (e.g.
		// admin/crm/campaigns:103, admin/crm/segments:95) has been shipping that
		// in the query string. Skip null/undefined entries and stringify arrays
		// as repeated `key=v1&key=v2`.
		const sp = new URLSearchParams();
		for (const [key, value] of Object.entries(options.params)) {
			if (value === undefined || value === null) continue;
			if (Array.isArray(value)) {
				for (const item of value) sp.append(key, String(item));
			} else {
				sp.append(key, String(value));
			}
		}
		const queryString = sp.toString();
		if (queryString) url += `?${queryString}`;
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
				const errorData: JsonValue = await response.json().catch(() => ({}));
				const errMessage =
					typeof errorData === 'object' &&
					errorData !== null &&
					!Array.isArray(errorData) &&
					typeof errorData.message === 'string'
						? errorData.message
						: getErrorMessage(response.status);

				// Don't retry on client errors (except 429)
				if (response.status >= 400 && response.status < 500 && response.status !== 429) {
					throw new ApiError(errMessage, response.status, errorData);
				}

				// Retry on server errors
				throw new ApiError(errMessage, response.status, errorData);
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
 *
 * `data` carries the parsed JSON body of the failed response (validation
 * errors, server-side error envelope, etc.). It is genuinely JSON-shaped —
 * declaring it as `JsonValue` keeps callers honest about narrowing before
 * property access while leaving the wire-format round-trippable.
 */
export class ApiError extends Error {
	constructor(
		message: string,
		public status: number,
		public data?: JsonValue
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
 *
 * Default response generic is `<T = unknown>` (matches the precedent in
 * `client.svelte.ts` — its `request<T>`, `get<T>`, etc. have no defaults so
 * TS forces inference at the call site). Callers that previously relied on
 * implicit `any` for property access must either pass an explicit generic
 * (`api.get<{ data: Foo[] }>(...)`) or narrow the returned `unknown` before
 * use; this is the point of the typed-envelope sweep.
 *
 * Body `data` is `unknown` — `JSON.stringify` accepts any value, so we
 * don't need to narrow at this boundary; the caller's serialiser already
 * picks the right shape. `params` is `QueryParams` (string/number/bool/array
 * scalars) — narrower than `Record<string, any>` so call sites can't
 * accidentally pass nested objects that would serialise to `[object Object]`.
 */
export const api = {
	get: <T = unknown>(endpoint: string, params?: QueryParams) =>
		apiFetch<T>(endpoint, params ? { method: 'GET', params } : { method: 'GET' }),

	post: <T = unknown>(endpoint: string, data?: unknown, params?: QueryParams) =>
		apiFetch<T>(endpoint, {
			method: 'POST',
			...(data !== undefined ? { body: JSON.stringify(data) } : {}),
			...(params ? { params } : {})
		}),

	put: <T = unknown>(endpoint: string, data?: unknown, params?: QueryParams) =>
		apiFetch<T>(endpoint, {
			method: 'PUT',
			...(data !== undefined ? { body: JSON.stringify(data) } : {}),
			...(params ? { params } : {})
		}),

	patch: <T = unknown>(endpoint: string, data?: unknown, params?: QueryParams) =>
		apiFetch<T>(endpoint, {
			method: 'PATCH',
			...(data !== undefined ? { body: JSON.stringify(data) } : {}),
			...(params ? { params } : {})
		}),

	delete: <T = unknown>(endpoint: string, params?: QueryParams) =>
		apiFetch<T>(endpoint, params ? { method: 'DELETE', params } : { method: 'DELETE' })
};

/**
 * Build full URL for external use
 */
export function buildApiUrl(endpoint: string, params?: QueryParams): string {
	let url = `${API_BASE_URL}${endpoint}`;

	if (params) {
		// Same null/undefined-skipping shape as `apiFetch` above — keeps the
		// two URL builders behaviourally aligned and avoids the literal
		// `key=undefined` footgun in the wire format.
		const sp = new URLSearchParams();
		for (const [key, value] of Object.entries(params)) {
			if (value === undefined || value === null) continue;
			if (Array.isArray(value)) {
				for (const item of value) sp.append(key, String(item));
			} else {
				sp.append(key, String(value));
			}
		}
		const queryString = sp.toString();
		if (queryString) url += `?${queryString}`;
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
export function isApiError(error: unknown): error is ApiError {
	return error instanceof ApiError;
}

/**
 * Extract validation errors from API error.
 *
 * `error.data` is `JsonValue` (parsed response body of the failed request).
 * Laravel-style validation responses use `{ errors: { field: ["msg"] } }`;
 * narrow before returning so the caller gets `Record<string, string[]>` or
 * an empty object — never a poisoned `any`.
 */
export function getValidationErrors(error: unknown): Record<string, string[]> {
	if (!isApiError(error)) return {};
	const data = error.data;
	if (
		typeof data !== 'object' ||
		data === null ||
		Array.isArray(data) ||
		typeof data.errors !== 'object' ||
		data.errors === null ||
		Array.isArray(data.errors)
	) {
		return {};
	}
	const out: Record<string, string[]> = {};
	for (const [field, msgs] of Object.entries(data.errors)) {
		if (Array.isArray(msgs)) {
			out[field] = msgs.filter((m): m is string => typeof m === 'string');
		}
	}
	return out;
}

export default api;

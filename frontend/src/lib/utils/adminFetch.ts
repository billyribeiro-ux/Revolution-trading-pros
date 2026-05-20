/**
 * Admin Fetch Utility - Apple ICT11+ Principal Engineer Implementation
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Centralized authenticated fetch for all admin API calls.
 * Uses the Vite proxy (/api -> backend) with proper auth headers.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */

import { getAuthToken } from '$lib/stores/auth.svelte';
import { goto } from '$app/navigation';
import { base } from '$app/paths';
import { browser } from '$app/environment';
// ICT 11+ CORB Fix: Use same-origin endpoints

/**
 * Response type for API calls
 */
export interface AdminApiResponse<T = unknown> {
	data?: T;
	message?: string;
	success?: boolean;
	errors?: Record<string, string[]>;
	meta?: {
		total?: number;
		page?: number;
		per_page?: number;
		last_page?: number;
	};
}

/**
 * Options for adminFetch
 */
export interface AdminFetchOptions extends Omit<RequestInit, 'headers'> {
	headers?: Record<string, string>;
	/** Skip redirect to login on 401 */
	skipAuthRedirect?: boolean;
	/** Return raw response instead of JSON */
	rawResponse?: boolean;
}

/**
 * Authenticated fetch for admin API endpoints
 * Automatically includes Bearer token and handles 401 responses
 *
 * @param endpoint - API endpoint (e.g., '/api/admin/posts')
 * @param options - Fetch options
 * @returns Promise with parsed JSON response
 */
/**
 * NOTE on the `T = any` default: tightening this to `unknown` cascades
 * to ~73 untyped admin callsites (`const data = await adminFetch(...);
 * data.connections;`) — that migration is tracked as a dedicated
 * follow-up. For this sweep we leave the default as `any` and accept
 * one residual eslint warning on this line; convenience wrappers below
 * (`adminApi.*`) and `AdminApiResponse` were tightened to `unknown`.
 */
// eslint-disable-next-line @typescript-eslint/no-explicit-any -- See note above; tightening default cascades to 73 admin callsites.
export async function adminFetch<T = any>(
	endpoint: string,
	options: AdminFetchOptions = {}
): Promise<T> {
	const { skipAuthRedirect = false, rawResponse = false, headers = {}, ...fetchOptions } = options;

	// Get auth token from secure store
	const token = getAuthToken();

	// Build headers
	const requestHeaders: Record<string, string> = {
		Accept: 'application/json',
		...headers
	};

	// Add Content-Type for non-FormData bodies
	if (fetchOptions.body && !(fetchOptions.body instanceof FormData)) {
		requestHeaders['Content-Type'] = 'application/json';
	}

	// Add auth token if available
	if (token) {
		requestHeaders['Authorization'] = `Bearer ${token}`;
	}

	// ICT 11+ FIX: Build absolute URL for Pages.dev compatibility (no proxy)
	// ICT 11+ CORB Fix: Use same-origin endpoints to prevent CORB
	const url = endpoint.startsWith('http')
		? endpoint
		: endpoint.startsWith('/api/')
			? endpoint
			: `/api${endpoint}`;

	try {
		const response = await fetch(url, {
			...fetchOptions,
			headers: requestHeaders
		});

		// Handle 401 Unauthorized
		if (response.status === 401 && !skipAuthRedirect && browser) {
			// Redirect to login with return URL
			const returnUrl = encodeURIComponent(window.location.pathname);
			goto(`${base}/login?redirect=${returnUrl}`);
			throw new AdminApiError('Unauthorized', 401);
		}

		// Return raw response if requested (for blob downloads, etc.)
		if (rawResponse) {
			return response as unknown as T;
		}

		// Parse JSON response
		const contentType = response.headers.get('content-type');
		if (contentType?.includes('application/json')) {
			const data = await response.json();

			// Throw error for non-OK responses
			if (!response.ok) {
				throw new AdminApiError(
					data.message || `Request failed with status ${response.status}`,
					response.status,
					data.errors
				);
			}

			return data;
		}

		// Handle non-JSON responses
		if (!response.ok) {
			throw new AdminApiError(`Request failed with status ${response.status}`, response.status);
		}

		// Return text for non-JSON responses
		const text = await response.text();
		return { data: text } as T;
	} catch (error) {
		if (error instanceof AdminApiError) {
			throw error;
		}

		// Network or other errors
		throw new AdminApiError(error instanceof Error ? error.message : 'Network error', 0);
	}
}

/**
 * Custom error class for admin API errors
 */
export class AdminApiError extends Error {
	constructor(
		message: string,
		public status: number,
		public errors?: Record<string, string[]>
	) {
		super(message);
		this.name = 'AdminApiError';
	}

	get isAuthError(): boolean {
		return this.status === 401 || this.status === 403;
	}

	get isValidationError(): boolean {
		return this.status === 422;
	}

	get isServerError(): boolean {
		return this.status >= 500;
	}
}

/**
 * Convenience methods.
 *
 * Note: the response generic defaults to `unknown` rather than `any`.
 * Callers that rely on the previous `any` ergonomics will see a narrow-
 * required compile error and should either supply the response type
 * explicitly (`adminApi.get<MyShape>(...)`) or narrow `unknown` at the
 * use site. Direct admin proxy callers using `adminFetch(...)` retain
 * their existing default for backward compatibility with the ~73 untyped
 * callsites; tightening that default is tracked as a follow-up sweep.
 *
 * Request `data` payloads are typed `unknown` — `JSON.stringify` accepts
 * unknown, and callers were already implicitly serializing arbitrary
 * shapes here.
 */
export const adminApi = {
	get: <T = unknown>(endpoint: string, options?: AdminFetchOptions) =>
		adminFetch<T>(endpoint, { ...options, method: 'GET' }),

	post: <T = unknown>(endpoint: string, data?: unknown, options?: AdminFetchOptions) =>
		adminFetch<T>(endpoint, {
			...options,
			method: 'POST',
			body: data instanceof FormData ? data : JSON.stringify(data)
		}),

	put: <T = unknown>(endpoint: string, data?: unknown, options?: AdminFetchOptions) =>
		adminFetch<T>(endpoint, {
			...options,
			method: 'PUT',
			body: JSON.stringify(data)
		}),

	patch: <T = unknown>(endpoint: string, data?: unknown, options?: AdminFetchOptions) =>
		adminFetch<T>(endpoint, {
			...options,
			method: 'PATCH',
			body: JSON.stringify(data)
		}),

	delete: <T = unknown>(endpoint: string, options?: AdminFetchOptions) =>
		adminFetch<T>(endpoint, { ...options, method: 'DELETE' })
};

export default adminFetch;

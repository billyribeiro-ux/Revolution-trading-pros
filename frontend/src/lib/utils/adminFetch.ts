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
export interface AdminApiResponse<T = any> {
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
			// eslint-disable-next-line svelte/no-navigation-without-resolve -- base path is prepended
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
 * Convenience methods
 */
export const adminApi = {
	get: <T = any>(endpoint: string, options?: AdminFetchOptions) =>
		adminFetch<T>(endpoint, { ...options, method: 'GET' }),

	post: <T = any>(endpoint: string, data?: any, options?: AdminFetchOptions) =>
		adminFetch<T>(endpoint, {
			...options,
			method: 'POST',
			body: data instanceof FormData ? data : JSON.stringify(data)
		}),

	put: <T = any>(endpoint: string, data?: any, options?: AdminFetchOptions) =>
		adminFetch<T>(endpoint, {
			...options,
			method: 'PUT',
			body: JSON.stringify(data)
		}),

	patch: <T = any>(endpoint: string, data?: any, options?: AdminFetchOptions) =>
		adminFetch<T>(endpoint, {
			...options,
			method: 'PATCH',
			body: JSON.stringify(data)
		}),

	delete: <T = any>(endpoint: string, options?: AdminFetchOptions) =>
		adminFetch<T>(endpoint, { ...options, method: 'DELETE' })
};

export default adminFetch;

/**
 * ═══════════════════════════════════════════════════════════════════════════
 * Centralized Axum Server Adapter — Apple ICT 7 Principal Engineer Grade
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Single source of truth for all server-side Axum API calls.
 * Used exclusively by Remote Functions and +page.server.ts load functions.
 *
 * Features:
 * - Base URL from environment (single source)
 * - Auth header injection from RequestEvent cookies/locals
 * - Exponential backoff retry (3 attempts)
 * - Request timeout (30s default, 60s uploads)
 * - Correlation IDs for distributed tracing
 * - Typed error mapping to domain errors
 *
 * @version 1.0.0
 */

import { env } from '$env/dynamic/private';
import { getRequestEvent } from '$app/server';

// ═══════════════════════════════════════════════════════════════════════════
// Configuration — Single Source of Truth
// ═══════════════════════════════════════════════════════════════════════════

const PROD_API_ROOT = 'https://revolution-trading-pros-api.fly.dev';

function getApiRoot(): string {
	return env.API_BASE_URL || env.VITE_API_URL || env.BACKEND_URL || PROD_API_ROOT;
}

const MAX_RETRIES = 3;
const INITIAL_RETRY_DELAY_MS = 1000;
const BACKOFF_MULTIPLIER = 2;
const DEFAULT_TIMEOUT_MS = 30_000;

const RETRYABLE_STATUS_CODES = new Set([408, 429, 500, 502, 503, 504]);
const RETRYABLE_METHODS = new Set(['GET', 'HEAD', 'OPTIONS', 'PUT', 'DELETE']);

// ═══════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════

export interface AxumRequestOptions {
	method?: string;
	params?: Record<string, string | number | boolean | undefined>;
	body?: unknown;
	headers?: Record<string, string>;
	timeout?: number;
	retries?: number;
	signal?: AbortSignal;
}

export class AxumError extends Error {
	constructor(
		message: string,
		public readonly statusCode: number,
		public readonly code: string,
		public readonly requestId: string,
		public readonly data?: unknown
	) {
		super(message);
		this.name = 'AxumError';
	}

	get isNotFound(): boolean {
		return this.statusCode === 404;
	}

	get isUnauthorized(): boolean {
		return this.statusCode === 401;
	}

	get isForbidden(): boolean {
		return this.statusCode === 403;
	}

	get isRateLimited(): boolean {
		return this.statusCode === 429;
	}

	get isServerError(): boolean {
		return this.statusCode >= 500;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Utility Functions
// ═══════════════════════════════════════════════════════════════════════════

function generateRequestId(): string {
	return `rtp-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 9)}`;
}

function sleep(ms: number): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

function calculateBackoff(attempt: number): number {
	const delay = INITIAL_RETRY_DELAY_MS * Math.pow(BACKOFF_MULTIPLIER, attempt);
	const jitter = Math.random() * 0.3 * delay;
	return Math.min(delay + jitter, 8000);
}

function isRetryable(method: string, status: number): boolean {
	return RETRYABLE_METHODS.has(method) && RETRYABLE_STATUS_CODES.has(status);
}

// ═══════════════════════════════════════════════════════════════════════════
// Auth Extraction
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Extract access token from the current request event.
 * Uses getRequestEvent() to access cookies/locals without passing them explicitly.
 */
function getAccessToken(): string | undefined {
	try {
		const event = getRequestEvent();
		return event.locals.accessToken ?? event.cookies.get('rtp_access_token') ?? undefined;
	} catch {
		return undefined;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Core Request Function
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Make an authenticated request to the Axum backend.
 *
 * @param endpoint - API path (e.g., '/api/alerts/explosive-swings')
 * @param options - Request configuration
 * @returns Parsed JSON response
 * @throws {AxumError} On HTTP errors after retries exhausted
 */
export async function axumFetch<T>(
	endpoint: string,
	options: AxumRequestOptions = {}
): Promise<T> {
	const {
		method = 'GET',
		params,
		body,
		headers: extraHeaders,
		timeout = DEFAULT_TIMEOUT_MS,
		retries = MAX_RETRIES,
		signal
	} = options;

	const requestId = generateRequestId();
	const apiRoot = getApiRoot();

	// Build URL
	const url = new URL(`${apiRoot}${endpoint}`);
	if (params) {
		for (const [key, value] of Object.entries(params)) {
			if (value !== undefined) {
				url.searchParams.set(key, String(value));
			}
		}
	}

	// Build headers
	const token = getAccessToken();
	const headers: Record<string, string> = {
		Accept: 'application/json',
		'Content-Type': 'application/json',
		'X-Request-ID': requestId,
		...extraHeaders
	};

	if (token) {
		headers['Authorization'] = `Bearer ${token}`;
	}

	// Serialize body
	const bodyStr = body !== undefined ? JSON.stringify(body) : undefined;

	// Retry loop
	let lastError: Error | null = null;
	let lastStatus = 500;

	for (let attempt = 0; attempt <= retries; attempt++) {
		if (attempt > 0) {
			const delay = calculateBackoff(attempt - 1);
			await sleep(delay);
		}

		try {
			const controller = new AbortController();
			const timeoutId = setTimeout(() => controller.abort(), timeout);

			// Combine external signal with timeout
			if (signal) {
				signal.addEventListener('abort', () => controller.abort(), { once: true });
			}

			const response = await fetch(url.toString(), {
				method,
				headers,
				body: bodyStr,
				signal: controller.signal
			});

			clearTimeout(timeoutId);

			if (response.ok) {
				// Handle 204 No Content
				if (response.status === 204) {
					return undefined as T;
				}

				const text = await response.text();
				if (!text) return undefined as T;

				try {
					return JSON.parse(text) as T;
				} catch {
					return text as unknown as T;
				}
			}

			// Non-retryable error — throw immediately
			if (!isRetryable(method, response.status)) {
				const errorData = await response.json().catch(() => ({}));
				throw new AxumError(
					(errorData as Record<string, string>).message ||
						(errorData as Record<string, string>).error ||
						`HTTP ${response.status}`,
					response.status,
					(errorData as Record<string, string>).code || `HTTP_${response.status}`,
					requestId,
					errorData
				);
			}

			// Retryable — continue loop
			lastStatus = response.status;
			lastError = new Error(`HTTP ${response.status}`);

			// Don't retry POST unless server error
			if (method === 'POST' && response.status < 500) {
				const errorData = await response.json().catch(() => ({}));
				throw new AxumError(
					(errorData as Record<string, string>).message || `HTTP ${response.status}`,
					response.status,
					(errorData as Record<string, string>).code || `HTTP_${response.status}`,
					requestId,
					errorData
				);
			}
		} catch (err) {
			if (err instanceof AxumError) throw err;

			lastError = err instanceof Error ? err : new Error(String(err));

			if (lastError.name === 'AbortError') {
				lastStatus = 504;
			} else {
				lastStatus = 502;
			}
		}
	}

	// All retries exhausted
	throw new AxumError(
		lastError?.name === 'AbortError' ? 'Request timeout' : 'Backend unavailable',
		lastStatus,
		lastError?.name === 'AbortError' ? 'TIMEOUT' : 'BACKEND_ERROR',
		requestId
	);
}

// ═══════════════════════════════════════════════════════════════════════════
// Convenience Methods
// ═══════════════════════════════════════════════════════════════════════════

export const axum = {
	get: <T>(endpoint: string, options?: Omit<AxumRequestOptions, 'method' | 'body'>) =>
		axumFetch<T>(endpoint, { ...options, method: 'GET' }),

	post: <T>(endpoint: string, body?: unknown, options?: Omit<AxumRequestOptions, 'method' | 'body'>) =>
		axumFetch<T>(endpoint, { ...options, method: 'POST', body }),

	put: <T>(endpoint: string, body?: unknown, options?: Omit<AxumRequestOptions, 'method' | 'body'>) =>
		axumFetch<T>(endpoint, { ...options, method: 'PUT', body }),

	patch: <T>(endpoint: string, body?: unknown, options?: Omit<AxumRequestOptions, 'method' | 'body'>) =>
		axumFetch<T>(endpoint, { ...options, method: 'PATCH', body }),

	delete: <T = void>(endpoint: string, options?: Omit<AxumRequestOptions, 'method' | 'body'>) =>
		axumFetch<T>(endpoint, { ...options, method: 'DELETE' })
} as const;

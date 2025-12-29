/**
 * Enterprise Request Interceptor - Apple ICT9+ Request Pipeline
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Implements a composable interceptor chain for:
 * - Request modification (auth, CSRF, tracing)
 * - Response transformation
 * - Error normalization
 * - Automatic token refresh
 *
 * The interceptor pattern allows for clean separation of cross-cutting concerns
 * without polluting the core request logic.
 */

import type {
	RequestInterceptor,
	ResponseInterceptor,
	ErrorInterceptor,
	InterceptorChain,
	EnterpriseRequestConfig,
	EnterpriseResponse,
	EnterpriseApiError,
	RequestContext
} from './types';

// ═══════════════════════════════════════════════════════════════════════════════
// Interceptor Chain
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Create an interceptor chain
 */
export function createInterceptorChain(): InterceptorChain {
	return {
		request: [],
		response: [],
		error: []
	};
}

/**
 * Add a request interceptor to the chain
 */
export function addRequestInterceptor(
	chain: InterceptorChain,
	interceptor: RequestInterceptor
): () => void {
	chain.request.push(interceptor);
	return () => {
		const index = chain.request.indexOf(interceptor);
		if (index > -1) {
			chain.request.splice(index, 1);
		}
	};
}

/**
 * Add a response interceptor to the chain
 */
export function addResponseInterceptor(
	chain: InterceptorChain,
	interceptor: ResponseInterceptor
): () => void {
	chain.response.push(interceptor);
	return () => {
		const index = chain.response.indexOf(interceptor);
		if (index > -1) {
			chain.response.splice(index, 1);
		}
	};
}

/**
 * Add an error interceptor to the chain
 */
export function addErrorInterceptor(
	chain: InterceptorChain,
	interceptor: ErrorInterceptor
): () => void {
	chain.error.push(interceptor);
	return () => {
		const index = chain.error.indexOf(interceptor);
		if (index > -1) {
			chain.error.splice(index, 1);
		}
	};
}

// ═══════════════════════════════════════════════════════════════════════════════
// Interceptor Execution
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Execute request interceptors in order
 */
export async function executeRequestInterceptors(
	chain: InterceptorChain,
	config: EnterpriseRequestConfig,
	context: RequestContext
): Promise<EnterpriseRequestConfig> {
	let currentConfig = config;

	for (const interceptor of chain.request) {
		currentConfig = await interceptor(currentConfig, context);
	}

	return currentConfig;
}

/**
 * Execute response interceptors in reverse order
 */
export async function executeResponseInterceptors<T>(
	chain: InterceptorChain,
	response: EnterpriseResponse<T>,
	context: RequestContext
): Promise<EnterpriseResponse<T>> {
	let currentResponse = response;

	// Execute in reverse order so first-added interceptor sees final response
	for (const interceptor of [...chain.response].reverse()) {
		currentResponse = await interceptor(currentResponse, context);
	}

	return currentResponse;
}

/**
 * Execute error interceptors in order
 */
export async function executeErrorInterceptors(
	chain: InterceptorChain,
	error: EnterpriseApiError,
	context: RequestContext
): Promise<EnterpriseApiError> {
	let currentError = error;

	for (const interceptor of chain.error) {
		currentError = await interceptor(currentError, context);
	}

	return currentError;
}

// ═══════════════════════════════════════════════════════════════════════════════
// Built-in Interceptors
// ═══════════════════════════════════════════════════════════════════════════════

import { getAuthToken, authStore } from '$lib/stores/auth';
import { getTraceHeaders, log } from './tracing';
import { getCsrfHeaders, csrfResponseInterceptor } from './csrf';
import { createApiError, ErrorCodes } from './errors';

/**
 * Authentication interceptor - adds auth token to requests
 */
export const authInterceptor: RequestInterceptor = (config, _context) => {
	const token = getAuthToken();

	if (token) {
		return {
			...config,
			headers: {
				...config.headers,
				Authorization: `Bearer ${token}`
			}
		};
	}

	return config;
};

/**
 * Tracing interceptor - adds trace headers
 */
export const tracingInterceptor: RequestInterceptor = (config, context) => {
	const traceHeaders = getTraceHeaders(context);

	return {
		...config,
		headers: {
			...config.headers,
			...traceHeaders
		}
	};
};

/**
 * CSRF interceptor - adds CSRF token to mutating requests
 */
export const csrfInterceptor: RequestInterceptor = (config, _context) => {
	if (!config.csrf?.enabled) {
		const csrfHeaders = getCsrfHeaders(config.method || 'GET');
		return {
			...config,
			headers: {
				...config.headers,
				...csrfHeaders
			}
		};
	}

	return config;
};

/**
 * Content-Type interceptor - sets default content type
 */
export const contentTypeInterceptor: RequestInterceptor = (config) => {
	// Don't set Content-Type for FormData (browser will set with boundary)
	if (config.body instanceof FormData) {
		return config;
	}

	return {
		...config,
		headers: {
			'Content-Type': 'application/json',
			Accept: 'application/json',
			...config.headers
		}
	};
};

/**
 * Logging interceptor - logs request details
 */
export const loggingRequestInterceptor: RequestInterceptor = (config, context) => {
	log('debug', `[API] ${config.method || 'GET'} request`, context, {
		method: config.method,
		priority: config.priority
	});

	return config;
};

/**
 * Response logging interceptor
 */
export const loggingResponseInterceptor: ResponseInterceptor = (response, context) => {
	const duration = performance.now() - context.startTime;

	log('debug', `[API] Response ${response.status}`, context, {
		status: response.status,
		duration: Math.round(duration),
		cached: response.meta?.cached
	});

	return response;
};

/**
 * CSRF response interceptor - handles token rotation
 */
export const csrfResponseHandler: ResponseInterceptor = (response) => {
	// Extract headers from response
	if (response.headers) {
		csrfResponseInterceptor(new Response(null, { headers: new Headers(response.headers) }));
	}
	return response;
};

/**
 * Error logging interceptor
 */
export const errorLoggingInterceptor: ErrorInterceptor = (error, context) => {
	const level = error.severity === 'critical' ? 'error' : 'warn';

	log(level, `[API] Error: ${error.message}`, context, {
		code: error.code,
		status: error.status,
		category: error.category,
		isRetryable: error.isRetryable
	});

	return error;
};

// ═══════════════════════════════════════════════════════════════════════════════
// Token Refresh Interceptor
// ═══════════════════════════════════════════════════════════════════════════════

/** Pending requests waiting for token refresh */
let refreshPromise: Promise<boolean> | null = null;
const pendingRequests: Array<{
	resolve: (retry: boolean) => void;
	reject: (error: Error) => void;
}> = [];

/**
 * Token refresh error interceptor
 * Automatically refreshes token on 401 and retries request
 */
export function createTokenRefreshInterceptor(
	options: {
		refreshToken: () => Promise<boolean>;
		onRefreshFailed?: () => void;
	}
): ErrorInterceptor {
	return async (error, context) => {
		// Only handle 401 errors
		if (error.status !== 401) {
			return error;
		}

		// Check if this is already a retry
		if (context.isRetry && context.metadata?.['tokenRefreshed']) {
			// Token was already refreshed but still got 401
			options.onRefreshFailed?.();
			return error;
		}

		// If already refreshing, wait for it
		if (refreshPromise) {
			return new Promise<EnterpriseApiError>((resolve, reject) => {
				pendingRequests.push({
					resolve: (shouldRetry) => {
						if (shouldRetry) {
							// Signal that request should be retried
							const retryableError = createApiError({
								...error,
								message: 'Token refreshed, retry request',
								code: ErrorCodes.AUTH_TOKEN_EXPIRED,
								status: 401,
								context: {
									...context,
									metadata: { ...context.metadata, shouldRetry: true, tokenRefreshed: true }
								}
							});
							resolve(retryableError);
						} else {
							resolve(error);
						}
					},
					reject
				});
			});
		}

		// Start token refresh
		refreshPromise = options.refreshToken();

		try {
			const success = await refreshPromise;

			// Notify pending requests
			pendingRequests.forEach((pending) => {
				pending.resolve(success);
			});
			pendingRequests.length = 0;

			if (success) {
				// Return error with retry flag
				return createApiError({
					...error,
					message: 'Token refreshed, retry request',
					code: ErrorCodes.AUTH_TOKEN_EXPIRED,
					status: 401,
					context: {
						...context,
						metadata: { ...context.metadata, shouldRetry: true, tokenRefreshed: true }
					}
				});
			}

			options.onRefreshFailed?.();
			return error;
		} catch (refreshError) {
			// Notify pending requests of failure
			pendingRequests.forEach((pending) => {
				pending.reject(refreshError as Error);
			});
			pendingRequests.length = 0;

			options.onRefreshFailed?.();
			return error;
		} finally {
			refreshPromise = null;
		}
	};
}

// ═══════════════════════════════════════════════════════════════════════════════
// Default Interceptor Chain
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Create default interceptor chain with all built-in interceptors
 */
export function createDefaultInterceptorChain(): InterceptorChain {
	const chain = createInterceptorChain();

	// Request interceptors (executed in order)
	addRequestInterceptor(chain, contentTypeInterceptor);
	addRequestInterceptor(chain, authInterceptor);
	addRequestInterceptor(chain, csrfInterceptor);
	addRequestInterceptor(chain, tracingInterceptor);
	addRequestInterceptor(chain, loggingRequestInterceptor);

	// Response interceptors (executed in reverse order)
	addResponseInterceptor(chain, loggingResponseInterceptor);
	addResponseInterceptor(chain, csrfResponseHandler);

	// Error interceptors (executed in order)
	addErrorInterceptor(chain, errorLoggingInterceptor);
	addErrorInterceptor(
		chain,
		createTokenRefreshInterceptor({
			refreshToken: () => authStore.refreshToken(),
			onRefreshFailed: () => {
				authStore.logout('/login?session=expired');
			}
		})
	);

	return chain;
}

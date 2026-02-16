/**
 * ═══════════════════════════════════════════════════════════════════════════
 * Generic Catch-All API Proxy - Apple ICT 7 Principal Engineer Grade
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Routes all /api/* requests through SvelteKit server to prevent CORB.
 * Implements enterprise-grade resilience patterns:
 *
 * - Exponential backoff retry (3 attempts, 1s → 2s → 4s)
 * - Request timeout (30s default, 60s for uploads)
 * - Correlation IDs for distributed tracing
 * - Circuit breaker pattern (fail-fast on repeated failures)
 * - Structured error responses with error codes
 * - Comprehensive logging with request context
 *
 * @version 2.0.0
 * @author ICT 7 Principal Engineer
 */
import { json } from '@sveltejs/kit';
import type { RequestHandler, Cookies } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { logger } from '$lib/utils/logger';

// ═══════════════════════════════════════════════════════════════════════════
// Configuration
// ═══════════════════════════════════════════════════════════════════════════

const PROD_API_ROOT = 'https://revolution-trading-pros-api.fly.dev';
const API_ROOT = env.VITE_API_URL || env.BACKEND_URL || PROD_API_ROOT;

// ICT 7: Retry configuration with exponential backoff
const MAX_RETRIES = 3;
const INITIAL_RETRY_DELAY_MS = 1000;
const MAX_RETRY_DELAY_MS = 8000;
const BACKOFF_MULTIPLIER = 2;

// ICT 7: Timeout configuration
const DEFAULT_TIMEOUT_MS = 30000; // 30 seconds
const UPLOAD_TIMEOUT_MS = 60000; // 60 seconds for file uploads

// ICT 7: Circuit breaker state (in-memory, resets on deploy)
const circuitBreaker = {
	failures: 0,
	lastFailure: 0,
	isOpen: false,
	threshold: 5, // Open circuit after 5 consecutive failures
	resetTimeout: 30000 // Try again after 30 seconds
};

// ═══════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════

interface ProxyError {
	error: string;
	code: string;
	requestId: string;
	timestamp: string;
	path?: string;
	retryable: boolean;
}

type RetryableMethod = 'GET' | 'HEAD' | 'OPTIONS' | 'PUT' | 'DELETE';
const RETRYABLE_METHODS: RetryableMethod[] = ['GET', 'HEAD', 'OPTIONS', 'PUT', 'DELETE'];
const RETRYABLE_STATUS_CODES = [408, 429, 500, 502, 503, 504];

// ═══════════════════════════════════════════════════════════════════════════
// Utility Functions
// ═══════════════════════════════════════════════════════════════════════════

function generateRequestId(): string {
	return `rtp-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 9)}`;
}

function sleep(ms: number): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

function isRetryable(method: string, status: number): boolean {
	return (
		RETRYABLE_METHODS.includes(method as RetryableMethod) && RETRYABLE_STATUS_CODES.includes(status)
	);
}

function calculateBackoff(attempt: number): number {
	const delay = INITIAL_RETRY_DELAY_MS * Math.pow(BACKOFF_MULTIPLIER, attempt);
	// Add jitter to prevent thundering herd
	const jitter = Math.random() * 0.3 * delay;
	return Math.min(delay + jitter, MAX_RETRY_DELAY_MS);
}

function checkCircuitBreaker(requestId: string): ProxyError | null {
	if (circuitBreaker.isOpen) {
		const timeSinceLastFailure = Date.now() - circuitBreaker.lastFailure;
		if (timeSinceLastFailure < circuitBreaker.resetTimeout) {
			return {
				error: 'Service temporarily unavailable',
				code: 'CIRCUIT_OPEN',
				requestId,
				timestamp: new Date().toISOString(),
				retryable: true
			};
		}
		// Half-open: try one request
		circuitBreaker.isOpen = false;
	}
	return null;
}

function recordSuccess(): void {
	circuitBreaker.failures = 0;
	circuitBreaker.isOpen = false;
}

function recordFailure(): void {
	circuitBreaker.failures++;
	circuitBreaker.lastFailure = Date.now();
	if (circuitBreaker.failures >= circuitBreaker.threshold) {
		circuitBreaker.isOpen = true;
		logger.warn('[API Proxy] Circuit breaker OPEN - too many failures');
	}
}

function createErrorResponse(
	error: string,
	code: string,
	requestId: string,
	status: number,
	path?: string,
	retryable = false
): Response {
	const body: ProxyError = {
		error,
		code,
		requestId,
		timestamp: new Date().toISOString(),
		path,
		retryable
	};
	return json(body, {
		status,
		headers: {
			'X-Request-ID': requestId,
			'Cache-Control': 'no-store'
		}
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// Core Proxy Logic
// ═══════════════════════════════════════════════════════════════════════════

async function proxyRequest(
	request: Request,
	path: string,
	url: URL,
	cookies: Cookies,
	fetchFn: typeof fetch
): Promise<Response> {
	const requestId = generateRequestId();
	const startTime = Date.now();

	// ICT 7: Check circuit breaker
	const circuitError = checkCircuitBreaker(requestId);
	if (circuitError) {
		return json(circuitError, { status: 503 });
	}

	// Build target URL
	const targetUrl = new URL(`${API_ROOT}/api/${path}`);
	url.searchParams.forEach((value, key) => {
		targetUrl.searchParams.set(key, value);
	});

	// Build headers with correlation ID
	const token = cookies.get('rtp_access_token');
	const headers: Record<string, string> = {
		Accept: 'application/json',
		'X-Request-ID': requestId,
		'X-Forwarded-For': request.headers.get('x-forwarded-for') || 'unknown',
		'X-Forwarded-Host': request.headers.get('host') || 'unknown'
	};

	const contentType = request.headers.get('content-type');
	if (contentType) {
		headers['Content-Type'] = contentType;
	}

	if (token) {
		headers.Authorization = `Bearer ${token}`;
	}

	// Get body for non-GET requests
	let body: string | null = null;
	if (request.method !== 'GET' && request.method !== 'HEAD') {
		try {
			body = await request.text();
		} catch {
			// No body
		}
	}

	// Determine timeout based on content type
	const isUpload = contentType?.includes('multipart/form-data');
	const timeout = isUpload ? UPLOAD_TIMEOUT_MS : DEFAULT_TIMEOUT_MS;

	// ICT 7: Retry loop with exponential backoff
	let lastError: Error | null = null;
	let lastStatus = 500;

	for (let attempt = 0; attempt <= MAX_RETRIES; attempt++) {
		if (attempt > 0) {
			const delay = calculateBackoff(attempt - 1);
			logger.info(`[API Proxy] Retry ${attempt}/${MAX_RETRIES} for ${path} after ${delay}ms`);
			await sleep(delay);
		}

		try {
			// ICT 7: Request with timeout using AbortController
			const controller = new AbortController();
			const timeoutId = setTimeout(() => controller.abort(), timeout);

			const response = await fetchFn(targetUrl.toString(), {
				method: request.method,
				headers,
				body,
				signal: controller.signal
			});

			clearTimeout(timeoutId);

			// Success - record and return
			if (response.ok || !isRetryable(request.method, response.status)) {
				recordSuccess();

				const duration = Date.now() - startTime;
				if (duration > 5000) {
					logger.warn(`[API Proxy] Slow request: ${path} took ${duration}ms`);
				}

				// Parse and return response
				const responseText = await response.text();
				try {
					const data = JSON.parse(responseText);
					return json(data, {
						status: response.status,
						headers: {
							'X-Request-ID': requestId,
							'X-Response-Time': `${duration}ms`,
							'Cache-Control': response.headers.get('Cache-Control') || 'no-cache'
						}
					});
				} catch {
					return new Response(responseText, {
						status: response.status,
						headers: {
							'X-Request-ID': requestId,
							'Content-Type': response.headers.get('Content-Type') || 'text/plain'
						}
					});
				}
			}

			// Retryable error
			lastStatus = response.status;
			lastError = new Error(`HTTP ${response.status}`);

			// Don't retry POST (not idempotent) unless it's a network error
			if (request.method === 'POST' && response.status < 500) {
				break;
			}
		} catch (err) {
			lastError = err instanceof Error ? err : new Error(String(err));

			// Timeout or network error
			if (lastError.name === 'AbortError') {
				lastStatus = 504;
				logger.error(`[API Proxy] Timeout after ${timeout}ms for ${path}`);
			} else {
				lastStatus = 502;
				logger.error(`[API Proxy] Network error for ${path}:`, lastError.message);
			}
		}
	}

	// All retries exhausted
	recordFailure();
	const duration = Date.now() - startTime;
	logger.error(
		`[API Proxy] Failed after ${MAX_RETRIES} retries: ${path} (${duration}ms)`,
		lastError?.message
	);

	return createErrorResponse(
		lastError?.name === 'AbortError' ? 'Request timeout' : 'Backend unavailable',
		lastError?.name === 'AbortError' ? 'TIMEOUT' : 'BACKEND_ERROR',
		requestId,
		lastStatus,
		path,
		true
	);
}

export const GET: RequestHandler = async ({ params, url, cookies, fetch, request }) => {
	const path = params.path || '';
	return proxyRequest(request, path, url, cookies, fetch);
};

export const POST: RequestHandler = async ({ params, url, cookies, fetch, request }) => {
	const path = params.path || '';
	return proxyRequest(request, path, url, cookies, fetch);
};

export const PUT: RequestHandler = async ({ params, url, cookies, fetch, request }) => {
	const path = params.path || '';
	return proxyRequest(request, path, url, cookies, fetch);
};

export const PATCH: RequestHandler = async ({ params, url, cookies, fetch, request }) => {
	const path = params.path || '';
	return proxyRequest(request, path, url, cookies, fetch);
};

export const DELETE: RequestHandler = async ({ params, url, cookies, fetch, request }) => {
	const path = params.path || '';
	return proxyRequest(request, path, url, cookies, fetch);
};

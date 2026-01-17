/**
 * Enterprise Error Handling - Apple ICT9+ Error Management
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Provides:
 * - Consistent error shape across all API boundaries
 * - Error categorization for routing and handling
 * - Severity levels for alerting
 * - Retry eligibility determination
 * - User-friendly error messages
 * - Error serialization for logging
 */

import type { EnterpriseApiError, ErrorCategory, ErrorSeverity, RequestContext } from './types';

// ═══════════════════════════════════════════════════════════════════════════════
// Error Codes
// ═══════════════════════════════════════════════════════════════════════════════

export const ErrorCodes = {
	// Authentication errors (1xxx)
	AUTH_REQUIRED: 'E1001',
	AUTH_INVALID_CREDENTIALS: 'E1002',
	AUTH_TOKEN_EXPIRED: 'E1003',
	AUTH_TOKEN_INVALID: 'E1004',
	AUTH_REFRESH_FAILED: 'E1005',
	AUTH_SESSION_EXPIRED: 'E1006',

	// Authorization errors (2xxx)
	FORBIDDEN: 'E2001',
	INSUFFICIENT_PERMISSIONS: 'E2002',
	RESOURCE_ACCESS_DENIED: 'E2003',

	// Validation errors (3xxx)
	VALIDATION_FAILED: 'E3001',
	INVALID_INPUT: 'E3002',
	MISSING_REQUIRED_FIELD: 'E3003',
	INVALID_FORMAT: 'E3004',

	// Network errors (4xxx)
	NETWORK_ERROR: 'E4001',
	NETWORK_TIMEOUT: 'E4002',
	NETWORK_OFFLINE: 'E4003',
	DNS_RESOLUTION_FAILED: 'E4004',
	CONNECTION_REFUSED: 'E4005',

	// Rate limiting (5xxx)
	RATE_LIMITED: 'E5001',
	QUOTA_EXCEEDED: 'E5002',
	CONCURRENT_LIMIT: 'E5003',

	// Server errors (6xxx)
	SERVER_ERROR: 'E6001',
	SERVICE_UNAVAILABLE: 'E6002',
	GATEWAY_ERROR: 'E6003',
	DATABASE_ERROR: 'E6004',
	EXTERNAL_SERVICE_ERROR: 'E6005',

	// Client errors (7xxx)
	BAD_REQUEST: 'E7001',
	NOT_FOUND: 'E7002',
	CONFLICT: 'E7003',
	GONE: 'E7004',
	PAYLOAD_TOO_LARGE: 'E7005',
	UNSUPPORTED_MEDIA_TYPE: 'E7006',

	// Circuit breaker (8xxx)
	CIRCUIT_OPEN: 'E8001',
	CIRCUIT_HALF_OPEN_REJECTED: 'E8002',

	// Unknown (9xxx)
	UNKNOWN: 'E9001',
	UNEXPECTED: 'E9002'
} as const;

export type ErrorCode = (typeof ErrorCodes)[keyof typeof ErrorCodes];

// ═══════════════════════════════════════════════════════════════════════════════
// Error Messages
// ═══════════════════════════════════════════════════════════════════════════════

const userFriendlyMessages: Record<ErrorCode, string> = {
	[ErrorCodes.AUTH_REQUIRED]: 'Please sign in to continue.',
	[ErrorCodes.AUTH_INVALID_CREDENTIALS]: 'The email or password you entered is incorrect.',
	[ErrorCodes.AUTH_TOKEN_EXPIRED]: 'Your session has expired. Please sign in again.',
	[ErrorCodes.AUTH_TOKEN_INVALID]: 'Your session is invalid. Please sign in again.',
	[ErrorCodes.AUTH_REFRESH_FAILED]: 'Unable to refresh your session. Please sign in again.',
	[ErrorCodes.AUTH_SESSION_EXPIRED]: 'Your session has ended. Please sign in again.',
	[ErrorCodes.FORBIDDEN]: "You don't have permission to perform this action.",
	[ErrorCodes.INSUFFICIENT_PERMISSIONS]: "You don't have the required permissions.",
	[ErrorCodes.RESOURCE_ACCESS_DENIED]: "You don't have access to this resource.",
	[ErrorCodes.VALIDATION_FAILED]: 'Please check your input and try again.',
	[ErrorCodes.INVALID_INPUT]: 'The information you provided is invalid.',
	[ErrorCodes.MISSING_REQUIRED_FIELD]: 'Please fill in all required fields.',
	[ErrorCodes.INVALID_FORMAT]: 'The format of your input is incorrect.',
	[ErrorCodes.NETWORK_ERROR]:
		'Unable to connect to the server. Please check your internet connection.',
	[ErrorCodes.NETWORK_TIMEOUT]: 'The request timed out. Please try again.',
	[ErrorCodes.NETWORK_OFFLINE]: 'You appear to be offline. Please check your internet connection.',
	[ErrorCodes.DNS_RESOLUTION_FAILED]: 'Unable to reach the server. Please try again later.',
	[ErrorCodes.CONNECTION_REFUSED]: 'Unable to connect to the server. Please try again later.',
	[ErrorCodes.RATE_LIMITED]: 'Too many requests. Please wait a moment and try again.',
	[ErrorCodes.QUOTA_EXCEEDED]: "You've reached your usage limit. Please upgrade your plan.",
	[ErrorCodes.CONCURRENT_LIMIT]: 'Too many simultaneous requests. Please wait and try again.',
	[ErrorCodes.SERVER_ERROR]: 'Something went wrong on our end. Please try again later.',
	[ErrorCodes.SERVICE_UNAVAILABLE]:
		'The service is temporarily unavailable. Please try again later.',
	[ErrorCodes.GATEWAY_ERROR]: 'Unable to reach the service. Please try again later.',
	[ErrorCodes.DATABASE_ERROR]: 'A database error occurred. Please try again later.',
	[ErrorCodes.EXTERNAL_SERVICE_ERROR]:
		'An external service error occurred. Please try again later.',
	[ErrorCodes.BAD_REQUEST]: 'The request could not be processed. Please try again.',
	[ErrorCodes.NOT_FOUND]: 'The requested resource was not found.',
	[ErrorCodes.CONFLICT]: 'A conflict occurred. The resource may have been modified.',
	[ErrorCodes.GONE]: 'The requested resource is no longer available.',
	[ErrorCodes.PAYLOAD_TOO_LARGE]: 'The request is too large. Please reduce the size and try again.',
	[ErrorCodes.UNSUPPORTED_MEDIA_TYPE]: 'The file type is not supported.',
	[ErrorCodes.CIRCUIT_OPEN]: 'The service is experiencing issues. Please try again later.',
	[ErrorCodes.CIRCUIT_HALF_OPEN_REJECTED]: 'The service is recovering. Please try again shortly.',
	[ErrorCodes.UNKNOWN]: 'An unexpected error occurred. Please try again.',
	[ErrorCodes.UNEXPECTED]: 'Something unexpected happened. Please try again.'
};

// ═══════════════════════════════════════════════════════════════════════════════
// Error Creation
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Create a standardized API error
 */
export function createApiError(options: {
	message: string;
	code: ErrorCode;
	status: number;
	category?: ErrorCategory;
	severity?: ErrorSeverity;
	validationErrors?: Record<string, string[]>;
	context?: RequestContext;
	retryAfter?: number;
	cause?: Error;
}): EnterpriseApiError {
	const error = new Error(options.message) as EnterpriseApiError;

	error.name = 'EnterpriseApiError';
	error.code = options.code;
	error.status = options.status;
	error.category = options.category ?? categorizeError(options.status, options.code);
	error.severity = options.severity ?? determineSeverity(options.status, options.category);
	if (options.validationErrors !== undefined) error.validationErrors = options.validationErrors;
	if (options.context !== undefined) error.context = options.context;
	if (options.retryAfter !== undefined) error.retryAfter = options.retryAfter;
	error.isRetryable = isRetryable(options.status, options.code);
	if (options.cause !== undefined) error.cause = options.cause;
	error.timestamp = new Date().toISOString();

	return error;
}

/**
 * Create error from HTTP response
 */
export async function createErrorFromResponse(
	response: Response,
	context?: RequestContext
): Promise<EnterpriseApiError> {
	let body: {
		message?: string;
		errors?: Record<string, string[]>;
		code?: string;
		error?: string;
	} = {};

	try {
		const text = await response.text();
		if (text) {
			body = JSON.parse(text);
		}
	} catch {
		// Response body is not JSON
	}

	const { code, status } = mapHttpStatusToError(response.status);
	const message = body.message || body.error || getDefaultMessage(response.status);

	const retryAfter = parseRetryAfter(response.headers.get('Retry-After'));
	return createApiError({
		message,
		code,
		status,
		...(body.errors !== undefined && { validationErrors: body.errors }),
		...(context !== undefined && { context }),
		...(retryAfter !== undefined && { retryAfter })
	});
}

/**
 * Create error from network failure
 */
export function createNetworkError(cause: Error, context?: RequestContext): EnterpriseApiError {
	const isTimeout = cause.name === 'AbortError' || cause.message.includes('timeout');
	const isOffline = !navigator.onLine;

	let code: ErrorCode;
	let message: string;

	if (isTimeout) {
		code = ErrorCodes.NETWORK_TIMEOUT;
		message = userFriendlyMessages[ErrorCodes.NETWORK_TIMEOUT];
	} else if (isOffline) {
		code = ErrorCodes.NETWORK_OFFLINE;
		message = userFriendlyMessages[ErrorCodes.NETWORK_OFFLINE];
	} else {
		code = ErrorCodes.NETWORK_ERROR;
		message = userFriendlyMessages[ErrorCodes.NETWORK_ERROR];
	}

	return createApiError({
		message,
		code,
		status: 0,
		category: 'network',
		cause,
		...(context !== undefined && { context })
	});
}

// ═══════════════════════════════════════════════════════════════════════════════
// Error Classification
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Map HTTP status to error code
 */
function mapHttpStatusToError(status: number): { code: ErrorCode; status: number } {
	switch (status) {
		case 400:
			return { code: ErrorCodes.BAD_REQUEST, status };
		case 401:
			return { code: ErrorCodes.AUTH_REQUIRED, status };
		case 403:
			return { code: ErrorCodes.FORBIDDEN, status };
		case 404:
			return { code: ErrorCodes.NOT_FOUND, status };
		case 409:
			return { code: ErrorCodes.CONFLICT, status };
		case 410:
			return { code: ErrorCodes.GONE, status };
		case 413:
			return { code: ErrorCodes.PAYLOAD_TOO_LARGE, status };
		case 415:
			return { code: ErrorCodes.UNSUPPORTED_MEDIA_TYPE, status };
		case 422:
			return { code: ErrorCodes.VALIDATION_FAILED, status };
		case 429:
			return { code: ErrorCodes.RATE_LIMITED, status };
		case 500:
			return { code: ErrorCodes.SERVER_ERROR, status };
		case 502:
			return { code: ErrorCodes.GATEWAY_ERROR, status };
		case 503:
			return { code: ErrorCodes.SERVICE_UNAVAILABLE, status };
		case 504:
			return { code: ErrorCodes.NETWORK_TIMEOUT, status };
		default:
			if (status >= 400 && status < 500) {
				return { code: ErrorCodes.BAD_REQUEST, status };
			}
			if (status >= 500) {
				return { code: ErrorCodes.SERVER_ERROR, status };
			}
			return { code: ErrorCodes.UNKNOWN, status };
	}
}

/**
 * Categorize error by status and code
 */
function categorizeError(status: number, code: ErrorCode): ErrorCategory {
	if (status === 401 || code.startsWith('E1')) {
		return 'authentication';
	}
	if (status === 403 || code.startsWith('E2')) {
		return 'authorization';
	}
	if (status === 422 || code.startsWith('E3')) {
		return 'validation';
	}
	if (status === 0 || code.startsWith('E4')) {
		return 'network';
	}
	if (status === 429 || code.startsWith('E5')) {
		return 'rate_limit';
	}
	if (status >= 500 || code.startsWith('E6')) {
		return 'server';
	}
	if (status >= 400 && status < 500) {
		return 'client';
	}
	return 'unknown';
}

/**
 * Determine error severity
 */
function determineSeverity(status: number, category?: ErrorCategory): ErrorSeverity {
	// Critical: Complete service failure
	if (status === 0 || status >= 500) {
		return 'critical';
	}

	// Error: Authentication/Authorization failures
	if (category === 'authentication' || category === 'authorization') {
		return 'error';
	}

	// Warning: Rate limiting, validation errors
	if (category === 'rate_limit' || category === 'validation') {
		return 'warning';
	}

	// Info: 404s and other client errors
	return 'info';
}

/**
 * Determine if error is retryable
 */
function isRetryable(status: number, code: ErrorCode): boolean {
	// Network errors are retryable
	if (status === 0) {
		return true;
	}

	// Rate limiting is retryable (after waiting)
	if (status === 429) {
		return true;
	}

	// Server errors are retryable
	if (status >= 500 && status !== 501) {
		return true;
	}

	// Specific retryable codes
	const retryableCodes: ErrorCode[] = [
		ErrorCodes.NETWORK_ERROR,
		ErrorCodes.NETWORK_TIMEOUT,
		ErrorCodes.CONNECTION_REFUSED,
		ErrorCodes.SERVICE_UNAVAILABLE,
		ErrorCodes.GATEWAY_ERROR,
		ErrorCodes.CIRCUIT_HALF_OPEN_REJECTED
	];

	return retryableCodes.includes(code);
}

/**
 * Get default error message for status
 */
function getDefaultMessage(status: number): string {
	const { code } = mapHttpStatusToError(status);
	return userFriendlyMessages[code] || userFriendlyMessages[ErrorCodes.UNKNOWN];
}

/**
 * Parse Retry-After header
 */
function parseRetryAfter(header: string | null): number | undefined {
	if (!header) return undefined;

	// Try parsing as seconds
	const seconds = parseInt(header, 10);
	if (!isNaN(seconds)) {
		return seconds;
	}

	// Try parsing as HTTP date
	const date = new Date(header);
	if (!isNaN(date.getTime())) {
		return Math.max(0, Math.ceil((date.getTime() - Date.now()) / 1000));
	}

	return undefined;
}

// ═══════════════════════════════════════════════════════════════════════════════
// Error Utilities
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Check if error is an EnterpriseApiError
 */
export function isApiError(error: unknown): error is EnterpriseApiError {
	return error instanceof Error && 'code' in error && 'status' in error && 'category' in error;
}

/**
 * Get user-friendly message for error
 */
export function getUserFriendlyMessage(error: unknown): string {
	if (isApiError(error)) {
		return userFriendlyMessages[error.code as ErrorCode] || error.message;
	}

	if (error instanceof Error) {
		return error.message;
	}

	return userFriendlyMessages[ErrorCodes.UNKNOWN];
}

/**
 * Get validation errors from API error
 */
export function getValidationErrors(error: unknown): Record<string, string[]> {
	if (isApiError(error) && error.validationErrors) {
		return error.validationErrors;
	}
	return {};
}

/**
 * Check if error requires authentication
 */
export function requiresAuthentication(error: unknown): boolean {
	return isApiError(error) && error.category === 'authentication';
}

/**
 * Check if error is a rate limit error
 */
export function isRateLimitError(error: unknown): boolean {
	return isApiError(error) && error.category === 'rate_limit';
}

/**
 * Serialize error for logging (removes circular references, sensitive data)
 */
export function serializeError(error: unknown): Record<string, unknown> {
	if (isApiError(error)) {
		return {
			name: error.name,
			message: error.message,
			code: error.code,
			status: error.status,
			category: error.category,
			severity: error.severity,
			isRetryable: error.isRetryable,
			timestamp: error.timestamp,
			traceId: error.context?.traceId,
			spanId: error.context?.spanId,
			retryAfter: error.retryAfter,
			// Don't include full context or validation errors in logs
			hasValidationErrors: !!error.validationErrors
		};
	}

	if (error instanceof Error) {
		return {
			name: error.name,
			message: error.message,
			stack: error.stack
		};
	}

	return { error: String(error) };
}

/**
 * API Error Hierarchy - TypeScript 5.9+ Implementation
 * =============================================================================
 *
 * Apple Principal Engineer ICT Level 7 Grade Quality
 * Built for the next 10 years - January 2026
 *
 * DESIGN PRINCIPLES:
 * 1. Single Responsibility - Each error class handles one error category
 * 2. Discriminated Unions - Error codes enable exhaustive type checking
 * 3. Immutable State - All error properties are readonly after construction
 * 4. User-Friendly Messages - Every error has a human-readable message
 * 5. Serialization-Safe - All errors can be safely logged/transmitted
 *
 * ERROR CATEGORIES:
 * - Network: Connection failures, DNS, offline
 * - Timeout: Request/response timeouts
 * - Authentication: 401, token expiry
 * - Authorization: 403, permission denied
 * - Validation: 422, field errors
 * - NotFound: 404, missing resources
 * - RateLimit: 429, quota exceeded
 * - Server: 5xx, backend failures
 * - ServiceUnavailable: 503, maintenance
 *
 * @version 2.0.0 - TypeScript 5.9+ Rewrite
 * @license MIT
 */

// =============================================================================
// ERROR CODE TYPES - Discriminated union for exhaustive checking
// =============================================================================

/**
 * All possible API error codes
 * Use this type with switch statements for exhaustive error handling
 */
export type ApiErrorCode =
	| 'NETWORK_ERROR'
	| 'TIMEOUT_ERROR'
	| 'AUTH_ERROR'
	| 'FORBIDDEN_ERROR'
	| 'VALIDATION_ERROR'
	| 'NOT_FOUND_ERROR'
	| 'RATE_LIMIT_ERROR'
	| 'SERVER_ERROR'
	| 'SERVICE_UNAVAILABLE'
	| 'CONFLICT_ERROR'
	| 'PAYLOAD_TOO_LARGE'
	| 'UNSUPPORTED_MEDIA_TYPE'
	| 'UNKNOWN_ERROR';

/**
 * Error severity levels for alerting and logging
 */
export type ErrorSeverity = 'critical' | 'error' | 'warning' | 'info';

/**
 * Error category for routing and handling decisions
 */
export type ErrorCategory =
	| 'network'
	| 'timeout'
	| 'authentication'
	| 'authorization'
	| 'validation'
	| 'notFound'
	| 'rateLimit'
	| 'server'
	| 'client'
	| 'unknown';

// =============================================================================
// USER-FRIENDLY ERROR MESSAGES
// =============================================================================

const USER_FRIENDLY_MESSAGES: Readonly<Record<ApiErrorCode, string>> = Object.freeze({
	NETWORK_ERROR: 'Unable to connect. Please check your internet connection and try again.',
	TIMEOUT_ERROR: 'The request took too long. Please try again.',
	AUTH_ERROR: 'Your session has expired. Please sign in again.',
	FORBIDDEN_ERROR: "You don't have permission to perform this action.",
	VALIDATION_ERROR: 'Please check your input and try again.',
	NOT_FOUND_ERROR: 'The requested resource could not be found.',
	RATE_LIMIT_ERROR: 'Too many requests. Please wait a moment and try again.',
	SERVER_ERROR: 'Something went wrong on our end. Please try again later.',
	SERVICE_UNAVAILABLE: 'The service is temporarily unavailable. Please try again later.',
	CONFLICT_ERROR: 'A conflict occurred. The resource may have been modified.',
	PAYLOAD_TOO_LARGE: 'The request is too large. Please reduce the size and try again.',
	UNSUPPORTED_MEDIA_TYPE: 'The file type is not supported.',
	UNKNOWN_ERROR: 'An unexpected error occurred. Please try again.'
});

// =============================================================================
// BASE API ERROR CLASS
// =============================================================================

/**
 * Abstract base class for all API errors
 * Provides consistent structure and behavior across all error types
 */
export abstract class ApiError extends Error {
	/** Error code for programmatic handling */
	abstract readonly code: ApiErrorCode;

	/** HTTP status code (0 for network errors) */
	abstract readonly statusCode: number;

	/** Timestamp when the error occurred */
	readonly timestamp: Date;

	/** Unique request ID for tracing */
	readonly requestId: string | undefined;

	/** Whether this error can be retried */
	readonly isRetryable: boolean;

	/** Error severity for alerting */
	readonly severity: ErrorSeverity;

	/** Error category for routing */
	readonly category: ErrorCategory;

	/** User-friendly error message */
	readonly userMessage: string;

	/** Validation errors by field (for validation errors) */
	readonly fields?: Readonly<Record<string, readonly string[]>>;

	/** Seconds until retry is allowed (for rate limits) */
	readonly retryAfter?: number;

	/** Original error if wrapped */
	override readonly cause?: Error;

	constructor(
		message: string,
		code: ApiErrorCode,
		statusCode: number,
		requestId?: string,
		options?: {
			fields?: Record<string, string[]>;
			retryAfter?: number;
			cause?: Error;
			isRetryable?: boolean;
			severity?: ErrorSeverity;
			category?: ErrorCategory;
		}
	) {
		super(message);

		// Maintain proper prototype chain for instanceof checks
		Object.setPrototypeOf(this, new.target.prototype);

		this.name = this.constructor.name;
		this.timestamp = new Date();
		this.requestId = requestId;
		this.userMessage = USER_FRIENDLY_MESSAGES[code];

		// Set default values, allow overrides
		this.isRetryable = options?.isRetryable ?? this.determineRetryable(statusCode, code);
		this.severity = options?.severity ?? this.determineSeverity(statusCode, code);
		this.category = options?.category ?? this.determineCategory(code);

		if (options?.fields) {
			this.fields = Object.freeze(
				Object.fromEntries(
					Object.entries(options.fields).map(([k, v]) => [k, Object.freeze([...v])])
				)
			);
		}

		if (options?.retryAfter !== undefined) {
			this.retryAfter = options.retryAfter;
		}

		if (options?.cause) {
			this.cause = options.cause;
		}

		// Capture stack trace
		if (Error.captureStackTrace) {
			Error.captureStackTrace(this, this.constructor);
		}
	}

	/**
	 * Determine if error is retryable based on status and code
	 */
	private determineRetryable(status: number, code: ApiErrorCode): boolean {
		// Network and timeout errors are retryable
		if (code === 'NETWORK_ERROR' || code === 'TIMEOUT_ERROR') {
			return true;
		}

		// Rate limiting is retryable (after waiting)
		if (code === 'RATE_LIMIT_ERROR') {
			return true;
		}

		// Server errors are retryable (except 501 Not Implemented)
		if (status >= 500 && status !== 501) {
			return true;
		}

		return false;
	}

	/**
	 * Determine error severity for alerting
	 */
	private determineSeverity(status: number, code: ApiErrorCode): ErrorSeverity {
		// Critical: Network failures, server errors
		if (code === 'NETWORK_ERROR' || status >= 500) {
			return 'critical';
		}

		// Error: Auth failures
		if (code === 'AUTH_ERROR' || code === 'FORBIDDEN_ERROR') {
			return 'error';
		}

		// Warning: Rate limits, validation
		if (code === 'RATE_LIMIT_ERROR' || code === 'VALIDATION_ERROR') {
			return 'warning';
		}

		// Info: 404s and other client errors
		return 'info';
	}

	/**
	 * Determine error category for routing
	 */
	private determineCategory(code: ApiErrorCode): ErrorCategory {
		const categoryMap: Record<ApiErrorCode, ErrorCategory> = {
			NETWORK_ERROR: 'network',
			TIMEOUT_ERROR: 'timeout',
			AUTH_ERROR: 'authentication',
			FORBIDDEN_ERROR: 'authorization',
			VALIDATION_ERROR: 'validation',
			NOT_FOUND_ERROR: 'notFound',
			RATE_LIMIT_ERROR: 'rateLimit',
			SERVER_ERROR: 'server',
			SERVICE_UNAVAILABLE: 'server',
			CONFLICT_ERROR: 'client',
			PAYLOAD_TOO_LARGE: 'client',
			UNSUPPORTED_MEDIA_TYPE: 'client',
			UNKNOWN_ERROR: 'unknown'
		};
		return categoryMap[code];
	}

	/**
	 * Serialize error for logging (removes circular references, sensitive data)
	 */
	toJSON(): Record<string, unknown> {
		return {
			name: this.name,
			message: this.message,
			code: this.code,
			statusCode: this.statusCode,
			category: this.category,
			severity: this.severity,
			isRetryable: this.isRetryable,
			timestamp: this.timestamp.toISOString(),
			requestId: this.requestId,
			retryAfter: this.retryAfter,
			hasFields: !!this.fields
		};
	}

	/**
	 * Get a human-readable string representation
	 */
	override toString(): string {
		return `[${this.code}] ${this.message}${this.requestId ? ` (${this.requestId})` : ''}`;
	}
}

// =============================================================================
// CONCRETE ERROR CLASSES
// =============================================================================

/**
 * Network connectivity error - DNS, offline, connection refused
 */
export class NetworkError extends ApiError {
	readonly code = 'NETWORK_ERROR' as const;
	readonly statusCode = 0;

	constructor(message: string, requestId?: string, cause?: Error) {
		super(message || 'Network connection failed', 'NETWORK_ERROR', 0, requestId, {
			cause,
			isRetryable: true,
			severity: 'critical',
			category: 'network'
		});
	}
}

/**
 * Request timeout error - Request took too long
 */
export class TimeoutError extends ApiError {
	readonly code = 'TIMEOUT_ERROR' as const;
	readonly statusCode = 408;

	constructor(message: string, requestId?: string, cause?: Error) {
		super(message || 'Request timed out', 'TIMEOUT_ERROR', 408, requestId, {
			cause,
			isRetryable: true,
			severity: 'warning',
			category: 'timeout'
		});
	}
}

/**
 * Authentication error - 401, invalid/expired token
 */
export class AuthenticationError extends ApiError {
	readonly code = 'AUTH_ERROR' as const;
	readonly statusCode = 401;

	constructor(message: string, requestId?: string) {
		super(message || 'Authentication required', 'AUTH_ERROR', 401, requestId, {
			isRetryable: false,
			severity: 'error',
			category: 'authentication'
		});
	}
}

/**
 * Authorization error - 403, permission denied
 */
export class AuthorizationError extends ApiError {
	readonly code = 'FORBIDDEN_ERROR' as const;
	readonly statusCode = 403;

	constructor(message: string, requestId?: string) {
		super(message || 'Permission denied', 'FORBIDDEN_ERROR', 403, requestId, {
			isRetryable: false,
			severity: 'error',
			category: 'authorization'
		});
	}
}

/**
 * Validation error - 422, field-level errors
 */
export class ValidationError extends ApiError {
	readonly code = 'VALIDATION_ERROR' as const;
	readonly statusCode = 422;

	constructor(message: string, requestId?: string, fields?: Record<string, string[]>) {
		super(message || 'Validation failed', 'VALIDATION_ERROR', 422, requestId, {
			fields,
			isRetryable: false,
			severity: 'warning',
			category: 'validation'
		});
	}

	/**
	 * Get error messages for a specific field
	 */
	getFieldErrors(field: string): readonly string[] {
		return this.fields?.[field] ?? [];
	}

	/**
	 * Check if a specific field has errors
	 */
	hasFieldError(field: string): boolean {
		return (this.fields?.[field]?.length ?? 0) > 0;
	}

	/**
	 * Get all field names with errors
	 */
	getFieldsWithErrors(): readonly string[] {
		return this.fields ? Object.keys(this.fields) : [];
	}
}

/**
 * Not found error - 404, resource doesn't exist
 */
export class NotFoundError extends ApiError {
	readonly code = 'NOT_FOUND_ERROR' as const;
	readonly statusCode = 404;

	constructor(message: string, requestId?: string) {
		super(message || 'Resource not found', 'NOT_FOUND_ERROR', 404, requestId, {
			isRetryable: false,
			severity: 'info',
			category: 'notFound'
		});
	}
}

/**
 * Rate limit error - 429, too many requests
 */
export class RateLimitError extends ApiError {
	readonly code = 'RATE_LIMIT_ERROR' as const;
	readonly statusCode = 429;

	constructor(message: string, requestId?: string, retryAfter: number = 60) {
		super(message || 'Rate limit exceeded', 'RATE_LIMIT_ERROR', 429, requestId, {
			retryAfter,
			isRetryable: true,
			severity: 'warning',
			category: 'rateLimit'
		});
	}

	/**
	 * Get the date when requests can be retried
	 */
	getRetryDate(): Date {
		return new Date(Date.now() + (this.retryAfter ?? 60) * 1000);
	}
}

/**
 * Server error - 5xx, backend failure
 */
export class ServerError extends ApiError {
	readonly code = 'SERVER_ERROR' as const;
	readonly statusCode: number;

	constructor(
		message: string,
		statusCode: number = 500,
		requestId?: string,
		options?: { retryAfter?: number }
	) {
		super(message || 'Internal server error', 'SERVER_ERROR', statusCode, requestId, {
			retryAfter: options?.retryAfter,
			isRetryable: statusCode !== 501, // 501 Not Implemented is not retryable
			severity: 'critical',
			category: 'server'
		});
		this.statusCode = statusCode;
	}
}

/**
 * Service unavailable error - 503, maintenance mode
 */
export class ServiceUnavailableError extends ApiError {
	readonly code = 'SERVICE_UNAVAILABLE' as const;
	readonly statusCode = 503;

	constructor(message: string, requestId?: string, retryAfter?: number) {
		super(message || 'Service temporarily unavailable', 'SERVICE_UNAVAILABLE', 503, requestId, {
			retryAfter,
			isRetryable: true,
			severity: 'critical',
			category: 'server'
		});
	}
}

/**
 * Conflict error - 409, resource conflict
 */
export class ConflictError extends ApiError {
	readonly code = 'CONFLICT_ERROR' as const;
	readonly statusCode = 409;

	constructor(message: string, requestId?: string) {
		super(message || 'Resource conflict', 'CONFLICT_ERROR', 409, requestId, {
			isRetryable: false,
			severity: 'warning',
			category: 'client'
		});
	}
}

/**
 * Unknown/unexpected error - Catch-all for unhandled cases
 */
export class UnknownError extends ApiError {
	readonly code = 'UNKNOWN_ERROR' as const;
	readonly statusCode: number;

	constructor(message: string, statusCode: number = 0, requestId?: string, cause?: Error) {
		super(message || 'An unexpected error occurred', 'UNKNOWN_ERROR', statusCode, requestId, {
			cause,
			isRetryable: false,
			severity: 'error',
			category: 'unknown'
		});
		this.statusCode = statusCode;
	}
}

// =============================================================================
// TYPE GUARDS - For narrowing error types
// =============================================================================

/**
 * Check if a value is an ApiError
 */
export function isApiError(error: unknown): error is ApiError {
	return error instanceof ApiError;
}

/**
 * Check if error is a network-related error
 */
export function isNetworkError(error: unknown): error is NetworkError | TimeoutError {
	return error instanceof NetworkError || error instanceof TimeoutError;
}

/**
 * Check if error requires authentication
 */
export function isAuthError(error: unknown): error is AuthenticationError | AuthorizationError {
	return error instanceof AuthenticationError || error instanceof AuthorizationError;
}

/**
 * Check if error is retryable
 */
export function isRetryableError(error: unknown): boolean {
	if (isApiError(error)) {
		return error.isRetryable;
	}
	return false;
}

/**
 * Check if error is a validation error
 */
export function isValidationError(error: unknown): error is ValidationError {
	return error instanceof ValidationError;
}

/**
 * Check if error is a server error (5xx)
 */
export function isServerError(error: unknown): error is ServerError | ServiceUnavailableError {
	return error instanceof ServerError || error instanceof ServiceUnavailableError;
}

/**
 * Check if error is a rate limit error
 */
export function isRateLimitError(error: unknown): error is RateLimitError {
	return error instanceof RateLimitError;
}

// =============================================================================
// UTILITY FUNCTIONS
// =============================================================================

/**
 * Get user-friendly message for any error
 */
export function getUserFriendlyMessage(error: unknown): string {
	if (isApiError(error)) {
		return error.userMessage;
	}

	if (error instanceof Error) {
		return error.message;
	}

	return USER_FRIENDLY_MESSAGES.UNKNOWN_ERROR;
}

/**
 * Get validation errors from any error
 */
export function getValidationErrors(error: unknown): Readonly<Record<string, readonly string[]>> {
	if (isValidationError(error) && error.fields) {
		return error.fields;
	}
	return Object.freeze({});
}

/**
 * Create an appropriate error from HTTP status code
 */
export function createErrorFromStatus(
	status: number,
	message: string,
	requestId?: string,
	options?: {
		fields?: Record<string, string[]>;
		retryAfter?: number;
	}
): ApiError {
	switch (status) {
		case 0:
			return new NetworkError(message, requestId);
		case 401:
			return new AuthenticationError(message, requestId);
		case 403:
			return new AuthorizationError(message, requestId);
		case 404:
			return new NotFoundError(message, requestId);
		case 408:
			return new TimeoutError(message, requestId);
		case 409:
			return new ConflictError(message, requestId);
		case 422:
			return new ValidationError(message, requestId, options?.fields);
		case 429:
			return new RateLimitError(message, requestId, options?.retryAfter);
		case 503:
			return new ServiceUnavailableError(message, requestId, options?.retryAfter);
		default:
			if (status >= 500) {
				return new ServerError(message, status, requestId, options);
			}
			return new UnknownError(message, status, requestId);
	}
}

/**
 * Wrap any error as an ApiError
 */
export function wrapError(error: unknown, requestId?: string): ApiError {
	if (isApiError(error)) {
		return error;
	}

	if (error instanceof Error) {
		if (error.name === 'AbortError') {
			return new TimeoutError('Request was cancelled', requestId, error);
		}

		if (error.name === 'TypeError' && error.message.includes('fetch')) {
			return new NetworkError(error.message, requestId, error);
		}

		return new UnknownError(error.message, 0, requestId, error);
	}

	return new UnknownError(String(error), 0, requestId);
}

/**
 * Serialize error for logging (handles any error type)
 */
export function serializeError(error: unknown): Record<string, unknown> {
	if (isApiError(error)) {
		return error.toJSON();
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

// =============================================================================
// EXHAUSTIVE ERROR HANDLING HELPER
// =============================================================================

/**
 * Handle all error codes exhaustively
 * TypeScript will error if a case is missing
 *
 * @example
 * ```ts
 * try {
 *   await api.get('/endpoint');
 * } catch (error) {
 *   if (isApiError(error)) {
 *     handleApiError(error.code, {
 *       NETWORK_ERROR: () => showOfflineBanner(),
 *       AUTH_ERROR: () => redirectToLogin(),
 *       VALIDATION_ERROR: () => showFieldErrors(error.fields),
 *       // ... handle all codes
 *     });
 *   }
 * }
 * ```
 */
export function handleApiError<T>(
	code: ApiErrorCode,
	handlers: { [K in ApiErrorCode]: () => T }
): T {
	return handlers[code]();
}

// =============================================================================
// ERROR BOUNDARY INTEGRATION
// =============================================================================

/**
 * Error info for error boundaries
 */
export interface ErrorBoundaryInfo {
	readonly error: ApiError;
	readonly canRetry: boolean;
	readonly userMessage: string;
	readonly technicalDetails: string;
	readonly severity: ErrorSeverity;
}

/**
 * Create error boundary info from any error
 */
export function createErrorBoundaryInfo(error: unknown): ErrorBoundaryInfo {
	const apiError = isApiError(error) ? error : wrapError(error);

	return Object.freeze({
		error: apiError,
		canRetry: apiError.isRetryable,
		userMessage: apiError.userMessage,
		technicalDetails: `${apiError.code} (${apiError.statusCode})${apiError.requestId ? ` - ${apiError.requestId}` : ''}`,
		severity: apiError.severity
	});
}

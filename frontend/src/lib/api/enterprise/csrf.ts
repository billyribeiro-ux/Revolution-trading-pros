/**
 * CSRF Protection - Apple ICT9+ Security
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Implements Double-Submit Cookie pattern:
 * 1. Server sets a CSRF token in an httpOnly=false cookie
 * 2. Client reads the cookie and includes token in request header
 * 3. Server validates that cookie value matches header value
 *
 * This provides CSRF protection because:
 * - Attacker cannot read the cookie due to SameSite policy
 * - Attacker cannot set custom headers in cross-origin requests
 * - Token value must be known to be included in header
 *
 * Additional security measures:
 * - Token rotation on each request (optional)
 * - Token binding to session
 * - Timing-safe token comparison (server-side)
 */

import { browser } from '$app/environment';

// ═══════════════════════════════════════════════════════════════════════════════
// Configuration
// ═══════════════════════════════════════════════════════════════════════════════

const CSRF_COOKIE_NAME = 'XSRF-TOKEN';
const CSRF_HEADER_NAME = 'X-XSRF-TOKEN';
const CSRF_TOKEN_LENGTH = 32;

// Methods that require CSRF protection (state-changing)
const CSRF_PROTECTED_METHODS = ['POST', 'PUT', 'PATCH', 'DELETE'];

// ═══════════════════════════════════════════════════════════════════════════════
// Token Generation
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Generate a cryptographically secure CSRF token
 */
export function generateCsrfToken(): string {
	if (browser && window.crypto && window.crypto.getRandomValues) {
		const bytes = new Uint8Array(CSRF_TOKEN_LENGTH);
		window.crypto.getRandomValues(bytes);
		return Array.from(bytes)
			.map((b) => b.toString(16).padStart(2, '0'))
			.join('');
	}

	// Fallback (less secure, but still reasonable for non-critical use)
	let token = '';
	for (let i = 0; i < CSRF_TOKEN_LENGTH * 2; i++) {
		token += Math.floor(Math.random() * 16).toString(16);
	}
	return token;
}

// ═══════════════════════════════════════════════════════════════════════════════
// Cookie Management
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Get CSRF token from cookie
 */
export function getCsrfTokenFromCookie(): string | null {
	if (!browser) return null;

	try {
		const cookies = document.cookie.split(';');
		for (const cookie of cookies) {
			const [name, ...valueParts] = cookie.trim().split('=');
			if (name === CSRF_COOKIE_NAME) {
				// Handle URL-encoded tokens
				const value = valueParts.join('=');
				return decodeURIComponent(value);
			}
		}
	} catch (error) {
		console.warn('[CSRF] Failed to read cookie:', error);
	}

	return null;
}

/**
 * Set CSRF token cookie (for SPA bootstrapping)
 * Note: In production, this should be set by the server
 */
export function setCsrfCookie(token: string, options: { maxAge?: number; path?: string } = {}): void {
	if (!browser) return;

	const { maxAge = 7200, path = '/' } = options; // 2 hours default

	try {
		// Set cookie with security attributes
		// Note: httpOnly is intentionally false so JS can read it for double-submit
		const cookieValue = [
			`${CSRF_COOKIE_NAME}=${encodeURIComponent(token)}`,
			`Max-Age=${maxAge}`,
			`Path=${path}`,
			'SameSite=Strict',
			// Only set Secure in production (HTTPS)
			...(window.location.protocol === 'https:' ? ['Secure'] : [])
		].join('; ');

		document.cookie = cookieValue;
	} catch (error) {
		console.error('[CSRF] Failed to set cookie:', error);
	}
}

/**
 * Clear CSRF token cookie
 */
export function clearCsrfCookie(): void {
	if (!browser) return;

	try {
		document.cookie = `${CSRF_COOKIE_NAME}=; Max-Age=0; Path=/; SameSite=Strict`;
	} catch (error) {
		console.warn('[CSRF] Failed to clear cookie:', error);
	}
}

// ═══════════════════════════════════════════════════════════════════════════════
// Token Storage (Memory)
// ═══════════════════════════════════════════════════════════════════════════════

/** In-memory token storage for double-submit pattern */
let memoryToken: string | null = null;

/**
 * Get current CSRF token
 * Priority: Memory > Cookie > Generate New
 */
export function getCsrfToken(): string {
	// First try memory
	if (memoryToken) {
		return memoryToken;
	}

	// Try cookie
	const cookieToken = getCsrfTokenFromCookie();
	if (cookieToken) {
		memoryToken = cookieToken;
		return cookieToken;
	}

	// Generate new token
	const newToken = generateCsrfToken();
	memoryToken = newToken;
	setCsrfCookie(newToken);
	return newToken;
}

/**
 * Set CSRF token (typically called after server provides new token)
 */
export function setCsrfToken(token: string): void {
	memoryToken = token;
	setCsrfCookie(token);
}

/**
 * Clear CSRF token (on logout)
 */
export function clearCsrfToken(): void {
	memoryToken = null;
	clearCsrfCookie();
}

/**
 * Refresh CSRF token (generate new and update cookie)
 */
export function refreshCsrfToken(): string {
	const newToken = generateCsrfToken();
	memoryToken = newToken;
	setCsrfCookie(newToken);
	return newToken;
}

// ═══════════════════════════════════════════════════════════════════════════════
// Header Management
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Check if method requires CSRF protection
 */
export function requiresCsrfProtection(method: string): boolean {
	return CSRF_PROTECTED_METHODS.includes(method.toUpperCase());
}

/**
 * Get CSRF header for request
 */
export function getCsrfHeader(): Record<string, string> {
	const token = getCsrfToken();
	return {
		[CSRF_HEADER_NAME]: token
	};
}

/**
 * Get all CSRF-related headers for a request
 */
export function getCsrfHeaders(method: string): Record<string, string> {
	if (!requiresCsrfProtection(method)) {
		return {};
	}

	return getCsrfHeader();
}

// ═══════════════════════════════════════════════════════════════════════════════
// Validation
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Validate CSRF token from response (for token rotation)
 * Call this when server sends a new token in response header
 */
export function handleCsrfTokenFromResponse(headers: Headers | Record<string, string>): void {
	let newToken: string | null = null;

	if (headers instanceof Headers) {
		newToken = headers.get(CSRF_HEADER_NAME) || headers.get(CSRF_HEADER_NAME.toLowerCase());
	} else {
		newToken = headers[CSRF_HEADER_NAME] || headers[CSRF_HEADER_NAME.toLowerCase()];
	}

	if (newToken && newToken !== memoryToken) {
		setCsrfToken(newToken);
	}
}

// ═══════════════════════════════════════════════════════════════════════════════
// CSRF Interceptor
// ═══════════════════════════════════════════════════════════════════════════════

export interface CsrfInterceptorConfig {
	/** Whether CSRF protection is enabled */
	enabled: boolean;
	/** Custom cookie name */
	cookieName?: string;
	/** Custom header name */
	headerName?: string;
	/** Whether to rotate token on each request */
	rotateOnRequest?: boolean;
}

const defaultConfig: CsrfInterceptorConfig = {
	enabled: true,
	cookieName: CSRF_COOKIE_NAME,
	headerName: CSRF_HEADER_NAME,
	rotateOnRequest: false
};

let interceptorConfig = { ...defaultConfig };

/**
 * Configure CSRF interceptor
 */
export function configureCsrf(config: Partial<CsrfInterceptorConfig>): void {
	interceptorConfig = { ...defaultConfig, ...config };
}

/**
 * CSRF request interceptor
 * Add this to the request interceptor chain
 */
export function csrfRequestInterceptor<T extends { method?: string; headers?: Record<string, string> }>(
	config: T
): T {
	if (!interceptorConfig.enabled) {
		return config;
	}

	const method = config.method || 'GET';

	if (!requiresCsrfProtection(method)) {
		return config;
	}

	const csrfHeaders = getCsrfHeaders(method);

	// Rotate token if configured
	if (interceptorConfig.rotateOnRequest) {
		refreshCsrfToken();
	}

	return {
		...config,
		headers: {
			...config.headers,
			...csrfHeaders
		}
	};
}

/**
 * CSRF response interceptor
 * Call this to handle token rotation from server
 */
export function csrfResponseInterceptor(response: Response): void {
	if (!interceptorConfig.enabled) {
		return;
	}

	handleCsrfTokenFromResponse(response.headers);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Initialization
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Initialize CSRF protection
 * Call this on app startup
 */
export function initializeCsrf(): void {
	if (!browser) return;

	// Ensure we have a token
	getCsrfToken();

	// Listen for storage events to sync token across tabs
	window.addEventListener('storage', (event) => {
		if (event.key === `csrf_token_sync`) {
			const newToken = event.newValue;
			if (newToken && newToken !== memoryToken) {
				memoryToken = newToken;
				setCsrfCookie(newToken);
			}
		}
	});
}

// Auto-initialize in browser
if (browser) {
	// Defer to avoid blocking
	queueMicrotask(initializeCsrf);
}

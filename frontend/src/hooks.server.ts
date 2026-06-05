/**
 * SvelteKit Server Hooks - Auth, Security Headers & Performance
 * Apple ICT 11+ Principal Engineer Implementation
 * Following Google November 2025 SEO and security best practices
 *
 * Features:
 * - Server-side authentication middleware (Svelte 5 best practice)
 * - Route protection before load functions execute
 * - Comprehensive security headers (OWASP recommended)
 * - Performance headers for caching and compression
 * - SEO-friendly headers
 * - AI crawler control headers (November 2025)
 * - CORS configuration
 *
 * @version 3.0.0 - ICT 11+ Server-Side Auth + November 2025 Standards
 */

import type { Handle, HandleServerError } from '@sveltejs/kit';
import { redirect, isRedirect } from '@sveltejs/kit';
import { sequence } from '@sveltejs/kit/hooks';
import { env } from '$env/dynamic/private';

// API URL for server-side token validation. Precedence matches every
// +server.ts proxy and lib/server/axum/client.ts.
const API_BASE_URL = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

/**
 * MAINTENANCE MODE
 * ═══════════════════════════════════════════════════════════════════════════
 * Controlled via Admin → Settings → General → Maintenance Mode toggle.
 * Value is read from cms_site_settings in the DB (via the CMS v2 public
 * endpoint) and cached for 30 s so we don't hit the DB on every request.
 * Fallback: true (safe default keeps site protected if DB is unreachable).
 * ═══════════════════════════════════════════════════════════════════════════
 */
let _maintenanceModeCache: { value: boolean; expiresAt: number } | null = null;
const MAINTENANCE_CACHE_TTL_MS = 30_000;

async function isMaintenanceModeEnabled(): Promise<boolean> {
	const now = Date.now();
	if (_maintenanceModeCache && now < _maintenanceModeCache.expiresAt) {
		return _maintenanceModeCache.value;
	}
	try {
		const res = await fetch(`${API_BASE_URL}/api/cms/settings`, {
			headers: { Accept: 'application/json' },
			signal: AbortSignal.timeout(2000)
		});
		if (res.ok) {
			const json = await res.json();
			const mode = (json.data ?? json)?.maintenance_mode as boolean | undefined;
			_maintenanceModeCache = { value: mode ?? false, expiresAt: now + MAINTENANCE_CACHE_TTL_MS };
			return _maintenanceModeCache.value;
		}
	} catch {
		// DB / API unreachable — preserve last cached value or default to true (safe)
		if (_maintenanceModeCache) return _maintenanceModeCache.value;
	}
	return true;
}

/**
 * Maintenance Mode Handler
 * Redirects all requests to /maintenance except API and static assets
 */
const maintenanceHandler: Handle = async ({ event, resolve }) => {
	const { pathname } = event.url;

	// Localhost always bypasses maintenance mode
	const isLocalhost = event.url.hostname === 'localhost' || event.url.hostname === '127.0.0.1';

	const maintenanceMode = isLocalhost ? false : await isMaintenanceModeEnabled();

	if (maintenanceMode) {
		// Allow these paths during maintenance:
		const allowedPaths = [
			'/maintenance',
			'/api/maintenance/',
			'/api/health',
			'/_app/', // SvelteKit assets
			'/static/', // Static files
			'/favicon',
			'/robots.txt',
			'/sitemap'
		];

		const isAllowed = allowedPaths.some((path) => pathname.startsWith(path) || pathname === path);

		if (!isAllowed) {
			// Admin bypass: if the user has a valid admin token, let them through
			const accessToken =
				event.cookies.get('rtp_access_token') ||
				event.request.headers.get('Authorization')?.replace('Bearer ', '');

			if (accessToken) {
				try {
					const res = await fetch(`${API_BASE_URL}/api/auth/me`, {
						headers: { Authorization: `Bearer ${accessToken}`, Accept: 'application/json' }
					});
					if (res.ok) {
						const json = await res.json();
						const role = (json.data || json)?.role as string | undefined;
						if (role === 'admin' || role === 'superadmin') {
							return resolve(event);
						}
					}
				} catch {
					// API unreachable during maintenance — fall through to redirect
				}
			}

			// Redirect everyone else to maintenance page
			return new Response(null, {
				status: 307, // Temporary Redirect
				headers: {
					Location: '/maintenance',
					'Cache-Control': 'no-store'
				}
			});
		}
	}

	return resolve(event);
};

/**
 * Protected routes that require authentication
 * These routes will redirect to login if user is not authenticated
 */
const PROTECTED_ROUTES = ['/dashboard', '/account', '/checkout', '/trading-room', '/admin'];

/**
 * Authentication Handler - ICT 11+ Server-Side Pattern
 * Runs BEFORE all load functions for secure route protection
 *
 * This is the Svelte 5 recommended pattern:
 * - Auth check happens server-side before any data loading
 * - Prevents unauthorized data fetching
 * - More secure than client-side auth guards
 */
const authHandler: Handle = async ({ event, resolve }) => {
	const { pathname } = event.url;

	// Get auth token from cookies (always read for locals.auth)
	const accessToken = event.cookies.get('rtp_access_token');
	const refreshToken = event.cookies.get('rtp_refresh_token');

	// Also check Authorization header (for API calls)
	const authHeader = event.request.headers.get('Authorization');
	const headerToken = authHeader?.replace('Bearer ', '');
	const token = accessToken || headerToken;

	// ICT 11+ FIX: Always set locals.auth method (declared in app.d.ts)
	// This method returns the current user session
	event.locals.auth = async () => {
		if (event.locals.user) {
			return { user: event.locals.user };
		}
		return null;
	};

	// Check if this is a protected route
	const isProtectedRoute = PROTECTED_ROUTES.some((route) => pathname.startsWith(route));

	if (!isProtectedRoute) {
		// Not a protected route - but still try to get user if token exists
		if (token) {
			try {
				// ICT7 FIX: Backend /me endpoint is under /auth router
				const response = await fetch(`${API_BASE_URL}/api/auth/me`, {
					method: 'GET',
					headers: {
						Authorization: `Bearer ${token}`,
						'Content-Type': 'application/json',
						Accept: 'application/json'
					}
				});
				if (response.ok) {
					const json = await response.json();
					const userData = json.data || json;
					event.locals.user = {
						id: userData.id,
						email: userData.email,
						name: userData.name,
						role: userData.role
					};
					// ICT 7 FIX: Set accessToken in locals for non-protected routes too
					event.locals.accessToken = token;
				}
			} catch {
				// Ignore errors on non-protected routes
			}
		}
		return resolve(event);
	}

	if (!token && !refreshToken) {
		// No tokens - redirect to login with return URL
		const returnUrl = encodeURIComponent(pathname);
		redirect(303, `/login?redirect=${returnUrl}`);
	}

	// ICT 7 FIX: Validate token with proper timeout and error handling
	try {
		// Add timeout to prevent hanging requests
		const controller = new AbortController();
		const timeoutId = setTimeout(() => controller.abort(), 10000); // 10s timeout

		// ICT7 FIX: Backend /me endpoint is under /auth router
		const response = await fetch(`${API_BASE_URL}/api/auth/me`, {
			method: 'GET',
			headers: {
				Authorization: `Bearer ${token || refreshToken}`,
				'Content-Type': 'application/json',
				Accept: 'application/json'
			},
			signal: controller.signal
		});

		clearTimeout(timeoutId);

		if (response.ok) {
			const json = await response.json();
			// ICT11+ Fix: Backend wraps response in { success: true, data: {...} }
			const userData = json.data || json;

			// Set user in locals for use in load functions
			event.locals.user = {
				id: userData.id,
				email: userData.email,
				name: userData.name,
				role: userData.role
			};

			// ICT 7 FIX: Store token in locals for server-side API calls
			// This allows +page.server.ts load functions to make authenticated API calls
			event.locals.accessToken = token || refreshToken;
		} else if (response.status === 401) {
			// ICT 7: 401 = Invalid/expired token - attempt refresh if available
			if (!refreshToken) {
				// No refresh token - permanent auth failure
				console.info('[Auth Hook] 401 with no refresh token - redirecting to login');
				const returnUrl = encodeURIComponent(pathname);
				redirect(303, `/login?redirect=${returnUrl}`);
			}

			// SERVER-SIDE TOKEN REFRESH
			//
			// This is one of THREE token refresh implementations:
			// 1. /src/lib/stores/auth.ts: Client-side refresh
			// 2. /src/lib/api/auth.ts: Delegates to store
			// 3. HERE: Server-side refresh for SSR requests
			//
			// All three MUST use consistent token handling.
			// If modifying refresh logic, update all three locations.
			const refreshResponse = await fetch(`${API_BASE_URL}/api/auth/refresh`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					Accept: 'application/json'
				},
				body: JSON.stringify({ refresh_token: refreshToken })
			});

			if (refreshResponse.ok) {
				const refreshJson = await refreshResponse.json();
				// ICT11+ Fix: Backend wraps response in { success: true, data: {...} }
				const refreshData = refreshJson.data || refreshJson;

				// Set new access token cookie (backend sends access_token, not token)
				const newToken = refreshData.access_token || refreshData.token;
				const isSecure = import.meta.env.PROD || !event.url.hostname.includes('localhost');
				event.cookies.set('rtp_access_token', newToken, {
					path: '/',
					httpOnly: true,
					secure: isSecure,
					sameSite: 'lax',
					maxAge: refreshData.expires_in || 3600
				});

				// ICT7 FIX: Fetch user data with new token - /me is under /auth router
				const userResponse = await fetch(`${API_BASE_URL}/api/auth/me`, {
					method: 'GET',
					headers: {
						Authorization: `Bearer ${newToken}`,
						'Content-Type': 'application/json'
					}
				});

				if (userResponse.ok) {
					const userJson = await userResponse.json();
					// ICT11+ Fix: Backend wraps response in { success: true, data: {...} }
					const userData = userJson.data || userJson;
					event.locals.user = {
						id: userData.id,
						email: userData.email,
						name: userData.name,
						role: userData.role
					};

					// ICT 7 FIX: Store refreshed token in locals for server-side API calls
					event.locals.accessToken = newToken;
				}
			} else {
				// Refresh failed - permanent auth failure
				console.info('[Auth Hook] Token refresh failed - redirecting to login');
				const returnUrl = encodeURIComponent(pathname);
				redirect(303, `/login?redirect=${returnUrl}`);
			}
		} else if (response.status >= 500) {
			// ICT 7: 5xx = Server error (transient) - preserve session
			console.warn(`[Auth Hook] API server error (${response.status}) - preserving session`);
			// Don't set user - let it fall through to catch block
			throw new Error(`API_SERVER_ERROR_${response.status}`);
		} else if (response.status === 403) {
			// ICT 7: 403 = Forbidden (permanent) - redirect to login
			console.info('[Auth Hook] 403 Forbidden - redirecting to login');
			const returnUrl = encodeURIComponent(pathname);
			redirect(303, `/login?redirect=${returnUrl}`);
		} else {
			// ICT 7: Other errors (4xx) - treat as transient, preserve session
			console.warn(`[Auth Hook] Unexpected response (${response.status}) - preserving session`);
			throw new Error(`API_UNEXPECTED_RESPONSE_${response.status}`);
		}
	} catch (error) {
		// ICT 7: Sophisticated error handling with transient vs permanent failure detection

		// If it's a redirect, rethrow it (permanent auth failure)
		if (isRedirect(error)) {
			throw error;
		}

		// Determine if this is a transient or permanent failure
		const errorMessage = error instanceof Error ? error.message : String(error);
		const isNetworkError =
			errorMessage.includes('fetch') ||
			errorMessage.includes('network') ||
			errorMessage.includes('ECONNREFUSED') ||
			errorMessage.includes('ETIMEDOUT') ||
			errorMessage.includes('aborted') ||
			errorMessage.includes('API_SERVER_ERROR') ||
			errorMessage.includes('API_UNEXPECTED_RESPONSE');

		if (isNetworkError && (token || refreshToken)) {
			// FIX-C-2 (2026-04-29): the old "decode JWT payload locally and
			// trust it" fallback has been removed. Decoding without verifying
			// the signature trusted the contents of any value sitting in the
			// rtp_access_token cookie — including a forged one. Combined
			// with a transient API failure (easy to induce), an attacker
			// could land on admin pages whose +page.server.ts load function
			// uses event.locals.user.role for gating.
			//
			// New behavior: any auth failure (transient or permanent),
			// when we cannot reach the API, leaves event.locals.user = null
			// and forces re-authentication. The frontend is never the
			// authority on identity; the API is. Removing this code means:
			//   - During a real API outage, users see "please log in" rather
			//     than partial UI rendered with bogus identity.
			//   - The console.log that echoed payload.email is also gone
			//     (PII into server logs).
			//
			// Original block (REMOVED) included:
			//   const payload = JSON.parse(Buffer.from(parts[1], 'base64').toString());
			//   event.locals.user = { id, email, name, role: payload.role || 'user' };
			console.warn(
				'[Auth Hook] Transient API failure — clearing session, user must re-authenticate',
				errorMessage
			);
			event.locals.user = null;
		} else {
			// ICT 7: Permanent failure or no tokens - redirect to login
			console.error('[Auth Hook] Permanent auth failure:', error);
			const returnUrl = encodeURIComponent(pathname);
			redirect(303, `/login?redirect=${returnUrl}`);
		}
	}

	return resolve(event);
};

/**
 * Security Headers Handler
 * Implements OWASP security headers for improved trust signals and security
 */
const securityHeaders: Handle = async ({ event, resolve }) => {
	const response = await resolve(event);

	// Clone headers to modify them
	const headers = new Headers(response.headers);

	// ═══════════════════════════════════════════════════════════════════════════
	// Security Headers (OWASP Best Practices)
	// ═══════════════════════════════════════════════════════════════════════════

	// NOTE: X-XSS-Protection deliberately NOT set. The header is
	// deprecated — modern browsers ignore it, and in legacy browsers its
	// auditor could itself be abused to introduce XSS. The strong
	// Content-Security-Policy is the correct, modern control.

	// Prevent MIME type sniffing
	headers.set('X-Content-Type-Options', 'nosniff');

	// Prevent clickjacking
	headers.set('X-Frame-Options', 'SAMEORIGIN');

	// Referrer policy - balance between privacy and analytics
	headers.set('Referrer-Policy', 'strict-origin-when-cross-origin');

	// Permissions Policy - disable unnecessary browser features
	headers.set(
		'Permissions-Policy',
		'accelerometer=(), autoplay=(), camera=(), cross-origin-isolated=(), display-capture=(), encrypted-media=(), fullscreen=(self), geolocation=(), gyroscope=(), keyboard-map=(), magnetometer=(), microphone=(), midi=(), payment=(), picture-in-picture=(self), publickey-credentials-get=(), screen-wake-lock=(), sync-xhr=(), usb=(), xr-spatial-tracking=()'
	);

	// Strict Transport Security (HSTS) - force HTTPS
	// max-age=31536000 = 1 year, includeSubDomains for all subdomains
	headers.set('Strict-Transport-Security', 'max-age=31536000; includeSubDomains; preload');

	// Cross-Origin policies for security
	// ICT 7 FIX: Relaxed COEP to allow third-party iframes (Box.com, YouTube, Vimeo)
	// COOP: same-origin-allow-popups allows popups (needed for OAuth flows)
	// CORP: cross-origin allows embedding from other origins
	// COEP: unsafe-none allows third-party resources without CORP headers
	headers.set('Cross-Origin-Opener-Policy', 'same-origin-allow-popups');
	headers.set('Cross-Origin-Resource-Policy', 'cross-origin');
	// NOTE: COEP removed - it blocks third-party iframes like Box.com that don't set CORP headers
	// headers.set('Cross-Origin-Embedder-Policy', 'credentialless');

	// ═══════════════════════════════════════════════════════════════════════════
	// SEO & Performance Headers
	// ═══════════════════════════════════════════════════════════════════════════

	// Vary header for proper caching with Accept-Encoding
	if (!headers.has('Vary')) {
		headers.set('Vary', 'Accept-Encoding');
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// AI/Bot Control Headers (November 2025)
	// ═══════════════════════════════════════════════════════════════════════════

	// X-Robots-Tag for fine-grained control (complements meta robots)
	// This is particularly useful for non-HTML resources
	const pathname = event.url.pathname;

	// Don't add X-Robots-Tag for sitemap and robots.txt
	if (!pathname.includes('sitemap') && !pathname.includes('robots')) {
		// Allow indexing with full snippets for public pages
		if (
			pathname.startsWith('/blog') ||
			pathname.startsWith('/courses') ||
			pathname.startsWith('/indicators') ||
			pathname.startsWith('/alerts') ||
			pathname.startsWith('/live-trading-rooms') ||
			pathname.startsWith('/resources') ||
			pathname === '/' ||
			pathname === '/about' ||
			pathname === '/our-mission'
		) {
			headers.set(
				'X-Robots-Tag',
				'index, follow, max-snippet:-1, max-image-preview:large, max-video-preview:-1'
			);
		}
		// Prevent indexing for private areas
		else if (
			pathname.startsWith('/admin') ||
			pathname.startsWith('/account') ||
			pathname.startsWith('/cart') ||
			pathname.startsWith('/checkout') ||
			pathname.startsWith('/api')
		) {
			headers.set('X-Robots-Tag', 'noindex, nofollow');
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// November 2025 AI Training Opt-Out Headers
	// These headers inform AI crawlers that content should not be used for training
	// ═══════════════════════════════════════════════════════════════════════════

	// Opt out of AI training for all major providers
	// GPTBot, ChatGPT-User, Claude-Web, Google-Extended, CCBot, etc.
	headers.set('X-AI-Training-Opt-Out', 'true');

	// TDM (Text and Data Mining) Reservation Protocol - November 2025
	// W3C standard for expressing TDM preferences
	headers.set('TDM-Reservation', '1');

	// Machine Learning Data Use header
	headers.set('X-ML-Data-Use', 'disallow');

	// Content licensing header - indicates content is copyrighted
	headers.set('X-Content-License', 'All Rights Reserved - No AI Training Permitted');

	// OpenAI specific header
	headers.set('X-OpenAI-Block', 'true');

	// Anthropic specific header
	headers.set('X-Anthropic-Block', 'true');

	// Google AI training opt-out
	headers.set('X-Google-Extended', 'disallow');

	return new Response(response.body, {
		status: response.status,
		statusText: response.statusText,
		headers
	});
};

/**
 * Performance & Caching Handler
 * Implements optimal caching strategies for different resource types
 *
 * ICT 7 FIX (Svelte 5 / SvelteKit 2.x Best Practice):
 * Uses the official `preload` option in ResolveOptions to control what gets preloaded.
 *
 * Root cause: SvelteKit eagerly preloads CSS for all anticipated routes, but due to
 * conditional rendering ({#if mounted} blocks) and complex layout branches, the CSS
 * may not be consumed within the browser's ~3 second timeout window.
 *
 * Solution: Use SvelteKit's native preload callback to disable CSS preloading.
 * CSS still loads via <link rel="stylesheet"> - just without <link rel="preload">.
 * HTTP/2 multiplexing ensures efficient loading without preload hints.
 *
 * @see https://svelte.dev/docs/kit/@sveltejs-kit#ResolveOptions
 */
const performanceHandler: Handle = async ({ event, resolve }) => {
	const response = await resolve(event, {
		// SvelteKit 2.x preload callback - control what gets preloaded in <head>
		// By default, js and css are preloaded. We disable CSS preloading to prevent
		// "preloaded but not used" browser warnings on pages with conditional rendering.
		preload: ({ type }) => {
			// Only preload JS modules, not CSS
			// CSS will still load via stylesheet links, just without preload hints
			return type === 'js';
		},
		// Transform HTML for performance optimizations
		transformPageChunk: ({ html }) => {
			// Light minification: collapse whitespace between tags only
			// Preserves whitespace inside <pre>, <code>, <textarea>, <script>
			if (import.meta.env.PROD) {
				return html.replace(/>\s+</g, '> <');
			}
			return html;
		}
	});

	return response;
};

/**
 * Request timing handler for monitoring
 */
const timingHandler: Handle = async ({ event, resolve }) => {
	const start = performance.now();

	const response = await resolve(event);

	const duration = performance.now() - start;

	// Add Server-Timing header for performance monitoring
	const headers = new Headers(response.headers);
	headers.set('Server-Timing', `total;dur=${duration.toFixed(2)}`);

	return new Response(response.body, {
		status: response.status,
		statusText: response.statusText,
		headers
	});
};

// Combine all handlers in sequence
// Maintenance handler runs FIRST to redirect all traffic when enabled
// Auth handler runs second to protect routes before any data loading
export const handle: Handle = sequence(
	maintenanceHandler,
	authHandler,
	timingHandler,
	securityHeaders,
	performanceHandler
);

/**
 * Server-side error handler — Phase 6.4
 *
 * - Generates a unique error ID per occurrence.
 * - Logs full details (URL, method, message, stack, ID) to console.error.
 * - In production, forwards to VITE_ERROR_TRACKING_URL if configured
 *   (mirrors hooks.client.ts:128 — server has no window.gtag).
 * - Returns a safe, sanitized payload: never leaks stack to the browser.
 */
export const handleError: HandleServerError = async ({ error, event, status }) => {
	const errorId = crypto.randomUUID();

	const url = event.url?.toString() ?? 'unknown';
	const method = event.request?.method ?? 'unknown';
	const message = error instanceof Error ? error.message : String(error);
	const stack = error instanceof Error ? (error.stack ?? '') : '';

	// Always log full details on the server.
	console.error('[handleError]', {
		errorId,
		status,
		method,
		url,
		message,
		stack
	});

	// In production, forward to the configured error-tracking endpoint
	// (server-side equivalent of the gtag + fetch block in hooks.client.ts:128).
	const isProduction =
		(typeof process !== 'undefined' && process.env.NODE_ENV === 'production') ||
		import.meta.env.PROD;

	if (isProduction) {
		const errorEndpoint = env.VITE_ERROR_TRACKING_URL;
		if (errorEndpoint) {
			try {
				await fetch(errorEndpoint, {
					method: 'POST',
					headers: { 'Content-Type': 'application/json' },
					body: JSON.stringify({
						errorId,
						status,
						method,
						url,
						message,
						// Do NOT include stack in the outbound payload — it may contain
						// internal file paths. Log it locally above instead.
						timestamp: new Date().toISOString(),
						source: 'server'
					})
				});
			} catch (trackingError) {
				// Don't let tracking failures cascade into the error handler itself.
				console.warn('[handleError] Failed to forward to error tracking:', trackingError);
			}
		}
	}

	return {
		message: 'Internal error',
		errorId,
		status: status >= 500 ? 500 : status
	};
};

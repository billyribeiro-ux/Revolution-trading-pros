import { building } from '$app/env';
import { SvelteDate } from 'svelte/reactivity';
import { isAdminRole } from '#lib/config/role-policy.js';
import { isAuthenticatedPath } from '#lib/config/private-routes.js';
import {
	authenticatedUser,
	privateAuthRedirect,
	privateAuthHeaders,
	secureAuthCookie
} from '#lib/server/auth-response.js';

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
import { sequence, type Handle, type HandleServerError } from '@sveltejs/kit/hooks';

import {
	API_BASE_URL as ENV_API_BASE_URL,
	BACKEND_URL,
	VITE_ERROR_TRACKING_URL
} from '$app/env/private';
import { PRODUCTION_ORIGIN, DEFAULT_LOCALE, isNoindexPath } from '#lib/seo/config.js';

// API URL for server-side token validation. Precedence matches every
// +server.ts proxy and lib/server/axum/client.ts.
const API_BASE_URL = ENV_API_BASE_URL || BACKEND_URL || 'http://localhost:8080';

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

	// Localhost and build-time prerendering always bypass maintenance mode
	const isLocalhost =
		building ||
		event.url.hostname === 'localhost' ||
		event.url.hostname === '127.0.0.1' ||
		event.url.hostname === 'sveltekit-prerender';

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
						if (isAdminRole(role)) {
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

	// Authentication is a runtime policy. Let Kit inspect each route's prerender
	// option, so crawler redirects cannot become static login pages for private
	// routes explicitly marked prerender=false (or conceal missing destinations).
	if (building) return resolve(event);
	let refreshedCookie: string | undefined;
	function privateRedirect(location: string): Response {
		return privateAuthRedirect(event, location, refreshedCookie);
	}

	// Check if this is a protected route
	const isProtectedRoute = isAuthenticatedPath(pathname);

	if (!isProtectedRoute) {
		// Not a protected route - but still try to get user if token exists
		if (token) {
			try {
				// ICT7 FIX: Backend /me endpoint is under /auth router
				// ICT 7 FIX: match the protected branch's 10s abort guard so a
				// hung API can't stall non-protected SSR requests indefinitely.
				const response = await fetch(`${API_BASE_URL}/api/auth/me`, {
					method: 'GET',
					headers: {
						Authorization: `Bearer ${token}`,
						'Content-Type': 'application/json',
						Accept: 'application/json'
					},
					signal: AbortSignal.timeout(10000)
				});
				if (response.ok) {
					const json = await response.json();
					event.locals.user = authenticatedUser(json);
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
		return privateRedirect(`/login?redirect=${returnUrl}`);
	}

	// ICT 7 FIX: Validate token with proper timeout and error handling
	try {
		// ICT7 FIX: Backend /me endpoint is under /auth router
		const response = await fetch(`${API_BASE_URL}/api/auth/me`, {
			method: 'GET',
			headers: {
				Authorization: `Bearer ${token || refreshToken}`,
				'Content-Type': 'application/json',
				Accept: 'application/json'
			},
			signal: AbortSignal.timeout(10000)
		});

		if (response.ok) {
			const json = await response.json();
			// ICT11+ Fix: Backend wraps response in { success: true, data: {...} }
			event.locals.user = authenticatedUser(json);

			// ICT 7 FIX: Store token in locals for server-side API calls
			// This allows +page.server.ts load functions to make authenticated API calls
			event.locals.accessToken = token || refreshToken;
		} else if (response.status === 401) {
			// ICT 7: 401 = Invalid/expired token - attempt refresh if available
			if (!refreshToken) {
				// No refresh token - permanent auth failure
				console.info('[Auth Hook] 401 with no refresh token - redirecting to login');
				const returnUrl = encodeURIComponent(pathname);
				return privateRedirect(`/login?redirect=${returnUrl}`);
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
				body: JSON.stringify({ refresh_token: refreshToken }),
				signal: AbortSignal.timeout(10000)
			});

			if (refreshResponse.ok) {
				const refreshJson = await refreshResponse.json();
				// ICT11+ Fix: Backend wraps response in { success: true, data: {...} }
				const refreshData = refreshJson.data || refreshJson;

				// Set new access token cookie (backend sends access_token, not token)

				const newToken = refreshData.access_token || refreshData.token;
				const expiresIn = refreshData.expires_in;
				if (
					typeof newToken !== 'string' ||
					!newToken ||
					typeof expiresIn !== 'number' ||
					!Number.isFinite(expiresIn) ||
					expiresIn <= 0
				)
					throw new Error('API_UNEXPECTED_RESPONSE_INVALID_REFRESH');
				const cookieOptions = {
					path: '/',
					httpOnly: true,
					secure: secureAuthCookie(event.url),
					sameSite: 'lax' as const,
					maxAge: expiresIn
				};
				event.cookies.set('rtp_access_token', newToken, cookieOptions);
				refreshedCookie = event.cookies.serialize('rtp_access_token', newToken, cookieOptions);

				// ICT7 FIX: Fetch user data with new token - /me is under /auth router
				const userResponse = await fetch(`${API_BASE_URL}/api/auth/me`, {
					method: 'GET',
					headers: {
						Authorization: `Bearer ${newToken}`,
						'Content-Type': 'application/json'
					},
					signal: AbortSignal.timeout(10000)
				});

				if (userResponse.ok) {
					const userJson = await userResponse.json();
					// ICT11+ Fix: Backend wraps response in { success: true, data: {...} }
					event.locals.user = authenticatedUser(userJson);

					// ICT 7 FIX: Store refreshed token in locals for server-side API calls
					event.locals.accessToken = newToken;
				} else if (userResponse.status === 401 || userResponse.status === 403) {
					return privateRedirect(`/login?redirect=${encodeURIComponent(pathname)}`);
				} else {
					throw new Error(`Identity service returned ${userResponse.status}`);
				}
			} else if (refreshResponse.status !== 401 && refreshResponse.status !== 403) {
				throw new Error(`Refresh service returned ${refreshResponse.status}`);
			} else {
				// Refresh rejected the credentials.
				console.info('[Auth Hook] Token refresh failed - redirecting to login');
				const returnUrl = encodeURIComponent(pathname);
				return privateRedirect(`/login?redirect=${returnUrl}`);
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
			return privateRedirect(`/login?redirect=${returnUrl}`);
		} else {
			// ICT 7: Other errors (4xx) - treat as transient, preserve session
			console.warn(`[Auth Hook] Unexpected response (${response.status}) - preserving session`);
			throw new Error(`API_UNEXPECTED_RESPONSE_${response.status}`);
		}
	} catch {
		// Unexpected/transport failures are not evidence of invalid credentials.
		// Fail closed, preserve the session, and never authorize from decoded JWT claims.
		event.locals.user = null;
		const headers = privateAuthHeaders(refreshedCookie);
		headers.set('Retry-After', '5');
		return new Response('Authentication service is temporarily unavailable. Please retry.', {
			status: 503,
			headers
		});
	}

	// Protect the HTML request as well as layout data, including CSR admin pages.
	if (
		(pathname === '/admin' || pathname.startsWith('/admin/')) &&
		event.locals.user &&
		!isAdminRole(event.locals.user.role)
	) {
		return privateRedirect('/?error=admin_required');
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
	const newsletterCallback = ['/newsletter/confirm', '/newsletter/unsubscribe'].includes(
		event.url.pathname
	);
	headers.set(
		'Referrer-Policy',
		newsletterCallback ? 'no-referrer' : 'strict-origin-when-cross-origin'
	);
	const privateAuthPath = [
		'/api/auth',
		'/api/admin',
		'/api/oauth',
		'/admin',
		'/dashboard',
		'/account',
		'/checkout',
		'/my',
		'/api/my'
	].some((prefix) => event.url.pathname === prefix || event.url.pathname.startsWith(`${prefix}/`));
	if (
		newsletterCallback ||
		event.locals.user ||
		privateAuthPath ||
		event.request.headers.has('Authorization')
	)
		headers.set('Cache-Control', 'no-store');

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
	// Enforce IDX-09: Non-production origins (localhost, 127.0.0.1, *.pages.dev) and
	// private/auth/dashboard/admin/API routes MUST carry X-Robots-Tag: noindex, nofollow.
	const pathname = event.url.pathname;
	const isNonProdOrigin = !building && event.url.origin !== PRODUCTION_ORIGIN;

	if (!pathname.includes('sitemap') && !pathname.includes('robots')) {
		if (isNonProdOrigin || isNoindexPath(pathname) || response.status >= 400) {
			headers.set('X-Robots-Tag', 'noindex, nofollow');
		} else {
			headers.set(
				'X-Robots-Tag',
				'index, follow, max-snippet:-1, max-image-preview:large, max-video-preview:-1'
			);
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
		// PERF-04: Preload critical fonts, JS modules, and CSS stylesheets
		preload: ({ type }) => type === 'font' || type === 'js' || type === 'css',
		// SEM-03: Transform %lang% and %dir% in app.html
		transformPageChunk: ({ html }) => {
			const withLocale = html.replace('%lang%', DEFAULT_LOCALE).replace('%dir%', 'ltr');
			if (import.meta.env.PROD) {
				return withLocale.replace(/>\s+</g, '> <');
			}
			return withLocale;
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
export const handleError: HandleServerError = async ({ kind, error, event }) => {
	const errorId = crypto.randomUUID();

	const url = event.url?.toString() ?? 'unknown';
	const method = event.request?.method ?? 'unknown';

	// SvelteKit 3 no longer passes a top-level `status`/`message`; they are
	// carried by the error itself and discriminated by `kind`. A status is only
	// observable here for `framework` and `validation` errors — for `app` errors
	// SvelteKit hands over the App.Error *body* (which has no status field), and
	// `unknown` errors have none at all. Left undefined rather than defaulted to
	// 500, so the log says "unknown" instead of asserting something false.
	const status = kind === 'framework' || kind === 'validation' ? error.status : undefined;
	const message =
		kind === 'app' || kind === 'framework' || kind === 'validation'
			? error.message
			: error instanceof Error
				? error.message
				: String(error);
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
		const errorEndpoint = VITE_ERROR_TRACKING_URL;
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
						timestamp: new SvelteDate().toISOString(),
						source: 'server'
					})
				});
			} catch (trackingError) {
				// Don't let tracking failures cascade into the error handler itself.
				console.warn('[handleError] Failed to forward to error tracking:', trackingError);
			}
		}
	}

	// Return ONLY `errorId`.
	//
	// In SvelteKit 3 a returned `status`/`message` is an OVERRIDE, and omitted
	// properties are inherited from the caught error. Inheriting is exactly what
	// every kind wants here:
	//   app        → the body passed to `error(...)`, i.e. the status and message
	//                the developer deliberately chose to show
	//   framework  → the real status (404 etc.) and SvelteKit's safe message
	//   validation → likewise
	//   unknown    → 500 / 'Internal Error', so internals are never leaked
	//
	// Returning a hardcoded `{ message: 'Internal error', status: 500 }` — which
	// is what this did on arrival in SvelteKit 3 — turned every `error(404, ...)`
	// in the app into a 500 "Internal error". SvelteKit 2 never routed expected
	// errors through this hook at all, so the bug had no analogue on `main`.
	return { errorId };
};

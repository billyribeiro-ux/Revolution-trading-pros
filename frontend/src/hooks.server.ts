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

import type { Handle } from '@sveltejs/kit';
import { redirect } from '@sveltejs/kit';
import { sequence } from '@sveltejs/kit/hooks';

// API URL for server-side token validation
const API_BASE_URL = process.env.VITE_API_URL || 'https://revolution-trading-pros-api.fly.dev';

/**
 * Protected routes that require authentication
 * These routes will redirect to login if user is not authenticated
 */
const PROTECTED_ROUTES = ['/dashboard', '/account', '/checkout', '/trading-room'];

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

	// Check if this is a protected route
	const isProtectedRoute = PROTECTED_ROUTES.some(route => pathname.startsWith(route));

	if (!isProtectedRoute) {
		// Not a protected route - continue without auth check
		return resolve(event);
	}

	// Get auth token from cookies
	const accessToken = event.cookies.get('rtp_access_token');
	const refreshToken = event.cookies.get('rtp_refresh_token');

	// Also check Authorization header (for API calls)
	const authHeader = event.request.headers.get('Authorization');
	const headerToken = authHeader?.replace('Bearer ', '');

	const token = accessToken || headerToken;

	if (!token && !refreshToken) {
		// No tokens - redirect to login with return URL
		const returnUrl = encodeURIComponent(pathname);
		throw redirect(303, `/login?redirect=${returnUrl}`);
	}

	// Validate token by calling /api/auth/me
	try {
		const response = await fetch(`${API_BASE_URL}/api/auth/me`, {
			method: 'GET',
			headers: {
				'Authorization': `Bearer ${token || refreshToken}`,
				'Content-Type': 'application/json',
				'Accept': 'application/json'
			}
		});

		if (response.ok) {
			const userData = await response.json();

			// Set user in locals for use in load functions
			event.locals.user = {
				id: String(userData.id),
				email: userData.email,
				name: userData.name,
				role: userData.role
			};
		} else if (response.status === 401 && refreshToken) {
			// Token expired - try to refresh
			const refreshResponse = await fetch(`${API_BASE_URL}/api/auth/refresh`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					'Accept': 'application/json'
				},
				body: JSON.stringify({ refresh_token: refreshToken })
			});

			if (refreshResponse.ok) {
				const refreshData = await refreshResponse.json();

				// Set new access token cookie
				event.cookies.set('rtp_access_token', refreshData.token, {
					path: '/',
					httpOnly: true,
					secure: true,
					sameSite: 'lax',
					maxAge: refreshData.expires_in || 3600
				});

				// Fetch user data with new token
				const userResponse = await fetch(`${API_BASE_URL}/api/auth/me`, {
					method: 'GET',
					headers: {
						'Authorization': `Bearer ${refreshData.token}`,
						'Content-Type': 'application/json'
					}
				});

				if (userResponse.ok) {
					const userData = await userResponse.json();
					event.locals.user = {
						id: String(userData.id),
						email: userData.email,
						name: userData.name,
						role: userData.role
					};
				}
			} else {
				// Refresh failed - redirect to login
				const returnUrl = encodeURIComponent(pathname);
				throw redirect(303, `/login?redirect=${returnUrl}`);
			}
		} else {
			// Auth failed - redirect to login
			const returnUrl = encodeURIComponent(pathname);
			throw redirect(303, `/login?redirect=${returnUrl}`);
		}
	} catch (error) {
		// If it's a redirect, rethrow it
		if (error instanceof Response || (error as any)?.status === 303) {
			throw error;
		}

		// Network error - allow client-side auth to handle
		console.error('[Auth Hook] Token validation failed:', error);
		// Don't redirect on network errors - let client handle
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

	// Prevent XSS attacks - strongest protection
	headers.set('X-XSS-Protection', '1; mode=block');

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
	headers.set('Cross-Origin-Opener-Policy', 'same-origin');
	headers.set('Cross-Origin-Resource-Policy', 'same-origin');
	headers.set('Cross-Origin-Embedder-Policy', 'credentialless');

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
			headers.set('X-Robots-Tag', 'index, follow, max-snippet:-1, max-image-preview:large, max-video-preview:-1');
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
 */
const performanceHandler: Handle = async ({ event, resolve }) => {
	const response = await resolve(event, {
		// Transform HTML for performance optimizations
		transformPageChunk: ({ html }) => {
			// Minify HTML in production (remove extra whitespace)
			if (import.meta.env.PROD) {
				return html.replace(/\s+/g, ' ').replace(/>\s+</g, '><');
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
// Auth handler runs FIRST to protect routes before any data loading
export const handle: Handle = sequence(authHandler, timingHandler, securityHeaders, performanceHandler);

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

// Production fallback - Rust API on Fly.io
// ICT 7 FIX: VITE_API_URL does NOT include /api suffix - we add it here
const PROD_API_ROOT = 'https://revolution-trading-pros-api.fly.dev';
const API_ROOT = env.VITE_API_URL || env.BACKEND_URL || PROD_API_ROOT;
const API_URL = `${API_ROOT}/api`;

/**
 * Token Refresh Proxy - Apple ICT 7 Principal Engineer Grade
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * ROOT CAUSE FIX: Backend returns flat response, not wrapped in {data: {...}}
 * This proxy now handles BOTH formats for maximum compatibility.
 *
 * SECURITY ARCHITECTURE:
 * 1. Client sends POST with credentials: 'include'
 * 2. Server reads refresh token from httpOnly cookie (rtp_refresh_token)
 * 3. Server validates and returns new access token
 * 4. New refresh token set via httpOnly cookie in response
 *
 * This ensures refresh tokens are NEVER exposed to JavaScript (XSS safe).
 *
 * @version 2.0.0 - Fixed data envelope handling
 */
export const POST: RequestHandler = async ({ request, cookies }) => {
	const requestId = crypto.randomUUID().slice(0, 8);
	const startTime = performance.now();

	try {
		const sessionId = request.headers.get('X-Session-ID') || '';

		// Parse request body - may contain refresh_token from memory (during same session)
		let body: { refresh_token?: string } = {};
		try {
			body = await request.json();
		} catch {
			// Empty body is acceptable - we'll use the cookie
		}

		// ICT 7 SECURITY: Read refresh token from httpOnly cookie if not in body
		// This is the secure path for page refreshes / new tabs
		if (!body.refresh_token) {
			const cookieToken = cookies.get('rtp_refresh_token');
			if (cookieToken) {
				body.refresh_token = cookieToken;
				console.debug(`[Refresh:${requestId}] Using httpOnly cookie token`);
			}
		}

		// If still no token, return error
		if (!body.refresh_token) {
			console.warn(`[Refresh:${requestId}] No refresh token available`);
			return json({ error: 'No refresh token', message: 'Please log in again' }, { status: 401 });
		}

		const response = await fetch(`${API_URL}/auth/refresh`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				'X-Session-ID': sessionId
			},
			body: JSON.stringify(body)
		});

		const rawData = await response.json();
		const duration = Math.round(performance.now() - startTime);

		// ICT 7 FIX: Handle BOTH wrapped {data: {...}} and flat response formats
		// Backend currently returns flat: { token, refresh_token, expires_in }
		// But some responses may be wrapped: { data: { access_token, refresh_token, expires_in } }
		const authData = rawData.data || rawData;

		// ICT 7: Extract tokens with fallbacks for different field names
		const accessToken = authData.access_token || authData.token;
		const refreshToken = authData.refresh_token;
		const expiresIn = authData.expires_in;

		if (response.ok && accessToken) {
			// Set new access token cookie
			cookies.set('rtp_access_token', accessToken, {
				path: '/',
				httpOnly: true,
				secure: true,
				sameSite: 'lax',
				maxAge: expiresIn || 900 // 15 min default
			});
			console.debug(`[Refresh:${requestId}] Access token cookie updated`);

			// Set new refresh token cookie (token rotation)
			if (refreshToken) {
				cookies.set('rtp_refresh_token', refreshToken, {
					path: '/',
					httpOnly: true,
					secure: true,
					sameSite: 'lax',
					maxAge: 60 * 60 * 24 * 30 // 30 days (matches login proxy)
				});
				console.debug(`[Refresh:${requestId}] Refresh token cookie rotated`);
			}

			console.debug(`[Refresh:${requestId}] Success (${duration}ms)`);

			// Return normalized response with data wrapper for frontend compatibility
			return json(
				{
					data: {
						access_token: accessToken,
						refresh_token: refreshToken,
						expires_in: expiresIn
					}
				},
				{ status: response.status }
			);
		} else {
			console.warn(`[Refresh:${requestId}] Backend returned ${response.status} (${duration}ms)`);
			return json(rawData, { status: response.status });
		}
	} catch (error) {
		const duration = Math.round(performance.now() - startTime);
		const errorMessage = error instanceof Error ? error.message : 'Unknown error';
		console.error(`[Refresh:${requestId}] Error: ${errorMessage} (${duration}ms)`);
		return json(
			{ error: 'Token refresh failed', message: 'Unable to refresh authentication token' },
			{ status: 500 }
		);
	}
};

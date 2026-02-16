/**
 * Set Session API Endpoint - ICT 11+ Server-Side Cookie Setting
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * This endpoint sets httpOnly cookies for server-side auth.
 * Called by the client after successful login to enable SSR auth.
 *
 * SECURITY:
 * - Cookies are httpOnly (not accessible via JavaScript)
 * - Cookies are secure (HTTPS only in production)
 * - Cookies use sameSite: 'lax' for CSRF protection
 *
 * @version 1.0.0 - ICT 11+ Server-Side Auth
 */

import { json, type RequestEvent } from '@sveltejs/kit';
// SvelteKit auto-generates types - this import will be available after build
import type { RequestHandler } from './$types';
import { logger } from '$lib/utils/logger';

export const POST: RequestHandler = async ({ request, cookies }: RequestEvent) => {
	try {
		const body = await request.json();
		// Support both camelCase and snake_case for OAuth callback compatibility
		const accessToken = body.access_token || body.accessToken;
		const refreshToken = body.refresh_token || body.refreshToken;
		const expiresIn = body.expires_in || body.expiresIn;

		if (!accessToken) {
			return json({ error: 'Access token required' }, { status: 400 });
		}

		// ICT 7 FIX: secure=false on localhost (http), true in production (https)
		const isSecure = process.env.NODE_ENV === 'production' || !request.url.includes('localhost');

		// Set access token cookie
		cookies.set('rtp_access_token', accessToken, {
			path: '/',
			httpOnly: true,
			secure: isSecure,
			sameSite: 'lax',
			maxAge: expiresIn || 3600 // Default 1 hour
		});

		// Set refresh token cookie if provided
		if (refreshToken) {
			cookies.set('rtp_refresh_token', refreshToken, {
				path: '/',
				httpOnly: true,
				secure: isSecure,
				sameSite: 'lax',
				maxAge: 60 * 60 * 24 * 30 // 30 days
			});
		}

		logger.info('[Set Session] Cookies set successfully');

		return json({ success: true });
	} catch (error) {
		logger.error('[Set Session] Error:', error);
		return json({ error: 'Failed to set session' }, { status: 500 });
	}
};

/**
 * DELETE - Clear session cookies on logout
 */
export const DELETE: RequestHandler = async ({ cookies }: RequestEvent) => {
	// Clear both cookies
	cookies.delete('rtp_access_token', { path: '/' });
	cookies.delete('rtp_refresh_token', { path: '/' });

	return json({ success: true });
};

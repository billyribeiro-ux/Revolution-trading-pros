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

export const POST: RequestHandler = async ({ request, cookies }: RequestEvent) => {
	try {
		const { accessToken, refreshToken, expiresIn } = await request.json();

		if (!accessToken) {
			return json({ error: 'Access token required' }, { status: 400 });
		}

		// ICT11+ Fix: Cloudflare Pages doesn't have process.env.NODE_ENV
		// Always use secure cookies - Cloudflare Pages is always HTTPS
		const isSecure = true;

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

		console.log('[Set Session] Cookies set successfully');

		return json({ success: true });
	} catch (error) {
		console.error('[Set Session] Error:', error);
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

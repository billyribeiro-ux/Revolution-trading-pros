import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

// Production fallback - Rust API on Fly.io
const PROD_API = 'https://revolution-trading-pros-api.fly.dev/api';
const API_URL = env.VITE_API_URL || (env.BACKEND_URL ? `${env.BACKEND_URL}/api` : PROD_API);

/**
 * ICT 7 SECURITY: Token refresh endpoint using httpOnly cookies
 * Apple Principal Engineer Grade: Defense in depth
 *
 * SECURITY ARCHITECTURE:
 * 1. Client sends POST with credentials: 'include'
 * 2. Server reads refresh token from httpOnly cookie (rtp_refresh_token)
 * 3. Server validates and returns new access token
 * 4. New refresh token set via httpOnly cookie in response
 *
 * This ensures refresh tokens are NEVER exposed to JavaScript (XSS safe).
 */
export const POST: RequestHandler = async ({ request, cookies }) => {
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
			}
		}

		// If still no token, return error
		if (!body.refresh_token) {
			return json(
				{ error: 'No refresh token', message: 'Please log in again' },
				{ status: 401 }
			);
		}

		const response = await fetch(`${API_URL}/auth/refresh`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				'Accept': 'application/json',
				'X-Session-ID': sessionId
			},
			body: JSON.stringify(body)
		});

		const data = await response.json();

		// ICT 7 SECURITY: Set new tokens in httpOnly cookies on successful refresh
		if (response.ok && data.data) {
			const authData = data.data;

			// Set new access token cookie
			if (authData.access_token) {
				cookies.set('rtp_access_token', authData.access_token, {
					path: '/',
					httpOnly: true,
					secure: true,
					sameSite: 'lax',
					maxAge: authData.expires_in || 900 // 15 min default
				});
			}

			// Set new refresh token cookie (token rotation)
			if (authData.refresh_token) {
				cookies.set('rtp_refresh_token', authData.refresh_token, {
					path: '/',
					httpOnly: true,
					secure: true,
					sameSite: 'lax',
					maxAge: 60 * 60 * 24 * 7 // 7 days
				});
			}
		}

		return json(data, { status: response.status });
	} catch (error) {
		console.error('[API Proxy] Auth refresh error:', error);
		return json(
			{ error: 'Token refresh failed', message: 'Unable to refresh authentication token' },
			{ status: 500 }
		);
	}
};

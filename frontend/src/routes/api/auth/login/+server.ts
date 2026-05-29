/**
 * Auth Login Proxy - ICT 7 Principal Engineer Grade
 *
 * Server-side proxy for login requests to avoid CORB issues.
 * Cloudflare Pages doesn't support 200 proxy redirects to external URLs,
 * so we proxy through SvelteKit server routes instead.
 */

import { json } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const API_URL = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

export const POST = async ({ request, cookies }: RequestEvent) => {
	try {
		const body = await request.json();

		// (audit 2026-05-17) Removed the temp "ICT 7 Debug" request log:
		// logging the email + auth-attempt shape on every login is PII /
		// security noise that should never reach production. Errors are
		// still surfaced via console.error below.

		// Forward request to backend
		const response = await fetch(`${API_URL}/api/auth/login`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json'
			},
			body: JSON.stringify(body)
		});

		// Get response text first to handle non-JSON responses
		const responseText = await response.text();
		let data;
		try {
			data = JSON.parse(responseText);
		} catch {
			console.error('[Auth Proxy] Non-JSON response:', responseText);
			return json({ error: 'Invalid response from auth server' }, { status: 502 });
		}

		// (audit 2026-05-17) Removed the temp "ICT 7 Debug" response log
		// (auth-token presence / user shape on every login). console.error
		// below still covers genuine failures.

		// If successful, set httpOnly cookies for the tokens
		if (response.ok && data.token) {
			// ICT 7 FIX: secure=false on localhost (http), true in production (https)
			const isSecure = process.env.NODE_ENV === 'production' || !request.url.includes('localhost');

			// Set access token cookie
			cookies.set('rtp_access_token', data.token, {
				path: '/',
				httpOnly: true,
				secure: isSecure,
				sameSite: 'lax',
				maxAge: data.expires_in || 3600
			});

			// Set refresh token cookie if provided
			if (data.refresh_token) {
				cookies.set('rtp_refresh_token', data.refresh_token, {
					path: '/',
					httpOnly: true,
					secure: isSecure,
					sameSite: 'lax',
					maxAge: 60 * 60 * 24 * 30 // 30 days
				});
			}
		}

		// Return the response with proper status
		return json(data, { status: response.status });
	} catch (error) {
		console.error('[Auth Proxy] Error:', error);
		return json({ error: 'Authentication service unavailable' }, { status: 503 });
	}
};

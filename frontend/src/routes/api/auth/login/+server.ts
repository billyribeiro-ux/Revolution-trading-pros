/**
 * Auth Login Proxy - ICT 7 Principal Engineer Grade
 *
 * Server-side proxy for login requests to avoid CORB issues.
 * Cloudflare Pages doesn't support 200 proxy redirects to external URLs,
 * so we proxy through SvelteKit server routes instead.
 */

import { json } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

const API_URL = 'https://revolution-trading-pros-api.fly.dev';

export const POST = async ({ request, cookies }: RequestEvent) => {
	try {
		const body = await request.json();

		// ICT 7 Debug: Log request details (mask password)
		console.log('[Auth Proxy] Login request:', {
			email: body.email,
			hasPassword: !!body.password,
			passwordLength: body.password?.length || 0
		});

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

		// ICT 7 Debug: Log full response for diagnosis
		console.log('[Auth Proxy] Backend response:', {
			status: response.status,
			error: data.error,
			hasToken: !!data.token,
			hasUser: !!data.user
		});

		// If successful, set httpOnly cookies for the tokens
		if (response.ok && data.token) {
			const isSecure = true; // Cloudflare Pages is always HTTPS

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

			console.log('[Auth Proxy] Cookies set successfully');
		}

		// Return the response with proper status
		return json(data, { status: response.status });
	} catch (error) {
		console.error('[Auth Proxy] Error:', error);
		return json({ error: 'Authentication service unavailable' }, { status: 503 });
	}
};

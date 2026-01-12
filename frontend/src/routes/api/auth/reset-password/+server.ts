/**
 * Auth Reset Password Proxy - ICT 11+ Principal Engineer Grade
 *
 * Server-side proxy for reset-password requests to avoid CORB issues.
 * Cloudflare Pages doesn't support 200 proxy redirects to external URLs,
 * so we proxy through SvelteKit server routes instead.
 */

import { json } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

const API_URL = 'https://revolution-trading-pros-api.fly.dev';

export const POST = async ({ request }: RequestEvent) => {
	try {
		const body = await request.json();

		// ICT 11+ Debug: Log request details (mask sensitive data)
		console.log('[Auth Proxy] Reset password request:', {
			email: body.email,
			hasToken: !!body.token,
			hasPassword: !!body.password
		});

		// Forward request to backend
		const response = await fetch(`${API_URL}/api/auth/reset-password`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				'Accept': 'application/json',
			},
			body: JSON.stringify(body),
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

		// ICT 11+ Debug: Log response for diagnosis
		console.log('[Auth Proxy] Reset password response:', {
			status: response.status,
			message: data.message
		});

		// Return the response with proper status
		return json(data, { status: response.status });

	} catch (error) {
		console.error('[Auth Proxy] Reset password error:', error);
		return json(
			{ error: 'Password reset service unavailable' },
			{ status: 503 }
		);
	}
};

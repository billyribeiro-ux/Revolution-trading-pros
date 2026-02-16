/**
 * Auth Register Proxy - ICT 11+ Principal Engineer Grade
 *
 * Server-side proxy for register requests to avoid CORB issues.
 * Cloudflare Pages doesn't support 200 proxy redirects to external URLs,
 * so we proxy through SvelteKit server routes instead.
 */

import { json } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';
import { logger } from '$lib/utils/logger';

const API_URL = 'https://revolution-trading-pros-api.fly.dev';

export const POST = async ({ request }: RequestEvent) => {
	try {
		const body = await request.json();

		// ICT 11+ Debug: Log request details (mask sensitive data)
		logger.info('[Auth Proxy] Register request:', {
			email: body.email,
			hasPassword: !!body.password,
			name: body.name
		});

		// Forward request to backend
		const response = await fetch(`${API_URL}/api/auth/register`, {
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
			logger.error('[Auth Proxy] Non-JSON response:', responseText);
			return json({ error: 'Invalid response from auth server' }, { status: 502 });
		}

		// ICT 11+ Debug: Log response for diagnosis
		logger.info('[Auth Proxy] Register response:', {
			status: response.status,
			message: data.message,
			error: data.error
		});

		// Return the response with proper status
		return json(data, { status: response.status });
	} catch (error) {
		logger.error('[Auth Proxy] Register error:', error);
		return json({ error: 'Registration service unavailable' }, { status: 503 });
	}
};

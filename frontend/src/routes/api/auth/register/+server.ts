/**
 * Auth Register Proxy - ICT 11+ Principal Engineer Grade
 *
 * Server-side proxy for register requests to avoid CORB issues.
 * Cloudflare Pages doesn't support 200 proxy redirects to external URLs,
 * so we proxy through SvelteKit server routes instead.
 */

import { json } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

export const POST = async ({ request }: RequestEvent) => {
	try {
		const body = await request.json();

		// (audit 2026-05-17) Removed temp "ICT 11+ Debug" request log
		// (logged email/name on every registration — PII noise).

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
			console.error('[Auth Proxy] Non-JSON response:', responseText);
			return json({ error: 'Invalid response from auth server' }, { status: 502 });
		}

		// (audit 2026-05-17) Removed temp "ICT 11+ Debug" response log.

		// Return the response with proper status
		return json(data, { status: response.status });
	} catch (error) {
		console.error('[Auth Proxy] Register error:', error);
		return json({ error: 'Registration service unavailable' }, { status: 503 });
	}
};

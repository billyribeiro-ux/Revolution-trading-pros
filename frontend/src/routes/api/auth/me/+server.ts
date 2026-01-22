/**
 * Auth Me Endpoint - Get Current User Info
 * ICT 7 Principal Engineer Grade
 *
 * Returns the current authenticated user's info including role.
 * Used by dashboard components to determine admin privileges.
 */

import { json } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

const API_URL = 'https://revolution-trading-pros-api.fly.dev';

export const GET = async ({ cookies }: RequestEvent) => {
	try {
		const accessToken = cookies.get('rtp_access_token');

		if (!accessToken) {
			return json({ user: null, error: 'Not authenticated' }, { status: 401 });
		}

		// Forward request to backend with auth token
		const response = await fetch(`${API_URL}/api/auth/me`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${accessToken}`
			}
		});

		if (!response.ok) {
			// Token might be expired
			if (response.status === 401) {
				// Clear invalid cookies
				cookies.delete('rtp_access_token', { path: '/' });
				return json({ user: null, error: 'Session expired' }, { status: 401 });
			}
			return json({ user: null, error: 'Auth check failed' }, { status: response.status });
		}

		const data = await response.json();

		return json(data, { status: 200 });
	} catch (error) {
		console.error('[Auth Me] Error:', error);
		return json({ user: null, error: 'Authentication service unavailable' }, { status: 503 });
	}
};

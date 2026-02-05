/**
 * Admin Membership Plans API Proxy
 * Forwards requests to backend /api/admin/membership-plans endpoint
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

const API_URL = import.meta.env.VITE_API_URL || 'https://revolution-trading-pros-api.fly.dev';

export const GET: RequestHandler = async ({ cookies }) => {
	try {
		const token = cookies.get('auth_token');
		
		const response = await fetch(`${API_URL}/api/admin/membership-plans`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				...(token ? { Authorization: `Bearer ${token}` } : {})
			}
		});

		// ICT 7 FIX: Check response.ok BEFORE parsing JSON to avoid 500 on auth errors
		if (!response.ok) {
			// Try to parse error response, fallback to status text
			let errorData;
			try {
				errorData = await response.json();
			} catch {
				errorData = { error: response.statusText || 'Request failed' };
			}
			return json(errorData, { status: response.status });
		}

		const data = await response.json();
		return json(data);
	} catch (err) {
		console.error('[API Proxy] Failed to fetch membership plans:', err);
		return json({ error: 'Failed to fetch membership plans' }, { status: 500 });
	}
};

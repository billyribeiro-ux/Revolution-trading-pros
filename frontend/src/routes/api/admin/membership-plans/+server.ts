/**
 * Admin Membership Plans API Proxy
 * Forwards requests to backend /api/admin/membership-plans endpoint
 */

import { json, error } from '@sveltejs/kit';
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

		const data = await response.json();
		
		if (!response.ok) {
			return json(data, { status: response.status });
		}

		return json(data);
	} catch (err) {
		console.error('[API Proxy] Failed to fetch membership plans:', err);
		return error(500, 'Failed to fetch membership plans');
	}
};

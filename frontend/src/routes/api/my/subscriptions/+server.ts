/**
 * User Subscriptions API Proxy
 * ICT 7 FIX: Proxy to backend /api/subscriptions/my endpoint
 * Returns empty array if backend is unavailable
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

const API_URL = process.env.VITE_API_URL || 'https://revolution-trading-pros-api.fly.dev';

export const GET: RequestHandler = async ({ cookies, fetch }) => {
	const token = cookies.get('rtp_access_token');
	
	if (!token) {
		// No token - return empty subscriptions (don't error out)
		return json({
			subscriptions: [],
			total: 0
		});
	}

	try {
		const response = await fetch(`${API_URL}/api/subscriptions/my`, {
			headers: {
				'Authorization': `Bearer ${token}`,
				'Content-Type': 'application/json',
				'Accept': 'application/json'
			}
		});

		if (!response.ok) {
			// Backend error - return empty subscriptions gracefully
			console.warn('[API Proxy] /api/subscriptions/my returned', response.status);
			return json({
				subscriptions: [],
				total: 0
			});
		}

		const data = await response.json();
		
		// Transform backend response to expected format
		return json({
			subscriptions: data.data || data.subscriptions || [],
			total: data.total || 0
		});
	} catch (error) {
		// Network error - return empty subscriptions gracefully
		console.error('[API Proxy] Error fetching subscriptions:', error);
		return json({
			subscriptions: [],
			total: 0
		});
	}
};

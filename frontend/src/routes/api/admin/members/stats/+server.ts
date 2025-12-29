/**
 * Members Stats API Proxy
 * Routes requests to Rust API backend to avoid CORS issues
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

// Production fallback - Rust API on Fly.io
const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';
const BACKEND_URL = env.BACKEND_URL || env.VITE_API_URL || PROD_BACKEND;

export const GET: RequestHandler = async ({ cookies, fetch }) => {
	try {
		const token = cookies.get('auth_token');

		const response = await fetch(`${BACKEND_URL}/api/admin/members/stats`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				'Accept': 'application/json',
				...(token ? { 'Authorization': `Bearer ${token}` } : {})
			}
		});

		if (!response.ok) {
			// Return mock data for development if backend unavailable
			if (response.status === 404 || response.status === 500) {
				return json({
					overview: {
						total_members: 0,
						active_members: 0,
						new_this_month: 0
					},
					subscriptions: {
						active: 0,
						cancelled: 0,
						expired: 0
					}
				});
			}

			const error = await response.json().catch(() => ({ message: 'Request failed' }));
			return json(error, { status: response.status });
		}

		const data = await response.json();
		return json(data);
	} catch (error) {
		console.error('[API] Members stats error:', error);
		// Return empty data on connection error
		return json({
			overview: {
				total_members: 0,
				active_members: 0,
				new_this_month: 0
			},
			subscriptions: {
				active: 0,
				cancelled: 0,
				expired: 0
			}
		});
	}
};

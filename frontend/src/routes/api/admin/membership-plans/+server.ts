/**
 * Admin Membership Plans API Proxy
 * ICT 7 FIX: Routes requests to Rust API backend with graceful fallback
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

		const response = await fetch(`${BACKEND_URL}/api/admin/membership-plans`, {
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
					plans: [],
					total: 0
				});
			}

			// Pass through auth errors
			if (response.status === 401) {
				return json({ message: 'Unauthorized' }, { status: 401 });
			}

			const error = await response.json().catch(() => ({ message: 'Request failed' }));
			return json(error, { status: response.status });
		}

		const data = await response.json();
		return json(data);
	} catch (error) {
		console.debug('[API] Membership plans not available');
		// Return empty plans on error - graceful degradation
		return json({
			plans: [],
			total: 0
		});
	}
};

export const POST: RequestHandler = async ({ cookies, fetch, request }) => {
	try {
		const token = cookies.get('auth_token');
		const body = await request.json();

		const response = await fetch(`${BACKEND_URL}/api/admin/membership-plans`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				'Accept': 'application/json',
				...(token ? { 'Authorization': `Bearer ${token}` } : {})
			},
			body: JSON.stringify(body)
		});

		if (!response.ok) {
			const error = await response.json().catch(() => ({ message: 'Request failed' }));
			return json(error, { status: response.status });
		}

		const data = await response.json();
		return json(data);
	} catch (error) {
		console.error('[API] Create membership plan error:', error);
		return json({ message: 'Failed to create membership plan' }, { status: 500 });
	}
};

/**
 * Coupons API Proxy
 * Routes requests to Laravel backend to avoid CORS issues
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { env } from '$env/dynamic/private';

const BACKEND_URL = env.BACKEND_URL || env.LARAVEL_API_URL || 'http://localhost:8000';

export const GET: RequestHandler = async ({ cookies, fetch }) => {
	try {
		const token = cookies.get('auth_token');

		const response = await fetch(`${BACKEND_URL}/api/admin/coupons`, {
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
					coupons: [],
					total: 0
				});
			}

			const error = await response.json().catch(() => ({ message: 'Request failed' }));
			return json(error, { status: response.status });
		}

		const data = await response.json();
		return json(data);
	} catch (error) {
		console.error('[API] Coupons error:', error);
		return json({
			coupons: [],
			total: 0
		});
	}
};

export const POST: RequestHandler = async ({ cookies, fetch, request }) => {
	try {
		const token = cookies.get('auth_token');
		const body = await request.json();

		const response = await fetch(`${BACKEND_URL}/api/admin/coupons`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				'Accept': 'application/json',
				...(token ? { 'Authorization': `Bearer ${token}` } : {})
			},
			body: JSON.stringify(body)
		});

		const data = await response.json();
		return json(data, { status: response.status });
	} catch (error) {
		console.error('[API] Create coupon error:', error);
		return json({ message: 'Failed to create coupon' }, { status: 500 });
	}
};

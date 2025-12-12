/**
 * Products Stats API Proxy
 * Routes requests to Laravel backend to avoid CORS issues
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { env } from '$env/dynamic/private';

const BACKEND_URL = env.BACKEND_URL || env.LARAVEL_API_URL || 'http://localhost:8000';

export const GET: RequestHandler = async ({ cookies, fetch }) => {
	try {
		const token = cookies.get('auth_token');

		const response = await fetch(`${BACKEND_URL}/api/admin/products/stats`, {
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
					total: 0,
					active: 0,
					draft: 0,
					featured: 0,
					total_revenue: 0,
					total_sales: 0,
					data: {
						total: 0
					}
				});
			}

			const error = await response.json().catch(() => ({ message: 'Request failed' }));
			return json(error, { status: response.status });
		}

		const data = await response.json();
		return json(data);
	} catch (error) {
		console.error('[API] Products stats error:', error);
		return json({
			total: 0,
			active: 0,
			draft: 0,
			featured: 0,
			total_revenue: 0,
			total_sales: 0,
			data: {
				total: 0
			}
		});
	}
};

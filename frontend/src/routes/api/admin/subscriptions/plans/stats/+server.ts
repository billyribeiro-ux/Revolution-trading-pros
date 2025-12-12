/**
 * Subscriptions Plans Stats API Proxy
 * Routes requests to Laravel backend to avoid CORS issues
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { env } from '$env/dynamic/private';

const BACKEND_URL = env.BACKEND_URL || env.LARAVEL_API_URL || 'http://localhost:8000';

export const GET: RequestHandler = async ({ cookies, fetch }) => {
	try {
		const token = cookies.get('auth_token');

		const response = await fetch(`${BACKEND_URL}/api/admin/subscriptions/plans/stats`, {
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
					data: {
						active_subscriptions: 0,
						total_active: 0,
						monthly_revenue: 0,
						mrr: 0,
						annual_revenue: 0,
						churn_rate: 0
					}
				});
			}

			const error = await response.json().catch(() => ({ message: 'Request failed' }));
			return json(error, { status: response.status });
		}

		const data = await response.json();
		return json(data);
	} catch (error) {
		console.error('[API] Subscriptions stats error:', error);
		return json({
			data: {
				active_subscriptions: 0,
				total_active: 0,
				monthly_revenue: 0,
				mrr: 0,
				annual_revenue: 0,
				churn_rate: 0
			}
		});
	}
};

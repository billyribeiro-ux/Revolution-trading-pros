/**
 * Subscriptions Metrics API Proxy
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

		const response = await fetch(`${BACKEND_URL}/api/subscriptions/metrics`, {
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
					total_subscriptions: 0,
					active_subscriptions: 0,
					cancelled_subscriptions: 0,
					paused_subscriptions: 0,
					mrr: 0,
					arr: 0,
					churn_rate: 0,
					growth_rate: 0,
					average_revenue_per_user: 0,
					lifetime_value: 0
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
		console.debug('[API] Subscriptions metrics not available');
		// Return mock metrics on error - graceful degradation
		return json({
			total_subscriptions: 0,
			active_subscriptions: 0,
			cancelled_subscriptions: 0,
			paused_subscriptions: 0,
			mrr: 0,
			arr: 0,
			churn_rate: 0,
			growth_rate: 0,
			average_revenue_per_user: 0,
			lifetime_value: 0
		});
	}
};

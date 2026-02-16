/**
 * Members Stats API Proxy
 * Fetches real stats from backend
 *
 * @version 2.0.0 - January 2026
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { logger } from '$lib/utils/logger';

const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';

export const GET: RequestHandler = async ({ request }) => {
	const backendUrl = PROD_BACKEND;
	const authHeader = request.headers.get('Authorization') || '';

	try {
		const response = await fetch(`${backendUrl}/api/admin/members/stats`, {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: authHeader
			}
		});

		if (response.ok) {
			const data = await response.json();
			return json(data);
		}

		logger.warn(`Backend members stats returned ${response.status}`);
	} catch (err) {
		logger.warn('Backend members stats not available:', err);
	}

	// Return empty stats on error, not mock data
	return json({
		overview: {
			total_members: 0,
			new_this_month: 0,
			new_last_month: 0,
			growth_rate: 0
		},
		subscriptions: {
			active: 0,
			trial: 0,
			churned: 0,
			churn_rate: 0
		},
		revenue: {
			mrr: 0,
			total: 0,
			avg_ltv: 0
		}
	});
};

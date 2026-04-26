/**
 * Members Stats API Proxy
 * Fetches real stats from backend
 *
 * @version 2.0.0 - January 2026
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { env } from '$env/dynamic/private';
const PROD_BACKEND =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

export const GET: RequestHandler = async ({ request, cookies }) => {
	const backendUrl = PROD_BACKEND;
	// FIX-2026-04-26: prefer canonical rtp_access_token cookie, fall back to header.
	// Old: const authHeader = request.headers.get('Authorization') || '';
	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) error(401, 'Unauthorized');
	const authHeader = `Bearer ${token}`;

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

		console.warn(`Backend members stats returned ${response.status}`);
	} catch (err) {
		console.warn('Backend members stats not available:', err);
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

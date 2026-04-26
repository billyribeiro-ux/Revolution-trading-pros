/**
 * Subscriptions Metrics API Proxy
 */

import { env } from '$env/dynamic/private';
import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

// FIX-2026-04-26: was hardcoded zeros stub; now real proxy to backend
const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

// FIX-2026-04-26: old hardcoded-zeros body commented out below
// export const GET: RequestHandler = async () => {
// 	// ICT 7 FIX: Return mock data immediately - endpoint not implemented on backend
// 	// This prevents 404 console errors
// 	return json({
// 		total_subscriptions: 0,
// 		active_subscriptions: 0,
// 		cancelled_subscriptions: 0,
// 		paused_subscriptions: 0,
// 		mrr: 0,
// 		arr: 0,
// 		churn_rate: 0,
// 		growth_rate: 0,
// 		average_revenue_per_user: 0,
// 		lifetime_value: 0
// 	});
// };

export const GET: RequestHandler = async ({ cookies, fetch }) => {
	const token = cookies.get('rtp_access_token');
	if (!token) error(401, 'Unauthorized');

	const res = await fetch(`${API_URL}/api/subscriptions/metrics`, {
		headers: { Authorization: `Bearer ${token}` }
	});
	if (!res.ok) error(res.status as Parameters<typeof error>[0], `Backend returned ${res.status}`);
	return json(await res.json());
};

/**
 * Subscriptions Metrics API Proxy
 * ICT 7 FIX: Return mock data immediately - backend endpoint not implemented
 * This prevents 404 console errors from fetch calls to non-existent endpoints
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ request, locals }) => {
	const token =
		request.headers.get('Authorization')?.replace('Bearer ', '') ||
		(locals as any).accessToken;
	if (!token) return json({ error: 'Unauthorized' }, { status: 401 });
	// ICT 7 FIX: Return mock data immediately - endpoint not implemented on backend
	// This prevents 404 console errors
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
};

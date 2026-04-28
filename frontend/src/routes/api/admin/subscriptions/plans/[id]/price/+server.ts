/**
 * Admin Subscription Plan Price-Change API Proxy
 *
 * Wires POST /api/admin/subscriptions/plans/:id/price through to
 * subscriptions_admin.rs::change_plan_price. Creates a new Stripe Price,
 * flips the DB pointer, and optionally migrates existing subscribers.
 *
 * Requires super_admin — immediate_proration triggers real-money movement.
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireSuperadmin } from '$lib/server/auth';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

async function relay(response: Response): Promise<Response> {
	const text = await response.text();
	if (!text) {
		return json(
			{ error: 'Empty response from backend' },
			{ status: response.status >= 400 ? response.status : 502 }
		);
	}
	try {
		const data = JSON.parse(text);
		return json(data, { status: response.status });
	} catch {
		return json({ error: 'Invalid JSON from backend' }, { status: 502 });
	}
}

export const POST: RequestHandler = async (event) => {
	const id = event.params.id;
	if (!id) return json({ error: 'Missing plan id' }, { status: 400 });

	const { token } = requireSuperadmin(event);

	try {
		const body = await event.request.json();
		const response = await fetch(
			`${API_URL}/api/admin/subscriptions/plans/${id}/price`,
			{
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					Accept: 'application/json',
					Authorization: `Bearer ${token}`
				},
				body: JSON.stringify(body)
			}
		);
		return relay(response);
	} catch (err) {
		console.error('[API Proxy] plan price change error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

/**
 * Admin Plan Price-History API Proxy (read-only)
 *
 * FIX-2026-04-26 (audit 02 §P0-4): Subscription Plans page calls
 *   GET /api/admin/subscriptions/plans/:id/price-history
 * to render the price-change audit log inside the price modal.
 *
 * Backend: `subscriptions_admin.rs::list_plan_price_history`
 *   (`/admin/subscriptions/plans/:id/price-history`, line 1109).
 *
 * Read-only and Stripe-clean — safe to wire through. The price *change*
 * itself is the deferred Stripe-touching op in `./price/+server.ts`.
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireAdminToken } from '$lib/server/auth';

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

export const GET: RequestHandler = async (event) => {
	const id = event.params.id;
	if (!id) return json({ error: 'Missing plan id' }, { status: 400 });
	const token = requireAdminToken(event);
	try {
		const response = await fetch(
			`${API_URL}/api/admin/subscriptions/plans/${id}/price-history`,
			{
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
					Accept: 'application/json',
					Authorization: `Bearer ${token}`
				}
			}
		);
		return relay(response);
	} catch (err) {
		console.error('[API Proxy] plan price-history error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

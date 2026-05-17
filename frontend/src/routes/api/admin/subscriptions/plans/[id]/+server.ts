/**
 * Admin Subscription Plan Detail API Proxy
 * FIX-2026-04-26 (P0-4): Forwards GET / PUT / DELETE on a single plan to
 * `subscriptions_admin.rs::plans_router()` (`/admin/subscriptions/plans/:id`).
 * The Subscription Plans page calls `PUT` for both edits and toggles.
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireAdmin } from '$lib/server/auth';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

async function relayResponse(response: Response): Promise<Response> {
	const text = await response.text();
	if (!text) {
		return json({ error: 'Empty response from server' }, { status: response.status || 500 });
	}
	try {
		const data = JSON.parse(text);
		return json(data, { status: response.status });
	} catch {
		return json({ error: 'Invalid response from server' }, { status: 500 });
	}
}

export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const id = event.params.id;
	if (!id) return json({ error: 'Missing plan id' }, { status: 400 });
	const auth = `Bearer ${token}`;
	try {
		const response = await fetch(`${API_URL}/api/admin/subscriptions/plans/${id}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: auth
			}
		});
		return relayResponse(response);
	} catch (err) {
		console.error('[API Proxy] Get subscription plan error:', err);
		return json({ error: 'Failed to get subscription plan' }, { status: 500 });
	}
};

export const PUT: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const id = event.params.id;
	if (!id) return json({ error: 'Missing plan id' }, { status: 400 });
	const auth = `Bearer ${token}`;
	try {
		const body = await event.request.json();
		const response = await fetch(`${API_URL}/api/admin/subscriptions/plans/${id}`, {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: auth
			},
			body: JSON.stringify(body)
		});
		return relayResponse(response);
	} catch (err) {
		console.error('[API Proxy] Update subscription plan error:', err);
		return json({ error: 'Failed to update subscription plan' }, { status: 500 });
	}
};

export const DELETE: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const id = event.params.id;
	if (!id) return json({ error: 'Missing plan id' }, { status: 400 });
	const auth = `Bearer ${token}`;
	try {
		const response = await fetch(`${API_URL}/api/admin/subscriptions/plans/${id}`, {
			method: 'DELETE',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: auth
			}
		});
		return relayResponse(response);
	} catch (err) {
		console.error('[API Proxy] Delete subscription plan error:', err);
		return json({ error: 'Failed to delete subscription plan' }, { status: 500 });
	}
};

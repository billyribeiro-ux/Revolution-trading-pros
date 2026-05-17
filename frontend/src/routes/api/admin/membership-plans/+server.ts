/**
 * Admin Membership Plans API Proxy
 * Forwards requests to backend /api/admin/membership-plans (GET — list)
 * and /api/admin/subscriptions/plans (POST — create) endpoints.
 *
 * FIX-2026-04-26 (P0-3): Added POST handler. The page also expects
 * PUT/PATCH/DELETE — those live in `[id]/+server.ts` since they take a path
 * param. The Rust CRUD lives at /admin/subscriptions/plans (see
 * subscriptions_admin.rs::plans_router); the GET list is also exposed at
 * /admin/membership-plans (admin.rs::list_all_plans), so the GET path stays
 * as-is for backward compatibility.
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireAdmin } from '$lib/server/auth';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const auth = `Bearer ${token}`;
	try {
		const response = await fetch(`${API_URL}/api/admin/membership-plans`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Authorization: auth
			}
		});

		if (!response.ok) {
			let errorData;
			try {
				errorData = await response.json();
			} catch {
				errorData = { error: response.statusText || 'Request failed' };
			}
			return json(errorData, { status: response.status });
		}

		const data = await response.json();
		return json(data);
	} catch (err) {
		console.error('[API Proxy] Failed to fetch membership plans:', err);
		return json({ error: 'Failed to fetch membership plans' }, { status: 500 });
	}
};

/**
 * POST /api/admin/membership-plans
 * Forwards to backend /api/admin/subscriptions/plans (the Rust CRUD root for
 * plans — see subscriptions_admin.rs::create_plan).
 */
export const POST: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const auth = `Bearer ${token}`;
	try {
		const body = await event.request.json();

		const response = await fetch(`${API_URL}/api/admin/subscriptions/plans`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: auth
			},
			body: JSON.stringify(body)
		});

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
	} catch (err) {
		console.error('[API Proxy] Failed to create membership plan:', err);
		return json({ error: 'Failed to create membership plan' }, { status: 500 });
	}
};

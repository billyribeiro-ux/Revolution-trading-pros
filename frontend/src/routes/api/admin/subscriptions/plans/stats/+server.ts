/**
 * Subscriptions Plans Stats API Proxy
 * Routes requests to Rust API backend to avoid CORS issues.
 *
 * R22-A: Deleted the 401/404/500 → mock-zeros fallback. The previous
 *   behaviour masked an expired admin token (401) as "0 active subscriptions"
 *   and reported a real backend 500 as the same dashboard-rendering zeros —
 *   admins could not distinguish "the system says zero" from "the system is
 *   broken." Now every non-2xx is forwarded to the client with the upstream
 *   status; network failure surfaces as 500.
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { env } from '$env/dynamic/private';
import { requireAdmin } from '$lib/server/auth';

// CLAUDE.md hard rule — API_BASE_URL primary, BACKEND_URL fallback, localhost last.
const BACKEND_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { fetch } = event;
	try {
		const response = await fetch(`${BACKEND_URL}/api/admin/subscriptions/plans/stats`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});

		if (!response.ok) {
			// R22-A: was: 401/404/500 → mock-zeros; everything else → forward.
			// Now: always forward the upstream status so the admin UI can
			// distinguish "no plans" (real 200 with empty array) from "outage."
			const error = await response.json().catch(() => ({ message: 'Request failed' }));
			return json(error, { status: response.status });
		}

		const data = await response.json();
		return json(data);
	} catch (error) {
		// R22-A: was: silent mock-zeros fallback. Now: 500 so an outage is visible.
		console.error('[Subscriptions stats proxy] Backend fetch failed:', error);
		return json(
			{ success: false, error: 'Subscriptions stats backend unreachable.' },
			{ status: 500 }
		);
	}
};

/**
 * Admin Member Analytics — segments breakdown
 * FIX-2026-04-26 (audit 02 §P1-2). See `metrics/+server.ts` for rationale.
 *
 * NOTE: this is the analytics summary at
 *   `/admin/members/analytics/segments`
 * (admin_members.rs:1304). The CRUD list at `/admin/members/segments`
 * (admin_members.rs:1271) lives at
 *   `frontend/src/routes/api/admin/members/segments/+server.ts`.
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireAdminToken } from '$lib/server/auth';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

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
	const token = requireAdminToken(event);
	try {
		const qs = event.url.search ?? '';
		const response = await fetch(`${API_URL}/api/admin/members/analytics/segments${qs}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});
		return relay(response);
	} catch (err) {
		console.error('[API Proxy] members/analytics/segments error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

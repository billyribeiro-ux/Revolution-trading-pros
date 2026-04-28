/**
 * Admin Member Analytics — metrics
 *
 * FIX-2026-04-26 (audit 02 §P1-2): Analytics page calls
 *   GET /api/admin/members/analytics/metrics?range=...
 * The backend route exists at `admin_members.rs:1299` but the proxy was
 * missing. Without this file the request 404'd and the page rendered the
 * "analytics service not connected" empty state forever.
 *
 * Backend failure surfaces as 502 — never silently empty-array.
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
	const token = requireAdminToken(event);
	try {
		const qs = event.url.search ?? '';
		const response = await fetch(`${API_URL}/api/admin/members/analytics/metrics${qs}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});
		return relay(response);
	} catch (err) {
		console.error('[API Proxy] members/analytics/metrics error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

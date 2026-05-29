/**
 * Admin Past-Members Dashboard Proxy (catch-all)
 *
 * FIX-2026-04-26 (audit 02 §P1-3): The Past Members page (1994 lines) and its
 * `lib/api/past-members-dashboard.ts` client both target a
 * `/api/admin/past-members-dashboard/*` namespace that had no proxy and (per
 * audit) likely no backend either. The page would render the
 * "session expired" branch in production because every load `Promise.all`
 * settled to a 404.
 *
 * This catch-all forwards every sub-path to the same path on the backend
 * with the canonical proxy pattern. If the backend route is missing it
 * returns the upstream 404 verbatim so the page can render a real
 * "endpoint not implemented" state — never a fake success.
 *
 * Backend wiring is tracked in `02-members-subscriptions-DEFERRED.md` §D4.
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireAdminToken } from '$lib/server/auth';

const API_URL = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

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

function backendUrl(event: Parameters<RequestHandler>[0]): string {
	const path = event.params.path ?? '';
	const qs = event.url.search ?? '';
	return `${API_URL}/api/admin/past-members-dashboard/${path}${qs}`;
}

export const GET: RequestHandler = async (event) => {
	const token = requireAdminToken(event);
	try {
		const response = await fetch(backendUrl(event), {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});
		return relay(response);
	} catch (err) {
		console.error('[API Proxy] past-members-dashboard GET error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

export const POST: RequestHandler = async (event) => {
	const token = requireAdminToken(event);
	try {
		const body = await event.request.text();
		const response = await fetch(backendUrl(event), {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			},
			body: body || undefined
		});
		return relay(response);
	} catch (err) {
		console.error('[API Proxy] past-members-dashboard POST error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

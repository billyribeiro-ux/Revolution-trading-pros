/**
 * Admin Membership Plan Detail API Proxy
 * FIX-2026-04-26 (P0-3): The frontend Memberships page calls
 * PUT/PATCH/DELETE on `/api/admin/membership-plans/:id`. The Rust CRUD for
 * plans actually lives under `/admin/subscriptions/plans/:id`, so this proxy
 * bridges the path mismatch documented in the audit.
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

function authHeaderFrom({
	cookies,
	request
}: {
	cookies: { get: (name: string) => string | undefined };
	request: Request;
}): string | null {
	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	return token ? `Bearer ${token}` : null;
}

async function forward(
	method: 'PUT' | 'PATCH' | 'DELETE' | 'GET',
	id: string,
	auth: string,
	body?: unknown
): Promise<Response> {
	const init: RequestInit = {
		method,
		headers: {
			'Content-Type': 'application/json',
			Accept: 'application/json',
			Authorization: auth
		}
	};
	if (body !== undefined) {
		init.body = JSON.stringify(body);
	}
	return fetch(`${API_URL}/api/admin/subscriptions/plans/${id}`, init);
}

async function handleResponse(response: Response): Promise<Response> {
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

export const GET: RequestHandler = async ({ params, request, cookies }) => {
	const id = params.id;
	if (!id) return json({ error: 'Missing plan id' }, { status: 400 });
	const auth = authHeaderFrom({ cookies, request });
	if (!auth) return json({ error: 'Missing or invalid authorization header' }, { status: 401 });
	try {
		return handleResponse(await forward('GET', id, auth));
	} catch (err) {
		console.error('[API Proxy] Get membership plan error:', err);
		return json({ error: 'Failed to get membership plan' }, { status: 500 });
	}
};

export const PUT: RequestHandler = async ({ params, request, cookies }) => {
	const id = params.id;
	if (!id) return json({ error: 'Missing plan id' }, { status: 400 });
	const auth = authHeaderFrom({ cookies, request });
	if (!auth) return json({ error: 'Missing or invalid authorization header' }, { status: 401 });
	try {
		const body = await request.json();
		return handleResponse(await forward('PUT', id, auth, body));
	} catch (err) {
		console.error('[API Proxy] Update membership plan error:', err);
		return json({ error: 'Failed to update membership plan' }, { status: 500 });
	}
};

/**
 * PATCH is forwarded as PUT to the backend (Rust route only exposes PUT for
 * partial updates today — see subscriptions_admin.rs::update_plan, which
 * treats every field as Option<T>).
 */
export const PATCH: RequestHandler = async ({ params, request, cookies }) => {
	const id = params.id;
	if (!id) return json({ error: 'Missing plan id' }, { status: 400 });
	const auth = authHeaderFrom({ cookies, request });
	if (!auth) return json({ error: 'Missing or invalid authorization header' }, { status: 401 });
	try {
		const body = await request.json();
		return handleResponse(await forward('PUT', id, auth, body));
	} catch (err) {
		console.error('[API Proxy] Patch membership plan error:', err);
		return json({ error: 'Failed to patch membership plan' }, { status: 500 });
	}
};

export const DELETE: RequestHandler = async ({ params, request, cookies }) => {
	const id = params.id;
	if (!id) return json({ error: 'Missing plan id' }, { status: 400 });
	const auth = authHeaderFrom({ cookies, request });
	if (!auth) return json({ error: 'Missing or invalid authorization header' }, { status: 401 });
	try {
		return handleResponse(await forward('DELETE', id, auth));
	} catch (err) {
		console.error('[API Proxy] Delete membership plan error:', err);
		return json({ error: 'Failed to delete membership plan' }, { status: 500 });
	}
};

/**
 * Admin User-Membership Detail API Proxy
 *
 * FIX-2026-04-26 (audit 02 §P0-5): Member detail page calls
 *   PUT  /api/admin/user-memberships/:id    (extend)
 *   DELETE /api/admin/user-memberships/:id  (revoke)
 *
 * Backend: `admin.rs::update_user_membership` / `revoke_membership`
 *   mounted at `/api/admin/user-memberships/:id` (admin.rs:1789-1794).
 *
 * Both PUT and DELETE are super-admin per the audit (privilege-bearing /
 * destructive). On backend failure we propagate the upstream status as 502
 * rather than fake success — admins must know if a revoke didn't actually
 * land.
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireSuperadmin } from '$lib/server/auth';

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

export const PUT: RequestHandler = async (event) => {
	const id = event.params.id;
	if (!id) return json({ error: 'Missing membership id' }, { status: 400 });
	const { token } = requireSuperadmin(event);
	try {
		const body = await event.request.json();
		const response = await fetch(`${API_URL}/api/admin/user-memberships/${id}`, {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			},
			body: JSON.stringify(body)
		});
		return relay(response);
	} catch (err) {
		console.error('[API Proxy] Update user-membership error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

export const DELETE: RequestHandler = async (event) => {
	const id = event.params.id;
	if (!id) return json({ error: 'Missing membership id' }, { status: 400 });
	const { token } = requireSuperadmin(event);
	try {
		const response = await fetch(`${API_URL}/api/admin/user-memberships/${id}`, {
			method: 'DELETE',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});
		return relay(response);
	} catch (err) {
		console.error('[API Proxy] Revoke user-membership error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

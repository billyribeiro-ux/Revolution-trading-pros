/**
 * Admin User-Memberships API Proxy (root)
 *
 * FIX-2026-04-26 (audit 02 §P0-5): The Member detail page calls
 * `POST /api/admin/user-memberships` to grant a membership and
 * `GET /api/admin/user-memberships` to list them. There was no proxy file at
 * all, so every Grant click was returning a 404 swallowed by the page's catch
 * block as a generic "Failed to grant membership" toast.
 *
 * Backend: `admin.rs::grant_membership` / `list_user_memberships`
 *   mounted at `/api/admin/user-memberships` (admin.rs:1786).
 *
 * If the backend route is missing or returns 5xx, we propagate the status as
 * 502 (Bad Gateway) — never fake success and never silently empty-array.
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireAdminToken, requireSuperadmin } from '$lib/server/auth';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

async function relay(response: Response): Promise<Response> {
	const text = await response.text();
	if (!text) {
		// Treat truly empty body as a backend hand-off failure rather than a 200.
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
		const response = await fetch(`${API_URL}/api/admin/user-memberships${qs}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});
		return relay(response);
	} catch (err) {
		console.error('[API Proxy] List user-memberships error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

/**
 * Granting a membership writes user_memberships and (per audit P1-10) is a
 * privilege-bearing action — gate it behind super-admin in the proxy as a
 * defense-in-depth layer over the (broader) backend role check.
 */
export const POST: RequestHandler = async (event) => {
	const { token } = requireSuperadmin(event);
	try {
		const body = await event.request.json();
		const response = await fetch(`${API_URL}/api/admin/user-memberships`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			},
			body: JSON.stringify(body)
		});
		return relay(response);
	} catch (err) {
		console.error('[API Proxy] Grant membership error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

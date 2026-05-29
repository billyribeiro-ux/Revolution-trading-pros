/**
 * Admin Member Filters API Proxy (root)
 * FIX-2026-04-26 (audit 02 §P1-4). Backend at
 *   `admin_members.rs::list_member_filters / create_member_filter`
 * mounted at `/admin/members/filters`.
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

export const GET: RequestHandler = async (event) => {
	const token = requireAdminToken(event);
	try {
		const qs = event.url.search ?? '';
		const response = await fetch(`${API_URL}/api/admin/members/filters${qs}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});
		return relay(response);
	} catch (err) {
		console.error('[API Proxy] members/filters list error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

export const POST: RequestHandler = async (event) => {
	const token = requireAdminToken(event);
	try {
		const body = await event.request.json();
		const response = await fetch(`${API_URL}/api/admin/members/filters`, {
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
		console.error('[API Proxy] members/filters create error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

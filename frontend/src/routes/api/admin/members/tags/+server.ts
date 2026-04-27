/**
 * Admin Member Tags API Proxy (root)
 * FIX-2026-04-26 (audit 02 §P1-4). Backend at
 *   `admin_members.rs::list_member_tags / create_member_tag`
 * mounted at `/admin/members/tags`.
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
		const response = await fetch(`${API_URL}/api/admin/members/tags${qs}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});
		return relay(response);
	} catch (err) {
		console.error('[API Proxy] members/tags list error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

export const POST: RequestHandler = async (event) => {
	const token = requireAdminToken(event);
	try {
		const body = await event.request.json();
		const response = await fetch(`${API_URL}/api/admin/members/tags`, {
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
		console.error('[API Proxy] members/tags create error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

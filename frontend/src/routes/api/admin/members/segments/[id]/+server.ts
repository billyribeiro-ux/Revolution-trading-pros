/**
 * Admin Member Segment Detail API Proxy
 * FIX-2026-04-26 (audit 02 §P1-4 / §P2-4). Backend at
 *   `admin_members.rs::get_segment / update_segment / delete_segment`
 * mounted at `/admin/members/segments/:id`.
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
	const id = event.params.id;
	if (!id) return json({ error: 'Missing segment id' }, { status: 400 });
	const token = requireAdminToken(event);
	try {
		const response = await fetch(`${API_URL}/api/admin/members/segments/${id}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});
		return relay(response);
	} catch (err) {
		console.error('[API Proxy] members/segments get error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

export const PUT: RequestHandler = async (event) => {
	const id = event.params.id;
	if (!id) return json({ error: 'Missing segment id' }, { status: 400 });
	const token = requireAdminToken(event);
	try {
		const body = await event.request.json();
		const response = await fetch(`${API_URL}/api/admin/members/segments/${id}`, {
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
		console.error('[API Proxy] members/segments update error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

export const DELETE: RequestHandler = async (event) => {
	const id = event.params.id;
	if (!id) return json({ error: 'Missing segment id' }, { status: 400 });
	const token = requireAdminToken(event);
	try {
		const response = await fetch(`${API_URL}/api/admin/members/segments/${id}`, {
			method: 'DELETE',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});
		return relay(response);
	} catch (err) {
		console.error('[API Proxy] members/segments delete error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

/**
 * Admin Member Notes API Proxy
 *
 * FIX-2026-04-26 (audit 02 §P1-1): Member detail page calls
 *   GET  /api/admin/members/:id/notes  → list notes
 *   POST /api/admin/members/:id/notes  → create note
 *
 * Backend: `admin_members.rs::get_member_notes` / `create_member_note`
 *   mounted at `/admin/members/:id/notes` (admin_members.rs:1306).
 *
 * Without this proxy, the page swallowed every 404 as an empty notes array
 * and (worse) the POST fallback inserted a fake `id: Date.now()` note that
 * vanished on refresh. Now: real backend round-trip; 502 on backend
 * unreachable.
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
	const id = event.params.id;
	if (!id) return json({ error: 'Missing member id' }, { status: 400 });
	const token = requireAdminToken(event);
	try {
		const response = await fetch(`${API_URL}/api/admin/members/${id}/notes`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});
		return relay(response);
	} catch (err) {
		console.error('[API Proxy] Get member notes error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

export const POST: RequestHandler = async (event) => {
	const id = event.params.id;
	if (!id) return json({ error: 'Missing member id' }, { status: 400 });
	const token = requireAdminToken(event);
	try {
		const body = await event.request.json();
		const response = await fetch(`${API_URL}/api/admin/members/${id}/notes`, {
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
		console.error('[API Proxy] Create member note error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

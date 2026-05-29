/**
 * Admin Member Segments API Proxy (root)
 *
 * FIX-2026-04-26 (audit 02 §P1-4): Segments page calls
 *   GET  /api/admin/members/segments
 *   POST /api/admin/members/segments
 * The page used to fall back to hardcoded data with `Math.random()` member
 * counts when these 404'd. Backend lives at
 *   `admin_members.rs::list_segments` / `create_segment`
 * mounted at `/admin/members/segments`.
 *
 * On backend unreachable we propagate 502 — never silently empty-array.
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
		const response = await fetch(`${API_URL}/api/admin/members/segments${qs}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});
		return relay(response);
	} catch (err) {
		console.error('[API Proxy] members/segments list error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

export const POST: RequestHandler = async (event) => {
	const token = requireAdminToken(event);
	try {
		const body = await event.request.json();
		const response = await fetch(`${API_URL}/api/admin/members/segments`, {
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
		console.error('[API Proxy] members/segments create error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

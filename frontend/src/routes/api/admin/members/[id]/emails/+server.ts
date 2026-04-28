/**
 * Admin Member Email-History API Proxy
 *
 * FIX-2026-04-26 (audit 02 §P1-1): Member detail page calls
 *   GET /api/admin/members/:id/emails
 *
 * Backend: `admin_members.rs::get_member_emails` mounted at
 *   `/admin/members/:id/emails` (admin_members.rs:1307).
 *
 * Without this proxy the page swallowed every 404 as "no email history" —
 * masking the missing wiring entirely. We now propagate real status; on
 * backend unreachable we surface 502 instead of faking an empty list.
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
	if (!id) return json({ error: 'Missing member id' }, { status: 400 });
	const token = requireAdminToken(event);
	try {
		const response = await fetch(`${API_URL}/api/admin/members/${id}/emails`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});
		return relay(response);
	} catch (err) {
		console.error('[API Proxy] Get member emails error:', err);
		return json({ error: 'Backend unreachable' }, { status: 502 });
	}
};

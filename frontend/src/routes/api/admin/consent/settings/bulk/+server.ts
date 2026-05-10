/**
 * Admin Consent Settings — Bulk Update Proxy.
 *
 * FIX-2026-04-26: created to satisfy `POST /api/admin/consent/settings/bulk`
 * for the `/admin/consent/settings` page (docs/audits/ADMIN_FAILURE_DATA.md §9.5).
 *
 * Lives in its own folder rather than under a `[...rest]` shim because the
 * page hits a single, fixed sub-path. Forwards body unchanged with the
 * canonical `rtp_access_token` Bearer.
 */
import { error, json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

export const POST: RequestHandler = async ({ cookies, fetch, request }) => {
	const token = cookies.get('rtp_access_token');
	if (!token) error(401, 'Unauthorized');

	const body = await request.text();

	const upstream = await fetch(`${API_URL}/api/admin/consent/settings/bulk`, {
		method: 'POST',
		headers: {
			Authorization: `Bearer ${token}`,
			'Content-Type': request.headers.get('content-type') ?? 'application/json',
			Accept: 'application/json'
		},
		body
	});

	const text = await upstream.text();
	let parsed: unknown;
	try {
		parsed = text ? JSON.parse(text) : {};
	} catch {
		parsed = { success: false, error: 'Invalid JSON from backend' };
	}

	return json(parsed, { status: upstream.status });
};

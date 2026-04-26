/**
 * Admin Consent Settings — Reset-to-Defaults Proxy.
 *
 * FIX-2026-04-26: created to satisfy `POST /api/admin/consent/settings/reset`
 * for the `/admin/consent/settings` page (ADMIN_FAILURE_DATA.md §9.5).
 *
 * The page sends an empty POST; we still forward whatever body is present so
 * future enhancements (e.g. partial reset) work without a proxy change.
 */
import { error, json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

export const POST: RequestHandler = async ({ cookies, fetch, request }) => {
	const token = cookies.get('rtp_access_token');
	if (!token) error(401, 'Unauthorized');

	const body = await request.text();

	const upstream = await fetch(`${API_URL}/api/admin/consent/settings/reset`, {
		method: 'POST',
		headers: {
			Authorization: `Bearer ${token}`,
			'Content-Type': request.headers.get('content-type') ?? 'application/json',
			Accept: 'application/json'
		},
		body: body || undefined
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

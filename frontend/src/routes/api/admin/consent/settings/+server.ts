/**
 * Admin Consent Settings Proxy — GET only.
 *
 * FIX-2026-04-26: created to satisfy the `/admin/consent/settings` page,
 * which previously 404'd on every load (see ADMIN_FAILURE_DATA.md §9.5).
 *
 * Forwards `GET /api/admin/consent/settings` to the Rust backend, attaching
 * the canonical `rtp_access_token` cookie as a Bearer token.
 *
 * The matching `bulk` and `reset` POSTs live in sibling `+server.ts` files
 * (`./bulk/+server.ts`, `./reset/+server.ts`) to dodge the SvelteKit
 * POST-405 cliff documented in `frontend/src/lib/utils/createProxyShim.ts`.
 */
import { error, json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

export const GET: RequestHandler = async ({ cookies, fetch, url }) => {
	const token = cookies.get('rtp_access_token');
	if (!token) error(401, 'Unauthorized');

	const upstream = await fetch(`${API_URL}/api/admin/consent/settings${url.search}`, {
		method: 'GET',
		headers: {
			Authorization: `Bearer ${token}`,
			Accept: 'application/json'
		}
	});

	const text = await upstream.text();
	let body: unknown = {};
	try {
		body = text ? JSON.parse(text) : {};
	} catch {
		body = { success: false, error: 'Invalid JSON from backend' };
	}

	return json(body, { status: upstream.status });
};

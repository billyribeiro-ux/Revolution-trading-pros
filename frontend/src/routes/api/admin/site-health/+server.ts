/**
 * Site Health Endpoint — Backend Proxy
 *
 * PRINCIPAL-2026-04-26 (audit 09-system §P1-5):
 *   - Built per CREATE-not-DELETE rule: previously the site-health page
 *     called `/api/admin/site-health` with no proxy and silently fell back
 *     to zeros, masking the missing endpoint.
 *   - Admin-gated read-only proxy.
 *
 * Backend route: `GET /api/admin/site-health` — if the Rust API does not
 * implement this yet, callers receive the upstream status verbatim (404 etc),
 * not a fake success.
 */

import { env } from '$env/dynamic/private';
import type { RequestHandler } from './$types';
import { requireAdmin } from '$lib/server/auth';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);

	const upstream = await fetch(`${API_URL}/api/admin/site-health`, {
		headers: {
			Authorization: `Bearer ${token}`,
			Accept: 'application/json'
		}
	});

	const text = await upstream.text();
	const headers: Record<string, string> = {};
	const ct = upstream.headers.get('content-type');
	if (ct) headers['Content-Type'] = ct;
	return new Response(text, { status: upstream.status, headers });
};

/**
 * Admin Orders API Proxy — top-level GET (list + stats)
 *
 * FIX-2026-04-26: backend `/api/admin/orders` was unmounted prior to this
 * fix. Now wired to `routes::admin_orders::router()` (Rust). This proxy
 * forwards GET to the canonical handler.
 *
 * Sub-paths (`[id]`, `[id]/status`, `[id]/refund`, `export`, `stats`, …)
 * are handled by the sibling `[...rest]/+server.ts` shim to avoid the
 * SvelteKit POST=405-cliff documented in `docs/audits/...` §3a.
 *
 * Auth: canonical `rtp_access_token` cookie → `Bearer` header.
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireAdmin } from '$lib/server/auth';

const API_URL = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { url, fetch } = event;

	const queryString = url.searchParams.toString();
	const target = `${API_URL}/api/admin/orders${queryString ? `?${queryString}` : ''}`;

	const upstream = await fetch(target, {
		method: 'GET',
		headers: {
			Authorization: `Bearer ${token}`,
			Accept: 'application/json'
		}
	});

	if (!upstream.ok) {
		// Forward backend error verbatim where useful; provide an empty-shape
		// fallback for 5xx so the page still renders without throwing.
		if (upstream.status >= 500) {
			return json(
				{
					data: [],
					stats: null,
					pagination: { page: 1, per_page: 25, total: 0, total_pages: 0 },
					error: 'Backend unavailable'
				},
				{ status: upstream.status }
			);
		}
	}

	const body = await upstream.text();
	return new Response(body, {
		status: upstream.status,
		headers: {
			'Content-Type': upstream.headers.get('content-type') ?? 'application/json'
		}
	});
};

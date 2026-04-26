/**
 * Admin Orders — individual order proxy.
 *
 * FIX-2026-04-26: handles GET / PUT / DELETE on `/api/admin/orders/:id`.
 *
 * NOTE: SvelteKit will route exact `/admin/orders/:id` requests through
 * this file, while requests to deeper sub-paths (`:id/status`, `:id/refund`,
 * etc.) are picked up by the sibling `[...rest]/+server.ts` shim. Both
 * forward to the same Rust backend; both honor the `rtp_access_token`
 * cookie.
 *
 * The Rust backend currently exposes:
 *   GET  /admin/orders/:id  → order detail
 *   POST /admin/orders/:id/* → status / refund / cancel / fulfill / resend
 *
 * PUT and DELETE are forwarded as-is so the frontend can call them when
 * the backend grows that surface; the upstream will return 405 today.
 */

import { error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

function buildHandler(method: 'GET' | 'PUT' | 'DELETE'): RequestHandler {
	return async ({ params, request, cookies, fetch, url }) => {
		const token = cookies.get('rtp_access_token');
		if (!token) error(401, 'Unauthorized');

		const id = params.id;
		const target = `${API_URL}/api/admin/orders/${id}${url.search}`;

		const headers: Record<string, string> = {
			Authorization: `Bearer ${token}`
		};
		const ct = request.headers.get('content-type');
		if (ct) headers['Content-Type'] = ct;
		const accept = request.headers.get('accept');
		if (accept) headers['Accept'] = accept;

		const body = method === 'GET' ? undefined : await request.text();

		const upstream = await fetch(target, { method, headers, body });

		return new Response(upstream.body, {
			status: upstream.status,
			headers: upstream.headers
		});
	};
}

export const GET = buildHandler('GET');
export const PUT = buildHandler('PUT');
export const DELETE = buildHandler('DELETE');

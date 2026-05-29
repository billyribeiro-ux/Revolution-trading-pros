/**
 * Admin CMS Datasources — catch-all proxy
 * ═══════════════════════════════════════════════════════════════════════════════════
 *
 * FIX-2026-04-26 (P1-7): The CMS datasources page was the only admin surface that
 * bypassed the SvelteKit proxy and hit the Rust backend directly with a token from
 * `$lib/stores/auth.svelte`. That coupled the surface to a CORS contract no other
 * admin surface needs, made token-rotation drift, and broke 401-redirect parity.
 *
 * This proxy is a thin pass-through to `/api/admin/cms/datasources/...` on the
 * backend, attaching the canonical `rtp_access_token` cookie as a Bearer token —
 * matching every other proxy in this audit.
 *
 * Supports JSON GET/POST/PUT/PATCH/DELETE, multipart POST (CSV import) and binary
 * GET (CSV export).
 *
 * @version 1.0.0
 */

import type { RequestHandler } from '@sveltejs/kit';

import { env } from '$env/dynamic/private';
import { requireAdmin } from '$lib/server/auth';
const PROD_BACKEND = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

function buildUpstreamUrl(pathSegments: string, search: string): string {
	const cleanPath = pathSegments ? `/${pathSegments}` : '';
	return `${PROD_BACKEND}/api/admin/cms/datasources${cleanPath}${search}`;
}

async function proxy(
	event: Parameters<RequestHandler>[0],
	method: string,
	token: string
): Promise<Response> {
	const url = buildUpstreamUrl(event.params.path ?? '', event.url.search);

	const upstreamHeaders: Record<string, string> = {
		Authorization: `Bearer ${token}`,
		Accept: event.request.headers.get('Accept') ?? 'application/json'
	};
	const contentType = event.request.headers.get('Content-Type');
	if (contentType) upstreamHeaders['Content-Type'] = contentType;

	let body: BodyInit | undefined;
	if (method !== 'GET' && method !== 'HEAD' && method !== 'DELETE') {
		// Forward raw body so multipart/form-data and JSON are both handled correctly.
		body = await event.request.arrayBuffer();
	}

	let response: Response;
	try {
		response = await fetch(url, {
			method,
			headers: upstreamHeaders,
			body
		});
	} catch (err) {
		console.warn(`CMS datasources backend unreachable for ${method} ${url}:`, err);
		return new Response(
			JSON.stringify({
				success: false,
				error: 'CMS datasources backend is not reachable.',
				_degraded: true
			}),
			{ status: 503, headers: { 'Content-Type': 'application/json' } }
		);
	}

	// Pass through status, body, and Content-Type / Content-Disposition (CSV export
	// relies on the latter for the download filename).
	const passHeaders: Record<string, string> = {};
	const ct = response.headers.get('Content-Type');
	if (ct) passHeaders['Content-Type'] = ct;
	const cd = response.headers.get('Content-Disposition');
	if (cd) passHeaders['Content-Disposition'] = cd;

	const buf = await response.arrayBuffer();
	return new Response(buf, { status: response.status, headers: passHeaders });
}

export const GET: RequestHandler = (event) => {
	const { token } = requireAdmin(event);
	return proxy(event, 'GET', token);
};

export const POST: RequestHandler = (event) => {
	const { token } = requireAdmin(event);
	return proxy(event, 'POST', token);
};

export const PUT: RequestHandler = (event) => {
	const { token } = requireAdmin(event);
	return proxy(event, 'PUT', token);
};

export const PATCH: RequestHandler = (event) => {
	const { token } = requireAdmin(event);
	return proxy(event, 'PATCH', token);
};

export const DELETE: RequestHandler = (event) => {
	const { token } = requireAdmin(event);
	return proxy(event, 'DELETE', token);
};

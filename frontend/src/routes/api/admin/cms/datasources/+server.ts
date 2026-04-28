/**
 * Admin CMS Datasources — root proxy
 * ═══════════════════════════════════════════════════════════════════════════════════
 *
 * FIX-2026-04-26 (P1-7): Same-origin proxy for the bare `/api/admin/cms/datasources`
 * collection (list + create). Sub-paths are handled by `[...path]/+server.ts`.
 *
 * @version 1.0.0
 */

import { error as kitError } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { env } from '$env/dynamic/private';
const PROD_BACKEND =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

function tokenFrom(event: Parameters<RequestHandler>[0]): string {
	const cookieToken = event.cookies.get('rtp_access_token');
	const headerToken = event.request.headers
		.get('Authorization')
		?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) kitError(401, 'Unauthorized');
	return token;
}

async function proxy(
	event: Parameters<RequestHandler>[0],
	method: string
): Promise<Response> {
	const token = tokenFrom(event);
	const url = `${PROD_BACKEND}/api/admin/cms/datasources${event.url.search}`;

	const upstreamHeaders: Record<string, string> = {
		Authorization: `Bearer ${token}`,
		Accept: event.request.headers.get('Accept') ?? 'application/json'
	};
	const contentType = event.request.headers.get('Content-Type');
	if (contentType) upstreamHeaders['Content-Type'] = contentType;

	let body: BodyInit | undefined;
	if (method !== 'GET' && method !== 'HEAD') {
		body = await event.request.arrayBuffer();
	}

	let response: Response;
	try {
		response = await fetch(url, { method, headers: upstreamHeaders, body });
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

	const passHeaders: Record<string, string> = {};
	const ct = response.headers.get('Content-Type');
	if (ct) passHeaders['Content-Type'] = ct;
	const buf = await response.arrayBuffer();
	return new Response(buf, { status: response.status, headers: passHeaders });
}

export const GET: RequestHandler = (event) => proxy(event, 'GET');
export const POST: RequestHandler = (event) => proxy(event, 'POST');

/**
 * FIX-2026-04-26: shim to break the SvelteKit POST=405 cliff for
 * /api/admin/courses/<id>/{modules,lessons,downloads,upload-url,video-upload}/...
 *
 * The parent `[id]/+server.ts` claims sub-paths under `courses/<id>/` for
 * routing purposes, blocking POST/PUT/PATCH/DELETE on every nested path.
 * (Same SvelteKit dev-resolver behaviour documented in
 * `frontend/src/lib/utils/createProxyShim.ts`.)
 *
 * `createProxyShim` is base-path-static, but here we need to interpolate the
 * dynamic `[id]` segment, so we mirror its internals inline.
 *
 * Caller sites (see `frontend/src/lib/api/courses.ts:450-615`):
 *   - GET/POST    /api/admin/courses/<id>/modules
 *   - PUT/DELETE  /api/admin/courses/<id>/modules/<moduleId>
 *   - PUT         /api/admin/courses/<id>/modules/reorder
 *   - GET/POST    /api/admin/courses/<id>/lessons
 *   - GET/PUT/DELETE /api/admin/courses/<id>/lessons/<lessonId>
 *   - PUT         /api/admin/courses/<id>/lessons/reorder
 *   - GET/POST    /api/admin/courses/<id>/downloads
 *   - PUT/DELETE  /api/admin/courses/<id>/downloads/<downloadId>
 *   - POST        /api/admin/courses/<id>/upload-url
 *   - POST        /api/admin/courses/<id>/video-upload
 */

import { error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

const handler =
	(method: string): RequestHandler =>
	async ({ params, request, cookies, fetch, url }) => {
		const cookieToken = cookies.get('rtp_access_token');
		if (!cookieToken) error(401, 'Unauthorized');

		const { id } = params as { id: string; rest?: string };
		const rest = (params as { rest?: string }).rest ?? '';
		const targetPath = `/api/admin/courses/${id}${rest ? `/${rest}` : ''}${url.search}`;

		const headers: Record<string, string> = {
			Authorization: `Bearer ${cookieToken}`
		};
		const ct = request.headers.get('content-type');
		if (ct) headers['Content-Type'] = ct;
		const accept = request.headers.get('accept');
		if (accept) headers['Accept'] = accept;

		const body = method === 'GET' || method === 'HEAD' ? undefined : await request.text();

		const upstream = await fetch(`${API_URL}${targetPath}`, {
			method,
			headers,
			body
		});

		return new Response(upstream.body, {
			status: upstream.status,
			headers: upstream.headers
		});
	};

export const GET = handler('GET');
export const POST = handler('POST');
export const PUT = handler('PUT');
export const PATCH = handler('PATCH');
export const DELETE = handler('DELETE');

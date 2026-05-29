/**
 * Proxy shim helper for the SvelteKit POST=405 cliff.
 *
 * FIX-2026-04-26: SvelteKit's dev resolver returns 405 for POST/PUT/PATCH
 * to any sub-path under a directory that has its own `+server.ts`, never
 * reaching the catch-all `/api/[...path]/+server.ts`. Empirically reproducible
 * (see docs/audits/ADMIN_FAILURE_DATA.md §3a).
 *
 * Workaround: each affected folder gets a sibling `[...rest]/+server.ts`
 * that uses this helper to delegate to the backend.
 *
 * Affected folders (7 as of 2026-04-26):
 *   - /api/admin/posts
 *   - /api/admin/categories
 *   - /api/admin/tags
 *   - /api/admin/coupons
 *   - /api/admin/users
 *   - /api/admin/email/templates
 *   - /api/admin/crm/contacts
 *
 * @example
 *   // frontend/src/routes/api/admin/posts/[...rest]/+server.ts
 *   import { createProxyShim } from '$lib/utils/createProxyShim';
 *   export const { GET, POST, PUT, PATCH, DELETE } = createProxyShim('/api/admin/posts');
 */

import { error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const API_URL = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

/**
 * Create a `[...rest]/+server.ts` proxy shim that forwards every HTTP method
 * to the same path on the Rust backend, attaching the canonical Bearer token
 * from the `rtp_access_token` cookie.
 *
 * @param basePath Backend path prefix (must start with `/`) — e.g. `/api/admin/posts`.
 *                 The `params.rest` segment plus query string are appended.
 */
export function createProxyShim(basePath: string): {
	GET: RequestHandler;
	POST: RequestHandler;
	PUT: RequestHandler;
	PATCH: RequestHandler;
	DELETE: RequestHandler;
} {
	const handler =
		(method: string): RequestHandler =>
		async ({ params, request, cookies, fetch, url }) => {
			const token = cookies.get('rtp_access_token');
			if (!token) error(401, 'Unauthorized');

			const rest = (params as { rest?: string }).rest ?? '';
			const targetPath = `${basePath}${rest ? `/${rest}` : ''}${url.search}`;

			const headers: Record<string, string> = {
				Authorization: `Bearer ${token}`
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

			// Forward response. Strip hop-by-hop encoding headers: the SvelteKit
			// server-side fetch already decompresses the body, so re-sending
			// content-encoding/transfer-encoding causes ERR_CONTENT_DECODING_FAILED
			// in the browser (double-decompression).
			const responseHeaders = new Headers(upstream.headers);
			responseHeaders.delete('content-encoding');
			responseHeaders.delete('transfer-encoding');
			return new Response(upstream.body, {
				status: upstream.status,
				headers: responseHeaders
			});
		};

	return {
		GET: handler('GET'),
		POST: handler('POST'),
		PUT: handler('PUT'),
		PATCH: handler('PATCH'),
		DELETE: handler('DELETE')
	};
}

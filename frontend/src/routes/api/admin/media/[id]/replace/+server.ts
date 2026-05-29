/**
 * Media Replace Proxy
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * POST /api/admin/media/[id]/replace
 *   - Accepts multipart/form-data with `file`.
 *   - Forwards the body verbatim to the backend's media-replace endpoint with
 *     Bearer auth.
 *
 * FIX-2026-04-26-audit (P1-7): `media/+page.svelte` was POSTing directly to
 * `/api/media/{id}/replace` with no proxy in place — every call 404'd. Same
 * pattern as the upload proxy: stream the multipart body verbatim through
 * arrayBuffer() so binary chunks aren't mangled.
 */

import { error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireAdmin } from '$lib/server/auth';

const BACKEND_URL = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

export const POST: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { request } = event;
	const id = event.params.id;
	if (!id) error(400, 'Missing media id');

	const contentType = request.headers.get('content-type');
	if (!contentType || !contentType.includes('multipart/form-data')) {
		error(400, 'Expected multipart/form-data');
	}
	const bodyBuffer = await request.arrayBuffer();
	if (!bodyBuffer || bodyBuffer.byteLength === 0) {
		error(400, 'Empty request body');
	}

	try {
		const upstream = await fetch(`${BACKEND_URL}/api/admin/media/${id}/replace`, {
			method: 'POST',
			headers: {
				'Content-Type': contentType,
				Authorization: `Bearer ${token}`,
				'Content-Length': bodyBuffer.byteLength.toString()
			},
			body: bodyBuffer
		});
		const text = await upstream.text();
		const respHeaders: Record<string, string> = {};
		const ct = upstream.headers.get('content-type');
		if (ct) respHeaders['Content-Type'] = ct;
		return new Response(text, { status: upstream.status, headers: respHeaders });
	} catch (err) {
		console.error('[media/replace proxy] forward failed:', err);
		error(502, err instanceof Error ? err.message : 'Failed to reach backend');
	}
};

/**
 * Media Upload Proxy
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * POST /api/admin/media/upload
 *   - Accepts multipart/form-data with `file` (and optionally `collection`).
 *   - Forwards the body verbatim to the Rust API at the same path with the
 *     `rtp_access_token` cookie translated into a Bearer header.
 *
 * FIX P0-4 (audits/admin-2026-04-26/01-shell-and-dashboard.md):
 * `MediaUploadHub.svelte` was POSTing directly to `${API_BASE_URL}/api/admin
 * /media/upload` with `xhr.withCredentials = true`. The frontend's
 * `rtp_access_token` cookie is scoped to the frontend host, so cross-origin
 * XHR drops it; uploads silently 401 in production. Same-origin proxy fixes
 * the auth path and matches every other admin proxy in the repo.
 *
 * @version 1.0.0 — 2026-04-26 audit
 */

import { error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireAdmin } from '$lib/server/auth';

// CLAUDE.md house style: API_BASE_URL || BACKEND_URL || prod fly URL.
const BACKEND_URL = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

export const POST: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { request } = event;

	// Forward the multipart body verbatim. We must NOT re-parse + re-build the
	// FormData because Node would mangle binary file chunks. Stream the buffer.
	const contentType = request.headers.get('content-type');
	if (!contentType || !contentType.includes('multipart/form-data')) {
		error(400, 'Expected multipart/form-data');
	}

	const bodyBuffer = await request.arrayBuffer();
	if (!bodyBuffer || bodyBuffer.byteLength === 0) {
		error(400, 'Empty request body');
	}

	try {
		const upstream = await fetch(`${BACKEND_URL}/api/admin/media/upload`, {
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
		const upstreamCt = upstream.headers.get('content-type');
		if (upstreamCt) respHeaders['Content-Type'] = upstreamCt;

		return new Response(text, { status: upstream.status, headers: respHeaders });
	} catch (err) {
		console.error('[media/upload proxy] forward failed:', err);
		error(502, err instanceof Error ? err.message : 'Failed to reach backend');
	}
};

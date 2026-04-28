/**
 * Bunny Video Upload Proxy API
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * POST /api/admin/bunny/upload - Upload video file to Bunny.net via backend proxy
 *
 * The frontend cannot upload directly to Bunny.net because the API key
 * must remain on the server. This endpoint proxies the upload through
 * our backend which has the API key.
 *
 * @version 1.0.0 - ICT 7 Principal Engineer Grade
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { requireAdmin } from '$lib/server/auth';

// FIX-2026-04-26-audit: align with repo-wide proxy env-var chain.
const BACKEND_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';
// FIX-2026-04-26-audit (P3): make the default Bunny library id env-driven (was a magic 585929).
const DEFAULT_BUNNY_LIBRARY_ID = env.BUNNY_VIDEO_LIBRARY_ID || '585929';

// PUT - Upload video file to Bunny.net via backend
export const PUT: RequestHandler = async (event) => {
	const { request } = event;

	// FIX-2026-04-26-audit (P1-2): defense-in-depth admin gate.
	const { token: cookieToken } = requireAdmin(event);

	try {
		// Get the video GUID and library ID from URL params
		const url = new URL(request.url);
		const videoGuid = url.searchParams.get('video_guid');
		const libraryId = url.searchParams.get('library_id') || DEFAULT_BUNNY_LIBRARY_ID;

		if (!videoGuid) {
			error(400, 'video_guid is required');
		}

		// Get the file from the request body
		const contentType = request.headers.get('content-type') || 'video/mp4';
		const fileBuffer = await request.arrayBuffer();

		if (!fileBuffer || fileBuffer.byteLength === 0) {
			error(400, 'No file data provided');
		}

		// Forward to backend upload endpoint
		const uploadUrl = `${BACKEND_URL}/api/admin/bunny/upload?video_guid=${videoGuid}&library_id=${libraryId}`;

		const response = await fetch(uploadUrl, {
			method: 'PUT',
			headers: {
				'Content-Type': contentType,
				Authorization: `Bearer ${cookieToken}`,
				'Content-Length': fileBuffer.byteLength.toString()
			},
			body: fileBuffer
		});

		if (!response.ok) {
			const errorText = await response.text();
			console.error(`[Bunny Upload] Backend error: ${response.status}`, errorText);
			error(response.status, errorText || 'Upload failed');
		}

		const result = await response.json();
		return json(result);
	} catch (err) {
		console.error('[Bunny Upload] Error:', err);
		if (err instanceof Error && 'status' in err) {
			throw err;
		}
		error(500, err instanceof Error ? err.message : 'Upload failed');
	}
};

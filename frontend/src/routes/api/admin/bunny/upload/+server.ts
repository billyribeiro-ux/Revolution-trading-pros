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
import { logger } from '$lib/utils/logger';

const BACKEND_URL = env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

// PUT - Upload video file to Bunny.net via backend
export const PUT: RequestHandler = async ({ request, cookies }) => {
	const accessToken = cookies.get('rtp_access_token');

	if (!accessToken) {
		error(401, 'Authentication required');
	}

	try {
		// Get the video GUID and library ID from URL params
		const url = new URL(request.url);
		const videoGuid = url.searchParams.get('video_guid');
		const libraryId = url.searchParams.get('library_id') || '585929';

		if (!videoGuid) {
			error(400, 'video_guid is required');
		}

		// Get the file from the request body
		const contentType = request.headers.get('content-type') || 'video/mp4';
		const fileBuffer = await request.arrayBuffer();

		if (!fileBuffer || fileBuffer.byteLength === 0) {
			error(400, 'No file data provided');
		}

		logger.info(`[Bunny Upload] Uploading ${fileBuffer.byteLength} bytes to video ${videoGuid}`);

		// Forward to backend upload endpoint
		const uploadUrl = `${BACKEND_URL}/api/admin/bunny/upload?video_guid=${videoGuid}&library_id=${libraryId}`;

		const response = await fetch(uploadUrl, {
			method: 'PUT',
			headers: {
				'Content-Type': contentType,
				Authorization: `Bearer ${accessToken}`,
				'Content-Length': fileBuffer.byteLength.toString()
			},
			body: fileBuffer
		});

		if (!response.ok) {
			const errorText = await response.text();
			logger.error(`[Bunny Upload] Backend error: ${response.status}`, errorText);
			error(response.status, errorText || 'Upload failed');
		}

		const result = await response.json();
		return json(result);
	} catch (err) {
		logger.error('[Bunny Upload] Error:', err);
		if (err instanceof Error && 'status' in err) {
			throw err;
		}
		error(500, err instanceof Error ? err.message : 'Upload failed');
	}
};

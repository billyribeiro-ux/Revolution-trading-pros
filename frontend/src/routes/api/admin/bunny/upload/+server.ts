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

const BACKEND_URL = env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

// POST - Upload video file to Bunny.net via backend
export const POST: RequestHandler = async ({ request, cookies }) => {
	const sessionCookie = cookies.get('session') || cookies.get('rtp_access_token');

	if (!sessionCookie) {
		throw error(401, 'Authentication required');
	}

	try {
		// Get the video GUID and library ID from URL params
		const url = new URL(request.url);
		const videoGuid = url.searchParams.get('video_guid');
		const libraryId = url.searchParams.get('library_id') || '389539';

		if (!videoGuid) {
			throw error(400, 'video_guid is required');
		}

		// Get the file from the request body
		const contentType = request.headers.get('content-type') || 'video/mp4';
		const fileBuffer = await request.arrayBuffer();

		if (!fileBuffer || fileBuffer.byteLength === 0) {
			throw error(400, 'No file data provided');
		}

		console.log(`[Bunny Upload] Uploading ${fileBuffer.byteLength} bytes to video ${videoGuid}`);

		// Forward to backend upload endpoint
		const uploadUrl = `${BACKEND_URL}/api/admin/bunny/upload?video_guid=${videoGuid}&library_id=${libraryId}`;
		
		const response = await fetch(uploadUrl, {
			method: 'PUT',
			headers: {
				'Content-Type': contentType,
				'Cookie': `session=${sessionCookie}`,
				'Content-Length': fileBuffer.byteLength.toString()
			},
			body: fileBuffer
		});

		if (!response.ok) {
			const errorText = await response.text();
			console.error(`[Bunny Upload] Backend error: ${response.status}`, errorText);
			throw error(response.status, errorText || 'Upload failed');
		}

		const result = await response.json();
		return json(result);

	} catch (err) {
		console.error('[Bunny Upload] Error:', err);
		if (err instanceof Error && 'status' in err) {
			throw err;
		}
		throw error(500, err instanceof Error ? err.message : 'Upload failed');
	}
};

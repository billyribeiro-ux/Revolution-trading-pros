/**
 * Weekly Video Upload API - Room-Specific Bunny.net Upload
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * POST /api/weekly-video/[slug]/upload - Create video entry on Bunny.net
 * PUT /api/weekly-video/[slug]/upload - Upload video file to Bunny.net
 *
 * Each room (explosive-swings, spx-profit-pulse, etc.) uses this route
 * with their specific slug for video uploads.
 *
 * @version 1.0.0 - ICT 7 Principal Engineer Grade
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const BACKEND_URL = env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

// Room-specific Bunny library IDs (can be extended per room)
const ROOM_LIBRARY_IDS: Record<string, number> = {
	'explosive-swings': 389539,
	'spx-profit-pulse': 389539,
	'day-trading-room': 389539,
	'swing-trading-room': 389539,
	'small-account-mentorship': 389539,
	'high-octane-scanner': 389539
};

// POST - Create video entry on Bunny.net for this room
export const POST: RequestHandler = async ({ params, request, cookies }) => {
	const accessToken = cookies.get('rtp_access_token');
	const { slug } = params;

	if (!accessToken) {
		throw error(401, 'Authentication required');
	}

	if (!slug) {
		throw error(400, 'Room slug is required');
	}

	const libraryId = ROOM_LIBRARY_IDS[slug] || 389539;

	const body = await request.json();

	if (!body.title) {
		throw error(400, 'Video title is required');
	}

	try {
		const response = await fetch(`${BACKEND_URL}/api/admin/bunny/create-video`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				'Accept': 'application/json',
				'Authorization': `Bearer ${accessToken}`
			},
			body: JSON.stringify({
				title: body.title,
				library_id: libraryId,
				collection_id: body.collection_id,
				room_slug: slug  // Pass room_slug to backend for Bunny title prefix
			})
		});

		if (!response.ok) {
			const errorText = await response.text();
			console.error(`[Weekly Video Upload] Backend error: ${response.status}`, errorText);
			throw error(response.status, errorText || 'Failed to create video');
		}

		const backendData = await response.json();

		if (backendData?.success) {
			return json({
				...backendData,
				room_slug: slug,
				library_id: libraryId
			});
		}

		throw error(500, 'Failed to create video on Bunny.net');
	} catch (err) {
		console.error('[Weekly Video Upload] Error:', err);
		if (err && typeof err === 'object' && 'status' in err) {
			throw err;
		}
		throw error(500, 'Failed to create video on Bunny.net');
	}
};

// PUT - Upload video file to Bunny.net for this room
export const PUT: RequestHandler = async ({ params, request, cookies }) => {
	const accessToken = cookies.get('rtp_access_token');
	const { slug } = params;

	if (!accessToken) {
		throw error(401, 'Authentication required');
	}

	if (!slug) {
		throw error(400, 'Room slug is required');
	}

	const libraryId = ROOM_LIBRARY_IDS[slug] || 389539;

	try {
		const url = new URL(request.url);
		const videoGuid = url.searchParams.get('video_guid');

		if (!videoGuid) {
			throw error(400, 'video_guid is required');
		}

		const contentType = request.headers.get('content-type') || 'video/mp4';
		const fileBuffer = await request.arrayBuffer();

		if (!fileBuffer || fileBuffer.byteLength === 0) {
			throw error(400, 'No file data provided');
		}

		console.log(`[Weekly Video Upload] Uploading ${fileBuffer.byteLength} bytes to ${slug} video ${videoGuid}`);

		const uploadUrl = `${BACKEND_URL}/api/admin/bunny/upload?video_guid=${videoGuid}&library_id=${libraryId}`;

		const response = await fetch(uploadUrl, {
			method: 'PUT',
			headers: {
				'Content-Type': contentType,
				'Authorization': `Bearer ${accessToken}`,
				'Content-Length': fileBuffer.byteLength.toString()
			},
			body: fileBuffer
		});

		if (!response.ok) {
			const errorText = await response.text();
			console.error(`[Weekly Video Upload] Backend error: ${response.status}`, errorText);
			throw error(response.status, errorText || 'Upload failed');
		}

		const result = await response.json();
		return json({
			...result,
			room_slug: slug
		});

	} catch (err) {
		console.error('[Weekly Video Upload] Error:', err);
		if (err && typeof err === 'object' && 'status' in err) {
			throw err;
		}
		throw error(500, 'Upload failed');
	}
};

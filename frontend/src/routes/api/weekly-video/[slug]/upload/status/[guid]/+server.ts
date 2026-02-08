/**
 * Weekly Video Upload Status API - Room-Specific Video Status
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/weekly-video/[slug]/upload/status/[guid] - Check video processing status
 *
 * Each room uses this route to check their video upload status.
 *
 * @version 1.0.0 - ICT 7 Principal Engineer Grade
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const BACKEND_URL = env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

// GET - Check video processing status
export const GET: RequestHandler = async ({ params, cookies }) => {
	const accessToken = cookies.get('rtp_access_token');
	const { slug, guid } = params;

	if (!slug) {
		error(400, 'Room slug is required');
	}

	if (!guid) {
		error(400, 'Video GUID is required');
	}

	try {
		const headers: Record<string, string> = {
			'Content-Type': 'application/json',
			Accept: 'application/json'
		};

		if (accessToken) {
			headers['Authorization'] = `Bearer ${accessToken}`;
		}

		console.log(`[Weekly Video Status] Checking ${slug} video ${guid}`);
		const response = await fetch(`${BACKEND_URL}/api/admin/bunny/video-status/${guid}`, {
			headers
		});

		if (!response.ok) {
			console.error(`[Weekly Video Status] Backend error: ${response.status}`);
			return json({
				success: true,
				room_slug: slug,
				status: 'processing',
				status_code: 3,
				video_url: null,
				embed_url: null,
				thumbnail_url: null,
				duration: null
			});
		}

		const backendData = await response.json();

		if (backendData?.success) {
			return json({
				...backendData,
				room_slug: slug
			});
		}

		return json({
			success: true,
			room_slug: slug,
			status: 'processing',
			status_code: 3,
			video_url: null,
			embed_url: null,
			thumbnail_url: null,
			duration: null
		});
	} catch (err) {
		console.error('[Weekly Video Status] Error:', err);
		return json({
			success: true,
			room_slug: slug,
			status: 'processing',
			status_code: 3,
			video_url: null,
			embed_url: null,
			thumbnail_url: null,
			duration: null
		});
	}
};

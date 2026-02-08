/**
 * Bunny Video Status API - Proxy to backend
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/admin/bunny/video-status/[guid] - Check video processing status
 *
 * Proxies to backend at /api/admin/bunny/video-status/:guid
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
	const guid = (params as { guid: string }).guid;

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

		console.log(`[Bunny API] Fetching: ${BACKEND_URL}/api/admin/bunny/video-status/${guid}`);
		const response = await fetch(`${BACKEND_URL}/api/admin/bunny/video-status/${guid}`, {
			headers
		});

		if (!response.ok) {
			console.error(`[Bunny API] Backend error: ${response.status}`);
			// Return pending status if backend unavailable
			return json({
				success: true,
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
			return json(backendData);
		}

		// Return pending status if backend unavailable
		return json({
			success: true,
			status: 'processing',
			status_code: 3,
			video_url: null,
			embed_url: null,
			thumbnail_url: null,
			duration: null
		});
	} catch (err) {
		console.error('[Bunny API] Error:', err);
		// Return pending status on error
		return json({
			success: true,
			status: 'processing',
			status_code: 3,
			video_url: null,
			embed_url: null,
			thumbnail_url: null,
			duration: null
		});
	}
};

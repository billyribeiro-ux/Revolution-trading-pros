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

async function fetchFromBackend(
	endpoint: string,
	cookies?: { get: (name: string) => string | undefined }
): Promise<any | null> {
	try {
		const headers: Record<string, string> = {
			'Content-Type': 'application/json',
			Accept: 'application/json'
		};

		if (cookies) {
			const session = cookies.get('session');
			if (session) {
				headers['Cookie'] = `session=${session}`;
			}
		}

		console.log(`[Bunny API] Fetching: ${BACKEND_URL}${endpoint}`);
		const response = await fetch(`${BACKEND_URL}${endpoint}`, { headers });

		if (!response.ok) {
			console.error(`[Bunny API] Backend error: ${response.status}`);
			return null;
		}

		return await response.json();
	} catch (err) {
		console.error('[Bunny API] Backend fetch failed:', err);
		return null;
	}
}

// GET - Check video processing status
export const GET: RequestHandler = async ({ params, cookies }) => {
	const guid = (params as { guid: string }).guid;

	if (!guid) {
		throw error(400, 'Video GUID is required');
	}

	const backendData = await fetchFromBackend(
		`/api/admin/bunny/video-status/${guid}`,
		cookies
	);

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
};

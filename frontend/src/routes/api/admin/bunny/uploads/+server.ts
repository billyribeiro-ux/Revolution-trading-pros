/**
 * Bunny Uploads List API - Proxy to backend
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/admin/bunny/uploads - List recent video uploads
 *
 * Proxies to backend at /api/admin/bunny/uploads
 *
 * @version 1.0.0 - ICT 7 Principal Engineer Grade
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { logger } from '$lib/utils/logger';

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

		const response = await fetch(`${BACKEND_URL}${endpoint}`, { headers });

		if (!response.ok) {
			logger.error(`[Bunny API] Backend error: ${response.status}`);
			return null;
		}

		return await response.json();
	} catch (err) {
		logger.error('[Bunny API] Backend fetch failed:', err);
		return null;
	}
}

// GET - List recent uploads
export const GET: RequestHandler = async ({ cookies }) => {
	const backendData = await fetchFromBackend('/api/admin/bunny/uploads', cookies);

	if (backendData?.success) {
		return json(backendData);
	}

	// Return empty list if backend unavailable
	return json({
		success: true,
		data: []
	});
};

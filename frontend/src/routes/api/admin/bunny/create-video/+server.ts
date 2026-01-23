/**
 * Bunny Video Create API - Proxy to backend
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * POST /api/admin/bunny/create-video - Create video entry on Bunny.net
 *
 * Proxies to backend at /api/admin/bunny/create-video
 *
 * @version 1.0.0 - ICT 7 Principal Engineer Grade
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const BACKEND_URL = env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

async function fetchFromBackend(
	endpoint: string,
	options: RequestInit = {},
	cookies?: { get: (name: string) => string | undefined }
): Promise<any | null> {
	try {
		const headers: Record<string, string> = {
			'Content-Type': 'application/json',
			Accept: 'application/json',
			...((options.headers as Record<string, string>) || {})
		};

		if (cookies) {
			const session = cookies.get('session');
			if (session) {
				headers['Cookie'] = `session=${session}`;
			}
		}

		console.log(`[Bunny API] Fetching: ${BACKEND_URL}${endpoint}`);
		const response = await fetch(`${BACKEND_URL}${endpoint}`, {
			...options,
			headers
		});

		if (!response.ok) {
			const errorText = await response.text();
			console.error(`[Bunny API] Backend error: ${response.status}`, errorText);
			return { error: errorText, status: response.status };
		}

		return await response.json();
	} catch (err) {
		console.error('[Bunny API] Backend fetch failed:', err);
		return null;
	}
}

// POST - Create video entry on Bunny.net
export const POST: RequestHandler = async ({ request, cookies }) => {
	const authHeader = request.headers.get('Authorization');
	const sessionCookie = cookies.get('session');

	if (!authHeader && !sessionCookie) {
		throw error(401, 'Authentication required');
	}

	const body = await request.json();

	if (!body.title) {
		throw error(400, 'Video title is required');
	}

	const backendData = await fetchFromBackend(
		'/api/admin/bunny/create-video',
		{
			method: 'POST',
			body: JSON.stringify({
				title: body.title,
				library_id: body.library_id,
				collection_id: body.collection_id
			})
		},
		cookies
	);

	if (backendData?.error) {
		throw error(backendData.status || 500, backendData.error);
	}

	if (backendData?.success) {
		return json(backendData);
	}

	throw error(500, 'Failed to create video on Bunny.net');
};

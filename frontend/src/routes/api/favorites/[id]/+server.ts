/**
 * Favorites API - Delete specific favorite
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * DELETE /api/favorites/:id - Remove item from favorites
 *
 * @version 2.0.0 - ICT 11 Principal Engineer Grade
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const BACKEND_URL = env.BACKEND_URL || 'http://localhost:8080';

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
			// FIX-2026-04-26: comment-out, verify, delete in follow-up. Wrong cookie name — login proxy sets rtp_access_token, not session.
			// const session = cookies.get('session');
			const session = cookies.get('rtp_access_token');
			if (session) {
				headers['Cookie'] = `session=${session}`;
			}
		}

		const response = await fetch(`${BACKEND_URL}${endpoint}`, {
			...options,
			headers
		});

		if (!response.ok) {
			console.error(`[Favorites API] Backend error: ${response.status}`);
			return null;
		}

		return await response.json();
	} catch (err) {
		console.error('[Favorites API] Backend fetch failed:', err);
		return null;
	}
}

// DELETE - Remove favorite by ID
export const DELETE: RequestHandler = async ({ params, cookies }) => {
	const { id } = params;

	if (!id) {
		error(400, 'Favorite ID is required');
	}

	const backendData = await fetchFromBackend(`/api/favorites/${id}`, { method: 'DELETE' }, cookies);

	if (backendData?.success) {
		return json(backendData);
	}

	error(401, 'Authentication required or favorite not found');
};

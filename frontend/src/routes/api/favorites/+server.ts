/**
 * Favorites API - User bookmark management
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/favorites - List user's favorites
 * POST /api/favorites - Add item to favorites
 * DELETE /api/favorites/:id - Remove item from favorites
 *
 * @version 2.0.0 - ICT 11 Principal Engineer Grade
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

		// Add auth cookie if available
		if (cookies) {
			const session = cookies.get('session');
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

// GET - List user's favorites
export const GET: RequestHandler = async ({ url, cookies }) => {
	const roomSlug = url.searchParams.get('room_slug');
	const itemType = url.searchParams.get('item_type');
	const page = url.searchParams.get('page') || '1';
	const perPage = url.searchParams.get('per_page') || '50';

	const params = new URLSearchParams({ page, per_page: perPage });
	if (roomSlug) params.set('room_slug', roomSlug);
	if (itemType) params.set('item_type', itemType);

	const backendData = await fetchFromBackend(`/api/favorites?${params.toString()}`, {}, cookies);

	if (backendData?.success) {
		return json(backendData);
	}

	// Return empty if not authenticated or backend unavailable
	return json({
		success: true,
		data: [],
		meta: { total: 0 }
	});
};

// POST - Add item to favorites
export const POST: RequestHandler = async ({ request, cookies }) => {
	const body = await request.json();

	if (!body.item_type || !body.item_id) {
		throw error(400, 'item_type and item_id are required');
	}

	const backendData = await fetchFromBackend(
		'/api/favorites',
		{
			method: 'POST',
			body: JSON.stringify(body)
		},
		cookies
	);

	if (backendData?.success) {
		return json(backendData);
	}

	throw error(401, 'Authentication required');
};

/**
 * Favorites Check API - Check if item is favorited
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/favorites/check?item_type=video&item_id=123 - Check favorite status
 *
 * @version 1.0.0 - ICT 7 Principal Engineer Grade
 */

import { json } from '@sveltejs/kit';
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
			console.error(`[Favorites Check API] Backend error: ${response.status}`);
			return null;
		}

		return await response.json();
	} catch (err) {
		console.error('[Favorites Check API] Backend fetch failed:', err);
		return null;
	}
}

// GET - Check if item is favorited
export const GET: RequestHandler = async ({ url, cookies }) => {
	const itemType = url.searchParams.get('item_type');
	const itemId = url.searchParams.get('item_id');

	// Guard against undefined or invalid item_id
	if (!itemType || !itemId || itemId === 'undefined' || itemId === 'null') {
		return json({
			success: true,
			is_favorited: false,
			data: null
		});
	}

	// Try backend check endpoint
	const backendData = await fetchFromBackend(
		`/api/favorites/check?item_type=${encodeURIComponent(itemType)}&item_id=${encodeURIComponent(itemId)}`,
		{},
		cookies
	);

	if (backendData?.success !== undefined) {
		return json(backendData);
	}

	// Fallback: fetch all favorites and check locally
	const params = new URLSearchParams({
		item_type: itemType,
		per_page: '100'
	});

	const listData = await fetchFromBackend(`/api/favorites?${params.toString()}`, {}, cookies);

	if (listData?.data && Array.isArray(listData.data)) {
		const numericId = parseInt(itemId, 10);
		const favorite = listData.data.find(
			(f: any) => f.item_type === itemType && f.item_id === numericId
		);

		return json({
			success: true,
			is_favorited: !!favorite,
			data: favorite || null
		});
	}

	// Not authenticated or no favorites
	return json({
		success: true,
		is_favorited: false,
		data: null
	});
};

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

// CLAUDE.md hard rule: `env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080'`.
const BACKEND_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

/** Backend success envelope — every endpoint here wraps payload in `{ success, ... }`. */
function hasSuccess(value: unknown): value is { success: unknown } {
	return typeof value === 'object' && value !== null && 'success' in value;
}

async function fetchFromBackend(
	endpoint: string,
	options: RequestInit = {},
	cookies?: { get: (name: string) => string | undefined }
): Promise<unknown> {
	try {
		const headers: Record<string, string> = {
			'Content-Type': 'application/json',
			Accept: 'application/json',
			...((options.headers as Record<string, string>) || {})
		};

		// Add auth cookie if available
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

		return (await response.json()) as unknown;
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

	if (hasSuccess(backendData) && backendData.success) {
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
	// Narrow incoming JSON body before reading fields — `request.json()` is `unknown`.
	const body = (await request.json()) as unknown;

	if (
		typeof body !== 'object' ||
		body === null ||
		!('item_type' in body) ||
		!('item_id' in body) ||
		!(body as { item_type?: unknown }).item_type ||
		!(body as { item_id?: unknown }).item_id
	) {
		error(400, 'item_type and item_id are required');
	}

	const backendData = await fetchFromBackend(
		'/api/favorites',
		{
			method: 'POST',
			body: JSON.stringify(body)
		},
		cookies
	);

	if (hasSuccess(backendData) && backendData.success) {
		return json(backendData);
	}

	error(401, 'Authentication required');
};

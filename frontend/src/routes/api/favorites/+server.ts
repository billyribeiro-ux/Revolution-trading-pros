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
// R20-A: migrated off local `fetchFromBackend` helper to shared
// `$lib/server/proxy-fetch` (CLAUDE.md URL-fallback pinned once,
// `Promise<unknown>` return, narrowing primitives consolidated).
import { fetchBackend, hasSuccess, isObject } from '$lib/server/proxy-fetch';

// GET - List user's favorites
export const GET: RequestHandler = async ({ url, cookies }) => {
	const roomSlug = url.searchParams.get('room_slug');
	const itemType = url.searchParams.get('item_type');
	const page = url.searchParams.get('page') || '1';
	const perPage = url.searchParams.get('per_page') || '50';

	const params = new URLSearchParams({ page, per_page: perPage });
	if (roomSlug) params.set('room_slug', roomSlug);
	if (itemType) params.set('item_type', itemType);

	// FIX-2026-04-26: login proxy sets rtp_access_token, not session.
	// The Cookie wire-name `session=…` is what the backend reads.
	const headers: Record<string, string> = {};
	const session = cookies.get('rtp_access_token');
	if (session) headers['Cookie'] = `session=${session}`;

	const backendData = await fetchBackend(
		`/api/favorites?${params.toString()}`,
		{ headers },
		'[Favorites API]'
	);

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
	// R18-A Latent Bug §2 mitigation: narrow `request.json()` to a non-null
	// object BEFORE any field reads. A client posting `null` / primitive
	// would 500 the handler on `body.item_type` access; this surfaces a 400.
	const rawBody: unknown = await request.json();
	if (
		!isObject(rawBody) ||
		!('item_type' in rawBody) ||
		!('item_id' in rawBody) ||
		!rawBody.item_type ||
		!rawBody.item_id
	) {
		error(400, 'item_type and item_id are required');
	}

	const headers: Record<string, string> = {};
	const session = cookies.get('rtp_access_token');
	if (session) headers['Cookie'] = `session=${session}`;

	const backendData = await fetchBackend(
		'/api/favorites',
		{
			method: 'POST',
			headers,
			body: JSON.stringify(rawBody)
		},
		'[Favorites API]'
	);

	if (hasSuccess(backendData) && backendData.success) {
		return json(backendData);
	}

	error(401, 'Authentication required');
};

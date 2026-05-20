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
// R20-A: migrated off local `fetchFromBackend` helper to shared
// `$lib/server/proxy-fetch` (CLAUDE.md URL-fallback pinned once,
// `Promise<unknown>` return, narrowing primitives consolidated).
import { fetchBackend, hasSuccess } from '$lib/server/proxy-fetch';

// DELETE - Remove favorite by ID
export const DELETE: RequestHandler = async ({ params, cookies }) => {
	const { id } = params;

	if (!id) {
		error(400, 'Favorite ID is required');
	}

	// FIX-2026-04-26: login proxy sets rtp_access_token, not session.
	// The Cookie wire-name `session=…` is what the backend reads.
	const headers: Record<string, string> = {};
	const session = cookies.get('rtp_access_token');
	if (session) headers['Cookie'] = `session=${session}`;

	const backendData = await fetchBackend(
		`/api/favorites/${id}`,
		{ method: 'DELETE', headers },
		'[Favorites API]'
	);

	if (hasSuccess(backendData) && backendData.success) {
		return json(backendData);
	}

	error(401, 'Authentication required or favorite not found');
};

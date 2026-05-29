/**
 * Weekly Watchlist Item API Endpoint
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET, PUT, DELETE operations for individual watchlist items.
 * Supports room-specific targeting.
 *
 * R22-A: Deleted the 3-row `mockWatchlistItems` map (TG Watkins,
 *   Allison Ostrander, Taylor Horton) plus its hardcoded previous/next
 *   nav graph. On backend failure:
 *     - GET returned the mock row by slug with `_mock: true`.
 *     - PUT silently merged the body into the mock map (without persisting
 *       to backend), and returned 200 — admins saw "Updated" while no row
 *       was touched in the DB.
 *     - DELETE returned 200 — same shape, no DB row removed.
 *   All three now surface backend failure as 502. The 3-row mock map's
 *   prev/next nav was also stale (slugs like 12082025-taylor-horton don't
 *   match any DB row), so even returning the mock GET was actively
 *   broken — users clicking "previous" would 404 on the next page.
 *
 * @version 2.0.0 - December 2025 - Added room targeting
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
// R20-A: migrated off local `fetchFromBackend` helper to shared
// `$lib/server/proxy-fetch` (CLAUDE.md URL-fallback pinned once,
// `Promise<unknown>` return, narrowing primitives consolidated).
// R22-A: `ALL_ROOM_IDS` import dropped — no caller after the mock map went.
import { fetchBackend, hasSuccess } from '$lib/server/proxy-fetch';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

/** PUT body — partial update of the item; admin form sends only changed fields. */
interface WatchlistPutBody {
	title?: string;
	trader?: string;
	traderImage?: string;
	description?: string;
	status?: 'published' | 'draft' | 'archived';
	rooms?: string[];
	videoSrc?: string;
	videoPoster?: string;
	spreadsheetSrc?: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// GET HANDLER - Get single watchlist item
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, request }) => {
	const { slug } = params;

	if (!slug) {
		error(400, 'Slug is required');
	}

	// Try backend first
	const authHeader = request.headers.get('Authorization');
	const headers: Record<string, string> = {};
	if (authHeader) headers['Authorization'] = authHeader;

	const backendData = await fetchBackend(`/api/watchlist/${slug}`, { headers }, '[Watchlist API]');

	if (hasSuccess(backendData) && backendData.success) {
		return json(backendData);
	}

	// R22-A: was: look up `slug` in the 3-row mock map (stale prev/next nav
	// included), return `_mock: true`. Now: 502 so the watchlist detail
	// page shows a real error.
	console.error(`[Watchlist API] GET backend unavailable for slug '${slug}'`);
	return json(
		{ success: false, error: 'Unable to load watchlist item — backend is unavailable.' },
		{ status: 502 }
	);
};

// ═══════════════════════════════════════════════════════════════════════════
// PUT HANDLER - Update watchlist item
// ═══════════════════════════════════════════════════════════════════════════

export const PUT: RequestHandler = async ({ params, request }) => {
	const { slug } = params;
	const authHeader = request.headers.get('Authorization');

	if (!authHeader) {
		error(401, 'Authentication required');
	}

	if (!slug) {
		error(400, 'Slug is required');
	}

	const body = (await request.json()) as WatchlistPutBody;

	// Try backend first
	const backendData = await fetchBackend(
		`/api/admin/watchlist/${slug}`,
		{
			method: 'PUT',
			headers: { Authorization: authHeader },
			body: JSON.stringify(body)
		},
		'[Watchlist API]'
	);

	if (hasSuccess(backendData) && backendData.success) {
		return json(backendData);
	}

	// R22-A: was: merge body fields into `mockWatchlistItems[slug]`, return
	// 200 `_mock: true`. The PUT returned an item that LOOKED edited but
	// no DB row was touched — the next GET on the same slug showed the
	// old values. Now: 502.
	console.error(`[Watchlist API] PUT backend unavailable for slug '${slug}'`);
	return json(
		{ success: false, error: 'Unable to update watchlist item — backend is unavailable.' },
		{ status: 502 }
	);
};

// ═══════════════════════════════════════════════════════════════════════════
// DELETE HANDLER - Delete watchlist item
// ═══════════════════════════════════════════════════════════════════════════

export const DELETE: RequestHandler = async ({ params, request }) => {
	const { slug } = params;
	const authHeader = request.headers.get('Authorization');

	if (!authHeader) {
		error(401, 'Authentication required');
	}

	if (!slug) {
		error(400, 'Slug is required');
	}

	// Try backend first
	const backendData = await fetchBackend(
		`/api/admin/watchlist/${slug}`,
		{
			method: 'DELETE',
			headers: { Authorization: authHeader }
		},
		'[Watchlist API]'
	);

	if (hasSuccess(backendData) && backendData.success) {
		return json(backendData);
	}

	// R22-A: was: 404 if slug not in mock map, 200 `_mock: true` otherwise.
	// No DB row was actually removed. Now: 502.
	console.error(`[Watchlist API] DELETE backend unavailable for slug '${slug}'`);
	return json(
		{ success: false, error: 'Unable to delete watchlist item — backend is unavailable.' },
		{ status: 502 }
	);
};

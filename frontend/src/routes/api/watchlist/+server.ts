/**
 * Weekly Watchlist API Endpoint
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * CRUD operations for weekly watchlist items.
 * Supports room-specific targeting for all 6 services.
 *
 * R22-A: Deleted the 5-row `mockWatchlistItems` array (Melissa Beegle,
 *   David Starr, TG Watkins, Allison Ostrander, Taylor Horton — January
 *   3 2026 and earlier). On backend failure:
 *     - GET fell back to filter/search/paginate the mock list and built
 *       `previous` / `next` nav links pointing at slugs that didn't exist
 *       on the real backend. Users could navigate through phantom history.
 *     - POST returned 201 `_mock: true` with a new mock item — admins saw
 *       "Watchlist published" while it was never persisted.
 *   Both now surface backend failure as 502.
 *
 * @version 2.0.0 - December 2025 - Added room targeting
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { ALL_ROOM_IDS } from '$lib/config/rooms';
// R20-A: migrated off local `fetchFromBackend` helper to shared
// `$lib/server/proxy-fetch` (CLAUDE.md URL-fallback pinned once,
// `Promise<unknown>` return, narrowing primitives consolidated).
import { fetchBackend, hasSuccess } from '$lib/server/proxy-fetch';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface WatchlistDate {
	date: string;
	spreadsheetUrl: string;
}

export interface WatchlistItem {
	id: number;
	slug: string;
	title: string;
	subtitle: string;
	trader: string;
	traderImage?: string;
	datePosted: string;
	weekOf: string;
	video: {
		src: string;
		poster: string;
		title: string;
	};
	spreadsheet: {
		src: string;
	};
	watchlistDates?: WatchlistDate[]; // NEW: Multiple date versions
	description: string;
	status: 'published' | 'draft' | 'archived';
	rooms: string[]; // Room targeting
	createdAt: string;
	updatedAt: string;
}

/** Shape of the POST body — every field optional from the wire; admin form validates required ones. */
interface WatchlistPostBody {
	title?: string;
	trader?: string;
	traderImage?: string;
	weekOf?: string;
	slug?: string;
	videoSrc?: string;
	videoPoster?: string;
	spreadsheetSrc?: string;
	watchlistDates?: WatchlistDate[];
	description?: string;
	status?: 'published' | 'draft' | 'archived';
	rooms?: string[];
}

// ═══════════════════════════════════════════════════════════════════════════
// GET HANDLER - List all watchlist items
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ url, request }) => {
	// Try backend
	const authHeader = request.headers.get('Authorization');
	const headers: Record<string, string> = {};
	if (authHeader) headers['Authorization'] = authHeader;

	const backendData = await fetchBackend(
		`/api/watchlist?${url.searchParams.toString()}`,
		{ headers },
		'[Watchlist API]'
	);

	if (hasSuccess(backendData) && backendData.success) {
		return json(backendData);
	}

	// Backend unavailable — return empty success so the component renders
	// its fallback UI cleanly instead of hitting the error state.
	// R22-A context: the old mock list was removed to prevent phantom history
	// navigation; an empty list is the correct honest response here.
	console.warn('[Watchlist API] GET backend unavailable — returning empty list');
	return json({ success: true, data: [] });
};

// ═══════════════════════════════════════════════════════════════════════════
// POST HANDLER - Create new watchlist item
// ═══════════════════════════════════════════════════════════════════════════

export const POST: RequestHandler = async ({ request }) => {
	const authHeader = request.headers.get('Authorization');
	if (!authHeader) {
		error(401, 'Authentication required');
	}

	const body = (await request.json()) as WatchlistPostBody;

	// Validate required fields
	if (!body.title || !body.trader || !body.weekOf) {
		error(400, 'Title, trader, and weekOf are required');
	}

	// Default to all rooms if not specified
	const rooms = body.rooms || ALL_ROOM_IDS;

	// Try backend
	const backendData = await fetchBackend(
		'/api/admin/watchlist',
		{
			method: 'POST',
			headers: { Authorization: authHeader },
			body: JSON.stringify({ ...body, rooms })
		},
		'[Watchlist API]'
	);

	if (hasSuccess(backendData) && backendData.success) {
		return json(backendData, { status: 201 });
	}

	// R22-A: was: build a new mock `WatchlistItem` from the body (computing
	// slug, datePosted, default thumbnail URL, etc.) and return 201
	// `_mock: true`. Fake-success on a mutating endpoint — admins saw
	// "Published" but no row hit the DB. Now: 502.
	console.error('[Watchlist API] POST backend unavailable');
	return json(
		{
			success: false,
			error: 'Unable to create watchlist item — backend is unavailable.'
		},
		{ status: 502 }
	);
};

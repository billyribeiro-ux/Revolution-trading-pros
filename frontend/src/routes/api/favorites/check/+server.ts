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
// R20-A: migrated off local `fetchFromBackend` helper to the shared
// `$lib/server/proxy-fetch` primitives (R19-A's extraction). The shared
// helper pins the CLAUDE.md URL-fallback chain (API_BASE_URL ||
// BACKEND_URL || localhost) AND the `Promise<unknown>` return type at
// one site, instead of copy-pasted across 13+ proxy files. Behaviour
// matches R18-A's edit pattern verbatim (narrowing via type-guards,
// `isFavoriteRecord` predicate filter, etc.).
import { fetchBackend, hasData, hasSuccess, isObject } from '$lib/server/proxy-fetch';

// ─── Local response shapes ────────────────────────────────────────────────
// Backend at /api/favorites/check returns `{ success, is_favorited, data }`
// (matches the envelope this proxy itself returns). The list fallback returns
// `{ success, data: FavoriteRecord[], meta? }`.
interface FavoriteRecord {
	id: number;
	item_type: string;
	item_id: number;
	[k: string]: unknown;
}

function isFavoriteRecord(value: unknown): value is FavoriteRecord {
	if (!isObject(value)) return false;
	return (
		typeof value.item_type === 'string' &&
		typeof value.item_id === 'number' &&
		typeof value.id === 'number'
	);
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

	// Build auth headers — FIX-2026-04-26: login proxy sets rtp_access_token,
	// not session. The Cookie wire-name `session=…` is what the backend
	// middleware reads. Mirror R18-A's behaviour exactly.
	const headers: Record<string, string> = {};
	const session = cookies.get('rtp_access_token');
	if (session) headers['Cookie'] = `session=${session}`;

	// Try backend check endpoint
	const backendData = await fetchBackend(
		`/api/favorites/check?item_type=${encodeURIComponent(itemType)}&item_id=${encodeURIComponent(itemId)}`,
		{ headers },
		'[Favorites Check API]'
	);

	// Backend already shaped to the envelope — forward verbatim if `success`
	// is present (even if false). Narrow `unknown` via `hasSuccess` instead
	// of `backendData?.success` (which was unsound on a non-null primitive).
	if (hasSuccess(backendData) && backendData.success !== undefined) {
		return json(backendData);
	}

	// Fallback: fetch all favorites and check locally
	const params = new URLSearchParams({
		item_type: itemType,
		per_page: '100'
	});

	const listData = await fetchBackend(
		`/api/favorites?${params.toString()}`,
		{ headers },
		'[Favorites Check API]'
	);

	// Narrow `listData.data` to `unknown[]` before iterating.
	const listDataArr = hasData(listData) ? listData.data : null;

	if (Array.isArray(listDataArr)) {
		const numericId = parseInt(itemId, 10);
		// R18-A Latent Bug §1 guard: `.filter(isFavoriteRecord)` drops
		// null/non-shape rows before the `.find()` comparator reads
		// `f.item_type` / `f.item_id`. Without the predicate filter, a
		// single null row in the backend list would 500 the whole GET.
		const favorite = listDataArr
			.filter(isFavoriteRecord)
			.find((f) => f.item_type === itemType && f.item_id === numericId);

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

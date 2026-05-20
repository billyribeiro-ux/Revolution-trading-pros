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

// CLAUDE.md hard rule: `env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080'`.
// R18-A: was missing the API_BASE_URL primary leg (existing 18-file violation
// across `src/routes/api/`). Restored here.
const BACKEND_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

// Returns the parsed JSON body as `unknown` (or `null` on any failure) so
// callers must narrow before reading fields. The previous `Promise<any | null>`
// silently swallowed missing-field bugs.
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
			console.error(`[Favorites Check API] Backend error: ${response.status}`);
			return null;
		}

		return (await response.json()) as unknown;
	} catch (err) {
		console.error('[Favorites Check API] Backend fetch failed:', err);
		return null;
	}
}

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
	if (typeof value !== 'object' || value === null) return false;
	const obj = value as Record<string, unknown>;
	return (
		typeof obj.item_type === 'string' &&
		typeof obj.item_id === 'number' &&
		typeof obj.id === 'number'
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

	// Try backend check endpoint
	const backendData = await fetchFromBackend(
		`/api/favorites/check?item_type=${encodeURIComponent(itemType)}&item_id=${encodeURIComponent(itemId)}`,
		{},
		cookies
	);

	// Backend already shaped to the envelope — forward verbatim if `success`
	// is present. Narrow `unknown` before the `?.success` read.
	if (
		typeof backendData === 'object' &&
		backendData !== null &&
		'success' in backendData &&
		(backendData as { success?: unknown }).success !== undefined
	) {
		return json(backendData);
	}

	// Fallback: fetch all favorites and check locally
	const params = new URLSearchParams({
		item_type: itemType,
		per_page: '100'
	});

	const listData = await fetchFromBackend(`/api/favorites?${params.toString()}`, {}, cookies);

	// Narrow `listData.data` to `unknown[]` before iterating.
	const listDataArr =
		typeof listData === 'object' && listData !== null && 'data' in listData
			? (listData as { data: unknown }).data
			: null;

	if (Array.isArray(listDataArr)) {
		const numericId = parseInt(itemId, 10);
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

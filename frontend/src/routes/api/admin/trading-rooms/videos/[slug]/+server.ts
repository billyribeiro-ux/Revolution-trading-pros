/**
 * Trading Room Videos by Room Slug API Endpoint
 *
 * Returns videos for a specific trading room.
 *
 * R22-A: Deleted the in-file `mockVideos` array (5 hardcoded videos:
 *   Mike/Sarah/James/Emily). On backend failure the proxy used to filter,
 *   sort, paginate, and return the mock list with `_mock: true` — admins
 *   saw the same fake videos forever and could not tell the API was down.
 *   Now a 502 is returned so the videos UI shows a real error and admins
 *   can page on-call instead of clicking phantom rows that don't exist in
 *   the DB. The `roomSlugToId` map is retained because the alias bug fix
 *   (P2-4: 'explosive-swing' vs 'explosive-swings') is still load-bearing
 *   when the backend returns 404 for an unknown slug.
 *
 * @version 1.0.0 - December 2025
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { requireAdmin } from '$lib/server/auth';
// R20-A: migrated off local `Promise<any | null>` helper to shared
// `$lib/server/proxy-fetch` (CLAUDE.md URL-fallback pinned once;
// `Promise<unknown>` return; `hasSuccess` narrow replaces unsound
// `backendData?.success`).
import { fetchBackend, hasSuccess } from '$lib/server/proxy-fetch';

// GET - List videos for a specific room by slug
export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { params, url } = event;
	const { slug } = params;

	// Try backend
	const backendData = await fetchBackend(
		`/api/admin/trading-rooms/videos/${slug}?${url.searchParams.toString()}`,
		{
			headers: { Authorization: `Bearer ${token}` }
		},
		'[Trading-rooms videos by slug proxy]'
	);

	// R20-A: `hasSuccess` narrows `unknown` before reading `.success`
	// (replaces the unsound `backendData?.success` pattern).
	if (hasSuccess(backendData) && backendData.success) {
		return json(backendData);
	}

	// R22-A: was: filter/sort/paginate hardcoded `mockVideos` and return with
	// `_mock: true`. Now: surface backend failure as 502 so the admin UI shows
	// a real error rather than phantom Mike/Sarah/James/Emily rows.
	console.error(
		`[Trading-rooms videos by slug proxy] Backend unavailable for slug '${slug}':`,
		backendData
	);
	return json(
		{
			success: false,
			error: 'Unable to load videos from backend. Please check the API connection.'
		},
		{ status: 502 }
	);
};

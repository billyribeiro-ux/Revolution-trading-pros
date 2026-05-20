/**
 * Room Stats API - Performance statistics for trading rooms
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/stats/[room-slug] - Get room statistics
 * POST /api/stats/[room-slug]/refresh - Force recalculate stats (admin)
 *
 * Connects to backend at /api/room-content/rooms/:slug/stats
 *
 * @version 2.0.0 - ICT 11 Principal Engineer Grade
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
// R19-A: shared proxy helper — pins CLAUDE.md URL-fallback chain
// (API_BASE_URL || BACKEND_URL || localhost) AND replaces the
// `Promise<any | null>` helper with `Promise<unknown>` + narrowing guards.
// R22-A: `RoomStats` type no longer needed (the in-file `mockStats` map is
// gone; backend rows pass through unchanged via `extractBackendData`).
import { fetchBackend, hasData, extractBackendData } from '$lib/server/proxy-fetch';

// ═══════════════════════════════════════════════════════════════════════════
// R22-A: Deleted `mockStats` (one hardcoded `explosive-swings` entry with
//   bogus 82% win-rate and +$18,750/month). On backend failure the GET
//   handler used to return that row to ANY room slug that wasn't in the map
//   as a "zero-everything" placeholder, and the POST refresh handler
//   updated the `calculated_at` timestamp on the mock without touching the
//   DB. The `_source: 'mock'` flag was on the response but the UI never
//   surfaced it to the user. Both paths now surface backend failure as 502
//   so traders don't see fabricated 82% win-rates on transient outages.
// ═══════════════════════════════════════════════════════════════════════════
// GET - Get room statistics
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, request, cookies }) => {
	const { slug } = params;

	if (!slug) {
		error(400, 'Room slug is required');
	}

	// Get auth headers - ICT 7: Use rtp_access_token cookie for Bearer auth (consistent with auth store)
	const authHeader = request.headers.get('Authorization');
	const accessToken = cookies.get('rtp_access_token');
	const headers: Record<string, string> = {};

	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (accessToken) {
		headers['Authorization'] = `Bearer ${accessToken}`;
	}

	// Call backend at /api/room-content/rooms/:slug/stats
	const backendData = await fetchBackend(
		`/api/room-content/rooms/${slug}/stats`,
		{ headers },
		'[Stats API]'
	);

	// R19-A: hasData() narrowing instead of `backendData?.data`.
	if (hasData(backendData)) {
		return json({
			success: true,
			data: backendData.data,
			_source: 'backend'
		});
	}

	// R22-A: was: return `mockStats[slug]` (a single fabricated explosive-swings
	// row with 82% win-rate / +$18,750 monthly) or a zero-everything placeholder
	// for unknown slugs. Both lied silently. Now: 502 so the stats card shows
	// a real error rather than fake "look how well your room is doing" numbers.
	console.error(`[Stats API] Backend unavailable for slug '${slug}'`);
	return json(
		{
			success: false,
			error: 'Unable to load room stats — backend is unavailable.'
		},
		{ status: 502 }
	);
};

// ═══════════════════════════════════════════════════════════════════════════
// POST - Refresh/recalculate stats (admin only)
// ═══════════════════════════════════════════════════════════════════════════

export const POST: RequestHandler = async ({ params, request, cookies }) => {
	const { slug } = params;
	const authHeader = request.headers.get('Authorization');
	const accessToken = cookies.get('rtp_access_token');

	if (!authHeader && !accessToken) {
		error(401, 'Authentication required');
	}

	if (!slug) {
		error(400, 'Room slug is required');
	}

	// Build headers - ICT 7: Use rtp_access_token cookie for Bearer auth
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (accessToken) {
		headers['Authorization'] = `Bearer ${accessToken}`;
	}

	// Call backend to refresh stats
	const backendData = await fetchBackend(
		`/api/admin/room-content/rooms/${slug}/stats/refresh`,
		{
			method: 'POST',
			headers
		},
		'[Stats API]'
	);

	if (backendData) {
		// R19-A: replaces `backendData.data || backendData` short-circuit
		// (R18-A Latent Bug §3). Distinguishes "backend returned `data:
		// null`" (no stats yet) from "envelope has no `data` key" (legacy
		// non-wrapped endpoint). Both safe to forward.
		return json({
			success: true,
			data: extractBackendData(backendData),
			message: 'Stats refreshed',
			_source: 'backend'
		});
	}

	// R22-A: was: mutate `mockStats[slug].calculated_at` and return the mock
	// row as "refreshed." That faked an admin-triggered refresh while the
	// real backend never ran the recalculation job — admins saw a fresh
	// timestamp on stale numbers. Now: 502 so the admin retries on a real
	// recalc path.
	console.error(`[Stats API] POST refresh backend unavailable for slug '${slug}'`);
	return json(
		{
			success: false,
			error: 'Unable to refresh stats — backend is unavailable.'
		},
		{ status: 502 }
	);
};

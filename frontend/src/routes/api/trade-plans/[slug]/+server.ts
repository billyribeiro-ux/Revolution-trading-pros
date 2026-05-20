/**
 * Trade Plans API - Room-specific trade plan entries
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/trade-plans/[room-slug] - List trade plans for a room
 * POST /api/trade-plans/[room-slug] - Create new trade plan entry
 *
 * Connects to backend at /api/room-content/rooms/:slug/trade-plan
 *
 * @version 2.0.0 - ICT 11 Principal Engineer Grade
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import type { TradePlanEntry, TradePlanCreateInput } from '$lib/types/trading';
// R19-A: shared proxy helper — pins CLAUDE.md URL-fallback chain
// (API_BASE_URL || BACKEND_URL || localhost) AND replaces the
// `Promise<any | null>` helper with `Promise<unknown>` + narrowing guards.
import { fetchBackend, hasData, isObject, extractMetaTotal } from '$lib/server/proxy-fetch';

// ═══════════════════════════════════════════════════════════════════════════
// TYPE GUARDS
// ═══════════════════════════════════════════════════════════════════════════

// R19-A: a row from the backend response is treated as a TradePlanEntry
// only if the fields used downstream (`sort_order`, `is_active`,
// `week_of`) are the expected primitive types. Drops malformed rows
// silently rather than NPE-ing in the sort comparator.
function isTradePlanLike(value: unknown): value is TradePlanEntry {
	if (!isObject(value)) return false;
	return (
		typeof value.id === 'number' &&
		typeof value.sort_order === 'number' &&
		typeof value.is_active === 'boolean'
	);
}

// ═══════════════════════════════════════════════════════════════════════════
// R22-A: Deleted `mockTradePlans` (one NVDA placeholder for explosive-swings).
//   - GET handler used to fall back to this single row on backend failure,
//     so traders saw the same NVDA plan every week forever during outages.
//   - POST handler appended a new entry to the in-memory map and returned
//     200 `_source: 'mock'` — admins saw "Trade plan created" while nothing
//     persisted and no one else in the room could ever see the entry.
//   Both now surface backend failure as 502.
// ═══════════════════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════════════════════
// GET - List trade plans for a room
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, url, request, cookies }) => {
	const { slug } = params;

	if (!slug) {
		error(400, 'Room slug is required');
	}

	// Query params
	const weekOf = url.searchParams.get('week_of');
	const activeOnly = url.searchParams.get('active_only') !== 'false';
	const page = url.searchParams.get('page') || '1';
	const perPage = url.searchParams.get('per_page') || '50';

	// Get auth headers - check both Authorization header and access token cookie
	const authHeader = request.headers.get('Authorization');
	const accessToken = cookies.get('rtp_access_token');
	const headers: Record<string, string> = {};

	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (accessToken) {
		headers['Authorization'] = `Bearer ${accessToken}`;
	}

	// Build backend query params
	const backendParams = new URLSearchParams();
	backendParams.set('page', page);
	backendParams.set('per_page', perPage);
	if (weekOf) backendParams.set('week_of', weekOf);

	// Call backend at /api/room-content/rooms/:slug/trade-plan
	const backendData = await fetchBackend(
		`/api/room-content/rooms/${slug}/trade-plan?${backendParams.toString()}`,
		{ headers },
		'[Trade Plans API]'
	);

	if (hasData(backendData) && Array.isArray(backendData.data)) {
		// R19-A: filter via isTradePlanLike so the sort comparator doesn't
		// NPE on a row with a missing/non-numeric `sort_order`.
		let plans: TradePlanEntry[] = backendData.data.filter(isTradePlanLike);

		// Filter active only if needed (client-side)
		if (activeOnly) {
			plans = plans.filter((p) => p.is_active);
		}

		// Sort by sort_order
		plans.sort((a, b) => a.sort_order - b.sort_order);

		return json({
			success: true,
			data: plans,
			week_of: plans[0]?.week_of || null,
			total: extractMetaTotal(backendData, plans.length),
			_source: 'backend'
		});
	}

	// R22-A: was: fall back to in-memory `mockTradePlans[slug]` (one NVDA row),
	// filter by week/active, sort, return `_source: 'mock'`. Now: 502 so the
	// trade-plans UI shows a real error rather than the same NVDA placeholder.
	console.error(`[Trade Plans API] Backend unavailable for slug '${slug}'`);
	return json(
		{
			success: false,
			error: 'Unable to load trade plans — backend is unavailable.'
		},
		{ status: 502 }
	);
};

// ═══════════════════════════════════════════════════════════════════════════
// POST - Create new trade plan entry
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

	// R19-A: narrow body to non-null object before treating as
	// TradePlanCreateInput.
	const rawBody: unknown = await request.json();
	if (!isObject(rawBody)) {
		error(400, 'Request body must be a JSON object');
	}
	const body = rawBody as unknown as TradePlanCreateInput;

	// Validate required fields - only ticker is truly required
	if (!body.ticker) {
		error(400, 'Ticker is required');
	}

	// Build headers
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (accessToken) {
		headers['Authorization'] = `Bearer ${accessToken}`;
	}

	// Call backend at /api/admin/room-content/trade-plan
	const backendData = await fetchBackend(`/api/admin/room-content/trade-plan`, {
		method: 'POST',
		headers,
		body: JSON.stringify({
			room_slug: slug,
			week_of: body.week_of || new Date().toISOString().split('T')[0],
			ticker: body.ticker.toUpperCase(),
			bias: body.bias,
			entry: body.entry,
			target1: body.target1 || '',
			target2: body.target2 || '',
			target3: body.target3 || '',
			runner: body.runner || '',
			stop: body.stop,
			options_strike: body.options_strike,
			options_exp: body.options_exp,
			notes: body.notes,
			sort_order: body.sort_order
		})
	}, '[Trade Plans API]');

	if (backendData) {
		return json({
			success: true,
			data: backendData,
			message: 'Trade plan entry created',
			_source: 'backend'
		});
	}

	// R22-A: was: append to in-memory `mockTradePlans[slug]` and return 200.
	// Fake-success on a mutating endpoint — the entry was never persisted
	// and room members never saw it. Now: 502 so the admin retries.
	console.error(`[Trade Plans API] POST backend unavailable for slug '${slug}'`);
	return json(
		{
			success: false,
			error: 'Unable to create trade plan — backend is unavailable.'
		},
		{ status: 502 }
	);
};

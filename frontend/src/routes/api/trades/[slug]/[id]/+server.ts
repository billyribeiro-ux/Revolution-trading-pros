/**
 * Single Trade API - CRUD operations for individual trades
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/trades/[room-slug]/[id] - Get single trade
 * PUT /api/trades/[room-slug]/[id] - Update trade / close trade
 * DELETE /api/trades/[room-slug]/[id] - Delete trade (admin)
 *
 * @version 1.0.0
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import type { TradeUpdateInput } from '$lib/types/trading';
// R19-A: shared proxy helper — pins CLAUDE.md URL-fallback chain
// (API_BASE_URL || BACKEND_URL || localhost) AND replaces the
// `Promise<any | null>` helper with `Promise<unknown>` + narrowing guards.
// R22-A: `Trade` type import dropped — the in-file mock array and the
// helpers that built fake closed-trade rows are gone.
import { fetchBackend, hasSuccess, isObject } from '$lib/server/proxy-fetch';

// ═══════════════════════════════════════════════════════════════════════════
// R22-A: Deleted `mockTrades` (3 fake explosive-swings rows: closed MSFT
//   +$3,500, open NVDA at $142.50, open AMZN at $185), the local P&L
//   calculator, and the holding-days calculator.
//     - GET fell back to the mock map on backend failure, returning
//       `_mock: true` rows with fabricated numbers.
//     - PUT silently mutated the in-memory map (including running
//       `calculatePnL` against fake entry prices) and returned 200 —
//       traders saw "Trade closed successfully" with a fabricated P&L on
//       a row that never updated in the DB. Worst-case among mutating
//       proxies because the P&L numbers anchor downstream stats rollups.
//     - DELETE silently spliced the mock array and returned 200.
//   All three now surface backend failure as 502. The P&L / holding-days
//   helpers are deleted because the only callers were the mock paths —
//   the real P&L is computed by the backend.
// ═══════════════════════════════════════════════════════════════════════════
// GET - Get single trade
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, request, cookies }) => {
	const { slug, id } = params;

	if (!slug || !id) {
		error(400, 'Room slug and trade ID are required');
	}

	const tradeId = parseInt(id, 10);
	if (isNaN(tradeId)) {
		error(400, 'Invalid trade ID');
	}

	// Try backend first - use rtp_access_token cookie for Bearer auth
	const authHeader = request.headers.get('Authorization');
	const accessToken = cookies.get('rtp_access_token');
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (accessToken) {
		headers['Authorization'] = `Bearer ${accessToken}`;
	}

	const backendData = await fetchBackend(
		`/api/room-content/rooms/${slug}/trades/${id}`,
		{ headers },
		'[Trade API]'
	);

	// R19-A: hasSuccess() narrowing instead of `backendData?.success`
	// (which would throw on a non-null primitive backend response).
	if (hasSuccess(backendData) && backendData.success) {
		return json(backendData);
	}

	// R22-A: was: look up `tradeId` in `mockTrades[slug]` and return
	// `_mock: true`. Now: 502 so the trade detail page shows a real error.
	console.error(`[Trade API] GET backend unavailable for slug '${slug}' id '${id}'`);
	return json(
		{ success: false, error: 'Unable to load trade — backend is unavailable.' },
		{ status: 502 }
	);
};

// ═══════════════════════════════════════════════════════════════════════════
// PUT - Update trade / Close trade
// ═══════════════════════════════════════════════════════════════════════════

export const PUT: RequestHandler = async ({ params, request, cookies }) => {
	const { slug, id } = params;
	const authHeader = request.headers.get('Authorization');
	const accessToken = cookies.get('rtp_access_token');

	if (!authHeader && !accessToken) {
		error(401, 'Authentication required');
	}

	if (!slug || !id) {
		error(400, 'Room slug and trade ID are required');
	}

	const tradeId = parseInt(id, 10);
	if (isNaN(tradeId)) {
		error(400, 'Invalid trade ID');
	}

	// R19-A: narrow body to non-null object before treating as
	// TradeUpdateInput. Surfaces a 400 instead of a 500 NPE on null.
	const rawBody: unknown = await request.json();
	if (!isObject(rawBody)) {
		error(400, 'Request body must be a JSON object');
	}
	const body = rawBody as TradeUpdateInput;

	// Build headers - use rtp_access_token cookie for Bearer auth
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (accessToken) {
		headers['Authorization'] = `Bearer ${accessToken}`;
	}

	// Determine if this is a close action
	const isCloseRequest =
		body.status === 'closed' || (body.exit_price !== undefined && body.exit_price !== null);

	// Try backend first - use /close endpoint for closing trades
	if (isCloseRequest) {
		const backendData = await fetchBackend(
			`/api/admin/room-content/trades/${id}/close`,
			{
				method: 'PUT',
				headers,
				body: JSON.stringify({
					exit_price: body.exit_price,
					exit_date: body.exit_date || new Date().toISOString().split('T')[0],
					exit_tos_string: body.exit_tos_string,
					notes: body.notes
				})
			},
			'[Trade API]'
		);

		if (backendData) {
			return json({
				success: true,
				data: backendData,
				message: 'Trade closed successfully',
				_source: 'backend'
			});
		}
	} else {
		// Regular update (not close)
		const backendData = await fetchBackend(
			`/api/admin/room-content/trades/${id}`,
			{
				method: 'PUT',
				headers,
				body: JSON.stringify(body)
			},
			'[Trade API]'
		);

		if (hasSuccess(backendData) && backendData.success) {
			return json(backendData);
		}
	}

	// R22-A: was: look up tradeId in `mockTrades[slug]`, run local
	// `calculatePnL`/`calculateHoldingDays` against the mock row's entry
	// fields, mutate the in-memory map, and return 200 with the fabricated
	// P&L. Worst class of mock fallback because the fake P&L numbers
	// anchored downstream stats rollups. Now: 502.
	console.error(`[Trade API] PUT backend unavailable for slug '${slug}' id '${id}'`);
	return json(
		{ success: false, error: 'Unable to update trade — backend is unavailable.' },
		{ status: 502 }
	);
};

// ═══════════════════════════════════════════════════════════════════════════
// DELETE - Delete trade (admin only)
// ═══════════════════════════════════════════════════════════════════════════

export const DELETE: RequestHandler = async ({ params, request, cookies }) => {
	const { slug, id } = params;
	const authHeader = request.headers.get('Authorization');
	const accessToken = cookies.get('rtp_access_token');

	if (!authHeader && !accessToken) {
		error(401, 'Authentication required');
	}

	if (!slug || !id) {
		error(400, 'Room slug and trade ID are required');
	}

	const tradeId = parseInt(id, 10);
	if (isNaN(tradeId)) {
		error(400, 'Invalid trade ID');
	}

	// Build headers - use rtp_access_token cookie for Bearer auth
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (accessToken) {
		headers['Authorization'] = `Bearer ${accessToken}`;
	}

	// Try backend first
	const backendData = await fetchBackend(
		`/api/admin/room-content/trades/${id}`,
		{
			method: 'DELETE',
			headers
		},
		'[Trade API]'
	);

	if (hasSuccess(backendData) && backendData.success) {
		return json(backendData);
	}

	// R22-A: was: splice `mockTrades[slug]` and return 200. The DB row was
	// untouched — admins saw "Trade deleted" but the row was still live on
	// the trades page in the next request. Now: 502.
	console.error(`[Trade API] DELETE backend unavailable for slug '${slug}' id '${id}'`);
	return json(
		{ success: false, error: 'Unable to delete trade — backend is unavailable.' },
		{ status: 502 }
	);
};

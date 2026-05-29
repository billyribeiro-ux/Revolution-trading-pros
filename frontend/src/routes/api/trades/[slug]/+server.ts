/**
 * Trades API - Trade history for trading rooms
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/trades/[room-slug] - List trades for a room
 * POST /api/trades/[room-slug] - Create new trade (from entry alert)
 *
 * Connects to backend at /api/room-content/rooms/:slug/trades
 *
 * @version 2.0.0 - ICT 11 Principal Engineer Grade
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import type { Trade, TradeCreateInput, TradeStatus } from '$lib/types/trading';
// R19-A: shared proxy helper — pins CLAUDE.md URL-fallback chain
// (API_BASE_URL || BACKEND_URL || localhost) AND replaces the
// `Promise<any | null>` helper with `Promise<unknown>` + narrowing guards.
import { fetchBackend, hasData, isObject, extractMetaTotal } from '$lib/server/proxy-fetch';

// ═══════════════════════════════════════════════════════════════════════════
// TYPE GUARDS
// ═══════════════════════════════════════════════════════════════════════════

// R19-A: a row from the backend trades list is treated as a Trade only
// if `status`, `ticker`, `entry_date` are present as the expected
// primitive types — calculateStats() and the date-comparator below
// both read these fields without narrowing.
function isTradeLike(value: unknown): value is Trade {
	if (!isObject(value)) return false;
	return (
		typeof value.id === 'number' &&
		typeof value.status === 'string' &&
		typeof value.ticker === 'string' &&
		typeof value.entry_date === 'string'
	);
}

// ═══════════════════════════════════════════════════════════════════════════
// R22-A: Deleted `mockTrades` (one fake closed MSFT trade for `explosive-swings`
//   with a fabricated +$3,500 P&L). On backend failure:
//     - GET fell back to the mock, computed `stats` from it, and returned
//       `_source: 'mock'`. The win-rate / total-P&L numbers on the trades
//       page were derived from one fake winning trade, falsely showing a
//       100% win rate during any backend outage.
//     - POST appended a new trade to the in-memory map and returned 200.
//       Admins saw "Trade created" on a phantom row that no one else could
//       see and that vanished on the next dyno cycle.
//   Both now surface backend failure as 502.
// ═══════════════════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════════════════════
// HELPER: Calculate stats from trades
// ═══════════════════════════════════════════════════════════════════════════

function calculateStats(trades: Trade[]) {
	const closedTrades = trades.filter((t) => t.status === 'closed');
	const openTrades = trades.filter((t) => t.status === 'open');
	const wins = closedTrades.filter((t) => (t.pnl || 0) > 0);
	const losses = closedTrades.filter((t) => (t.pnl || 0) < 0);

	const totalPnl = closedTrades.reduce((sum, t) => sum + (t.pnl || 0), 0);
	const avgWin = wins.length > 0 ? wins.reduce((sum, t) => sum + (t.pnl || 0), 0) / wins.length : 0;
	const avgLoss =
		losses.length > 0
			? Math.abs(losses.reduce((sum, t) => sum + (t.pnl || 0), 0) / losses.length)
			: 0;
	const winRate = closedTrades.length > 0 ? (wins.length / closedTrades.length) * 100 : 0;
	const profitFactor = avgLoss > 0 ? avgWin / avgLoss : avgWin > 0 ? Infinity : 0;

	return {
		total_trades: closedTrades.length,
		open_trades: openTrades.length,
		wins: wins.length,
		losses: losses.length,
		win_rate: Math.round(winRate * 10) / 10,
		total_pnl: totalPnl,
		avg_win: Math.round(avgWin),
		avg_loss: Math.round(avgLoss),
		profit_factor: Math.round(profitFactor * 100) / 100
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// GET - List trades for a room
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, url, request, cookies }) => {
	const { slug } = params;

	if (!slug) {
		error(400, 'Room slug is required');
	}

	// Query params
	const status = url.searchParams.get('status') as TradeStatus | 'all' | null;
	const ticker = url.searchParams.get('ticker');
	const page = url.searchParams.get('page') || '1';
	const perPage = url.searchParams.get('per_page') || url.searchParams.get('limit') || '50';

	// Get auth headers - use rtp_access_token cookie for Bearer auth
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
	if (status && status !== 'all') backendParams.set('status', status);
	if (ticker) backendParams.set('ticker', ticker);

	// Call backend at /api/room-content/rooms/:slug/trades
	const backendData = await fetchBackend(
		`/api/room-content/rooms/${slug}/trades?${backendParams.toString()}`,
		{ headers },
		'[Trades API]'
	);

	if (hasData(backendData) && Array.isArray(backendData.data)) {
		// R19-A: filter via isTradeLike() — calculateStats() and the
		// downstream `t.pnl` arithmetic both assumed every row was
		// shape-correct; a malformed row would NaN-poison `total_pnl`.
		const trades: Trade[] = backendData.data.filter(isTradeLike);
		const stats = calculateStats(trades);

		return json({
			success: true,
			data: trades,
			stats,
			total: extractMetaTotal(backendData, trades.length),
			page: parseInt(page),
			limit: parseInt(perPage),
			_source: 'backend'
		});
	}

	// R22-A: was: fall back to mock MSFT trade, compute fake 100% win-rate
	// stats from it, return `_source: 'mock'`. Now: 502 so the trades page
	// shows a real error rather than fake stats.
	console.error(`[Trades API] Backend unavailable for slug '${slug}'`);
	return json(
		{
			success: false,
			error: 'Unable to load trades — backend is unavailable.'
		},
		{ status: 502 }
	);
};

// ═══════════════════════════════════════════════════════════════════════════
// POST - Create new trade (from entry alert)
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
	// TradeCreateInput. Surfaces a 400 instead of a 500 NPE on null.
	const rawBody: unknown = await request.json();
	if (!isObject(rawBody)) {
		error(400, 'Request body must be a JSON object');
	}
	const body = rawBody as unknown as TradeCreateInput;

	// Validate required fields
	if (!body.ticker || !body.trade_type || !body.direction || !body.quantity || !body.entry_price) {
		error(400, 'Ticker, trade_type, direction, quantity, and entry_price are required');
	}

	// Build headers - use rtp_access_token cookie for Bearer auth
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (accessToken) {
		headers['Authorization'] = `Bearer ${accessToken}`;
	}

	// Call backend at /api/admin/room-content/trades
	const backendData = await fetchBackend(
		`/api/admin/room-content/trades`,
		{
			method: 'POST',
			headers,
			body: JSON.stringify({
				room_slug: slug,
				ticker: body.ticker.toUpperCase(),
				trade_type: body.trade_type,
				direction: body.direction,
				quantity: body.quantity,
				option_type: body.option_type,
				strike: body.strike,
				expiration: body.expiration,
				contract_type: body.contract_type,
				entry_alert_id: body.entry_alert_id,
				entry_price: body.entry_price,
				entry_date: body.entry_date || new Date().toISOString().split('T')[0],
				entry_tos_string: body.entry_tos_string,
				setup: body.setup,
				notes: body.notes
			})
		},
		'[Trades API]'
	);

	if (backendData) {
		return json({
			success: true,
			data: backendData,
			message: 'Trade created successfully',
			_source: 'backend'
		});
	}

	// R22-A: was: append a new trade to in-memory `mockTrades[slug]` and
	// return 200. Fake-success on a mutating endpoint — the trade was never
	// persisted and the linked entry alert pointed nowhere. Now: 502.
	console.error(`[Trades API] POST backend unavailable for slug '${slug}'`);
	return json(
		{
			success: false,
			error: 'Unable to create trade — backend is unavailable.'
		},
		{ status: 502 }
	);
};

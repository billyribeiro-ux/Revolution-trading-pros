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
import { env } from '$env/dynamic/private';
import type { Trade, TradeCreateInput, TradeStatus } from '$lib/types/trading';

// ═══════════════════════════════════════════════════════════════════════════
// BACKEND CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════

const BACKEND_URL = env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

// ═══════════════════════════════════════════════════════════════════════════
// BACKEND FETCH HELPER
// ═══════════════════════════════════════════════════════════════════════════

async function fetchFromBackend(endpoint: string, options: RequestInit = {}): Promise<any | null> {
	try {
		console.log(`[Trades API] Fetching: ${BACKEND_URL}${endpoint}`);
		const response = await fetch(`${BACKEND_URL}${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...options.headers
			}
		});

		if (!response.ok) {
			console.error(`[Trades API] Backend error: ${response.status} ${response.statusText}`);
			return null;
		}

		const data = await response.json();
		console.log(`[Trades API] Backend success:`, data?.data?.length || 0, 'items');
		return data;
	} catch (err) {
		console.error('[Trades API] Backend fetch failed:', err);
		return null;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// FALLBACK MOCK DATA
// ═══════════════════════════════════════════════════════════════════════════

const mockTrades: Record<string, Trade[]> = {
	'explosive-swings': [
		{
			id: 1,
			room_id: 4,
			room_slug: 'explosive-swings',
			ticker: 'MSFT',
			trade_type: 'shares',
			direction: 'long',
			quantity: 100,
			option_type: null,
			strike: null,
			expiration: null,
			entry_alert_id: null,
			entry_price: 425.0,
			entry_date: '2026-01-05',
			exit_alert_id: null,
			exit_price: 460.0,
			exit_date: '2026-01-10',
			setup: 'Breakout',
			status: 'closed',
			pnl: 3500,
			pnl_percent: 8.24,
			holding_days: 5,
			notes: 'Perfect breakout setup.',
			created_at: new Date().toISOString(),
			updated_at: new Date().toISOString()
		}
	]
};

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
		throw error(400, 'Room slug is required');
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
	const backendData = await fetchFromBackend(
		`/api/room-content/rooms/${slug}/trades?${backendParams.toString()}`,
		{ headers }
	);

	if (backendData?.data) {
		const trades = backendData.data;
		const stats = calculateStats(trades);

		return json({
			success: true,
			data: trades,
			stats,
			total: backendData.meta?.total || trades.length,
			page: parseInt(page),
			limit: parseInt(perPage),
			_source: 'backend'
		});
	}

	// Fallback to mock data
	console.log(`[Trades API] Using mock data for ${slug}`);
	let trades = mockTrades[slug] || [];

	// Filter by status
	if (status && status !== 'all') {
		trades = trades.filter((t) => t.status === status);
	}

	// Filter by ticker
	if (ticker) {
		trades = trades.filter((t) => t.ticker.toUpperCase() === ticker.toUpperCase());
	}

	// Sort by entry date (newest first)
	trades.sort((a, b) => new Date(b.entry_date).getTime() - new Date(a.entry_date).getTime());

	// Calculate stats
	const stats = calculateStats(mockTrades[slug] || []);

	// Paginate
	const offset = (parseInt(page) - 1) * parseInt(perPage);
	const total = trades.length;
	trades = trades.slice(offset, offset + parseInt(perPage));

	return json({
		success: true,
		data: trades,
		stats,
		total,
		page: parseInt(page),
		limit: parseInt(perPage),
		_source: 'mock'
	});
};

// ═══════════════════════════════════════════════════════════════════════════
// POST - Create new trade (from entry alert)
// ═══════════════════════════════════════════════════════════════════════════

export const POST: RequestHandler = async ({ params, request, cookies }) => {
	const { slug } = params;
	const authHeader = request.headers.get('Authorization');
	const accessToken = cookies.get('rtp_access_token');

	if (!authHeader && !accessToken) {
		throw error(401, 'Authentication required');
	}

	if (!slug) {
		throw error(400, 'Room slug is required');
	}

	const body: TradeCreateInput = await request.json();

	// Validate required fields
	if (!body.ticker || !body.trade_type || !body.direction || !body.quantity || !body.entry_price) {
		throw error(400, 'Ticker, trade_type, direction, quantity, and entry_price are required');
	}

	// Build headers - use rtp_access_token cookie for Bearer auth
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (accessToken) {
		headers['Authorization'] = `Bearer ${accessToken}`;
	}

	// Call backend at /api/admin/room-content/trades
	const backendData = await fetchFromBackend(`/api/admin/room-content/trades`, {
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
	});

	if (backendData) {
		return json({
			success: true,
			data: backendData,
			message: 'Trade created successfully',
			_source: 'backend'
		});
	}

	// Mock create fallback
	const trades = mockTrades[slug] || [];
	const maxId = trades.reduce((max, t) => Math.max(max, t.id), 0);

	const newTrade: Trade = {
		id: maxId + 1,
		room_id: 4,
		room_slug: slug,
		ticker: body.ticker.toUpperCase(),
		trade_type: body.trade_type,
		direction: body.direction,
		quantity: body.quantity,
		option_type: body.option_type || null,
		strike: body.strike || null,
		expiration: body.expiration || null,
		entry_alert_id: body.entry_alert_id || null,
		entry_price: body.entry_price,
		entry_date: body.entry_date || new Date().toISOString().split('T')[0],
		exit_alert_id: null,
		exit_price: null,
		exit_date: null,
		setup: body.setup || null,
		status: 'open',
		pnl: null,
		pnl_percent: null,
		holding_days: null,
		notes: body.notes || null,
		created_at: new Date().toISOString(),
		updated_at: new Date().toISOString()
	};

	if (!mockTrades[slug]) {
		mockTrades[slug] = [];
	}
	mockTrades[slug].unshift(newTrade);

	return json({
		success: true,
		data: newTrade,
		message: 'Trade created successfully',
		_source: 'mock'
	});
};

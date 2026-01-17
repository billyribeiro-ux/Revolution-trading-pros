/**
 * Trades API - Trade history for trading rooms
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/trades/[room-slug] - List trades for a room
 * POST /api/trades/[room-slug] - Create new trade (from entry alert)
 *
 * @version 1.0.0
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import type { Trade, TradeCreateInput, TradeStatus } from '$lib/types/trading';

// ═══════════════════════════════════════════════════════════════════════════
// MOCK DATA - Matches Trade Tracker page
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
			notes: 'Perfect breakout setup. Held through consolidation and exited at T2.',
			created_at: '2026-01-05T10:00:00Z',
			updated_at: '2026-01-10T15:45:00Z'
		},
		{
			id: 2,
			room_id: 4,
			room_slug: 'explosive-swings',
			ticker: 'META',
			trade_type: 'options',
			direction: 'long',
			quantity: 2,
			option_type: 'CALL',
			strike: 590,
			expiration: '2026-01-24',
			entry_alert_id: 4,
			entry_price: 12.5,
			entry_date: '2026-01-08',
			exit_alert_id: null,
			exit_price: 18.75,
			exit_date: '2026-01-12',
			setup: 'Momentum',
			status: 'closed',
			pnl: 1250,
			pnl_percent: 50.0,
			holding_days: 4,
			notes: 'Strong momentum play. Quick move to target.',
			created_at: '2026-01-08T11:20:00Z',
			updated_at: '2026-01-12T14:30:00Z'
		},
		{
			id: 3,
			room_id: 4,
			room_slug: 'explosive-swings',
			ticker: 'AMD',
			trade_type: 'shares',
			direction: 'short',
			quantity: 200,
			option_type: null,
			strike: null,
			expiration: null,
			entry_alert_id: 5,
			entry_price: 125.0,
			entry_date: '2026-01-09',
			exit_alert_id: null,
			exit_price: 127.0,
			exit_date: '2026-01-11',
			setup: 'Reversal',
			status: 'closed',
			pnl: -400,
			pnl_percent: -1.6,
			holding_days: 2,
			notes: "Stopped out. Reversal didn't materialize.",
			created_at: '2026-01-09T14:30:00Z',
			updated_at: '2026-01-11T10:15:00Z'
		},
		{
			id: 4,
			room_id: 4,
			room_slug: 'explosive-swings',
			ticker: 'NFLX',
			trade_type: 'shares',
			direction: 'long',
			quantity: 60,
			option_type: null,
			strike: null,
			expiration: null,
			entry_alert_id: null,
			entry_price: 520.0,
			entry_date: '2026-01-02',
			exit_alert_id: null,
			exit_price: 570.0,
			exit_date: '2026-01-09',
			setup: 'Earnings',
			status: 'closed',
			pnl: 3000,
			pnl_percent: 9.62,
			holding_days: 7,
			notes: 'Earnings catalyst worked perfectly. Held for runner target.',
			created_at: '2026-01-02T09:35:00Z',
			updated_at: '2026-01-09T15:00:00Z'
		},
		{
			id: 5,
			room_id: 4,
			room_slug: 'explosive-swings',
			ticker: 'NVDA',
			trade_type: 'shares',
			direction: 'long',
			quantity: 150,
			option_type: null,
			strike: null,
			expiration: null,
			entry_alert_id: 1,
			entry_price: 142.5,
			entry_date: '2026-01-13',
			exit_alert_id: null,
			exit_price: null,
			exit_date: null,
			setup: 'Breakout',
			status: 'open',
			pnl: null,
			pnl_percent: null,
			holding_days: null,
			notes: 'Currently holding. Watching for T1 at $148.',
			created_at: '2026-01-13T10:32:00Z',
			updated_at: '2026-01-13T10:32:00Z'
		},
		{
			id: 6,
			room_id: 4,
			room_slug: 'explosive-swings',
			ticker: 'TSLA',
			trade_type: 'shares',
			direction: 'long',
			quantity: 80,
			option_type: null,
			strike: null,
			expiration: null,
			entry_alert_id: null,
			entry_price: 235.0,
			entry_date: '2025-12-28',
			exit_alert_id: null,
			exit_price: 265.0,
			exit_date: '2026-01-05',
			setup: 'Momentum',
			status: 'closed',
			pnl: 2400,
			pnl_percent: 12.77,
			holding_days: 8,
			notes: 'Great momentum trade. Hit T3 and exited.',
			created_at: '2025-12-28T10:15:00Z',
			updated_at: '2026-01-05T14:20:00Z'
		},
		{
			id: 7,
			room_id: 4,
			room_slug: 'explosive-swings',
			ticker: 'AMZN',
			trade_type: 'shares',
			direction: 'long',
			quantity: 100,
			option_type: null,
			strike: null,
			expiration: null,
			entry_alert_id: null,
			entry_price: 185.0,
			entry_date: '2026-01-11',
			exit_alert_id: null,
			exit_price: null,
			exit_date: null,
			setup: 'Breakout',
			status: 'open',
			pnl: null,
			pnl_percent: null,
			holding_days: null,
			notes: 'Active position. Currently at T1.',
			created_at: '2026-01-11T11:15:00Z',
			updated_at: '2026-01-11T11:15:00Z'
		},
		{
			id: 8,
			room_id: 4,
			room_slug: 'explosive-swings',
			ticker: 'GOOGL',
			trade_type: 'shares',
			direction: 'long',
			quantity: 120,
			option_type: null,
			strike: null,
			expiration: null,
			entry_alert_id: null,
			entry_price: 168.0,
			entry_date: '2025-12-20',
			exit_alert_id: null,
			exit_price: 175.0,
			exit_date: '2025-12-30',
			setup: 'Momentum',
			status: 'closed',
			pnl: 840,
			pnl_percent: 4.17,
			holding_days: 10,
			notes: 'Slow grind higher. Took profits at T1.',
			created_at: '2025-12-20T09:45:00Z',
			updated_at: '2025-12-30T15:30:00Z'
		}
	]
};

// ═══════════════════════════════════════════════════════════════════════════
// BACKEND FETCH
// ═══════════════════════════════════════════════════════════════════════════

async function fetchFromBackend(endpoint: string, options: RequestInit = {}): Promise<any | null> {
	const BACKEND_URL = env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

	try {
		const response = await fetch(`${BACKEND_URL}${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...options.headers
			}
		});

		if (!response.ok) return null;
		return await response.json();
	} catch {
		return null;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// GET - List trades for a room
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, url, request }) => {
	const { slug } = params;

	if (!slug) {
		throw error(400, 'Room slug is required');
	}

	// Query params
	const status = url.searchParams.get('status') as TradeStatus | 'all' | null;
	const ticker = url.searchParams.get('ticker');
	const limit = parseInt(url.searchParams.get('limit') || '50', 10);
	const offset = parseInt(url.searchParams.get('offset') || '0', 10);

	// Try backend first
	const authHeader = request.headers.get('Authorization');
	const headers: Record<string, string> = {};
	if (authHeader) headers['Authorization'] = authHeader;

	const backendData = await fetchFromBackend(`/api/trades/${slug}?${url.searchParams.toString()}`, {
		headers
	});

	if (backendData?.success) {
		return json(backendData);
	}

	// Fallback to mock data
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

	// Calculate stats from trades
	const closedTrades = (mockTrades[slug] || []).filter((t) => t.status === 'closed');
	const openTrades = (mockTrades[slug] || []).filter((t) => t.status === 'open');
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

	const stats = {
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

	// Paginate
	const total = trades.length;
	trades = trades.slice(offset, offset + limit);

	return json({
		success: true,
		data: trades,
		stats,
		total,
		limit,
		offset,
		_mock: true
	});
};

// ═══════════════════════════════════════════════════════════════════════════
// POST - Create new trade (from entry alert)
// ═══════════════════════════════════════════════════════════════════════════

export const POST: RequestHandler = async ({ params, request }) => {
	const { slug } = params;
	const authHeader = request.headers.get('Authorization');

	if (!authHeader) {
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

	// Try backend first
	const backendData = await fetchFromBackend(`/api/trades/${slug}`, {
		method: 'POST',
		headers: { Authorization: authHeader },
		body: JSON.stringify(body)
	});

	if (backendData?.success) {
		return json(backendData);
	}

	// Mock create
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

	// Add to mock data
	if (!mockTrades[slug]) {
		mockTrades[slug] = [];
	}
	mockTrades[slug].unshift(newTrade);

	return json({
		success: true,
		data: newTrade,
		message: 'Trade created successfully',
		_mock: true
	});
};

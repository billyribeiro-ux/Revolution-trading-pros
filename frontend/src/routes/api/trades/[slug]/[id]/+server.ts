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
import { env } from '$env/dynamic/private';
import type { Trade, TradeUpdateInput } from '$lib/types/trading';

// Reference to mock data from parent route
// In production, this would come from database
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
// UTILITY FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

function calculatePnL(
	direction: 'long' | 'short',
	tradeType: 'shares' | 'options',
	quantity: number,
	entryPrice: number,
	exitPrice: number
): { pnl: number; pnl_percent: number } {
	let pnl: number;

	if (direction === 'long') {
		// Long: profit when exit > entry
		pnl = (exitPrice - entryPrice) * quantity;
	} else {
		// Short: profit when exit < entry
		pnl = (entryPrice - exitPrice) * quantity;
	}

	// For options, multiply by contract size (100 shares per contract)
	if (tradeType === 'options') {
		pnl *= 100;
	}

	const pnl_percent =
		((exitPrice - entryPrice) / entryPrice) * 100 * (direction === 'short' ? -1 : 1);

	return {
		pnl: Math.round(pnl * 100) / 100,
		pnl_percent: Math.round(pnl_percent * 100) / 100
	};
}

function calculateHoldingDays(entryDate: string, exitDate: string): number {
	const entry = new Date(entryDate);
	const exit = new Date(exitDate);
	const diffTime = Math.abs(exit.getTime() - entry.getTime());
	return Math.ceil(diffTime / (1000 * 60 * 60 * 24));
}

// ═══════════════════════════════════════════════════════════════════════════
// GET - Get single trade
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, request, cookies }) => {
	const { slug, id } = params;

	if (!slug || !id) {
		throw error(400, 'Room slug and trade ID are required');
	}

	const tradeId = parseInt(id, 10);
	if (isNaN(tradeId)) {
		throw error(400, 'Invalid trade ID');
	}

	// Try backend first
	const authHeader = request.headers.get('Authorization');
	const sessionCookie = cookies.get('session');
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (sessionCookie) {
		headers['Cookie'] = `session=${sessionCookie}`;
	}

	const backendData = await fetchFromBackend(`/api/room-content/rooms/${slug}/trades/${id}`, { headers });

	if (backendData?.success) {
		return json(backendData);
	}

	// Fallback to mock data
	const trades = mockTrades[slug] || [];
	const trade = trades.find((t) => t.id === tradeId);

	if (!trade) {
		throw error(404, `Trade ${id} not found`);
	}

	return json({
		success: true,
		data: trade,
		_mock: true
	});
};

// ═══════════════════════════════════════════════════════════════════════════
// PUT - Update trade / Close trade
// ═══════════════════════════════════════════════════════════════════════════

export const PUT: RequestHandler = async ({ params, request, cookies }) => {
	const { slug, id } = params;
	const authHeader = request.headers.get('Authorization');
	const sessionCookie = cookies.get('session');

	if (!authHeader && !sessionCookie) {
		throw error(401, 'Authentication required');
	}

	if (!slug || !id) {
		throw error(400, 'Room slug and trade ID are required');
	}

	const tradeId = parseInt(id, 10);
	if (isNaN(tradeId)) {
		throw error(400, 'Invalid trade ID');
	}

	const body: TradeUpdateInput = await request.json();

	// Build headers
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (sessionCookie) {
		headers['Cookie'] = `session=${sessionCookie}`;
	}

	// Determine if this is a close action
	const isCloseRequest = body.status === 'closed' || 
		(body.exit_price !== undefined && body.exit_price !== null);

	// Try backend first - use /close endpoint for closing trades
	if (isCloseRequest) {
		const backendData = await fetchFromBackend(`/api/admin/room-content/trades/${id}/close`, {
			method: 'PUT',
			headers,
			body: JSON.stringify({
				exit_price: body.exit_price,
				exit_date: body.exit_date || new Date().toISOString().split('T')[0],
				exit_tos_string: body.exit_tos_string,
				notes: body.notes
			})
		});

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
		const backendData = await fetchFromBackend(`/api/admin/room-content/trades/${id}`, {
			method: 'PUT',
			headers,
			body: JSON.stringify(body)
		});

		if (backendData?.success) {
			return json(backendData);
		}
	}

	// Fallback to mock update
	const trades = mockTrades[slug] || [];
	const tradeIndex = trades.findIndex((t) => t.id === tradeId);

	if (tradeIndex === -1) {
		throw error(404, `Trade ${id} not found`);
	}

	const existingTrade = trades[tradeIndex];

	// Determine if this is a close action
	const isClosing =
		body.status === 'closed' ||
		(body.exit_price !== undefined && body.exit_price !== null && existingTrade.status === 'open');

	let pnl = existingTrade.pnl;
	let pnl_percent = existingTrade.pnl_percent;
	let holding_days = existingTrade.holding_days;

	// Calculate P&L if closing
	if (isClosing && body.exit_price) {
		const exitPrice = body.exit_price;
		const calculation = calculatePnL(
			existingTrade.direction,
			existingTrade.trade_type,
			existingTrade.quantity,
			existingTrade.entry_price,
			exitPrice
		);
		pnl = calculation.pnl;
		pnl_percent = calculation.pnl_percent;

		const exitDate = body.exit_date || new Date().toISOString().split('T')[0];
		holding_days = calculateHoldingDays(existingTrade.entry_date, exitDate);
	}

	// Update trade
	const updatedTrade: Trade = {
		...existingTrade,
		exit_alert_id: body.exit_alert_id ?? existingTrade.exit_alert_id,
		exit_price: body.exit_price ?? existingTrade.exit_price,
		exit_date:
			body.exit_date ??
			(isClosing ? new Date().toISOString().split('T')[0] : existingTrade.exit_date),
		status: isClosing ? 'closed' : (body.status ?? existingTrade.status),
		pnl,
		pnl_percent,
		holding_days,
		notes: body.notes ?? existingTrade.notes,
		updated_at: new Date().toISOString()
	};

	// Update in mock data
	mockTrades[slug][tradeIndex] = updatedTrade;

	return json({
		success: true,
		data: updatedTrade,
		message: isClosing ? 'Trade closed successfully' : 'Trade updated successfully',
		_mock: true
	});
};

// ═══════════════════════════════════════════════════════════════════════════
// DELETE - Delete trade (admin only)
// ═══════════════════════════════════════════════════════════════════════════

export const DELETE: RequestHandler = async ({ params, request, cookies }) => {
	const { slug, id } = params;
	const authHeader = request.headers.get('Authorization');
	const sessionCookie = cookies.get('session');

	if (!authHeader && !sessionCookie) {
		throw error(401, 'Authentication required');
	}

	if (!slug || !id) {
		throw error(400, 'Room slug and trade ID are required');
	}

	const tradeId = parseInt(id, 10);
	if (isNaN(tradeId)) {
		throw error(400, 'Invalid trade ID');
	}

	// Build headers
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (sessionCookie) {
		headers['Cookie'] = `session=${sessionCookie}`;
	}

	// Try backend first
	const backendData = await fetchFromBackend(`/api/admin/room-content/trades/${id}`, {
		method: 'DELETE',
		headers
	});

	if (backendData?.success) {
		return json(backendData);
	}

	// Fallback to mock delete
	const trades = mockTrades[slug] || [];
	const tradeIndex = trades.findIndex((t) => t.id === tradeId);

	if (tradeIndex === -1) {
		throw error(404, `Trade ${id} not found`);
	}

	// Remove from mock data
	mockTrades[slug].splice(tradeIndex, 1);

	return json({
		success: true,
		message: `Trade ${id} deleted successfully`,
		_mock: true
	});
};

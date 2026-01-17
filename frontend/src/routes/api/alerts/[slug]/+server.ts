/**
 * Alerts API - Room-specific trading alerts
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/alerts/[room-slug] - List alerts for a room
 * POST /api/alerts/[room-slug] - Create new alert (admin)
 *
 * @version 1.0.0
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import type { RoomAlert, AlertCreateInput, AlertType } from '$lib/types/trading';
import { buildTosString, validateTosParams } from '$lib/utils/tos-builder';

// ═══════════════════════════════════════════════════════════════════════════
// MOCK DATA - Matches Explosive Swings alerts
// ═══════════════════════════════════════════════════════════════════════════

const mockAlerts: Record<string, RoomAlert[]> = {
	'explosive-swings': [
		{
			id: 1,
			room_id: 4,
			room_slug: 'explosive-swings',
			alert_type: 'ENTRY',
			ticker: 'NVDA',
			title: 'Opening NVDA Swing Position',
			message:
				'Entering NVDA at $142.50. First target $148, stop at $136. See trade plan for full details.',
			notes:
				'Entry based on breakout above $142 resistance with strong volume confirmation. RSI at 62 showing momentum. Watch for pullback to $140 support if entry missed. Position size: 150 shares. Risk/reward: 2.8:1 to T2.',
			trade_type: 'shares',
			action: 'BUY',
			quantity: 150,
			option_type: null,
			strike: null,
			expiration: null,
			contract_type: null,
			order_type: 'LMT',
			limit_price: 142.5,
			fill_price: 142.5,
			tos_string: 'BUY +150 NVDA @142.50 LMT',
			entry_alert_id: null,
			trade_plan_id: 1,
			is_new: true,
			is_published: true,
			is_pinned: false,
			published_at: '2026-01-17T10:32:00Z',
			created_at: '2026-01-17T10:32:00Z',
			updated_at: '2026-01-17T10:32:00Z'
		},
		{
			id: 2,
			room_id: 4,
			room_slug: 'explosive-swings',
			alert_type: 'UPDATE',
			ticker: 'TSLA',
			title: 'TSLA Approaching Entry Zone',
			message: 'TSLA pulling back to our entry zone. Be ready. Will alert when triggered.',
			notes:
				'Watching $248 entry level closely. Pullback is healthy after recent run. Volume declining on pullback (bullish). If entry triggers, will send immediate alert with exact entry price and position sizing.',
			trade_type: null,
			action: null,
			quantity: null,
			option_type: null,
			strike: null,
			expiration: null,
			contract_type: null,
			order_type: null,
			limit_price: null,
			fill_price: null,
			tos_string: null,
			entry_alert_id: null,
			trade_plan_id: 2,
			is_new: true,
			is_published: true,
			is_pinned: false,
			published_at: '2026-01-17T09:15:00Z',
			created_at: '2026-01-17T09:15:00Z',
			updated_at: '2026-01-17T09:15:00Z'
		},
		{
			id: 3,
			room_id: 4,
			room_slug: 'explosive-swings',
			alert_type: 'EXIT',
			ticker: 'MSFT',
			title: 'Closing MSFT for +8.2%',
			message: 'Taking profits on MSFT. Hit second target. +$2,450 on this trade.',
			notes:
				'Excellent trade execution. Entered at $425, scaled out 1/3 at T1 ($435), another 1/3 at T2 ($445). Final exit at $460. Held for 5 days. Key lesson: Patience paid off - almost exited early on day 3 consolidation.',
			trade_type: 'shares',
			action: 'SELL',
			quantity: 100,
			option_type: null,
			strike: null,
			expiration: null,
			contract_type: null,
			order_type: 'LMT',
			limit_price: 460.0,
			fill_price: 460.0,
			tos_string: 'SELL -100 MSFT @460.00 LMT',
			entry_alert_id: null,
			trade_plan_id: null,
			is_new: false,
			is_published: true,
			is_pinned: false,
			published_at: '2026-01-16T15:45:00Z',
			created_at: '2026-01-16T15:45:00Z',
			updated_at: '2026-01-16T15:45:00Z'
		},
		{
			id: 4,
			room_id: 4,
			room_slug: 'explosive-swings',
			alert_type: 'ENTRY',
			ticker: 'META',
			title: 'META Entry Triggered',
			message: 'META hit our entry at $585. Position active. Targets in trade plan.',
			notes:
				'Entry confirmed at $585 with volume spike. Stop placed at $565 (3.4% risk). Currently up 1.3% and holding well. Momentum strong with AI revenue narrative. Will trail stop after T1 hit.',
			trade_type: 'options',
			action: 'BUY',
			quantity: 2,
			option_type: 'CALL',
			strike: 590,
			expiration: '2026-01-24',
			contract_type: 'Weeklys',
			order_type: 'LMT',
			limit_price: 12.5,
			fill_price: 12.5,
			tos_string: 'BUY +2 META 100 (Weeklys) 24 JAN 26 590 CALL @12.50 LMT',
			entry_alert_id: null,
			trade_plan_id: 5,
			is_new: false,
			is_published: true,
			is_pinned: false,
			published_at: '2026-01-16T11:20:00Z',
			created_at: '2026-01-16T11:20:00Z',
			updated_at: '2026-01-16T11:20:00Z'
		},
		{
			id: 5,
			room_id: 4,
			room_slug: 'explosive-swings',
			alert_type: 'UPDATE',
			ticker: 'AMD',
			title: 'AMD Short Setup Active',
			message: 'Bearish setup triggered on AMD. Short at $125 with stop at $132.',
			notes:
				'Bearish breakdown confirmed. Entered short at $125, currently at $123.50 (-1.2%). Stop at $132 gives us 5.6% risk. First target $120, second target $115. Watch for bounce at $120 psychological level.',
			trade_type: 'shares',
			action: 'SELL',
			quantity: 200,
			option_type: null,
			strike: null,
			expiration: null,
			contract_type: null,
			order_type: 'LMT',
			limit_price: 125.0,
			fill_price: 125.0,
			tos_string: 'SELL -200 AMD @125.00 LMT',
			entry_alert_id: null,
			trade_plan_id: 6,
			is_new: false,
			is_published: true,
			is_pinned: false,
			published_at: '2026-01-10T14:30:00Z',
			created_at: '2026-01-10T14:30:00Z',
			updated_at: '2026-01-10T14:30:00Z'
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
// GET - List alerts for a room
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, url, request }) => {
	const { slug } = params;

	if (!slug) {
		throw error(400, 'Room slug is required');
	}

	// Query params
	const alertType = url.searchParams.get('alert_type') as AlertType | 'all' | null;
	const ticker = url.searchParams.get('ticker');
	const limit = parseInt(url.searchParams.get('limit') || '50', 10);
	const offset = parseInt(url.searchParams.get('offset') || '0', 10);

	// Try backend first
	const authHeader = request.headers.get('Authorization');
	const headers: Record<string, string> = {};
	if (authHeader) headers['Authorization'] = authHeader;

	const backendData = await fetchFromBackend(`/api/alerts/${slug}?${url.searchParams.toString()}`, {
		headers
	});

	if (backendData?.success) {
		return json(backendData);
	}

	// Fallback to mock data
	let alerts = mockAlerts[slug] || [];

	// Filter by alert type
	if (alertType && alertType !== 'all') {
		alerts = alerts.filter((a) => a.alert_type === alertType);
	}

	// Filter by ticker
	if (ticker) {
		alerts = alerts.filter((a) => a.ticker.toUpperCase() === ticker.toUpperCase());
	}

	// Filter published only for non-admin requests
	if (!authHeader) {
		alerts = alerts.filter((a) => a.is_published);
	}

	// Sort by published date (newest first)
	alerts.sort((a, b) => new Date(b.published_at).getTime() - new Date(a.published_at).getTime());

	// Paginate
	const total = alerts.length;
	alerts = alerts.slice(offset, offset + limit);

	return json({
		success: true,
		data: alerts,
		total,
		limit,
		offset,
		_mock: true
	});
};

// ═══════════════════════════════════════════════════════════════════════════
// POST - Create new alert (admin only)
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

	const body: AlertCreateInput = await request.json();

	// Validate required fields
	if (!body.alert_type || !body.ticker || !body.title || !body.message) {
		throw error(400, 'Alert type, ticker, title, and message are required');
	}

	// Build TOS string if trade details provided
	let tosString: string | null = null;
	if (body.trade_type && body.action && body.quantity && body.order_type) {
		const tosParams = {
			trade_type: body.trade_type,
			action: body.action,
			quantity: body.quantity,
			ticker: body.ticker,
			option_type: body.option_type,
			strike: body.strike,
			expiration: body.expiration,
			contract_type: body.contract_type,
			order_type: body.order_type,
			limit_price: body.limit_price
		};

		const errors = validateTosParams(tosParams);
		if (errors.length > 0) {
			throw error(400, `Invalid trade details: ${errors.join(', ')}`);
		}

		tosString = buildTosString(tosParams);
	}

	// Try backend first
	const backendData = await fetchFromBackend(`/api/alerts/${slug}`, {
		method: 'POST',
		headers: { Authorization: authHeader },
		body: JSON.stringify({ ...body, tos_string: tosString })
	});

	if (backendData?.success) {
		return json(backendData);
	}

	// Mock create
	const alerts = mockAlerts[slug] || [];
	const maxId = alerts.reduce((max, a) => Math.max(max, a.id), 0);

	const newAlert: RoomAlert = {
		id: maxId + 1,
		room_id: 4,
		room_slug: slug,
		alert_type: body.alert_type,
		ticker: body.ticker.toUpperCase(),
		title: body.title,
		message: body.message,
		notes: body.notes || null,
		trade_type: body.trade_type || null,
		action: body.action || null,
		quantity: body.quantity || null,
		option_type: body.option_type || null,
		strike: body.strike || null,
		expiration: body.expiration || null,
		contract_type: body.contract_type || null,
		order_type: body.order_type || null,
		limit_price: body.limit_price || null,
		fill_price: body.fill_price || body.limit_price || null,
		tos_string: tosString,
		entry_alert_id: body.entry_alert_id || null,
		trade_plan_id: body.trade_plan_id || null,
		is_new: true,
		is_published: true,
		is_pinned: body.is_pinned || false,
		published_at: new Date().toISOString(),
		created_at: new Date().toISOString(),
		updated_at: new Date().toISOString()
	};

	// Add to mock data
	if (!mockAlerts[slug]) {
		mockAlerts[slug] = [];
	}
	mockAlerts[slug].unshift(newAlert);

	return json({
		success: true,
		data: newAlert,
		message: 'Alert created successfully',
		_mock: true
	});
};

/**
 * Alerts API - Room-specific trading alerts
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/alerts/[room-slug] - List alerts for a room
 * POST /api/alerts/[room-slug] - Create new alert (admin)
 *
 * Connects to backend at /api/room-content/rooms/:slug/alerts
 *
 * @version 2.0.0 - ICT 11 Principal Engineer Grade
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import type { RoomAlert, AlertCreateInput } from '$lib/types/trading';
import { buildTosString, validateTosParams } from '$lib/utils/tos-builder';

// ═══════════════════════════════════════════════════════════════════════════
// BACKEND CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════

const BACKEND_URL = env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

// ═══════════════════════════════════════════════════════════════════════════
// BACKEND FETCH HELPER
// ═══════════════════════════════════════════════════════════════════════════

async function fetchFromBackend(endpoint: string, options: RequestInit = {}): Promise<any | null> {
	try {
		console.log(`[Alerts API] Fetching: ${BACKEND_URL}${endpoint}`);
		const response = await fetch(`${BACKEND_URL}${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...options.headers
			}
		});

		if (!response.ok) {
			console.error(`[Alerts API] Backend error: ${response.status} ${response.statusText}`);
			return null;
		}

		const data = await response.json();
		console.log(`[Alerts API] Backend success:`, data?.data?.length || 0, 'items');
		return data;
	} catch (err) {
		console.error('[Alerts API] Backend fetch failed:', err);
		return null;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// FALLBACK MOCK DATA (only used when backend unavailable)
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
			message: 'Entering NVDA at $142.50. First target $148, stop at $136.',
			notes: 'Entry based on breakout above $142 resistance with strong volume confirmation.',
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
			published_at: new Date().toISOString(),
			created_at: new Date().toISOString(),
			updated_at: new Date().toISOString()
		}
	]
};

// ═══════════════════════════════════════════════════════════════════════════
// GET - List alerts for a room
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, url, request, cookies }) => {
	const { slug } = params;

	if (!slug) {
		error(400, 'Room slug is required');
	}

	// Build query params
	const page = url.searchParams.get('page') || '1';
	const perPage = url.searchParams.get('per_page') || url.searchParams.get('limit') || '50';
	const alertType = url.searchParams.get('alert_type');
	const ticker = url.searchParams.get('ticker');

	// Get auth headers - ICT 7: Use rtp_access_token cookie for Bearer auth (consistent with auth store)
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

	// Call backend at /api/room-content/rooms/:slug/alerts
	const backendData = await fetchFromBackend(
		`/api/room-content/rooms/${slug}/alerts?${backendParams.toString()}`,
		{ headers }
	);

	if (backendData?.data) {
		// Transform backend response to frontend format
		let alerts = backendData.data;

		// Apply client-side filters if needed
		if (alertType && alertType !== 'all') {
			alerts = alerts.filter((a: RoomAlert) => a.alert_type === alertType);
		}
		if (ticker) {
			alerts = alerts.filter((a: RoomAlert) => a.ticker.toUpperCase() === ticker.toUpperCase());
		}

		return json({
			success: true,
			data: alerts,
			total: backendData.meta?.total || alerts.length,
			page: parseInt(page),
			limit: parseInt(perPage),
			_source: 'backend'
		});
	}

	// Fallback to mock data
	console.log(`[Alerts API] Using mock data for ${slug}`);
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
	const offset = (parseInt(page) - 1) * parseInt(perPage);
	const total = alerts.length;
	alerts = alerts.slice(offset, offset + parseInt(perPage));

	return json({
		success: true,
		data: alerts,
		total,
		page: parseInt(page),
		limit: parseInt(perPage),
		_source: 'mock'
	});
};

// ═══════════════════════════════════════════════════════════════════════════
// POST - Create new alert (admin only)
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

	const body: AlertCreateInput = await request.json();

	// Validate required fields
	if (!body.alert_type || !body.ticker || !body.title || !body.message) {
		error(400, 'Alert type, ticker, title, and message are required');
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
			error(400, `Invalid trade details: ${errors.join(', ')}`);
		}

		tosString = buildTosString(tosParams);
	}

	// Build headers - ICT 7: Use rtp_access_token cookie for Bearer auth
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (accessToken) {
		headers['Authorization'] = `Bearer ${accessToken}`;
	}

	// Call backend at /api/admin/room-content/alerts
	const backendData = await fetchFromBackend(`/api/admin/room-content/alerts`, {
		method: 'POST',
		headers,
		body: JSON.stringify({
			room_slug: slug,
			alert_type: body.alert_type,
			ticker: body.ticker.toUpperCase(),
			title: body.title,
			message: body.message,
			notes: body.notes,
			trade_type: body.trade_type,
			action: body.action,
			quantity: body.quantity,
			option_type: body.option_type,
			strike: body.strike,
			expiration: body.expiration,
			contract_type: body.contract_type,
			order_type: body.order_type,
			limit_price: body.limit_price,
			fill_price: body.fill_price || body.limit_price,
			tos_string: tosString,
			entry_alert_id: body.entry_alert_id,
			trade_plan_id: body.trade_plan_id,
			is_pinned: body.is_pinned
		})
	});

	if (backendData) {
		return json({
			success: true,
			data: backendData,
			message: 'Alert created successfully',
			_source: 'backend'
		});
	}

	// Mock create fallback
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

	if (!mockAlerts[slug]) {
		mockAlerts[slug] = [];
	}
	mockAlerts[slug].unshift(newAlert);

	return json({
		success: true,
		data: newAlert,
		message: 'Alert created successfully',
		_source: 'mock'
	});
};

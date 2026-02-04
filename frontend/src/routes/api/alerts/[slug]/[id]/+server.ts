/**
 * Single Alert API - CRUD operations for individual alerts
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/alerts/[room-slug]/[id] - Get single alert
 * PUT /api/alerts/[room-slug]/[id] - Update alert (admin)
 * DELETE /api/alerts/[room-slug]/[id] - Delete alert (admin)
 *
 * Connects to backend at /api/admin/room-content/alerts/:id
 *
 * @version 2.0.0 - ICT 11 Principal Engineer Grade
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import type { RoomAlert, AlertUpdateInput } from '$lib/types/trading';
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
		const response = await fetch(`${BACKEND_URL}${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...options.headers
			}
		});

		if (!response.ok) {
			return null;
		}

		const data = await response.json();
		return data;
	} catch {
		return null;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// FALLBACK MOCK DATA
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
			notes: 'Entry based on breakout above $142 resistance.',
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
// GET - Get single alert
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, request, cookies }) => {
	const { slug, id } = params;

	if (!slug || !id) {
		throw error(400, 'Room slug and alert ID are required');
	}

	const alertId = parseInt(id, 10);
	if (isNaN(alertId)) {
		throw error(400, 'Invalid alert ID');
	}

	// Get auth headers
	const authHeader = request.headers.get('Authorization');
	const sessionCookie = cookies.get('session');
	const headers: Record<string, string> = {};

	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (sessionCookie) {
		headers['Cookie'] = `session=${sessionCookie}`;
	}

	// Try backend - note: there's no single alert GET endpoint in room_content.rs
	// So we fetch all and filter
	const backendData = await fetchFromBackend(`/api/room-content/rooms/${slug}/alerts`, { headers });

	if (backendData?.data) {
		const alert = backendData.data.find((a: RoomAlert) => a.id === alertId);
		if (alert) {
			return json({
				success: true,
				data: alert,
				_source: 'backend'
			});
		}
	}

	// Fallback to mock data
	const alerts = mockAlerts[slug] || [];
	const alert = alerts.find((a) => a.id === alertId);

	if (!alert) {
		throw error(404, `Alert ${id} not found`);
	}

	return json({
		success: true,
		data: alert,
		_source: 'mock'
	});
};

// ═══════════════════════════════════════════════════════════════════════════
// PUT - Update alert (admin only)
// ═══════════════════════════════════════════════════════════════════════════

export const PUT: RequestHandler = async ({ params, request, cookies }) => {
	const { slug, id } = params;
	const authHeader = request.headers.get('Authorization');
	const sessionCookie = cookies.get('session');

	if (!authHeader && !sessionCookie) {
		throw error(401, 'Authentication required');
	}

	if (!slug || !id) {
		throw error(400, 'Room slug and alert ID are required');
	}

	const alertId = parseInt(id, 10);
	if (isNaN(alertId)) {
		throw error(400, 'Invalid alert ID');
	}

	const body: AlertUpdateInput = await request.json();

	// Build headers
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (sessionCookie) {
		headers['Cookie'] = `session=${sessionCookie}`;
	}

	// Call backend at /api/admin/room-content/alerts/:id
	const backendData = await fetchFromBackend(`/api/admin/room-content/alerts/${id}`, {
		method: 'PUT',
		headers,
		body: JSON.stringify(body)
	});

	if (backendData) {
		return json({
			success: true,
			data: backendData,
			message: 'Alert updated successfully',
			_source: 'backend'
		});
	}

	// Fallback to mock update
	const alerts = mockAlerts[slug] || [];
	const alertIndex = alerts.findIndex((a) => a.id === alertId);

	if (alertIndex === -1) {
		throw error(404, `Alert ${id} not found`);
	}

	const existingAlert = alerts[alertIndex];

	// Rebuild TOS string if trade details changed
	let tosString = existingAlert.tos_string;
	const tradeType = body.trade_type ?? existingAlert.trade_type;
	const action = body.action ?? existingAlert.action;
	const quantity = body.quantity ?? existingAlert.quantity;
	const orderType = body.order_type ?? existingAlert.order_type;

	if (tradeType && action && quantity && orderType) {
		const tosParams = {
			trade_type: tradeType,
			action: action,
			quantity: quantity,
			ticker: body.ticker ?? existingAlert.ticker,
			option_type: body.option_type ?? existingAlert.option_type ?? undefined,
			strike: body.strike ?? existingAlert.strike ?? undefined,
			expiration: body.expiration ?? existingAlert.expiration ?? undefined,
			contract_type: body.contract_type ?? existingAlert.contract_type ?? undefined,
			order_type: orderType,
			limit_price: body.limit_price ?? existingAlert.limit_price ?? undefined
		};

		const errors = validateTosParams(tosParams);
		if (errors.length === 0) {
			tosString = buildTosString(tosParams);
		}
	}

	// Update alert
	const updatedAlert: RoomAlert = {
		...existingAlert,
		alert_type: body.alert_type ?? existingAlert.alert_type,
		ticker: body.ticker?.toUpperCase() ?? existingAlert.ticker,
		title: body.title ?? existingAlert.title,
		message: body.message ?? existingAlert.message,
		notes: body.notes ?? existingAlert.notes,
		trade_type: body.trade_type ?? existingAlert.trade_type,
		action: body.action ?? existingAlert.action,
		quantity: body.quantity ?? existingAlert.quantity,
		option_type: body.option_type ?? existingAlert.option_type,
		strike: body.strike ?? existingAlert.strike,
		expiration: body.expiration ?? existingAlert.expiration,
		contract_type: body.contract_type ?? existingAlert.contract_type,
		order_type: body.order_type ?? existingAlert.order_type,
		limit_price: body.limit_price ?? existingAlert.limit_price,
		entry_alert_id: body.entry_alert_id ?? existingAlert.entry_alert_id,
		is_new: body.is_new ?? existingAlert.is_new,
		is_published: body.is_published ?? existingAlert.is_published,
		is_pinned: body.is_pinned ?? existingAlert.is_pinned,
		tos_string: tosString,
		updated_at: new Date().toISOString()
	};

	mockAlerts[slug][alertIndex] = updatedAlert;

	return json({
		success: true,
		data: updatedAlert,
		message: 'Alert updated successfully',
		_source: 'mock'
	});
};

// ═══════════════════════════════════════════════════════════════════════════
// DELETE - Delete alert (admin only)
// ═══════════════════════════════════════════════════════════════════════════

export const DELETE: RequestHandler = async ({ params, request, cookies }) => {
	const { slug, id } = params;
	const authHeader = request.headers.get('Authorization');
	const sessionCookie = cookies.get('session');

	if (!authHeader && !sessionCookie) {
		throw error(401, 'Authentication required');
	}

	if (!slug || !id) {
		throw error(400, 'Room slug and alert ID are required');
	}

	const alertId = parseInt(id, 10);
	if (isNaN(alertId)) {
		throw error(400, 'Invalid alert ID');
	}

	// Build headers
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (sessionCookie) {
		headers['Cookie'] = `session=${sessionCookie}`;
	}

	// Call backend at /api/admin/room-content/alerts/:id
	const backendData = await fetchFromBackend(`/api/admin/room-content/alerts/${id}`, {
		method: 'DELETE',
		headers
	});

	if (backendData) {
		return json({
			success: true,
			message: `Alert ${id} deleted successfully`,
			_source: 'backend'
		});
	}

	// Fallback to mock delete
	const alerts = mockAlerts[slug] || [];
	const alertIndex = alerts.findIndex((a) => a.id === alertId);

	if (alertIndex === -1) {
		throw error(404, `Alert ${id} not found`);
	}

	mockAlerts[slug].splice(alertIndex, 1);

	return json({
		success: true,
		message: `Alert ${id} deleted successfully`,
		_source: 'mock'
	});
};

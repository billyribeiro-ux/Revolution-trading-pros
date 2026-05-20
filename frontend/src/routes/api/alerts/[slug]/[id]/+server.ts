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
import type { RoomAlert, AlertUpdateInput } from '$lib/types/trading';
import { buildTosString, validateTosParams } from '$lib/utils/tos-builder';
// R19-A: shared proxy helper — pins CLAUDE.md URL-fallback chain
// (API_BASE_URL || BACKEND_URL || localhost) AND replaces the
// `Promise<any | null>` helper with `Promise<unknown>` + narrowing guards.
import { fetchBackend, hasData, isObject } from '$lib/server/proxy-fetch';

// ═══════════════════════════════════════════════════════════════════════════
// TYPE GUARDS
// ═══════════════════════════════════════════════════════════════════════════

function isAlertWithId(value: unknown): value is RoomAlert {
	return isObject(value) && typeof value.id === 'number';
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
		error(400, 'Room slug and alert ID are required');
	}

	const alertId = parseInt(id, 10);
	if (isNaN(alertId)) {
		error(400, 'Invalid alert ID');
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
	const backendData = await fetchBackend(
		`/api/room-content/rooms/${slug}/alerts`,
		{ headers },
		'[Alert API]'
	);

	// R19-A: filter via isAlertWithId rather than `(a: RoomAlert)` cast —
	// the cast was a lie (the row could be `null` or a primitive). Find
	// returns `RoomAlert | undefined` correctly now.
	if (hasData(backendData) && Array.isArray(backendData.data)) {
		const alert = backendData.data.filter(isAlertWithId).find((a) => a.id === alertId);
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
		error(404, `Alert ${id} not found`);
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
		error(401, 'Authentication required');
	}

	if (!slug || !id) {
		error(400, 'Room slug and alert ID are required');
	}

	const alertId = parseInt(id, 10);
	if (isNaN(alertId)) {
		error(400, 'Invalid alert ID');
	}

	// R19-A: narrow body to non-null object before treating as
	// AlertUpdateInput. Surfaces a 400 instead of a 500 NPE if the client
	// POSTs `null`/primitive (R18-A Latent Bug §2 class).
	const rawBody: unknown = await request.json();
	if (!isObject(rawBody)) {
		error(400, 'Request body must be a JSON object');
	}
	const body = rawBody as AlertUpdateInput;

	// Build headers
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (sessionCookie) {
		headers['Cookie'] = `session=${sessionCookie}`;
	}

	// Call backend at /api/admin/room-content/alerts/:id
	const backendData = await fetchBackend(
		`/api/admin/room-content/alerts/${id}`,
		{
			method: 'PUT',
			headers,
			body: JSON.stringify(body)
		},
		'[Alert API]'
	);

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
		error(404, `Alert ${id} not found`);
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
		error(401, 'Authentication required');
	}

	if (!slug || !id) {
		error(400, 'Room slug and alert ID are required');
	}

	const alertId = parseInt(id, 10);
	if (isNaN(alertId)) {
		error(400, 'Invalid alert ID');
	}

	// Build headers
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (sessionCookie) {
		headers['Cookie'] = `session=${sessionCookie}`;
	}

	// Call backend at /api/admin/room-content/alerts/:id
	const backendData = await fetchBackend(
		`/api/admin/room-content/alerts/${id}`,
		{
			method: 'DELETE',
			headers
		},
		'[Alert API]'
	);

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
		error(404, `Alert ${id} not found`);
	}

	mockAlerts[slug].splice(alertIndex, 1);

	return json({
		success: true,
		message: `Alert ${id} deleted successfully`,
		_source: 'mock'
	});
};

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
import type { RoomAlert, AlertCreateInput } from '$lib/types/trading';
import { buildTosString, validateTosParams } from '$lib/utils/tos-builder';
// R19-A: shared proxy helper — pins CLAUDE.md URL-fallback chain
// (API_BASE_URL || BACKEND_URL || localhost) AND replaces the
// `Promise<any | null>` helper with `Promise<unknown>` + narrowing guards.
import { fetchBackend, hasData, isObject, extractMetaTotal } from '$lib/server/proxy-fetch';

// ═══════════════════════════════════════════════════════════════════════════
// TYPE GUARDS
// ═══════════════════════════════════════════════════════════════════════════

// R19-A Latent Bug guard: backend could (in principle) return a row with
// missing/null `alert_type` or `ticker`. The downstream `.ticker.toUpperCase()`
// access in the filter loop would NPE on such a row. Filter via this guard
// before reading fields.
function isRoomAlertLike(value: unknown): value is RoomAlert {
	if (!isObject(value)) return false;
	return (
		typeof value.id === 'number' &&
		typeof value.alert_type === 'string' &&
		typeof value.ticker === 'string'
	);
}

// ═══════════════════════════════════════════════════════════════════════════
// R22-A: Deleted `mockAlerts` (one fake NVDA entry alert for `explosive-swings`).
//   On backend failure the GET handler used to return that NVDA row and the
//   POST handler used to append a new mock alert to the in-memory map — the
//   latter is the worst kind: traders clicking "Create Alert" got a 200 with
//   `_source: 'mock'`, the row was lost on the next dyno cycle, AND no one
//   else in the room ever received the notification. Both paths now surface
//   a backend failure with the right status (502 for non-2xx, 500 for
//   network failure) so the alert form shows a real error and the trader
//   can retry against a working backend.
// ═══════════════════════════════════════════════════════════════════════════

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
	const backendData = await fetchBackend(
		`/api/room-content/rooms/${slug}/alerts?${backendParams.toString()}`,
		{ headers },
		'[Alerts API]'
	);

	// R19-A: narrow `unknown` via hasData() instead of `backendData?.data`
	// (which would throw on a non-null primitive backend response). Filter
	// to only entries that look like RoomAlert before applying the
	// downstream string comparisons — drops malformed rows silently rather
	// than 500-ing the whole list (R18-A Latent Bug §1 class).
	if (hasData(backendData) && Array.isArray(backendData.data)) {
		let alerts: RoomAlert[] = backendData.data.filter(isRoomAlertLike);

		// Apply client-side filters if needed
		if (alertType && alertType !== 'all') {
			alerts = alerts.filter((a) => a.alert_type === alertType);
		}
		if (ticker) {
			alerts = alerts.filter((a) => a.ticker.toUpperCase() === ticker.toUpperCase());
		}

		return json({
			success: true,
			data: alerts,
			total: extractMetaTotal(backendData, alerts.length),
			page: parseInt(page),
			limit: parseInt(perPage),
			_source: 'backend'
		});
	}

	// R22-A: was: fall back to in-memory `mockAlerts[slug]`, filter, sort,
	// paginate, return with `_source: 'mock'`. Now: surface the backend
	// failure as 502 so the alerts UI shows a real error rather than the
	// same single NVDA placeholder row forever.
	console.error(`[Alerts API] Backend unavailable for slug '${slug}'`);
	return json(
		{
			success: false,
			error: 'Unable to load alerts from backend. Please check the API connection.'
		},
		{ status: 502 }
	);
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

	// R19-A: SvelteKit's `request.json()` returns `Promise<any>`, narrow the
	// body to a non-null object BEFORE field reads to surface a 400 (rather
	// than a 500 NPE) if a client POSTs `null` or a primitive (R18-A Latent
	// Bug §2 class). The `AlertCreateInput` cast then documents the shape
	// we're assuming for the downstream field reads.
	const rawBody: unknown = await request.json();
	if (!isObject(rawBody)) {
		error(400, 'Request body must be a JSON object');
	}
	const body = rawBody as unknown as AlertCreateInput;

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
	const backendData = await fetchBackend(
		`/api/admin/room-content/alerts`,
		{
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
		},
		'[Alerts API]'
	);

	if (backendData) {
		return json({
			success: true,
			data: backendData,
			message: 'Alert created successfully',
			_source: 'backend'
		});
	}

	// R22-A: was: append a new alert to in-memory `mockAlerts[slug]` and
	// return 200 `_source: 'mock'`. That was the worst class of mock chain:
	// the admin saw "Alert created" while the row was never persisted and
	// no room member ever received the notification. The fake-success
	// poisoned the chain — entry_alert_ids referenced by trade rows pointed
	// at IDs that vanished on the next dyno cycle. Now: 502 so the form
	// surfaces a real failure.
	console.error(`[Alerts API] POST backend unavailable for slug '${slug}'`);
	return json(
		{
			success: false,
			error: 'Unable to create alert — backend is unavailable. Please try again.'
		},
		{ status: 502 }
	);
};

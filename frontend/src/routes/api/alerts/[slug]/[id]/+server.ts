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
// R19-A: shared proxy helper — pins CLAUDE.md URL-fallback chain
// (API_BASE_URL || BACKEND_URL || localhost) AND replaces the
// `Promise<any | null>` helper with `Promise<unknown>` + narrowing guards.
// R22-A: `buildTosString`/`validateTosParams` no longer imported — the
// mock-update path that rebuilt the TOS string client-side is gone.
import { fetchBackend, hasData, isObject } from '$lib/server/proxy-fetch';

// ═══════════════════════════════════════════════════════════════════════════
// TYPE GUARDS
// ═══════════════════════════════════════════════════════════════════════════

function isAlertWithId(value: unknown): value is RoomAlert {
	return isObject(value) && typeof value.id === 'number';
}

// ═══════════════════════════════════════════════════════════════════════════
// R22-A: Deleted `mockAlerts` (the parent route's NVDA placeholder). On
//   backend failure:
//     - GET fell back to the mock map by id and returned `_source: 'mock'`.
//     - PUT silently mutated the in-memory map, returning 200 while no row
//       was persisted (worst case: trader sees "Alert updated" but the
//       broadcast never fires).
//     - DELETE silently spliced the mock array, returning 200 with no DB
//       row touched.
//   All three now surface the upstream failure as 502 so the alerts UI
//   shows a real error instead of phantom success on mutating ops.
// ═══════════════════════════════════════════════════════════════════════════

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

	// R22-A: was: look up alertId in `mockAlerts[slug]` and return `_source: 'mock'`.
	// Now: surface backend failure as 502 — the alert genuinely cannot be retrieved.
	console.error(`[Alert API] GET backend unavailable for slug '${slug}' id '${id}'`);
	return json(
		{ success: false, error: 'Unable to load alert — backend is unavailable.' },
		{ status: 502 }
	);
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

	// R22-A: was: rebuild TOS string, mutate `mockAlerts[slug][alertIndex]`,
	// return `_source: 'mock'`. The mock write was lost on every dyno cycle
	// and never broadcast to room members — a silent fake-success on a
	// mutating endpoint. Surface the failure as 502 so the admin retries.
	console.error(`[Alert API] PUT backend unavailable for slug '${slug}' id '${id}'`);
	return json(
		{ success: false, error: 'Unable to update alert — backend is unavailable.' },
		{ status: 502 }
	);
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

	// R22-A: was: splice the in-memory `mockAlerts[slug]` and return 200.
	// Now: surface backend failure as 502 — the DB row is still there.
	console.error(`[Alert API] DELETE backend unavailable for slug '${slug}' id '${id}'`);
	return json(
		{ success: false, error: 'Unable to delete alert — backend is unavailable.' },
		{ status: 502 }
	);
};

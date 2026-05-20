/**
 * Trade Plans API - Single Entry Operations
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/trade-plans/[room-slug]/[id] - Get single trade plan entry
 * PUT /api/trade-plans/[room-slug]/[id] - Update trade plan entry
 * DELETE /api/trade-plans/[room-slug]/[id] - Delete trade plan entry
 *
 * Connects to backend at /api/admin/room-content/trade-plan/:id
 *
 * @version 1.0.0 - ICT 7 Principal Engineer Grade
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
// R19-A: shared proxy helper — pins CLAUDE.md URL-fallback chain
// (API_BASE_URL || BACKEND_URL || localhost) AND replaces the
// `Promise<any | null>` helper with `Promise<unknown>` + narrowing guards.
import { fetchBackend, isObject, extractBackendData } from '$lib/server/proxy-fetch';

interface TradePlanPutBody {
	ticker?: string;
	bias?: string;
	entry?: string;
	target1?: string;
	target2?: string;
	target3?: string;
	runner?: string;
	stop?: string;
	runner_stop?: string | null;
	options_strike?: string | null;
	options_exp?: string | null;
	notes?: string | null;
	sort_order?: number;
	is_active?: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════
// GET - Get single trade plan entry
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, request, cookies }) => {
	const { slug, id } = params;

	if (!slug || !id) {
		error(400, 'Room slug and trade plan ID are required');
	}

	const planId = parseInt(id, 10);
	if (isNaN(planId)) {
		error(400, 'Invalid trade plan ID');
	}

	// Get auth headers - use rtp_access_token cookie
	const authHeader = request.headers.get('Authorization');
	const accessToken = cookies.get('rtp_access_token');
	const headers: Record<string, string> = {};

	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (accessToken) {
		headers['Authorization'] = `Bearer ${accessToken}`;
	}

	const backendData = await fetchBackend(
		`/api/room-content/rooms/${slug}/trade-plan/${id}`,
		{ headers },
		'[Trade Plans API]'
	);

	if (backendData) {
		// R19-A: extractBackendData() preserves "envelope.data === null"
		// semantics (returns null, not the envelope) which `||` did not.
		return json({
			success: true,
			data: extractBackendData(backendData),
			_source: 'backend'
		});
	}

	error(404, `Trade plan ${id} not found`);
};

// ═══════════════════════════════════════════════════════════════════════════
// PUT - Update trade plan entry
// ═══════════════════════════════════════════════════════════════════════════

export const PUT: RequestHandler = async ({ params, request, cookies }) => {
	const { slug, id } = params;
	const authHeader = request.headers.get('Authorization');
	const accessToken = cookies.get('rtp_access_token');

	if (!authHeader && !accessToken) {
		error(401, 'Authentication required');
	}

	if (!slug || !id) {
		error(400, 'Room slug and trade plan ID are required');
	}

	const planId = parseInt(id, 10);
	if (isNaN(planId)) {
		error(400, 'Invalid trade plan ID');
	}

	// R19-A: narrow body to non-null object — original code read
	// `body.ticker?.toUpperCase()` on a potentially-null `body`, which
	// would 500 the PUT on a `null` JSON body (R18-A Latent Bug §2 class).
	const rawBody: unknown = await request.json();
	if (!isObject(rawBody)) {
		error(400, 'Request body must be a JSON object');
	}
	const body = rawBody as TradePlanPutBody;

	// Build headers - use Bearer token format
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (accessToken) {
		headers['Authorization'] = `Bearer ${accessToken}`;
	}

	// Call backend at /api/admin/room-content/trade-plan/:id
	const backendData = await fetchBackend(
		`/api/admin/room-content/trade-plan/${id}`,
		{
			method: 'PUT',
			headers,
			body: JSON.stringify({
				ticker: body.ticker?.toUpperCase(),
				bias: body.bias,
				entry: body.entry,
				target1: body.target1,
				target2: body.target2,
				target3: body.target3,
				runner: body.runner,
				stop: body.stop,
				runner_stop: body.runner_stop,
				options_strike: body.options_strike,
				options_exp: body.options_exp,
				notes: body.notes,
				sort_order: body.sort_order,
				is_active: body.is_active
			})
		},
		'[Trade Plans API]'
	);

	if (backendData) {
		return json({
			success: true,
			data: extractBackendData(backendData),
			message: 'Trade plan entry updated',
			_source: 'backend'
		});
	}

	error(500, 'Failed to update trade plan entry');
};

// ═══════════════════════════════════════════════════════════════════════════
// DELETE - Delete trade plan entry
// ═══════════════════════════════════════════════════════════════════════════

export const DELETE: RequestHandler = async ({ params, request, cookies }) => {
	const { slug, id } = params;
	const authHeader = request.headers.get('Authorization');
	const accessToken = cookies.get('rtp_access_token');

	if (!authHeader && !accessToken) {
		error(401, 'Authentication required');
	}

	if (!slug || !id) {
		error(400, 'Room slug and trade plan ID are required');
	}

	const planId = parseInt(id, 10);
	if (isNaN(planId)) {
		error(400, 'Invalid trade plan ID');
	}

	// Build headers - use Bearer token format
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (accessToken) {
		headers['Authorization'] = `Bearer ${accessToken}`;
	}

	// Call backend at /api/admin/room-content/trade-plan/:id
	const backendData = await fetchBackend(
		`/api/admin/room-content/trade-plan/${id}`,
		{
			method: 'DELETE',
			headers
		},
		'[Trade Plans API]'
	);

	if (backendData) {
		return json({
			success: true,
			message: `Trade plan entry ${id} deleted`,
			_source: 'backend'
		});
	}

	error(500, 'Failed to delete trade plan entry');
};

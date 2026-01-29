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
import { env } from '$env/dynamic/private';

const BACKEND_URL = env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

async function fetchFromBackend(endpoint: string, options: RequestInit = {}): Promise<any | null> {
	try {
		console.log(`[Trade Plans API] Fetching: ${BACKEND_URL}${endpoint}`);
		const response = await fetch(`${BACKEND_URL}${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...options.headers
			}
		});

		if (!response.ok) {
			console.error(`[Trade Plans API] Backend error: ${response.status} ${response.statusText}`);
			const errorText = await response.text();
			console.error(`[Trade Plans API] Error body:`, errorText);
			return null;
		}

		return await response.json();
	} catch (err) {
		console.error('[Trade Plans API] Backend fetch failed:', err);
		return null;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// GET - Get single trade plan entry
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, request, cookies }) => {
	const { slug, id } = params;

	if (!slug || !id) {
		throw error(400, 'Room slug and trade plan ID are required');
	}

	const planId = parseInt(id, 10);
	if (isNaN(planId)) {
		throw error(400, 'Invalid trade plan ID');
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

	const backendData = await fetchFromBackend(`/api/room-content/rooms/${slug}/trade-plan/${id}`, {
		headers
	});

	if (backendData) {
		return json({
			success: true,
			data: backendData.data || backendData,
			_source: 'backend'
		});
	}

	throw error(404, `Trade plan ${id} not found`);
};

// ═══════════════════════════════════════════════════════════════════════════
// PUT - Update trade plan entry
// ═══════════════════════════════════════════════════════════════════════════

export const PUT: RequestHandler = async ({ params, request, cookies }) => {
	const { slug, id } = params;
	const authHeader = request.headers.get('Authorization');
	const accessToken = cookies.get('rtp_access_token');

	if (!authHeader && !accessToken) {
		throw error(401, 'Authentication required');
	}

	if (!slug || !id) {
		throw error(400, 'Room slug and trade plan ID are required');
	}

	const planId = parseInt(id, 10);
	if (isNaN(planId)) {
		throw error(400, 'Invalid trade plan ID');
	}

	const body = await request.json();

	// Build headers - use Bearer token format
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (accessToken) {
		headers['Authorization'] = `Bearer ${accessToken}`;
	}

	// Call backend at /api/admin/room-content/trade-plan/:id
	const backendData = await fetchFromBackend(`/api/admin/room-content/trade-plan/${id}`, {
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
	});

	if (backendData) {
		return json({
			success: true,
			data: backendData.data || backendData,
			message: 'Trade plan entry updated',
			_source: 'backend'
		});
	}

	throw error(500, 'Failed to update trade plan entry');
};

// ═══════════════════════════════════════════════════════════════════════════
// DELETE - Delete trade plan entry
// ═══════════════════════════════════════════════════════════════════════════

export const DELETE: RequestHandler = async ({ params, request, cookies }) => {
	const { slug, id } = params;
	const authHeader = request.headers.get('Authorization');
	const accessToken = cookies.get('rtp_access_token');

	if (!authHeader && !accessToken) {
		throw error(401, 'Authentication required');
	}

	if (!slug || !id) {
		throw error(400, 'Room slug and trade plan ID are required');
	}

	const planId = parseInt(id, 10);
	if (isNaN(planId)) {
		throw error(400, 'Invalid trade plan ID');
	}

	// Build headers - use Bearer token format
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (accessToken) {
		headers['Authorization'] = `Bearer ${accessToken}`;
	}

	// Call backend at /api/admin/room-content/trade-plan/:id
	const backendData = await fetchFromBackend(`/api/admin/room-content/trade-plan/${id}`, {
		method: 'DELETE',
		headers
	});

	if (backendData) {
		return json({
			success: true,
			message: `Trade plan entry ${id} deleted`,
			_source: 'backend'
		});
	}

	throw error(500, 'Failed to delete trade plan entry');
};

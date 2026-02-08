/**
 * Trade Plans API - Room-specific trade plan entries
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/trade-plans/[room-slug] - List trade plans for a room
 * POST /api/trade-plans/[room-slug] - Create new trade plan entry
 *
 * Connects to backend at /api/room-content/rooms/:slug/trade-plan
 *
 * @version 2.0.0 - ICT 11 Principal Engineer Grade
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import type { TradePlanEntry, TradePlanCreateInput } from '$lib/types/trading';

// ═══════════════════════════════════════════════════════════════════════════
// BACKEND CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════

const BACKEND_URL = env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

// ═══════════════════════════════════════════════════════════════════════════
// BACKEND FETCH HELPER
// ═══════════════════════════════════════════════════════════════════════════

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
			return null;
		}

		const data = await response.json();
		console.log(`[Trade Plans API] Backend success:`, data?.data?.length || 0, 'items');
		return data;
	} catch (err) {
		console.error('[Trade Plans API] Backend fetch failed:', err);
		return null;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// FALLBACK MOCK DATA
// ═══════════════════════════════════════════════════════════════════════════

const mockTradePlans: Record<string, TradePlanEntry[]> = {
	'explosive-swings': [
		{
			id: 1,
			room_id: 4,
			room_slug: 'explosive-swings',
			week_of: new Date().toISOString().split('T')[0],
			ticker: 'NVDA',
			bias: 'BULLISH',
			entry: '$142.50',
			target1: '$148.00',
			target2: '$152.00',
			target3: '$158.00',
			runner: '$165.00+',
			runner_stop: '$160.00',
			stop: '$136.00',
			options_strike: '$145 Call',
			options_exp: '2026-01-24',
			notes: 'Breakout above consolidation. Wait for pullback to entry zone.',
			sort_order: 1,
			is_active: true,
			created_at: new Date().toISOString(),
			updated_at: new Date().toISOString()
		}
	]
};

// ═══════════════════════════════════════════════════════════════════════════
// GET - List trade plans for a room
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, url, request, cookies }) => {
	const { slug } = params;

	if (!slug) {
		error(400, 'Room slug is required');
	}

	// Query params
	const weekOf = url.searchParams.get('week_of');
	const activeOnly = url.searchParams.get('active_only') !== 'false';
	const page = url.searchParams.get('page') || '1';
	const perPage = url.searchParams.get('per_page') || '50';

	// Get auth headers - check both Authorization header and access token cookie
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
	if (weekOf) backendParams.set('week_of', weekOf);

	// Call backend at /api/room-content/rooms/:slug/trade-plan
	const backendData = await fetchFromBackend(
		`/api/room-content/rooms/${slug}/trade-plan?${backendParams.toString()}`,
		{ headers }
	);

	if (backendData?.data) {
		let plans = backendData.data;

		// Filter active only if needed (client-side)
		if (activeOnly) {
			plans = plans.filter((p: TradePlanEntry) => p.is_active);
		}

		// Sort by sort_order
		plans.sort((a: TradePlanEntry, b: TradePlanEntry) => a.sort_order - b.sort_order);

		return json({
			success: true,
			data: plans,
			week_of: plans[0]?.week_of || null,
			total: backendData.meta?.total || plans.length,
			_source: 'backend'
		});
	}

	// Fallback to mock data
	console.log(`[Trade Plans API] Using mock data for ${slug}`);
	let plans = mockTradePlans[slug] || [];

	// Filter by week if specified
	if (weekOf) {
		plans = plans.filter((p) => p.week_of === weekOf);
	}

	// Filter active only
	if (activeOnly) {
		plans = plans.filter((p) => p.is_active);
	}

	// Sort by sort_order
	plans.sort((a, b) => a.sort_order - b.sort_order);

	return json({
		success: true,
		data: plans,
		week_of: weekOf || plans[0]?.week_of || null,
		total: plans.length,
		_source: 'mock'
	});
};

// ═══════════════════════════════════════════════════════════════════════════
// POST - Create new trade plan entry
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

	const body: TradePlanCreateInput = await request.json();

	// Validate required fields - only ticker is truly required
	if (!body.ticker) {
		error(400, 'Ticker is required');
	}

	// Build headers
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (accessToken) {
		headers['Authorization'] = `Bearer ${accessToken}`;
	}

	// Call backend at /api/admin/room-content/trade-plan
	const backendData = await fetchFromBackend(`/api/admin/room-content/trade-plan`, {
		method: 'POST',
		headers,
		body: JSON.stringify({
			room_slug: slug,
			week_of: body.week_of || new Date().toISOString().split('T')[0],
			ticker: body.ticker.toUpperCase(),
			bias: body.bias,
			entry: body.entry,
			target1: body.target1 || '',
			target2: body.target2 || '',
			target3: body.target3 || '',
			runner: body.runner || '',
			stop: body.stop,
			options_strike: body.options_strike,
			options_exp: body.options_exp,
			notes: body.notes,
			sort_order: body.sort_order
		})
	});

	if (backendData) {
		return json({
			success: true,
			data: backendData,
			message: 'Trade plan entry created',
			_source: 'backend'
		});
	}

	// Mock create fallback
	const plans = mockTradePlans[slug] || [];
	const maxId = plans.reduce((max, p) => Math.max(max, p.id), 0);
	const maxOrder = plans.reduce((max, p) => Math.max(max, p.sort_order), 0);

	const newPlan: TradePlanEntry = {
		id: maxId + 1,
		room_id: 4,
		room_slug: slug,
		week_of: body.week_of || new Date().toISOString().split('T')[0],
		ticker: body.ticker.toUpperCase(),
		bias: body.bias,
		entry: body.entry || '',
		target1: body.target1 || '',
		target2: body.target2 || '',
		target3: body.target3 || '',
		runner: body.runner || '',
		runner_stop: body.runner_stop || null,
		stop: body.stop || '',
		options_strike: body.options_strike || null,
		options_exp: body.options_exp || null,
		notes: body.notes || null,
		sort_order: body.sort_order ?? maxOrder + 1,
		is_active: true,
		created_at: new Date().toISOString(),
		updated_at: new Date().toISOString()
	};

	if (!mockTradePlans[slug]) {
		mockTradePlans[slug] = [];
	}
	mockTradePlans[slug].push(newPlan);

	return json({
		success: true,
		data: newPlan,
		message: 'Trade plan entry created',
		_source: 'mock'
	});
};

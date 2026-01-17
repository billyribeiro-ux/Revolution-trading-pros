/**
 * Trade Plans API - Room-specific trade plan entries
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/trade-plans/[room-slug] - List trade plans for a room
 * POST /api/trade-plans/[room-slug] - Create new trade plan entry
 *
 * @version 1.0.0
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import type { TradePlanEntry, TradePlanCreateInput } from '$lib/types/trading';

// ═══════════════════════════════════════════════════════════════════════════
// MOCK DATA - Matches Explosive Swings dashboard
// ═══════════════════════════════════════════════════════════════════════════

const mockTradePlans: Record<string, TradePlanEntry[]> = {
	'explosive-swings': [
		{
			id: 1,
			room_id: 4,
			room_slug: 'explosive-swings',
			week_of: '2026-01-13',
			ticker: 'NVDA',
			bias: 'BULLISH',
			entry: '$142.50',
			target1: '$148.00',
			target2: '$152.00',
			target3: '$158.00',
			runner: '$165.00+',
			stop: '$136.00',
			options_strike: '$145 Call',
			options_exp: '2026-01-24',
			notes:
				'Breakout above consolidation. Wait for pullback to entry zone. Strong momentum with volume confirmation.',
			sort_order: 1,
			is_active: true,
			created_at: '2026-01-13T09:00:00Z',
			updated_at: '2026-01-13T09:00:00Z'
		},
		{
			id: 2,
			room_id: 4,
			room_slug: 'explosive-swings',
			week_of: '2026-01-13',
			ticker: 'TSLA',
			bias: 'BULLISH',
			entry: '$248.00',
			target1: '$255.00',
			target2: '$265.00',
			target3: '$275.00',
			runner: '$290.00+',
			stop: '$235.00',
			options_strike: '$250 Call',
			options_exp: '2026-01-31',
			notes: 'Momentum building. Earnings catalyst ahead. Watch for continuation above $250.',
			sort_order: 2,
			is_active: true,
			created_at: '2026-01-13T09:00:00Z',
			updated_at: '2026-01-13T09:00:00Z'
		},
		{
			id: 3,
			room_id: 4,
			room_slug: 'explosive-swings',
			week_of: '2026-01-13',
			ticker: 'AMZN',
			bias: 'BULLISH',
			entry: '$185.00',
			target1: '$190.00',
			target2: '$195.00',
			target3: '$198.00',
			runner: '$205.00+',
			stop: '$178.00',
			options_strike: '$185 Call',
			options_exp: '2026-01-24',
			notes:
				'Breaking above key resistance. Strong volume confirmation. Cloud growth story intact.',
			sort_order: 3,
			is_active: true,
			created_at: '2026-01-13T09:00:00Z',
			updated_at: '2026-01-13T09:00:00Z'
		},
		{
			id: 4,
			room_id: 4,
			room_slug: 'explosive-swings',
			week_of: '2026-01-13',
			ticker: 'GOOGL',
			bias: 'NEUTRAL',
			entry: '$175.50',
			target1: '$180.00',
			target2: '$185.00',
			target3: '$188.00',
			runner: '$195.00+',
			stop: '$168.00',
			options_strike: '$177.50 Call',
			options_exp: '2026-02-07',
			notes: 'Watching for breakout. Not triggered yet. Need to see price action above $176.',
			sort_order: 4,
			is_active: true,
			created_at: '2026-01-13T09:00:00Z',
			updated_at: '2026-01-13T09:00:00Z'
		},
		{
			id: 5,
			room_id: 4,
			room_slug: 'explosive-swings',
			week_of: '2026-01-13',
			ticker: 'META',
			bias: 'BULLISH',
			entry: '$585.00',
			target1: '$600.00',
			target2: '$615.00',
			target3: '$630.00',
			runner: '$650.00+',
			stop: '$565.00',
			options_strike: '$590 Call',
			options_exp: '2026-01-24',
			notes: 'Strong trend. Buy dips to support. AI monetization story gaining traction.',
			sort_order: 5,
			is_active: true,
			created_at: '2026-01-13T09:00:00Z',
			updated_at: '2026-01-13T09:00:00Z'
		},
		{
			id: 6,
			room_id: 4,
			room_slug: 'explosive-swings',
			week_of: '2026-01-13',
			ticker: 'AMD',
			bias: 'BEARISH',
			entry: '$125.00',
			target1: '$120.00',
			target2: '$115.00',
			target3: '$110.00',
			runner: '$100.00',
			stop: '$132.00',
			options_strike: '$122 Put',
			options_exp: '2026-01-31',
			notes: 'Breakdown in progress. Short on bounces. Losing market share concerns.',
			sort_order: 6,
			is_active: true,
			created_at: '2026-01-13T09:00:00Z',
			updated_at: '2026-01-13T09:00:00Z'
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
// GET - List trade plans for a room
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, url, request }) => {
	const { slug } = params;

	if (!slug) {
		throw error(400, 'Room slug is required');
	}

	// Query params
	const weekOf = url.searchParams.get('week_of');
	const activeOnly = url.searchParams.get('active_only') !== 'false';

	// Try backend first
	const authHeader = request.headers.get('Authorization');
	const headers: Record<string, string> = {};
	if (authHeader) headers['Authorization'] = authHeader;

	const backendData = await fetchFromBackend(
		`/api/trade-plans/${slug}?${url.searchParams.toString()}`,
		{ headers }
	);

	if (backendData?.success) {
		return json(backendData);
	}

	// Fallback to mock data
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
		_mock: true
	});
};

// ═══════════════════════════════════════════════════════════════════════════
// POST - Create new trade plan entry
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

	const body: TradePlanCreateInput = await request.json();

	// Validate required fields
	if (!body.ticker || !body.bias || !body.entry || !body.stop) {
		throw error(400, 'Ticker, bias, entry, and stop are required');
	}

	// Try backend first
	const backendData = await fetchFromBackend(`/api/trade-plans/${slug}`, {
		method: 'POST',
		headers: { Authorization: authHeader },
		body: JSON.stringify(body)
	});

	if (backendData?.success) {
		return json(backendData);
	}

	// Mock create
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
		entry: body.entry,
		target1: body.target1 || '',
		target2: body.target2 || '',
		target3: body.target3 || '',
		runner: body.runner || '',
		stop: body.stop,
		options_strike: body.options_strike || null,
		options_exp: body.options_exp || null,
		notes: body.notes || null,
		sort_order: body.sort_order ?? maxOrder + 1,
		is_active: true,
		created_at: new Date().toISOString(),
		updated_at: new Date().toISOString()
	};

	// Add to mock data
	if (!mockTradePlans[slug]) {
		mockTradePlans[slug] = [];
	}
	mockTradePlans[slug].push(newPlan);

	return json({
		success: true,
		data: newPlan,
		message: 'Trade plan entry created',
		_mock: true
	});
};

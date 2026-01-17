/**
 * Room Stats API - Performance statistics for trading rooms
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/stats/[room-slug] - Get room statistics
 * POST /api/stats/[room-slug]/refresh - Force recalculate stats (admin)
 *
 * @version 1.0.0
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import type { RoomStats } from '$lib/types/trading';

// ═══════════════════════════════════════════════════════════════════════════
// MOCK DATA - Matches Explosive Swings stats
// ═══════════════════════════════════════════════════════════════════════════

const mockStats: Record<string, RoomStats> = {
	'explosive-swings': {
		room_id: 4,
		room_slug: 'explosive-swings',
		win_rate: 82,
		weekly_profit: '+$4,850',
		monthly_profit: '+$18,750',
		active_trades: 4,
		closed_this_week: 2,
		total_trades: 28,
		wins: 23,
		losses: 5,
		avg_win: 1250,
		avg_loss: 425,
		profit_factor: 2.94,
		avg_holding_days: 4.2,
		largest_win: 3100,
		largest_loss: 890,
		current_streak: 5,
		calculated_at: new Date().toISOString()
	}
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
// GET - Get room statistics
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, request }) => {
	const { slug } = params;

	if (!slug) {
		throw error(400, 'Room slug is required');
	}

	// Try backend first
	const authHeader = request.headers.get('Authorization');
	const headers: Record<string, string> = {};
	if (authHeader) headers['Authorization'] = authHeader;

	const backendData = await fetchFromBackend(`/api/stats/${slug}`, { headers });

	if (backendData?.success) {
		return json(backendData);
	}

	// Fallback to mock data
	const stats = mockStats[slug];

	if (!stats) {
		// Return empty stats for rooms without data
		return json({
			success: true,
			data: {
				room_slug: slug,
				win_rate: 0,
				weekly_profit: '$0',
				monthly_profit: '$0',
				active_trades: 0,
				closed_this_week: 0,
				total_trades: 0,
				wins: 0,
				losses: 0,
				avg_win: 0,
				avg_loss: 0,
				profit_factor: 0,
				avg_holding_days: 0,
				largest_win: 0,
				largest_loss: 0,
				current_streak: 0,
				calculated_at: new Date().toISOString()
			},
			_mock: true
		});
	}

	return json({
		success: true,
		data: stats,
		_mock: true
	});
};

// ═══════════════════════════════════════════════════════════════════════════
// POST - Refresh/recalculate stats (admin only)
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

	// Try backend first
	const backendData = await fetchFromBackend(`/api/stats/${slug}/refresh`, {
		method: 'POST',
		headers: { Authorization: authHeader }
	});

	if (backendData?.success) {
		return json(backendData);
	}

	// Mock refresh - just update the calculated_at timestamp
	if (mockStats[slug]) {
		mockStats[slug].calculated_at = new Date().toISOString();
	}

	return json({
		success: true,
		data: mockStats[slug] || null,
		message: 'Stats refreshed',
		_mock: true
	});
};

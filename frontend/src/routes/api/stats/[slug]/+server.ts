/**
 * Room Stats API - Performance statistics for trading rooms
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/stats/[room-slug] - Get room statistics
 * POST /api/stats/[room-slug]/refresh - Force recalculate stats (admin)
 *
 * Connects to backend at /api/room-content/rooms/:slug/stats
 *
 * @version 2.0.0 - ICT 11 Principal Engineer Grade
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import type { RoomStats } from '$lib/types/trading';
// R19-A: shared proxy helper — pins CLAUDE.md URL-fallback chain
// (API_BASE_URL || BACKEND_URL || localhost) AND replaces the
// `Promise<any | null>` helper with `Promise<unknown>` + narrowing guards.
import { fetchBackend, hasData, extractBackendData } from '$lib/server/proxy-fetch';

// ═══════════════════════════════════════════════════════════════════════════
// FALLBACK MOCK DATA
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
// GET - Get room statistics
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, request, cookies }) => {
	const { slug } = params;

	if (!slug) {
		error(400, 'Room slug is required');
	}

	// Get auth headers - ICT 7: Use rtp_access_token cookie for Bearer auth (consistent with auth store)
	const authHeader = request.headers.get('Authorization');
	const accessToken = cookies.get('rtp_access_token');
	const headers: Record<string, string> = {};

	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (accessToken) {
		headers['Authorization'] = `Bearer ${accessToken}`;
	}

	// Call backend at /api/room-content/rooms/:slug/stats
	const backendData = await fetchBackend(
		`/api/room-content/rooms/${slug}/stats`,
		{ headers },
		'[Stats API]'
	);

	// R19-A: hasData() narrowing instead of `backendData?.data`.
	if (hasData(backendData)) {
		return json({
			success: true,
			data: backendData.data,
			_source: 'backend'
		});
	}

	// Fallback to mock data
	console.info(`[Stats API] Using mock data for ${slug}`);
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
			_source: 'mock'
		});
	}

	return json({
		success: true,
		data: stats,
		_source: 'mock'
	});
};

// ═══════════════════════════════════════════════════════════════════════════
// POST - Refresh/recalculate stats (admin only)
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

	// Build headers - ICT 7: Use rtp_access_token cookie for Bearer auth
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (accessToken) {
		headers['Authorization'] = `Bearer ${accessToken}`;
	}

	// Call backend to refresh stats
	const backendData = await fetchBackend(
		`/api/admin/room-content/rooms/${slug}/stats/refresh`,
		{
			method: 'POST',
			headers
		},
		'[Stats API]'
	);

	if (backendData) {
		// R19-A: replaces `backendData.data || backendData` short-circuit
		// (R18-A Latent Bug §3). Distinguishes "backend returned `data:
		// null`" (no stats yet) from "envelope has no `data` key" (legacy
		// non-wrapped endpoint). Both safe to forward.
		return json({
			success: true,
			data: extractBackendData(backendData),
			message: 'Stats refreshed',
			_source: 'backend'
		});
	}

	// Mock refresh - just update the calculated_at timestamp
	if (mockStats[slug]) {
		mockStats[slug].calculated_at = new Date().toISOString();
	}

	return json({
		success: true,
		data: mockStats[slug] || null,
		message: 'Stats refreshed',
		_source: 'mock'
	});
};

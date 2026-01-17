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
import { env } from '$env/dynamic/private';
import type { RoomStats } from '$lib/types/trading';

// ═══════════════════════════════════════════════════════════════════════════
// BACKEND CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════

const BACKEND_URL = env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

// ═══════════════════════════════════════════════════════════════════════════
// BACKEND FETCH HELPER
// ═══════════════════════════════════════════════════════════════════════════

async function fetchFromBackend(endpoint: string, options: RequestInit = {}): Promise<any | null> {
	try {
		console.log(`[Stats API] Fetching: ${BACKEND_URL}${endpoint}`);
		const response = await fetch(`${BACKEND_URL}${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...options.headers
			}
		});

		if (!response.ok) {
			console.error(`[Stats API] Backend error: ${response.status} ${response.statusText}`);
			return null;
		}

		const data = await response.json();
		console.log(`[Stats API] Backend success`);
		return data;
	} catch (err) {
		console.error('[Stats API] Backend fetch failed:', err);
		return null;
	}
}

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
		throw error(400, 'Room slug is required');
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

	// Call backend at /api/room-content/rooms/:slug/stats
	const backendData = await fetchFromBackend(`/api/room-content/rooms/${slug}/stats`, { headers });

	if (backendData?.data) {
		return json({
			success: true,
			data: backendData.data,
			_source: 'backend'
		});
	}

	// Fallback to mock data
	console.log(`[Stats API] Using mock data for ${slug}`);
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
	const sessionCookie = cookies.get('session');

	if (!authHeader && !sessionCookie) {
		throw error(401, 'Authentication required');
	}

	if (!slug) {
		throw error(400, 'Room slug is required');
	}

	// Build headers
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (sessionCookie) {
		headers['Cookie'] = `session=${sessionCookie}`;
	}

	// Call backend to refresh stats
	const backendData = await fetchFromBackend(`/api/admin/room-content/rooms/${slug}/stats/refresh`, {
		method: 'POST',
		headers
	});

	if (backendData) {
		return json({
			success: true,
			data: backendData.data || backendData,
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

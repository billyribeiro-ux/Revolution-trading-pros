/**
 * ===============================================================================
 * API Response Mocks for Testing
 * ===============================================================================
 *
 * @description Mock data and factory functions for API responses
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Vitest January 2026 Patterns
 *
 * This module provides:
 * - Type-safe mock data matching API response shapes
 * - Factory functions for creating customizable mocks
 * - Helper to create mock fetch implementations
 */

import { vi } from 'vitest';
import type {
	ApiTrade,
	ApiWeeklyVideo,
	QuickStats,
	PaginationMeta
} from '../../types';

// ===============================================================================
// API ALERT TYPES (matches backend response shape)
// ===============================================================================

export interface ApiAlert {
	id: number;
	room_slug: string;
	alert_type: 'ENTRY' | 'UPDATE' | 'EXIT';
	ticker: string;
	title: string;
	message: string;
	trade_type?: 'stock' | 'options' | 'spread';
	action?: string;
	quantity?: number;
	option_type?: 'CALL' | 'PUT';
	strike?: number;
	expiration?: string;
	tos_string?: string;
	is_new: boolean;
	published_at: string;
	created_at: string;
}

export interface ApiTradePlanEntry {
	ticker: string;
	bias: 'BULLISH' | 'BEARISH' | 'NEUTRAL';
	entry: string;
	target1: string;
	target2: string;
	target3: string;
	runner: string;
	stop: string;
	options_strike: string | null;
	options_exp: string | null;
	notes: string | null;
}

// ===============================================================================
// MOCK DATA - Default instances
// ===============================================================================

export const mockAlert: ApiAlert = {
	id: 1,
	room_slug: 'explosive-swings',
	alert_type: 'ENTRY',
	ticker: 'NVDA',
	title: 'Test Entry Alert',
	message: 'Testing entry alert',
	trade_type: 'options',
	action: 'BUY',
	quantity: 10,
	option_type: 'CALL',
	strike: 150,
	expiration: '2026-02-07',
	tos_string: 'BUY +10 NVDA 100 (Weeklys) 7 FEB 26 150 CALL @2.50 LMT',
	is_new: true,
	published_at: new Date().toISOString(),
	created_at: new Date().toISOString()
};

export const mockTrade: ApiTrade = {
	id: 1,
	ticker: 'AAPL',
	status: 'open',
	entry_price: 180.5,
	exit_price: null,
	pnl_percent: null,
	entry_date: '2026-01-20',
	exit_date: null,
	direction: 'long'
};

export const mockClosedTrade: ApiTrade = {
	id: 2,
	ticker: 'MSFT',
	status: 'closed',
	entry_price: 425.0,
	exit_price: 460.0,
	pnl_percent: 8.2,
	entry_date: '2026-01-15',
	exit_date: '2026-01-20',
	direction: 'long'
};

export const mockTradePlanEntry: ApiTradePlanEntry = {
	ticker: 'NVDA',
	bias: 'BULLISH',
	entry: '$142.50',
	target1: '$148.00',
	target2: '$152.00',
	target3: '$158.00',
	runner: '$165.00+',
	stop: '$136.00',
	options_strike: '$145 Call',
	options_exp: 'Jan 24, 2026',
	notes: 'Breakout above consolidation'
};

export const mockStats: QuickStats = {
	winRate: 72.5,
	weeklyProfit: '+$2,500',
	activeTrades: 3,
	closedThisWeek: 5
};

export const mockWeeklyVideo: ApiWeeklyVideo = {
	id: 1,
	video_title: 'Weekly Breakdown: Top Swing Setups',
	video_url: 'https://iframe.mediadelivery.net/embed/585929/test-video-id',
	thumbnail_url: 'https://placehold.co/640x360',
	duration: '24:35',
	published_at: new Date().toISOString(),
	week_title: 'Week of January 20, 2026'
};

// ===============================================================================
// API RESPONSE BUILDERS
// ===============================================================================

export const mockAlertsResponse = {
	success: true,
	data: [mockAlert],
	total: 1
};

export const mockTradesResponse = {
	success: true,
	data: [mockTrade, mockClosedTrade]
};

export const mockTradePlanResponse = {
	success: true,
	data: [mockTradePlanEntry]
};

export const mockStatsResponse = {
	success: true,
	data: {
		win_rate: 72.5,
		weekly_profit: '+$2,500',
		active_trades: 3,
		closed_this_week: 5
	}
};

export const mockWeeklyVideoResponse = {
	success: true,
	data: mockWeeklyVideo
};

export const mockAdminResponse = {
	is_admin: true,
	role: 'admin'
};

export const mockUserResponse = {
	is_admin: false,
	role: 'user'
};

// ===============================================================================
// FACTORY FUNCTIONS
// ===============================================================================

/**
 * Create a mock alert with customizable properties
 */
export function createApiAlert(overrides: Partial<ApiAlert> = {}): ApiAlert {
	return {
		id: Math.floor(Math.random() * 10000),
		room_slug: 'explosive-swings',
		alert_type: 'ENTRY',
		ticker: 'NVDA',
		title: 'Test Alert',
		message: 'Test message',
		trade_type: 'options',
		action: 'BUY',
		quantity: 10,
		option_type: 'CALL',
		strike: 150,
		expiration: '2026-02-07',
		tos_string: 'BUY +10 NVDA 100 150 CALL @2.50',
		is_new: true,
		published_at: new Date().toISOString(),
		created_at: new Date().toISOString(),
		...overrides
	};
}

/**
 * Create a mock trade with customizable properties
 */
export function createApiTrade(overrides: Partial<ApiTrade> = {}): ApiTrade {
	return {
		id: Math.floor(Math.random() * 10000),
		ticker: 'NVDA',
		status: 'open',
		entry_price: 142.5,
		exit_price: null,
		pnl_percent: null,
		entry_date: new Date().toISOString().split('T')[0],
		exit_date: null,
		direction: 'long',
		...overrides
	};
}

/**
 * Create a mock trade plan entry with customizable properties
 */
export function createApiTradePlanEntry(
	overrides: Partial<ApiTradePlanEntry> = {}
): ApiTradePlanEntry {
	return {
		ticker: 'NVDA',
		bias: 'BULLISH',
		entry: '$142.50',
		target1: '$148.00',
		target2: '$152.00',
		target3: '$158.00',
		runner: '$165.00+',
		stop: '$136.00',
		options_strike: '$145 Call',
		options_exp: 'Jan 24, 2026',
		notes: 'Test trade plan entry',
		...overrides
	};
}

/**
 * Create a mock stats object with customizable properties
 */
export function createApiStats(overrides: Partial<QuickStats> = {}): QuickStats {
	return {
		winRate: 82,
		weeklyProfit: '+$4,850',
		activeTrades: 4,
		closedThisWeek: 2,
		...overrides
	};
}

/**
 * Create a mock weekly video with customizable properties
 */
export function createApiWeeklyVideo(overrides: Partial<ApiWeeklyVideo> = {}): ApiWeeklyVideo {
	return {
		id: Math.floor(Math.random() * 10000),
		video_title: 'Weekly Breakdown',
		video_url: 'https://iframe.mediadelivery.net/embed/585929/test-video-id',
		thumbnail_url: 'https://placehold.co/640x360',
		duration: '24:35',
		published_at: new Date().toISOString(),
		week_title: 'Week of January 20, 2026',
		...overrides
	};
}

// ===============================================================================
// MOCK FETCH UTILITIES
// ===============================================================================

/**
 * Create a mock fetch function that returns different responses based on URL path
 *
 * @example
 * ```ts
 * const fetch = createMockFetch({
 *   '/api/alerts/explosive-swings': mockAlertsResponse,
 *   '/api/stats/explosive-swings': mockStatsResponse,
 * });
 * vi.stubGlobal('fetch', fetch);
 * ```
 */
export function createMockFetch(responses: Record<string, unknown>) {
	return vi.fn().mockImplementation((url: string) => {
		// Handle both full URLs and relative paths
		let path: string;
		try {
			path = new URL(url, 'http://localhost').pathname;
		} catch {
			path = url;
		}

		// Remove query parameters for matching
		const pathWithoutQuery = path.split('?')[0];

		// Find matching response
		const response = responses[pathWithoutQuery] ?? responses[path];

		if (response === undefined) {
			return Promise.resolve({
				ok: false,
				status: 404,
				json: () => Promise.resolve({ success: false, error: 'Not found' }),
				text: () => Promise.resolve('Not found')
			});
		}

		return Promise.resolve({
			ok: true,
			status: 200,
			json: () => Promise.resolve(response),
			text: () => Promise.resolve(JSON.stringify(response)),
			headers: new Headers()
		});
	});
}

/**
 * Create a mock fetch that simulates network delay
 */
export function createDelayedMockFetch(responses: Record<string, unknown>, delayMs = 100) {
	const baseFetch = createMockFetch(responses);
	return vi.fn().mockImplementation(async (url: string, options?: RequestInit) => {
		await new Promise((resolve) => setTimeout(resolve, delayMs));
		return baseFetch(url, options);
	});
}

/**
 * Create a mock fetch that fails with a specific error
 */
export function createFailingMockFetch(status: number, errorMessage: string) {
	return vi.fn().mockResolvedValue({
		ok: false,
		status,
		json: () => Promise.resolve({ success: false, error: errorMessage }),
		text: () => Promise.resolve(errorMessage),
		headers: new Headers()
	});
}

/**
 * Create a mock fetch that throws a network error
 */
export function createNetworkErrorMockFetch(errorMessage = 'Network error') {
	return vi.fn().mockRejectedValue(new Error(errorMessage));
}

// ===============================================================================
// PAGINATED RESPONSE HELPERS
// ===============================================================================

/**
 * Create a paginated alerts response
 */
export function createPaginatedAlertsResponse(
	alerts: ApiAlert[],
	page = 1,
	perPage = 10,
	total?: number
) {
	const actualTotal = total ?? alerts.length;
	const totalPages = Math.ceil(actualTotal / perPage);

	return {
		success: true,
		data: alerts,
		total: actualTotal,
		pagination: {
			page,
			per_page: perPage,
			total: actualTotal,
			total_pages: totalPages,
			has_next: page < totalPages,
			has_prev: page > 1
		} as PaginationMeta
	};
}

/**
 * Create a batch of mock alerts for testing pagination
 */
export function createAlertBatch(count: number, startId = 1): ApiAlert[] {
	return Array.from({ length: count }, (_, i) =>
		createApiAlert({
			id: startId + i,
			ticker: ['NVDA', 'TSLA', 'AAPL', 'MSFT', 'META'][i % 5],
			alert_type: (['ENTRY', 'UPDATE', 'EXIT'] as const)[i % 3],
			is_new: i < 3
		})
	);
}

/**
 * Create a batch of mock trades for testing
 */
export function createTradeBatch(count: number, startId = 1): ApiTrade[] {
	return Array.from({ length: count }, (_, i) =>
		createApiTrade({
			id: startId + i,
			ticker: ['NVDA', 'TSLA', 'AAPL', 'MSFT', 'META'][i % 5],
			status: i % 3 === 0 ? 'closed' : 'open',
			pnl_percent: i % 3 === 0 ? Math.random() * 20 - 5 : null
		})
	);
}

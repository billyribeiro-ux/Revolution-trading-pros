/**
 * =============================================================================
 * Explosive Swings - API Module (Enterprise Edition)
 * =============================================================================
 *
 * @description Centralized API calls for the Explosive Swings dashboard
 * @version 2.0.0 - Enterprise API Client Integration
 * @standards Apple Principal Engineer ICT Level 7 Grade Quality
 *
 * FEATURES:
 * - Type-safe API calls with branded error types
 * - Automatic retry with exponential backoff
 * - Request caching with stale-while-revalidate
 * - Request cancellation support
 * - Comprehensive error handling
 */

import { browser } from '$app/environment';
import { api, getApiClient, type RequestOptions } from '$lib/api/client';
import {
	ApiError,
	isApiError,
	getUserFriendlyMessage,
	createErrorFromStatus
} from '$lib/api/errors';
import { getCache, type CacheConfig } from '$lib/api/cache';
import type {
	ApiTrade,
	ApiWeeklyVideo,
	PaginationState,
	QuickStats,
	TradePlanEntry
} from './types';
import { formatTimeAgo } from './utils/formatters';

// =============================================================================
// CACHE CONFIGURATION - Optimized for trading data
// =============================================================================

/** Cache tags for grouped invalidation */
const CACHE_TAGS = {
	ALERTS: 'explosive-swings:alerts',
	TRADES: 'explosive-swings:trades',
	STATS: 'explosive-swings:stats',
	TRADE_PLAN: 'explosive-swings:trade-plan',
	VIDEO: 'explosive-swings:video',
	USER: 'explosive-swings:user'
} as const;

/** Default cache configurations by data type */
const CACHE_CONFIGS = {
	/** Alerts are frequently updated - short TTL */
	alerts: {
		ttl: 30_000, // 30 seconds
		staleWhileRevalidate: true,
		tags: [CACHE_TAGS.ALERTS]
	},
	/** Trades update less frequently */
	trades: {
		ttl: 60_000, // 1 minute
		staleWhileRevalidate: true,
		tags: [CACHE_TAGS.TRADES]
	},
	/** Stats can be cached longer */
	stats: {
		ttl: 120_000, // 2 minutes
		staleWhileRevalidate: true,
		tags: [CACHE_TAGS.STATS]
	},
	/** Trade plan changes infrequently */
	tradePlan: {
		ttl: 300_000, // 5 minutes
		staleWhileRevalidate: true,
		tags: [CACHE_TAGS.TRADE_PLAN]
	},
	/** Weekly video rarely changes */
	video: {
		ttl: 600_000, // 10 minutes
		staleWhileRevalidate: true,
		tags: [CACHE_TAGS.VIDEO]
	},
	/** User status can be cached briefly */
	user: {
		ttl: 60_000, // 1 minute
		staleWhileRevalidate: true,
		tags: [CACHE_TAGS.USER]
	}
} as const satisfies Record<string, Partial<CacheConfig>>;

// =============================================================================
// RESPONSE TYPES
// =============================================================================

export interface AlertsResponse {
	alerts: FormattedAlert[];
	pagination: PaginationState;
}

export interface FormattedAlert {
	id: number;
	type: 'ENTRY' | 'EXIT' | 'UPDATE';
	ticker: string;
	title: string;
	time: string;
	message: string;
	isNew: boolean;
	notes: string;
	tosString?: string;
}

// =============================================================================
// API REQUEST OPTIONS FACTORY
// =============================================================================

/**
 * Create request options with cancellation support
 */
function createRequestOptions(
	signal?: AbortSignal,
	cacheConfig?: Partial<CacheConfig>
): RequestOptions {
	return {
		signal,
		cache: cacheConfig
		// Use default retry settings from client (3 retries, exponential backoff)
	};
}

// =============================================================================
// ALERTS API
// =============================================================================

/**
 * Fetch paginated alerts for a trading room
 *
 * @param roomSlug - Trading room identifier
 * @param page - Page number (1-indexed)
 * @param limit - Items per page
 * @param signal - Optional AbortSignal for cancellation
 * @returns Formatted alerts with pagination metadata
 * @throws {ApiError} On network, auth, or server errors
 */
export async function fetchAlerts(
	roomSlug: string,
	page: number,
	limit: number,
	signal?: AbortSignal
): Promise<AlertsResponse> {
	const offset = (page - 1) * limit;

	const data = await api.get<{
		data: Record<string, unknown>[];
		total?: number;
	}>(`/api/alerts/${roomSlug}`, {
		params: { limit, offset },
		...createRequestOptions(signal, CACHE_CONFIGS.alerts)
	});

	// Handle the response structure
	const rawData = Array.isArray(data) ? data : (data?.data ?? []);

	return {
		alerts: rawData.map(formatAlert),
		pagination: {
			total: (data as { total?: number }).total ?? rawData.length,
			limit,
			offset
		}
	};
}

/**
 * Create a reactive alerts query hook (for Svelte 5 components)
 */
export function createAlertsQuery(
	roomSlug: string,
	page: number = 1,
	limit: number = 20,
	options?: { enabled?: boolean; refetchInterval?: number }
) {
	// Lazy import to avoid circular dependencies
	return import('$lib/api/hooks').then(({ createQuery }) =>
		createQuery(() => fetchAlerts(roomSlug, page, limit), {
			enabled: options?.enabled ?? true,
			refetchInterval: options?.refetchInterval ?? 30_000, // 30 second polling
			cache: CACHE_CONFIGS.alerts
		})
	);
}

// =============================================================================
// TRADE PLAN API
// =============================================================================

/**
 * Fetch trade plan entries for a trading room
 *
 * @param roomSlug - Trading room identifier
 * @param signal - Optional AbortSignal for cancellation
 * @returns Array of trade plan entries
 * @throws {ApiError} On network, auth, or server errors
 */
export async function fetchTradePlan(
	roomSlug: string,
	signal?: AbortSignal
): Promise<TradePlanEntry[]> {
	const data = await api.get<Record<string, unknown>[]>(
		`/api/trade-plans/${roomSlug}`,
		createRequestOptions(signal, CACHE_CONFIGS.tradePlan)
	);

	return (Array.isArray(data) ? data : []).map(formatTradePlanEntry);
}

// =============================================================================
// STATS API
// =============================================================================

/**
 * Fetch quick stats for a trading room
 *
 * @param roomSlug - Trading room identifier
 * @param signal - Optional AbortSignal for cancellation
 * @returns Quick stats object
 * @throws {ApiError} On network, auth, or server errors
 */
export async function fetchStats(roomSlug: string, signal?: AbortSignal): Promise<QuickStats> {
	const data = await api.get<{
		win_rate: number;
		weekly_profit: string;
		active_trades: number;
		closed_this_week: number;
	}>(`/api/stats/${roomSlug}`, createRequestOptions(signal, CACHE_CONFIGS.stats));

	return {
		winRate: data.win_rate,
		weeklyProfit: data.weekly_profit,
		activeTrades: data.active_trades,
		closedThisWeek: data.closed_this_week
	};
}

// =============================================================================
// TRADES API
// =============================================================================

/**
 * Fetch all trades for a trading room
 *
 * @param roomSlug - Trading room identifier
 * @param signal - Optional AbortSignal for cancellation
 * @returns Array of trade objects
 * @throws {ApiError} On network, auth, or server errors
 */
export async function fetchAllTrades(roomSlug: string, signal?: AbortSignal): Promise<ApiTrade[]> {
	return api.get<ApiTrade[]>(`/api/trades/${roomSlug}`, {
		params: { per_page: 100 },
		...createRequestOptions(signal, CACHE_CONFIGS.trades)
	});
}

// =============================================================================
// WEEKLY VIDEO API
// =============================================================================

/**
 * Fetch weekly video for a trading room
 *
 * @param roomSlug - Trading room identifier
 * @param signal - Optional AbortSignal for cancellation
 * @returns Weekly video object or null if none exists
 * @throws {ApiError} On network, auth, or server errors (except 404)
 */
export async function fetchWeeklyVideo(
	roomSlug: string,
	signal?: AbortSignal
): Promise<ApiWeeklyVideo | null> {
	try {
		return await api.get<ApiWeeklyVideo>(
			`/api/weekly-video/${roomSlug}`,
			createRequestOptions(signal, CACHE_CONFIGS.video)
		);
	} catch (error) {
		// Return null for 404 (no video), re-throw other errors
		if (isApiError(error) && error.statusCode === 404) {
			return null;
		}
		throw error;
	}
}

// =============================================================================
// ADMIN API
// =============================================================================

/**
 * Check if current user is an admin
 *
 * @returns Boolean indicating admin status
 */
export async function checkAdminStatus(): Promise<boolean> {
	try {
		const data = await api.get<{
			is_admin?: boolean;
			role?: string;
		}>('/api/auth/me', createRequestOptions(undefined, CACHE_CONFIGS.user));

		return data.is_admin === true || data.role === 'admin' || data.role === 'super_admin';
	} catch {
		// Not authenticated or error - not admin
		return false;
	}
}

// =============================================================================
// CACHE INVALIDATION
// =============================================================================

/**
 * Invalidate all explosive swings caches
 * Call after mutations that affect dashboard data
 */
export function invalidateAllCaches(): void {
	if (!browser) return;

	const cache = getCache();
	cache.invalidateByTags([
		CACHE_TAGS.ALERTS,
		CACHE_TAGS.TRADES,
		CACHE_TAGS.STATS,
		CACHE_TAGS.TRADE_PLAN
	]);
}

/**
 * Invalidate specific cache types
 */
export function invalidateCache(
	types: Array<'alerts' | 'trades' | 'stats' | 'tradePlan' | 'video' | 'user'>
): void {
	if (!browser) return;

	const cache = getCache();
	const tags = types.map((type) => {
		switch (type) {
			case 'alerts':
				return CACHE_TAGS.ALERTS;
			case 'trades':
				return CACHE_TAGS.TRADES;
			case 'stats':
				return CACHE_TAGS.STATS;
			case 'tradePlan':
				return CACHE_TAGS.TRADE_PLAN;
			case 'video':
				return CACHE_TAGS.VIDEO;
			case 'user':
				return CACHE_TAGS.USER;
		}
	});

	cache.invalidateByTags(tags);
}

// =============================================================================
// BATCH FETCH - Load all dashboard data efficiently
// =============================================================================

/**
 * Dashboard data bundle type
 */
export interface DashboardData {
	alerts: AlertsResponse;
	tradePlan: TradePlanEntry[];
	stats: QuickStats;
	trades: ApiTrade[];
	weeklyVideo: ApiWeeklyVideo | null;
	isAdmin: boolean;
}

/**
 * Fetch all dashboard data in parallel
 *
 * @param roomSlug - Trading room identifier
 * @param signal - Optional AbortSignal for cancellation
 * @returns Complete dashboard data bundle
 */
export async function fetchDashboardData(
	roomSlug: string,
	signal?: AbortSignal
): Promise<DashboardData> {
	// Run all fetches in parallel for optimal performance
	const [alerts, tradePlan, stats, trades, weeklyVideo, isAdmin] = await Promise.all([
		fetchAlerts(roomSlug, 1, 20, signal),
		fetchTradePlan(roomSlug, signal),
		fetchStats(roomSlug, signal),
		fetchAllTrades(roomSlug, signal),
		fetchWeeklyVideo(roomSlug, signal),
		checkAdminStatus()
	]);

	return {
		alerts,
		tradePlan,
		stats,
		trades,
		weeklyVideo,
		isAdmin
	};
}

// =============================================================================
// FORMATTERS - Transform API responses to internal types
// =============================================================================

function formatAlert(apiAlert: Record<string, unknown>): FormattedAlert {
	return {
		id: apiAlert.id as number,
		type: apiAlert.alert_type as 'ENTRY' | 'EXIT' | 'UPDATE',
		ticker: apiAlert.ticker as string,
		title: apiAlert.title as string,
		time: formatTimeAgo(apiAlert.published_at as string),
		message: apiAlert.message as string,
		isNew: apiAlert.is_new as boolean,
		notes: (apiAlert.notes as string) || '',
		tosString: (apiAlert.tos_string as string) || undefined
	};
}

function formatTradePlanEntry(apiEntry: Record<string, unknown>): TradePlanEntry {
	return {
		ticker: apiEntry.ticker as string,
		bias: apiEntry.bias as 'BULLISH' | 'BEARISH' | 'NEUTRAL',
		entry: apiEntry.entry as string,
		target1: apiEntry.target1 as string,
		target2: apiEntry.target2 as string,
		target3: apiEntry.target3 as string,
		runner: apiEntry.runner as string,
		stop: apiEntry.stop as string,
		optionsStrike: (apiEntry.options_strike as string) || '-',
		optionsExp: (apiEntry.options_exp as string) || '-',
		notes: (apiEntry.notes as string) || ''
	};
}

// formatTimeAgo imported from './utils/formatters' - ICT 7 Single Source of Truth

// =============================================================================
// ERROR UTILITIES
// =============================================================================

/**
 * Get user-friendly error message from any error
 */
export { getUserFriendlyMessage };

/**
 * Check if error is an API error
 */
export { isApiError };

/**
 * Export error types for consumers
 */
export { ApiError };

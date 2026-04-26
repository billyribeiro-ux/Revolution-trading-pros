/**
 * ═══════════════════════════════════════════════════════════════════════════
 * Explosive Swings — Remote Functions (Queries)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Server-side query functions for the Explosive Swings dashboard.
 * These run on the server and are called from client components
 * via auto-generated fetch wrappers.
 *
 * @version 1.0.0
 * @boundary Remote Functions (query / query.batch)
 */

import * as v from 'valibot';
import { query } from '$app/server';
import {
	axumAlerts,
	axumTrades,
	axumTradePlans,
	axumStats,
	axumVideos,
	axumAuth
} from '$lib/server/axum';
import {
	FetchAlertsInputSchema,
	FetchTradePlanInputSchema,
	FetchStatsInputSchema,
	FetchTradesInputSchema,
	FetchWeeklyVideoInputSchema
} from '$lib/shared/schemas/trades';
import { formatTimeAgo } from './utils/formatters';
import type {
	QuickStats,
	TradePlanEntry,
	ApiTrade,
	ApiWeeklyVideo,
	PaginationState
} from './types';

// ═══════════════════════════════════════════════════════════════════════════
// Response Types
// ═══════════════════════════════════════════════════════════════════════════

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

export interface AlertsResponse {
	alerts: FormattedAlert[];
	pagination: PaginationState;
}

export interface DashboardData {
	alerts: AlertsResponse;
	tradePlan: TradePlanEntry[];
	stats: QuickStats;
	trades: ApiTrade[];
	weeklyVideo: ApiWeeklyVideo | null;
	isAdmin: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════
// Formatters — Transform Axum snake_case to camelCase
// ═══════════════════════════════════════════════════════════════════════════

function formatAlert(raw: {
	id: number;
	alert_type: string;
	ticker: string;
	title: string;
	published_at: string;
	message: string;
	is_new: boolean;
	notes: string | null;
	tos_string: string | null;
}): FormattedAlert {
	return {
		id: raw.id,
		type: raw.alert_type as 'ENTRY' | 'EXIT' | 'UPDATE',
		ticker: raw.ticker,
		title: raw.title,
		time: formatTimeAgo(raw.published_at),
		message: raw.message,
		isNew: raw.is_new,
		notes: raw.notes || '',
		tosString: raw.tos_string || undefined
	};
}

function formatTradePlanEntry(raw: {
	ticker: string;
	bias: string;
	entry: string;
	target1: string;
	target2: string;
	target3: string;
	runner: string;
	stop: string;
	options_strike: string | null;
	options_exp: string | null;
	notes: string | null;
}): TradePlanEntry {
	return {
		ticker: raw.ticker,
		bias: raw.bias as 'BULLISH' | 'BEARISH' | 'NEUTRAL',
		entry: raw.entry,
		target1: raw.target1,
		target2: raw.target2,
		target3: raw.target3,
		runner: raw.runner,
		stop: raw.stop,
		optionsStrike: raw.options_strike || '-',
		optionsExp: raw.options_exp || '-',
		notes: raw.notes || ''
	};
}

function formatStats(raw: {
	win_rate: number;
	weekly_profit: string;
	active_trades: number;
	closed_this_week: number;
}): QuickStats {
	return {
		winRate: raw.win_rate,
		weeklyProfit: raw.weekly_profit,
		activeTrades: raw.active_trades,
		closedThisWeek: raw.closed_this_week
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// Remote Queries
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Fetch paginated alerts for a trading room.
 */
export const getAlerts = query(
	FetchAlertsInputSchema,
	async ({ roomSlug, page, limit }): Promise<AlertsResponse> => {
		const offset = (page - 1) * limit;
		const data = await axumAlerts.fetchAlerts(roomSlug, limit, offset);

		return {
			alerts: data.data.map(formatAlert),
			pagination: {
				total: data.total,
				limit,
				offset
			}
		};
	}
);

/**
 * Fetch trade plan entries for a trading room.
 */
export const getTradePlan = query(
	FetchTradePlanInputSchema,
	async ({ roomSlug }): Promise<TradePlanEntry[]> => {
		const data = await axumTradePlans.fetchTradePlan(roomSlug);
		return data.map(formatTradePlanEntry);
	}
);

/**
 * Fetch quick stats for a trading room.
 */
export const getStats = query(FetchStatsInputSchema, async ({ roomSlug }): Promise<QuickStats> => {
	const raw = await axumStats.fetchStats(roomSlug);
	return formatStats(raw);
});

/**
 * Fetch all trades for a trading room.
 */
export const getTrades = query(
	FetchTradesInputSchema,
	async ({ roomSlug }): Promise<ApiTrade[]> => {
		return axumTrades.fetchTrades(roomSlug);
	}
);

/**
 * Fetch weekly video for a trading room.
 * Returns null if no video exists (404).
 */
export const getWeeklyVideo = query(
	FetchWeeklyVideoInputSchema,
	async ({ roomSlug }): Promise<ApiWeeklyVideo | null> => {
		return axumVideos.fetchWeeklyVideo(roomSlug);
	}
);

/**
 * Check if the current user is an admin.
 */
export const getAdminStatus = query(async (): Promise<boolean> => {
	return axumAuth.checkAdminStatus();
});

/**
 * Fetch all dashboard data in parallel (batched read).
 * Used for initial dashboard hydration.
 */
export const getDashboardData = query(
	v.object({ roomSlug: v.pipe(v.string(), v.nonEmpty()) }),
	async ({ roomSlug }): Promise<DashboardData> => {
		const [alerts, tradePlan, stats, trades, weeklyVideo, isAdmin] = await Promise.all([
			axumAlerts.fetchAlerts(roomSlug, 20, 0).then((data) => ({
				alerts: data.data.map(formatAlert),
				pagination: { total: data.total, limit: 20, offset: 0 }
			})),
			axumTradePlans.fetchTradePlan(roomSlug).then((data) => data.map(formatTradePlanEntry)),
			axumStats.fetchStats(roomSlug).then(formatStats),
			axumTrades.fetchTrades(roomSlug),
			axumVideos.fetchWeeklyVideo(roomSlug),
			axumAuth.checkAdminStatus()
		]);

		return { alerts, tradePlan, stats, trades, weeklyVideo, isAdmin };
	}
);

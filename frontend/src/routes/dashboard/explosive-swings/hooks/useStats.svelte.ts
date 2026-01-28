/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Explosive Swings - useStats Hook
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Svelte 5 hook for managing trading room statistics
 * @version 1.0.0
 * @standards Svelte 5 January 2026 Syntax
 *
 * Features:
 * - Fetch room statistics (win rate, trades, profit)
 * - Derived performance calculations
 * - Auto-refresh capability
 * - Fallback data support
 */

import { getEnterpriseClient } from '$lib/api/enterprise/client';
import { ROOM_SLUG } from '../constants';
import type { QuickStats, WeeklyPerformance } from '../types';

// ═══════════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════════

export interface UseStatsOptions {
	/** Enable auto-refresh (default: false) */
	autoRefresh?: boolean;
	/** Auto-refresh interval in ms (default: 60000) */
	refreshInterval?: number;
	/** Fallback stats when API fails */
	fallbackStats?: QuickStats;
}

export interface UseStatsReturn {
	// State (read-only via getters)
	readonly stats: QuickStats | null;
	readonly weeklyPerformance: WeeklyPerformance | null;
	readonly isLoading: boolean;
	readonly error: string | null;
	readonly hasData: boolean;

	// Derived stats
	readonly winRate: number;
	readonly activeTrades: number;
	readonly closedThisWeek: number;
	readonly weeklyProfit: string;
	readonly totalTradesThisWeek: number;

	// Actions
	fetchStats: () => Promise<void>;
	refresh: () => Promise<void>;
}

// ═══════════════════════════════════════════════════════════════════════════════
// DEFAULT FALLBACK DATA
// ═══════════════════════════════════════════════════════════════════════════════

const defaultFallbackStats: QuickStats = {
	winRate: 0,
	weeklyProfit: '$0',
	activeTrades: 0,
	closedThisWeek: 0
};

const defaultWeeklyPerformance: WeeklyPerformance = {
	winRate: 0,
	totalTrades: 0,
	winningTrades: 0,
	avgWinPercent: 0,
	avgLossPercent: 0,
	riskRewardRatio: 0
};

// ═══════════════════════════════════════════════════════════════════════════════
// HOOK IMPLEMENTATION
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Creates a reactive stats state manager for the Explosive Swings dashboard.
 * Uses Svelte 5 runes for reactive state management.
 */
export function useStats(options: UseStatsOptions = {}): UseStatsReturn {
	const {
		autoRefresh = false,
		refreshInterval = 60000,
		fallbackStats = defaultFallbackStats
	} = options;

	// ═══════════════════════════════════════════════════════════════════════════
	// REACTIVE STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let apiStats = $state<QuickStats | null>(null);
	let isLoading = $state(false);
	let error = $state<string | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const stats = $derived<QuickStats | null>(apiStats ?? fallbackStats);
	const hasData = $derived(apiStats !== null);

	// Individual stat getters with fallback
	const winRate = $derived(stats?.winRate ?? 0);
	const activeTrades = $derived(stats?.activeTrades ?? 0);
	const closedThisWeek = $derived(stats?.closedThisWeek ?? 0);
	const weeklyProfit = $derived(stats?.weeklyProfit ?? '$0');
	const totalTradesThisWeek = $derived(activeTrades + closedThisWeek);

	// Weekly performance derived from stats
	const weeklyPerformance = $derived.by<WeeklyPerformance | null>(() => {
		if (!stats) return defaultWeeklyPerformance;

		const totalTrades = stats.activeTrades + stats.closedThisWeek;
		const winningTrades = Math.round((stats.winRate / 100) * totalTrades);

		return {
			winRate: stats.winRate,
			totalTrades,
			winningTrades,
			avgWinPercent: 5.7, // TODO: Calculate from actual data
			avgLossPercent: 2.1, // TODO: Calculate from actual data
			riskRewardRatio: 2.7 // TODO: Calculate from actual data
		};
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// API CLIENT
	// ═══════════════════════════════════════════════════════════════════════════

	const client = getEnterpriseClient();

	// ═══════════════════════════════════════════════════════════════════════════
	// ACTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function fetchStats(): Promise<void> {
		isLoading = true;
		error = null;

		try {
			const response = await client.get<{
				success: boolean;
				data: {
					win_rate: number;
					weekly_profit: string;
					active_trades: number;
					closed_this_week: number;
				};
				error?: string;
			}>(`/api/stats/${ROOM_SLUG}`);

			if (!response.success) {
				throw new Error(response.error || 'Failed to fetch stats');
			}

			apiStats = {
				winRate: response.data.win_rate,
				weeklyProfit: response.data.weekly_profit,
				activeTrades: response.data.active_trades,
				closedThisWeek: response.data.closed_this_week
			};
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to fetch stats';
			console.error('Failed to fetch stats:', e);
		} finally {
			isLoading = false;
		}
	}

	async function refresh(): Promise<void> {
		await fetchStats();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	// Auto-refresh effect
	$effect(() => {
		if (!autoRefresh) return;

		const interval = setInterval(fetchStats, refreshInterval);
		return () => clearInterval(interval);
	});

	// Initial fetch effect
	$effect(() => {
		fetchStats();
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// RETURN PUBLIC API
	// ═══════════════════════════════════════════════════════════════════════════

	return {
		// State (read-only via getters)
		get stats() {
			return stats;
		},
		get weeklyPerformance() {
			return weeklyPerformance;
		},
		get isLoading() {
			return isLoading;
		},
		get error() {
			return error;
		},
		get hasData() {
			return hasData;
		},

		// Derived stats
		get winRate() {
			return winRate;
		},
		get activeTrades() {
			return activeTrades;
		},
		get closedThisWeek() {
			return closedThisWeek;
		},
		get weeklyProfit() {
			return weeklyProfit;
		},
		get totalTradesThisWeek() {
			return totalTradesThisWeek;
		},

		// Actions
		fetchStats,
		refresh
	};
}

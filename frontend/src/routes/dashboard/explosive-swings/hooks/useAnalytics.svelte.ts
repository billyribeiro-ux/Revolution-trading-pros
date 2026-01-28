/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Explosive Swings - useAnalytics Hook
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Svelte 5 hook for managing trading analytics and performance data
 * @version 1.0.0
 * @standards Svelte 5 January 2026 Syntax
 *
 * Features:
 * - Historical performance tracking
 * - Win/loss analysis
 * - Streak calculations
 * - Performance comparisons
 * - Time-based analytics (daily, weekly, monthly)
 */

import { getEnterpriseClient } from '$lib/api/enterprise/client';
import { ROOM_SLUG } from '../constants';

// ═══════════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════════

export interface UseAnalyticsOptions {
	/** Time period for analytics (default: '30d') */
	period?: AnalyticsPeriod;
	/** Enable auto-refresh (default: false) */
	autoRefresh?: boolean;
	/** Auto-refresh interval in ms (default: 300000 / 5 min) */
	refreshInterval?: number;
}

export type AnalyticsPeriod = '7d' | '30d' | '90d' | 'ytd' | 'all';

export interface PerformanceMetrics {
	/** Total number of trades in period */
	totalTrades: number;
	/** Number of winning trades */
	winningTrades: number;
	/** Number of losing trades */
	losingTrades: number;
	/** Win rate percentage (0-100) */
	winRate: number;
	/** Average win percentage */
	avgWinPercent: number;
	/** Average loss percentage */
	avgLossPercent: number;
	/** Profit factor (gross profit / gross loss) */
	profitFactor: number;
	/** Risk/reward ratio */
	riskRewardRatio: number;
	/** Current win streak */
	currentStreak: number;
	/** Is current streak winning? */
	isWinningStreak: boolean;
	/** Best win streak */
	bestWinStreak: number;
	/** Worst loss streak */
	worstLossStreak: number;
	/** Largest winning trade percentage */
	largestWin: number;
	/** Largest losing trade percentage */
	largestLoss: number;
}

export interface DailyPerformance {
	date: string;
	trades: number;
	wins: number;
	losses: number;
	netPnlPercent: number;
}

export interface TickerPerformance {
	ticker: string;
	trades: number;
	wins: number;
	losses: number;
	winRate: number;
	avgPnlPercent: number;
	totalPnlPercent: number;
}

export interface UseAnalyticsReturn {
	// State (read-only via getters)
	readonly metrics: PerformanceMetrics | null;
	readonly dailyPerformance: DailyPerformance[];
	readonly tickerPerformance: TickerPerformance[];
	readonly isLoading: boolean;
	readonly error: string | null;
	readonly selectedPeriod: AnalyticsPeriod;
	readonly hasData: boolean;

	// Derived analytics
	readonly isOnWinningStreak: boolean;
	readonly currentStreakCount: number;
	readonly performanceTrend: 'improving' | 'declining' | 'stable';
	readonly topPerformingTicker: TickerPerformance | null;
	readonly worstPerformingTicker: TickerPerformance | null;

	// Actions
	fetchAnalytics: () => Promise<void>;
	setPeriod: (period: AnalyticsPeriod) => void;
	refresh: () => Promise<void>;
}

// ═══════════════════════════════════════════════════════════════════════════════
// DEFAULT DATA
// ═══════════════════════════════════════════════════════════════════════════════

const defaultMetrics: PerformanceMetrics = {
	totalTrades: 0,
	winningTrades: 0,
	losingTrades: 0,
	winRate: 0,
	avgWinPercent: 0,
	avgLossPercent: 0,
	profitFactor: 0,
	riskRewardRatio: 0,
	currentStreak: 0,
	isWinningStreak: false,
	bestWinStreak: 0,
	worstLossStreak: 0,
	largestWin: 0,
	largestLoss: 0
};

// ═══════════════════════════════════════════════════════════════════════════════
// HOOK IMPLEMENTATION
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Creates a reactive analytics state manager for the Explosive Swings dashboard.
 * Uses Svelte 5 runes for reactive state management.
 */
export function useAnalytics(options: UseAnalyticsOptions = {}): UseAnalyticsReturn {
	const { period = '30d', autoRefresh = false, refreshInterval = 300000 } = options;

	// ═══════════════════════════════════════════════════════════════════════════
	// REACTIVE STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let metrics = $state<PerformanceMetrics | null>(null);
	let dailyPerformance = $state<DailyPerformance[]>([]);
	let tickerPerformance = $state<TickerPerformance[]>([]);
	let isLoading = $state(false);
	let error = $state<string | null>(null);
	let selectedPeriod = $state<AnalyticsPeriod>(period);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const hasData = $derived(metrics !== null);

	const isOnWinningStreak = $derived(metrics?.isWinningStreak ?? false);
	const currentStreakCount = $derived(metrics?.currentStreak ?? 0);

	// Calculate performance trend based on recent daily data
	const performanceTrend = $derived.by<'improving' | 'declining' | 'stable'>(() => {
		if (dailyPerformance.length < 3) return 'stable';

		const recent = dailyPerformance.slice(-7);
		const firstHalf = recent.slice(0, Math.floor(recent.length / 2));
		const secondHalf = recent.slice(Math.floor(recent.length / 2));

		const firstHalfAvg =
			firstHalf.reduce((sum, d) => sum + d.netPnlPercent, 0) / (firstHalf.length || 1);
		const secondHalfAvg =
			secondHalf.reduce((sum, d) => sum + d.netPnlPercent, 0) / (secondHalf.length || 1);

		const diff = secondHalfAvg - firstHalfAvg;
		if (diff > 1) return 'improving';
		if (diff < -1) return 'declining';
		return 'stable';
	});

	// Top and worst performing tickers
	const topPerformingTicker = $derived.by<TickerPerformance | null>(() => {
		if (tickerPerformance.length === 0) return null;
		return [...tickerPerformance].sort((a, b) => b.totalPnlPercent - a.totalPnlPercent)[0];
	});

	const worstPerformingTicker = $derived.by<TickerPerformance | null>(() => {
		if (tickerPerformance.length === 0) return null;
		return [...tickerPerformance].sort((a, b) => a.totalPnlPercent - b.totalPnlPercent)[0];
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// API CLIENT
	// ═══════════════════════════════════════════════════════════════════════════

	const client = getEnterpriseClient();

	// ═══════════════════════════════════════════════════════════════════════════
	// ACTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function fetchAnalytics(): Promise<void> {
		isLoading = true;
		error = null;

		try {
			const response = await client.get<{
				success: boolean;
				data: {
					metrics: {
						total_trades: number;
						winning_trades: number;
						losing_trades: number;
						win_rate: number;
						avg_win_percent: number;
						avg_loss_percent: number;
						profit_factor: number;
						risk_reward_ratio: number;
						current_streak: number;
						is_winning_streak: boolean;
						best_win_streak: number;
						worst_loss_streak: number;
						largest_win: number;
						largest_loss: number;
					};
					daily_performance: Array<{
						date: string;
						trades: number;
						wins: number;
						losses: number;
						net_pnl_percent: number;
					}>;
					ticker_performance: Array<{
						ticker: string;
						trades: number;
						wins: number;
						losses: number;
						win_rate: number;
						avg_pnl_percent: number;
						total_pnl_percent: number;
					}>;
				};
				error?: string;
			}>(`/api/analytics/${ROOM_SLUG}`, {
				params: { period: selectedPeriod }
			});

			if (!response.success) {
				throw new Error(response.error || 'Failed to fetch analytics');
			}

			// Transform API response to internal types
			metrics = {
				totalTrades: response.data.metrics.total_trades,
				winningTrades: response.data.metrics.winning_trades,
				losingTrades: response.data.metrics.losing_trades,
				winRate: response.data.metrics.win_rate,
				avgWinPercent: response.data.metrics.avg_win_percent,
				avgLossPercent: response.data.metrics.avg_loss_percent,
				profitFactor: response.data.metrics.profit_factor,
				riskRewardRatio: response.data.metrics.risk_reward_ratio,
				currentStreak: response.data.metrics.current_streak,
				isWinningStreak: response.data.metrics.is_winning_streak,
				bestWinStreak: response.data.metrics.best_win_streak,
				worstLossStreak: response.data.metrics.worst_loss_streak,
				largestWin: response.data.metrics.largest_win,
				largestLoss: response.data.metrics.largest_loss
			};

			dailyPerformance = response.data.daily_performance.map((d) => ({
				date: d.date,
				trades: d.trades,
				wins: d.wins,
				losses: d.losses,
				netPnlPercent: d.net_pnl_percent
			}));

			tickerPerformance = response.data.ticker_performance.map((t) => ({
				ticker: t.ticker,
				trades: t.trades,
				wins: t.wins,
				losses: t.losses,
				winRate: t.win_rate,
				avgPnlPercent: t.avg_pnl_percent,
				totalPnlPercent: t.total_pnl_percent
			}));
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to fetch analytics';
			console.error('Failed to fetch analytics:', e);

			// Set defaults on error
			metrics = defaultMetrics;
			dailyPerformance = [];
			tickerPerformance = [];
		} finally {
			isLoading = false;
		}
	}

	function setPeriod(newPeriod: AnalyticsPeriod): void {
		if (newPeriod === selectedPeriod) return;
		selectedPeriod = newPeriod;
		fetchAnalytics();
	}

	async function refresh(): Promise<void> {
		await fetchAnalytics();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	// Auto-refresh effect
	$effect(() => {
		if (!autoRefresh) return;

		const interval = setInterval(fetchAnalytics, refreshInterval);
		return () => clearInterval(interval);
	});

	// Initial fetch effect
	$effect(() => {
		fetchAnalytics();
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// RETURN PUBLIC API
	// ═══════════════════════════════════════════════════════════════════════════

	return {
		// State (read-only via getters)
		get metrics() {
			return metrics;
		},
		get dailyPerformance() {
			return dailyPerformance;
		},
		get tickerPerformance() {
			return tickerPerformance;
		},
		get isLoading() {
			return isLoading;
		},
		get error() {
			return error;
		},
		get selectedPeriod() {
			return selectedPeriod;
		},
		get hasData() {
			return hasData;
		},

		// Derived analytics
		get isOnWinningStreak() {
			return isOnWinningStreak;
		},
		get currentStreakCount() {
			return currentStreakCount;
		},
		get performanceTrend() {
			return performanceTrend;
		},
		get topPerformingTicker() {
			return topPerformingTicker;
		},
		get worstPerformingTicker() {
			return worstPerformingTicker;
		},

		// Actions
		fetchAnalytics,
		setPeriod,
		refresh
	};
}

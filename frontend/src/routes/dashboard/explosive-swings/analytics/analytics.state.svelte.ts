/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Explosive Swings - Analytics State Module
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Centralized state management for Analytics Dashboard using Svelte 5 runes
 * @version 2.0.0 - Phase 4 Analytics Dashboard
 * @standards Svelte 5 January 2026 Syntax | Apple Principal Engineer ICT 7+
 *
 * This module exports a factory function that creates all reactive state
 * for the Explosive Swings Analytics Dashboard with comprehensive metrics.
 */

import { browser } from '$app/environment';
import { ROOM_SLUG } from '../constants';

// ═══════════════════════════════════════════════════════════════════════════
// TYPE DEFINITIONS - Matching Backend API Response
// ═══════════════════════════════════════════════════════════════════════════

export interface DateRange {
	from: string;
	to: string;
}

export interface AnalyticsSummary {
	total_alerts: number;
	total_trades: number;
	win_rate: number;
	profit_factor: number;
	sharpe_ratio: number;
	max_drawdown: number;
	max_drawdown_percent: number;
	avg_holding_days: number;
	total_pnl: number;
	total_pnl_percent: number;
	best_month: MonthlyPerformance | null;
	worst_month: MonthlyPerformance | null;
	avg_win_percent: number;
	avg_loss_percent: number;
	largest_win_percent: number;
	largest_loss_percent: number;
	risk_reward_ratio: number;
	expectancy: number;
}

export interface TickerPerformance {
	ticker: string;
	total_trades: number;
	wins: number;
	losses: number;
	win_rate: number;
	total_pnl: number;
	total_pnl_percent: number;
	avg_pnl: number;
	avg_pnl_percent: number;
	avg_holding_days: number;
	largest_win_percent: number;
	largest_loss_percent: number;
}

export interface SetupPerformance {
	setup: string;
	total_trades: number;
	wins: number;
	losses: number;
	win_rate: number;
	total_pnl: number;
	avg_pnl: number;
	profit_factor: number;
}

export interface MonthlyPerformance {
	year: number;
	month: number;
	month_name: string;
	total_trades: number;
	wins: number;
	losses: number;
	win_rate: number;
	pnl: number;
	pnl_percent: number;
	is_positive: boolean;
}

export interface DailyPnL {
	date: string;
	pnl: number;
	pnl_percent: number;
	cumulative_pnl: number;
	cumulative_pnl_percent: number;
	trade_count: number;
}

export interface AlertEffectiveness {
	total_alerts: number;
	alerts_with_trades: number;
	alerts_without_trades: number;
	conversion_rate: number;
	profitable_conversion_rate: number;
	avg_time_to_trade_hours: number;
	entry_alerts: number;
	update_alerts: number;
	exit_alerts: number;
}

export interface StreakAnalysis {
	current_streak: number;
	current_streak_type: string;
	max_win_streak: number;
	max_loss_streak: number;
	avg_win_streak: number;
	avg_loss_streak: number;
}

export interface EquityPoint {
	date: string;
	equity: number;
	equity_percent: number;
	drawdown: number;
	drawdown_percent: number;
	trade_count: number;
}

export interface DrawdownPeriod {
	start_date: string;
	end_date: string | null;
	recovery_date: string | null;
	max_drawdown: number;
	max_drawdown_percent: number;
	duration_days: number;
	recovery_days: number | null;
	is_recovered: boolean;
}

export interface RoomAnalytics {
	summary: AnalyticsSummary;
	performance_by_ticker: TickerPerformance[];
	performance_by_setup: SetupPerformance[];
	monthly_performance: MonthlyPerformance[];
	daily_pnl: DailyPnL[];
	alert_effectiveness: AlertEffectiveness;
	streak_analysis: StreakAnalysis;
}

export type TimePeriod = '30d' | '90d' | '180d' | '365d' | 'ytd' | 'all';

// ═══════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════

const API_BASE = '/api/analytics/room';

/**
 * Get date range based on preset period
 */
function getDateRangeForPeriod(period: TimePeriod): DateRange {
	const to = new Date();
	const from = new Date();

	switch (period) {
		case '30d':
			from.setDate(from.getDate() - 30);
			break;
		case '90d':
			from.setDate(from.getDate() - 90);
			break;
		case '180d':
			from.setDate(from.getDate() - 180);
			break;
		case '365d':
			from.setFullYear(from.getFullYear() - 1);
			break;
		case 'ytd':
			from.setMonth(0, 1);
			break;
		case 'all':
			from.setFullYear(2020, 0, 1);
			break;
	}

	return {
		from: from.toISOString().split('T')[0],
		to: to.toISOString().split('T')[0]
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// DEFAULT VALUES
// ═══════════════════════════════════════════════════════════════════════════

const defaultSummary: AnalyticsSummary = {
	total_alerts: 0,
	total_trades: 0,
	win_rate: 0,
	profit_factor: 0,
	sharpe_ratio: 0,
	max_drawdown: 0,
	max_drawdown_percent: 0,
	avg_holding_days: 0,
	total_pnl: 0,
	total_pnl_percent: 0,
	best_month: null,
	worst_month: null,
	avg_win_percent: 0,
	avg_loss_percent: 0,
	largest_win_percent: 0,
	largest_loss_percent: 0,
	risk_reward_ratio: 0,
	expectancy: 0
};

const defaultAlertEffectiveness: AlertEffectiveness = {
	total_alerts: 0,
	alerts_with_trades: 0,
	alerts_without_trades: 0,
	conversion_rate: 0,
	profitable_conversion_rate: 0,
	avg_time_to_trade_hours: 0,
	entry_alerts: 0,
	update_alerts: 0,
	exit_alerts: 0
};

const defaultStreakAnalysis: StreakAnalysis = {
	current_streak: 0,
	current_streak_type: 'none',
	max_win_streak: 0,
	max_loss_streak: 0,
	avg_win_streak: 0,
	avg_loss_streak: 0
};

// ═══════════════════════════════════════════════════════════════════════════
// STATE FACTORY
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Creates the reactive state store for the Analytics Dashboard.
 * Call this once in +page.svelte.
 */
export function createAnalyticsState() {
	// ═══════════════════════════════════════════════════════════════════════════
	// FILTER STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let selectedPeriod = $state<TimePeriod>('365d');
	let dateRange = $state<DateRange>(getDateRangeForPeriod('365d'));

	// ═══════════════════════════════════════════════════════════════════════════
	// DATA STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let summary = $state<AnalyticsSummary>(defaultSummary);
	let tickerPerformance = $state<TickerPerformance[]>([]);
	let setupPerformance = $state<SetupPerformance[]>([]);
	let monthlyPerformance = $state<MonthlyPerformance[]>([]);
	let dailyPnl = $state<DailyPnL[]>([]);
	let alertEffectiveness = $state<AlertEffectiveness>(defaultAlertEffectiveness);
	let streakAnalysis = $state<StreakAnalysis>(defaultStreakAnalysis);
	let equityCurve = $state<EquityPoint[]>([]);
	let drawdowns = $state<DrawdownPeriod[]>([]);

	// ═══════════════════════════════════════════════════════════════════════════
	// LOADING STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let isLoading = $state(false);
	let isLoadingEquityCurve = $state(false);
	let isLoadingDrawdowns = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════
	// ERROR STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let error = $state<string | null>(null);
	let equityCurveError = $state<string | null>(null);
	let drawdownsError = $state<string | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const hasData = $derived(summary.total_trades > 0);

	const periodLabel = $derived(
		{
			'30d': 'Last 30 Days',
			'90d': 'Last 90 Days',
			'180d': 'Last 6 Months',
			'365d': 'Last Year',
			ytd: 'Year to Date',
			all: 'All Time'
		}[selectedPeriod]
	);

	// Top performing tickers (by total P&L %)
	const topTickers = $derived(
		[...tickerPerformance].sort((a, b) => b.total_pnl_percent - a.total_pnl_percent).slice(0, 10)
	);

	// Worst performing tickers
	const worstTickers = $derived(
		[...tickerPerformance].sort((a, b) => a.total_pnl_percent - b.total_pnl_percent).slice(0, 5)
	);

	// Best and worst months
	const bestMonth = $derived(
		monthlyPerformance.length > 0
			? [...monthlyPerformance].sort((a, b) => b.pnl_percent - a.pnl_percent)[0]
			: null
	);

	const worstMonth = $derived(
		monthlyPerformance.length > 0
			? [...monthlyPerformance].sort((a, b) => a.pnl_percent - b.pnl_percent)[0]
			: null
	);

	// Has any error
	const hasAnyError = $derived(
		error !== null || equityCurveError !== null || drawdownsError !== null
	);
	const isInitialLoading = $derived(isLoading && !hasData);

	// ═══════════════════════════════════════════════════════════════════════════
	// API ACTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function fetchAnalytics() {
		if (!browser) return;

		isLoading = true;
		error = null;

		try {
			const params = new URLSearchParams({
				from: dateRange.from,
				to: dateRange.to
			});

			const response = await fetch(`${API_BASE}/${ROOM_SLUG}?${params}`, {
				credentials: 'include'
			});

			if (!response.ok) {
				throw new Error(`HTTP ${response.status}: Failed to fetch analytics`);
			}

			const data = await response.json();

			if (!data.success) {
				throw new Error(data.error || 'Failed to fetch analytics');
			}

			const analytics = data.data as RoomAnalytics;
			summary = analytics.summary;
			tickerPerformance = analytics.performance_by_ticker;
			setupPerformance = analytics.performance_by_setup;
			monthlyPerformance = analytics.monthly_performance;
			dailyPnl = analytics.daily_pnl;
			alertEffectiveness = analytics.alert_effectiveness;
			streakAnalysis = analytics.streak_analysis;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load analytics';
			console.error('[analytics] Failed to fetch analytics:', err);
		} finally {
			isLoading = false;
		}
	}

	async function fetchEquityCurve() {
		if (!browser) return;

		isLoadingEquityCurve = true;
		equityCurveError = null;

		try {
			const params = new URLSearchParams({
				from: dateRange.from,
				to: dateRange.to
			});

			const response = await fetch(`${API_BASE}/${ROOM_SLUG}/equity-curve?${params}`, {
				credentials: 'include'
			});

			if (!response.ok) {
				throw new Error(`HTTP ${response.status}: Failed to fetch equity curve`);
			}

			const data = await response.json();

			if (!data.success) {
				throw new Error(data.error || 'Failed to fetch equity curve');
			}

			equityCurve = data.data;
		} catch (err) {
			equityCurveError = err instanceof Error ? err.message : 'Failed to load equity curve';
			console.error('[analytics] Failed to fetch equity curve:', err);
		} finally {
			isLoadingEquityCurve = false;
		}
	}

	async function fetchDrawdowns() {
		if (!browser) return;

		isLoadingDrawdowns = true;
		drawdownsError = null;

		try {
			const params = new URLSearchParams({
				from: dateRange.from,
				to: dateRange.to
			});

			const response = await fetch(`${API_BASE}/${ROOM_SLUG}/drawdowns?${params}`, {
				credentials: 'include'
			});

			if (!response.ok) {
				throw new Error(`HTTP ${response.status}: Failed to fetch drawdowns`);
			}

			const data = await response.json();

			if (!data.success) {
				throw new Error(data.error || 'Failed to fetch drawdowns');
			}

			drawdowns = data.data;
		} catch (err) {
			drawdownsError = err instanceof Error ? err.message : 'Failed to load drawdowns';
			console.error('[analytics] Failed to fetch drawdowns:', err);
		} finally {
			isLoadingDrawdowns = false;
		}
	}

	/**
	 * Fetch all analytics data
	 */
	async function fetchAll() {
		await Promise.all([fetchAnalytics(), fetchEquityCurve(), fetchDrawdowns()]);
	}

	/**
	 * Set period and refetch data
	 */
	function setPeriod(period: TimePeriod) {
		if (period === selectedPeriod) return;
		selectedPeriod = period;
		dateRange = getDateRangeForPeriod(period);
		fetchAll();
	}

	/**
	 * Set custom date range
	 */
	function setDateRange(range: DateRange) {
		dateRange = range;
		fetchAll();
	}

	/**
	 * Initialize dashboard data
	 */
	function initializeData() {
		fetchAll();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// RETURN PUBLIC API
	// ═══════════════════════════════════════════════════════════════════════════

	return {
		// Filter state
		get selectedPeriod() {
			return selectedPeriod;
		},
		get dateRange() {
			return dateRange;
		},
		get periodLabel() {
			return periodLabel;
		},

		// Data state
		get summary() {
			return summary;
		},
		get tickerPerformance() {
			return tickerPerformance;
		},
		get setupPerformance() {
			return setupPerformance;
		},
		get monthlyPerformance() {
			return monthlyPerformance;
		},
		get dailyPnl() {
			return dailyPnl;
		},
		get alertEffectiveness() {
			return alertEffectiveness;
		},
		get streakAnalysis() {
			return streakAnalysis;
		},
		get equityCurve() {
			return equityCurve;
		},
		get drawdowns() {
			return drawdowns;
		},

		// Derived state
		get hasData() {
			return hasData;
		},
		get topTickers() {
			return topTickers;
		},
		get worstTickers() {
			return worstTickers;
		},
		get bestMonth() {
			return bestMonth;
		},
		get worstMonth() {
			return worstMonth;
		},

		// Loading states
		get isLoading() {
			return isLoading;
		},
		get isLoadingEquityCurve() {
			return isLoadingEquityCurve;
		},
		get isLoadingDrawdowns() {
			return isLoadingDrawdowns;
		},
		get isInitialLoading() {
			return isInitialLoading;
		},

		// Error states
		get error() {
			return error;
		},
		get equityCurveError() {
			return equityCurveError;
		},
		get drawdownsError() {
			return drawdownsError;
		},
		get hasAnyError() {
			return hasAnyError;
		},

		// Actions
		initializeData,
		fetchAll,
		fetchAnalytics,
		fetchEquityCurve,
		fetchDrawdowns,
		setPeriod,
		setDateRange,

		// Constants
		ROOM_SLUG
	};
}

export type AnalyticsState = ReturnType<typeof createAnalyticsState>;

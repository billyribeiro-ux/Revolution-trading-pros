/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Explosive Swings Analytics - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description SSR pre-fetch for analytics data with graceful degradation
 * @version 1.0.0
 * @author Revolution Trading Pros Engineering
 * @standards Apple ICT 7+ | SvelteKit 2.0+ | Svelte 5 (January 2026)
 *
 * @architecture
 * - Server-side data fetching for SEO and initial load performance
 * - Parallel fetch for optimal performance
 * - Graceful degradation on API failures
 */

import { env } from '$env/dynamic/private';
import type { PageServerLoad } from './$types';
import { ROOM_SLUG } from '../constants';

// ═══════════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════════

interface PerformanceMetrics {
	winRate: number;
	profitFactor: number;
	totalTrades: number;
	winningTrades: number;
	losingTrades: number;
	avgWinPercent: number;
	avgLossPercent: number;
	largestWin: number;
	largestLoss: number;
	expectancy: number;
	sharpeRatio: number;
	maxDrawdown: number;
	totalPnlPercent: number;
}

interface EquityCurvePoint {
	date: string;
	cumulativePnl: number;
	tradeCount: number;
}

interface TickerPerformance {
	ticker: string;
	trades: number;
	winRate: number;
	totalPnlPercent: number;
	avgPnlPercent: number;
}

interface MonthlyReturn {
	month: string;
	year: number;
	pnlPercent: number;
	trades: number;
	winRate: number;
}

interface DrawdownPoint {
	date: string;
	drawdownPercent: number;
	peakValue: number;
	currentValue: number;
}

interface StreakInfo {
	currentStreak: number;
	streakType: 'win' | 'loss' | 'none';
	longestWinStreak: number;
	longestLossStreak: number;
	lastTradeDate: string | null;
}

interface AnalyticsPageData {
	performance: PerformanceMetrics | null;
	equityCurve: EquityCurvePoint[];
	tickerPerformance: TickerPerformance[];
	monthlyReturns: MonthlyReturn[];
	drawdown: DrawdownPoint[];
	streak: StreakInfo | null;
	period: string;
}

// ═══════════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════

const LOG_PREFIX = '[analytics-ssr]';
const DEFAULT_API_URL = 'https://revolution-trading-pros-api.fly.dev';
const DEFAULT_PERIOD = '30d';

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Safely fetch analytics data with error handling
 */
async function fetchAnalyticsData(
	fetchFn: typeof fetch,
	baseUrl: string,
	period: string
): Promise<Partial<AnalyticsPageData>> {
	try {
		const response = await fetchFn(
			`${baseUrl}/api/analytics/${ROOM_SLUG}?period=${period}`
		);

		if (!response.ok) {
			console.warn(`${LOG_PREFIX} Analytics fetch returned:`, response.status);
			return {};
		}

		const data = await response.json();

		if (!data.success || !data.data) {
			return {};
		}

		const apiData = data.data;

		return {
			performance: {
				winRate: apiData.win_rate ?? 0,
				profitFactor: apiData.profit_factor ?? 0,
				totalTrades: apiData.total_trades ?? 0,
				winningTrades: apiData.winning_trades ?? 0,
				losingTrades: apiData.losing_trades ?? 0,
				avgWinPercent: apiData.avg_win_percent ?? 0,
				avgLossPercent: apiData.avg_loss_percent ?? 0,
				largestWin: apiData.largest_win ?? 0,
				largestLoss: apiData.largest_loss ?? 0,
				expectancy: apiData.expectancy ?? 0,
				sharpeRatio: apiData.sharpe_ratio ?? 0,
				maxDrawdown: apiData.max_drawdown ?? 0,
				totalPnlPercent: apiData.total_pnl_percent ?? 0
			},
			equityCurve: Array.isArray(apiData.equity_curve)
				? apiData.equity_curve.map((point: Record<string, unknown>) => ({
						date: point.date ?? '',
						cumulativePnl: point.cumulative_pnl ?? 0,
						tradeCount: point.trade_count ?? 0
					}))
				: [],
			tickerPerformance: Array.isArray(apiData.ticker_performance)
				? apiData.ticker_performance.map((t: Record<string, unknown>) => ({
						ticker: t.ticker ?? '',
						trades: t.trades ?? 0,
						winRate: t.win_rate ?? 0,
						totalPnlPercent: t.total_pnl_percent ?? 0,
						avgPnlPercent: t.avg_pnl_percent ?? 0
					}))
				: [],
			monthlyReturns: Array.isArray(apiData.monthly_returns)
				? apiData.monthly_returns.map((m: Record<string, unknown>) => ({
						month: m.month ?? '',
						year: m.year ?? new Date().getFullYear(),
						pnlPercent: m.pnl_percent ?? 0,
						trades: m.trades ?? 0,
						winRate: m.win_rate ?? 0
					}))
				: [],
			drawdown: Array.isArray(apiData.drawdown)
				? apiData.drawdown.map((d: Record<string, unknown>) => ({
						date: d.date ?? '',
						drawdownPercent: d.drawdown_percent ?? 0,
						peakValue: d.peak_value ?? 0,
						currentValue: d.current_value ?? 0
					}))
				: [],
			streak: apiData.streak
				? {
						currentStreak: apiData.streak.current_streak ?? 0,
						streakType: (apiData.streak.streak_type ?? 'none') as 'win' | 'loss' | 'none',
						longestWinStreak: apiData.streak.longest_win_streak ?? 0,
						longestLossStreak: apiData.streak.longest_loss_streak ?? 0,
						lastTradeDate: apiData.streak.last_trade_date ?? null
					}
				: null
		};
	} catch (error) {
		console.error(`${LOG_PREFIX} Analytics fetch error:`, error);
		return {};
	}
}

// ═══════════════════════════════════════════════════════════════════════════════
// LOAD FUNCTION
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Server load function for Analytics page
 *
 * @description Fetches analytics data server-side for SEO and initial load.
 *              Never throws - returns fallback data on failure.
 */
export const load = (async ({ fetch, url }) => {
	const baseUrl = env.API_BASE_URL || DEFAULT_API_URL;
	const period = url.searchParams.get('period') || DEFAULT_PERIOD;

	try {
		const analyticsData = await fetchAnalyticsData(fetch, baseUrl, period);

		return {
			performance: analyticsData.performance ?? null,
			equityCurve: analyticsData.equityCurve ?? [],
			tickerPerformance: analyticsData.tickerPerformance ?? [],
			monthlyReturns: analyticsData.monthlyReturns ?? [],
			drawdown: analyticsData.drawdown ?? [],
			streak: analyticsData.streak ?? null,
			period
		};
	} catch (error) {
		console.error(`${LOG_PREFIX} FATAL ERROR in load function:`, error);

		return {
			performance: null,
			equityCurve: [],
			tickerPerformance: [],
			monthlyReturns: [],
			drawdown: [],
			streak: null,
			period
		};
	}
}) satisfies PageServerLoad;

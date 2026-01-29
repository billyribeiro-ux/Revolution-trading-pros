/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Trade Tracker - API Module
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+
 */

import type { Trade, TradeStats, ApiTradeResponse, ApiStatsResponse, TradeSetup } from './types';

const ROOM_SLUG = 'explosive-swings';

interface FetchTradesResult {
	trades: Trade[];
	stats: TradeStats | null;
}

/**
 * Format ISO date string to display format
 */
function formatDate(dateString: string): string {
	const date = new Date(dateString);
	return date.toLocaleDateString('en-US', {
		month: 'short',
		day: 'numeric',
		year: 'numeric'
	});
}

/**
 * Transform API trade to display format
 */
function transformTrade(apiTrade: ApiTradeResponse): Trade {
	return {
		id: apiTrade.id,
		ticker: apiTrade.ticker,
		entryDate: formatDate(apiTrade.entry_date),
		exitDate: apiTrade.exit_date ? formatDate(apiTrade.exit_date) : null,
		entryPrice: apiTrade.entry_price,
		exitPrice: apiTrade.exit_price,
		shares: apiTrade.quantity,
		profit: apiTrade.pnl ?? 0,
		profitPercent: apiTrade.pnl_percent ?? 0,
		duration: apiTrade.holding_days ?? 0,
		setup: (apiTrade.setup as TradeSetup) || 'Breakout',
		result: apiTrade.status === 'open' ? 'ACTIVE' : (apiTrade.pnl ?? 0) >= 0 ? 'WIN' : 'LOSS',
		notes: apiTrade.notes ?? '',
		tradeType: apiTrade.trade_type
	};
}

/**
 * Transform API stats to display format
 */
function transformStats(apiStats: ApiStatsResponse): TradeStats {
	return {
		totalTrades: (apiStats.wins ?? 0) + (apiStats.losses ?? 0),
		wins: apiStats.wins ?? 0,
		losses: apiStats.losses ?? 0,
		winRate: (apiStats.win_rate ?? 0).toFixed(1),
		totalProfit: apiStats.total_pnl ?? 0,
		avgWin: (apiStats.avg_win ?? 0).toFixed(0),
		avgLoss: (apiStats.avg_loss ?? 0).toFixed(0),
		profitFactor: (apiStats.profit_factor ?? 0).toFixed(2)
	};
}

/**
 * Fetch trades from API
 */
export async function fetchTrades(limit: number = 100): Promise<FetchTradesResult> {
	const response = await fetch(`/api/trades/${ROOM_SLUG}?limit=${limit}`, {
		credentials: 'include'
	});

	if (!response.ok) {
		throw new Error(`HTTP ${response.status}: Failed to fetch trades`);
	}

	const data = await response.json();

	if (!data.success) {
		throw new Error(data.error || 'Failed to fetch trades');
	}

	return {
		trades: (data.data as ApiTradeResponse[]).map(transformTrade),
		stats: data.stats ? transformStats(data.stats) : null
	};
}

/**
 * Calculate stats from trades (fallback when API doesn't return stats)
 */
export function calculateStats(trades: Trade[]): TradeStats {
	const closedTrades = trades.filter((t) => t.result !== 'ACTIVE');
	const wins = closedTrades.filter((t) => t.result === 'WIN');
	const losses = closedTrades.filter((t) => t.result === 'LOSS');

	const totalProfit = closedTrades.reduce((sum, t) => sum + t.profit, 0);
	const avgWin = wins.length > 0 ? wins.reduce((sum, t) => sum + t.profit, 0) / wins.length : 0;
	const avgLoss =
		losses.length > 0 ? Math.abs(losses.reduce((sum, t) => sum + t.profit, 0) / losses.length) : 0;

	return {
		totalTrades: closedTrades.length,
		wins: wins.length,
		losses: losses.length,
		winRate:
			closedTrades.length > 0 ? ((wins.length / closedTrades.length) * 100).toFixed(1) : '0.0',
		totalProfit,
		avgWin: avgWin.toFixed(0),
		avgLoss: avgLoss.toFixed(0),
		profitFactor: avgLoss > 0 ? (avgWin / avgLoss).toFixed(2) : '0.00'
	};
}

/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Trade Tracker - Type Definitions
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+
 */

export type TradeSetup = 'Breakout' | 'Momentum' | 'Reversal' | 'Earnings' | 'Pullback';
export type TradeResult = 'WIN' | 'LOSS' | 'ACTIVE' | 'INVALIDATED';
export type TradeType = 'shares' | 'options';
export type FilterStatus = 'all' | 'active' | 'win' | 'loss';

export interface Trade {
	id: number;
	ticker: string;
	entryDate: string;
	exitDate: string | null;
	entryPrice: number;
	exitPrice: number | null;
	shares: number;
	profit: number;
	profitPercent: number;
	duration: number;
	setup: TradeSetup;
	result: TradeResult;
	notes: string;
	tradeType?: TradeType;
}

export interface TradeStats {
	totalTrades: number;
	wins: number;
	losses: number;
	winRate: string;
	totalProfit: number;
	avgWin: string;
	avgLoss: string;
	profitFactor: string;
}

export interface ApiTradeResponse {
	id: number;
	ticker: string;
	entry_date: string;
	exit_date: string | null;
	entry_price: number;
	exit_price: number | null;
	quantity: number;
	pnl: number | null;
	pnl_percent: number | null;
	holding_days: number | null;
	setup: string | null;
	status: 'open' | 'closed' | 'invalidated';
	notes: string | null;
	trade_type?: TradeType;
	was_updated?: boolean;
	invalidation_reason?: string;
}

export interface ApiStatsResponse {
	total_pnl: number;
	win_rate: number;
	wins: number;
	losses: number;
	avg_win: number;
	avg_loss: number;
	profit_factor: number;
}

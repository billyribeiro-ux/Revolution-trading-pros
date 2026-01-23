/**
 * Trading Room Types - Explosive Swings & Alert Services
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Type definitions for trade plans, alerts, trades, and stats.
 * Matches database schema from 20260113_000001_trading_room_content.sql
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */

// ═══════════════════════════════════════════════════════════════════════════
// TRADE PLAN TYPES
// ═══════════════════════════════════════════════════════════════════════════

export type TradeBias = 'BULLISH' | 'BEARISH' | 'NEUTRAL';

export interface TradePlanEntry {
	id: number;
	room_id: number;
	room_slug: string;
	week_of: string;
	ticker: string;
	bias: TradeBias;
	entry: string;
	target1: string;
	target2: string;
	target3: string;
	runner: string;
	runner_stop: string | null;
	stop: string;
	options_strike: string | null;
	options_exp: string | null;
	notes: string | null;
	sort_order: number;
	is_active: boolean;
	created_at: string;
	updated_at: string;
}

export interface TradePlanCreateInput {
	room_slug: string;
	week_of?: string;
	ticker: string;
	bias: TradeBias;
	entry: string;
	target1: string;
	target2: string;
	target3: string;
	runner: string;
	stop: string;
	options_strike?: string;
	options_exp?: string;
	notes?: string;
	sort_order?: number;
}

export interface TradePlanUpdateInput {
	ticker?: string;
	bias?: TradeBias;
	entry?: string;
	target1?: string;
	target2?: string;
	target3?: string;
	runner?: string;
	stop?: string;
	options_strike?: string;
	options_exp?: string;
	notes?: string;
	sort_order?: number;
	is_active?: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════
// ALERT TYPES
// ═══════════════════════════════════════════════════════════════════════════

export type AlertType = 'ENTRY' | 'EXIT' | 'UPDATE';
export type TradeType = 'options' | 'shares';
export type TradeAction = 'BUY' | 'SELL';
export type OptionType = 'CALL' | 'PUT';
export type OrderType = 'MKT' | 'LMT';
export type ContractType = 'Weeklys' | 'Monthly' | 'LEAPS' | 'Standard';

export interface RoomAlert {
	id: number;
	room_id: number;
	room_slug: string;
	alert_type: AlertType;
	ticker: string;
	title: string;
	message: string;
	notes: string | null;
	// TOS format fields
	trade_type: TradeType | null;
	action: TradeAction | null;
	quantity: number | null;
	option_type: OptionType | null;
	strike: number | null;
	expiration: string | null;
	contract_type: ContractType | null;
	order_type: OrderType | null;
	limit_price: number | null;
	fill_price: number | null;
	tos_string: string | null;
	// Linking
	entry_alert_id: number | null;
	trade_plan_id: number | null;
	// Status
	is_new: boolean;
	is_published: boolean;
	is_pinned: boolean;
	// Timestamps
	published_at: string;
	created_at: string;
	updated_at: string;
}

export interface AlertCreateInput {
	room_slug: string;
	alert_type: AlertType;
	ticker: string;
	title: string;
	message: string;
	notes?: string;
	// TOS format fields
	trade_type?: TradeType;
	action?: TradeAction;
	quantity?: number;
	option_type?: OptionType;
	strike?: number;
	expiration?: string;
	contract_type?: ContractType;
	order_type?: OrderType;
	limit_price?: number;
	fill_price?: number;
	// Linking
	entry_alert_id?: number;
	trade_plan_id?: number;
	// Status
	is_pinned?: boolean;
}

export interface AlertUpdateInput {
	alert_type?: AlertType;
	ticker?: string;
	title?: string;
	message?: string;
	notes?: string;
	trade_type?: TradeType;
	action?: TradeAction;
	quantity?: number;
	option_type?: OptionType;
	strike?: number;
	expiration?: string;
	contract_type?: ContractType;
	order_type?: OrderType;
	limit_price?: number;
	fill_price?: number;
	entry_alert_id?: number;
	is_new?: boolean;
	is_published?: boolean;
	is_pinned?: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════
// TRADE TYPES (for Trade Tracker)
// ═══════════════════════════════════════════════════════════════════════════

export type TradeStatus = 'open' | 'closed' | 'partial';
export type TradeResult = 'WIN' | 'LOSS' | 'ACTIVE';
export type TradeSetup = 'Breakout' | 'Momentum' | 'Reversal' | 'Earnings' | 'Pullback';

export interface Trade {
	id: number;
	room_id: number;
	room_slug: string;
	ticker: string;
	trade_type: TradeType;
	direction: 'long' | 'short';
	quantity: number;
	// Options specific
	option_type: OptionType | null;
	strike: number | null;
	expiration: string | null;
	// Entry
	entry_alert_id: number | null;
	entry_price: number;
	entry_date: string;
	// Exit (null if open)
	exit_alert_id: number | null;
	exit_price: number | null;
	exit_date: string | null;
	// Setup & Status
	setup: TradeSetup | null;
	status: TradeStatus;
	// P&L
	pnl: number | null;
	pnl_percent: number | null;
	holding_days: number | null;
	// Notes
	notes: string | null;
	// Timestamps
	created_at: string;
	updated_at: string;
}

export interface TradeCreateInput {
	room_slug: string;
	ticker: string;
	trade_type: TradeType;
	direction: 'long' | 'short';
	quantity: number;
	option_type?: OptionType;
	strike?: number;
	expiration?: string;
	contract_type?: ContractType;
	entry_alert_id?: number;
	entry_price: number;
	entry_date: string;
	entry_tos_string?: string;
	setup?: TradeSetup;
	notes?: string;
}

export interface TradeCloseInput {
	exit_alert_id?: number;
	exit_price: number;
	exit_date?: string;
	quantity?: number; // For partial closes
	notes?: string;
}

export interface TradeUpdateInput {
	exit_alert_id?: number;
	exit_price?: number;
	exit_date?: string;
	exit_tos_string?: string;
	status?: TradeStatus;
	notes?: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// STATS TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface RoomStats {
	room_id: number;
	room_slug: string;
	win_rate: number;
	weekly_profit: string;
	monthly_profit: string;
	active_trades: number;
	closed_this_week: number;
	total_trades: number;
	wins: number;
	losses: number;
	avg_win: number;
	avg_loss: number;
	profit_factor: number;
	avg_holding_days: number;
	largest_win: number;
	largest_loss: number;
	current_streak: number;
	calculated_at: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// TOS STRING BUILDER TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface TosStringParams {
	trade_type: TradeType;
	action: TradeAction;
	quantity: number;
	ticker: string;
	// Options only
	option_type?: OptionType;
	strike?: number;
	expiration?: string; // ISO date string
	contract_type?: ContractType;
	// Pricing
	order_type: OrderType;
	limit_price?: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// API RESPONSE TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface ApiResponse<T> {
	success: boolean;
	data: T;
	message?: string;
	_mock?: boolean;
}

export interface PaginatedResponse<T> {
	success: boolean;
	data: T[];
	total: number;
	page: number;
	limit: number;
	_mock?: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════
// FILTER TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface AlertFilters {
	alert_type?: AlertType | 'all';
	ticker?: string;
	from_date?: string;
	to_date?: string;
	is_published?: boolean;
	limit?: number;
	offset?: number;
}

export interface TradeFilters {
	status?: TradeStatus | 'all';
	ticker?: string;
	from_date?: string;
	to_date?: string;
	limit?: number;
	offset?: number;
}

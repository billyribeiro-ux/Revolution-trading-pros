/**
 * Explosive Swings Dashboard - TypeScript Interfaces
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple Principal Engineer ICT 11+ Standards
 * Single Source of Truth for all dashboard-related types
 *
 * @version 1.0.0
 * @since January 2026
 */

// ═══════════════════════════════════════════════════════════════════════════
// ALERT TYPES
// ═══════════════════════════════════════════════════════════════════════════

export type AlertType = 'ENTRY' | 'EXIT' | 'UPDATE';
export type AlertFilter = 'all' | 'entry' | 'exit' | 'update';

export interface Alert {
	id: number;
	type: AlertType;
	ticker: string;
	title: string;
	time: string;
	message: string;
	isNew: boolean;
	notes: string;
	tosString?: string;
}

export interface RoomAlert {
	id: number;
	room_id: number;
	alert_type: AlertType;
	ticker: string;
	title: string;
	message: string;
	notes?: string;
	tos_string?: string;
	is_new: boolean;
	published_at: string;
	created_at: string;
	updated_at: string;
}

export interface AlertCreateInput {
	alert_type: AlertType;
	ticker: string;
	title: string;
	message: string;
	notes?: string;
	tos_string?: string;
}

export interface AlertUpdateInput extends Partial<AlertCreateInput> {
	is_new?: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════
// PAGINATION TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface PaginationState {
	total: number;
	limit: number;
	offset: number;
}

export interface PaginationProps {
	currentPage: number;
	totalPages: number;
	totalItems: number;
	itemsPerPage: number;
	onPageChange: (page: number) => void;
}

// ═══════════════════════════════════════════════════════════════════════════
// STATS TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface QuickStats {
	winRate: number;
	weeklyProfit: string;
	activeTrades: number;
	closedThisWeek: number;
}

export interface RoomStats {
	win_rate: number;
	weekly_profit: string;
	active_trades: number;
	closed_this_week: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// VIDEO TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface VideoUpdate {
	id: number;
	title: string;
	date: string;
	excerpt: string;
	href: string;
	image: string;
	isVideo: boolean;
	duration: string;
}

export interface WeeklyContent {
	title: string;
	videoTitle: string;
	videoUrl: string;
	thumbnail: string;
	duration: string;
	publishedDate: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// TRADE PLAN TYPES
// ═══════════════════════════════════════════════════════════════════════════

export type TradeBias = 'BULLISH' | 'BEARISH' | 'NEUTRAL';

export interface TradePlanEntry {
	ticker: string;
	bias: TradeBias;
	entry: string;
	target1: string;
	target2: string;
	target3: string;
	runner: string;
	stop: string;
	optionsStrike: string;
	optionsExp: string;
	notes: string;
}

export interface ApiTradePlanEntry {
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
}

// ═══════════════════════════════════════════════════════════════════════════
// FILTER COUNTS
// ═══════════════════════════════════════════════════════════════════════════

export interface FilterCounts {
	all: number;
	entry: number;
	exit: number;
	update: number;
}

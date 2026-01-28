/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Explosive Swings - Type Definitions
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Centralized type definitions for the Explosive Swings dashboard
 * @version 2.0.0 - Nuclear Refactor Consolidation
 * @standards TypeScript Strict Mode Compliant | Apple Principal Engineer ICT 7+
 *
 * IMPORTANT: This is the SINGLE SOURCE OF TRUTH for all types used in this route.
 * Never define types inline in components. Always import from here.
 *
 * CRITICAL: NO DOLLAR AMOUNTS in display types - percentages only
 * This prevents FOMO and encourages proper position sizing
 */

// ═══════════════════════════════════════════════════════════════════════════
// CLOSED TRADES - Ticker Pills Display
// ═══════════════════════════════════════════════════════════════════════════

export interface ClosedTrade {
	/** Unique identifier */
	id: string;
	/** Stock ticker symbol (uppercase) */
	ticker: string;
	/** Percentage gain/loss (positive for wins, negative for losses) */
	percentageGain: number;
	/** Whether the trade was profitable */
	isWinner: boolean;
	/** When the trade was closed */
	closedAt: Date;
	/** Entry price for reference */
	entryPrice: number;
	/** Exit price for reference */
	exitPrice: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// ACTIVE POSITIONS - Position Cards Display
// ═══════════════════════════════════════════════════════════════════════════

export type PositionStatus = 'ENTRY' | 'WATCHING' | 'ACTIVE';

export interface PositionTarget {
	/** Target price */
	price: number;
	/** Percentage from entry to this target */
	percentFromEntry: number;
	/** Display label (e.g., "Target 1", "Target 2") */
	label: string;
}

export interface ActivePosition {
	/** Unique identifier */
	id: string;
	/** Stock ticker symbol (uppercase) */
	ticker: string;
	/** Current position status */
	status: PositionStatus;
	/** Entry price (null if WATCHING) */
	entryPrice: number | null;
	/** Entry zone for WATCHING status */
	entryZone?: { low: number; high: number };
	/** Current market price */
	currentPrice: number;
	/** Unrealized percentage gain/loss (null if WATCHING) */
	unrealizedPercent: number | null;
	/** Array of price targets with percentages */
	targets: PositionTarget[];
	/** Stop loss with price and percentage */
	stopLoss: { price: number; percentFromEntry: number };
	/** Optional notes (e.g., "Waiting for pullback to zone") */
	notes?: string;
	/** Progress towards first target (0-100) */
	progressToTarget1: number;
	/** When the position was triggered */
	triggeredAt?: Date;
	/** When the position was last updated */
	updatedAt?: Date;
	/** Whether this position was manually updated */
	wasUpdated?: boolean;
	/** Invalidation reason if invalidated */
	invalidationReason?: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// ALERTS - Live Alerts Feed
// ═══════════════════════════════════════════════════════════════════════════

export type AlertType = 'ENTRY' | 'UPDATE' | 'EXIT';

export interface Alert {
	/** Unique identifier - number from API, supports both for compatibility */
	id: number;
	/** Stock ticker symbol (uppercase) */
	ticker: string;
	/** Alert type determines visual styling */
	type: AlertType;
	/** Alert title/headline */
	title: string;
	/** Alert message (API format) */
	message: string;
	/** Formatted time string (API format) */
	time: string;
	/** True if < 30 minutes old (triggers pulse animation) */
	isNew: boolean;
	/** Additional notes or context */
	notes?: string;
	/** ThinkOrSwim share string */
	tosString?: string;
	// Legacy fields for backward compatibility:
	/** @deprecated Use message instead */
	description?: string;
	/** @deprecated Use time instead */
	timestamp?: Date;
	/** Entry price for ENTRY alerts */
	entryPrice?: number;
	/** Target price for ENTRY alerts */
	targetPrice?: number;
	/** Stop loss price for ENTRY alerts */
	stopPrice?: number;
	/** Result percentage for EXIT alerts */
	resultPercent?: number;
	/** Whether this alert has an associated video */
	hasVideo?: boolean;
	/** URL to the video explanation */
	videoUrl?: string;
}

export type AlertFilterType = 'all' | 'entries' | 'updates' | 'exits';

// ═══════════════════════════════════════════════════════════════════════════
// PERFORMANCE METRICS
// ═══════════════════════════════════════════════════════════════════════════

export interface WeeklyPerformance {
	/** Win rate as percentage (0-100) */
	winRate: number;
	/** Total trades closed this week */
	totalTrades: number;
	/** Number of winning trades */
	winningTrades: number;
	/** Average percentage gain on winners */
	avgWinPercent: number;
	/** Average percentage loss on losers */
	avgLossPercent: number;
	/** Risk/Reward ratio */
	riskRewardRatio: number;
}

export interface ThirtyDayPerformance {
	/** Win rate as percentage (0-100) */
	winRate: number;
	/** Total alerts in 30-day period */
	totalAlerts: number;
	/** Number of profitable alerts */
	profitableAlerts: number;
	/** Average percentage gain on winners */
	avgWinPercent: number;
	/** Average percentage loss on losers */
	avgLossPercent: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// VIDEO CONTENT
// ═══════════════════════════════════════════════════════════════════════════

export interface WeeklyVideo {
	/** Video title */
	title: string;
	/** Thumbnail image URL */
	thumbnailUrl: string;
	/** Video embed/player URL */
	videoUrl: string;
	/** Formatted duration (e.g., "24:35") */
	duration: string;
	/** When the video was published */
	publishedAt: Date;
}

export interface Video {
	/** Unique identifier */
	id: string;
	/** Video title */
	title: string;
	/** Thumbnail image URL */
	thumbnailUrl: string;
	/** Video embed/player URL */
	videoUrl: string;
	/** Formatted duration (e.g., "12:15") */
	duration: string;
	/** When the video was published */
	publishedAt: Date;
	/** Associated ticker symbol (if any) */
	ticker?: string;
	/** Video type for overlay badge */
	type?: 'ENTRY' | 'EXIT' | 'UPDATE' | 'BREAKDOWN';
	/** Whether this is the featured video */
	isFeatured?: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════
// DASHBOARD DATA - Complete Page Data Structure
// ═══════════════════════════════════════════════════════════════════════════

export interface ExplosiveSwingsDashboardData {
	/** Week identifier (e.g., "Week of January 13, 2026") */
	weekOf: string;
	/** This week's performance summary */
	performance: WeeklyPerformance;
	/** Closed trades for ticker pills display */
	closedTrades: ClosedTrade[];
	/** Active positions for position cards */
	activePositions: ActivePosition[];
	/** Live alerts feed */
	alerts: Alert[];
	/** Weekly breakdown video */
	weeklyVideo: WeeklyVideo;
	/** Recent video updates for grid */
	recentVideos: Video[];
	/** 30-day track record for sidebar */
	thirtyDayPerformance: ThirtyDayPerformance;
}

// ═══════════════════════════════════════════════════════════════════════════
// COLOR MAPPINGS - Semantic Color Tokens
// ═══════════════════════════════════════════════════════════════════════════

export const alertColors = {
	entry: {
		border: 'border-l-teal-500',
		bg: 'bg-teal-50/50',
		badge: 'bg-teal-100 text-teal-700',
		accent: 'text-teal-600'
	},
	update: {
		border: 'border-l-amber-500',
		bg: 'bg-amber-50/50',
		badge: 'bg-amber-100 text-amber-700',
		accent: 'text-amber-600'
	},
	exitWin: {
		border: 'border-l-emerald-500',
		bg: 'bg-emerald-50/50',
		badge: 'bg-emerald-100 text-emerald-700',
		accent: 'text-emerald-600'
	},
	exitLoss: {
		border: 'border-l-red-500',
		bg: 'bg-red-50/50',
		badge: 'bg-red-100 text-red-700',
		accent: 'text-red-600'
	},
	watching: {
		border: 'border-l-amber-500',
		bg: 'bg-amber-50/30',
		badge: 'bg-amber-100 text-amber-700',
		accent: 'text-amber-600'
	}
} as const;

export const positionStatusColors = {
	ENTRY: 'bg-teal-100 text-teal-700 border-teal-300',
	WATCHING: 'bg-amber-100 text-amber-700 border-amber-300',
	ACTIVE_PROFIT: 'bg-emerald-100 text-emerald-700 border-emerald-300',
	ACTIVE_LOSS: 'bg-red-100 text-red-700 border-red-300'
} as const;

export const performanceColors = {
	positive: 'text-emerald-600',
	negative: 'text-red-600',
	neutral: 'text-slate-600'
} as const;

// ═══════════════════════════════════════════════════════════════════════════
// WEEKLY CONTENT - Hero Section Display
// ═══════════════════════════════════════════════════════════════════════════

export interface WeeklyContent {
	/** Week title (e.g., "Week of January 13, 2026") */
	title: string;
	/** Video title */
	videoTitle: string;
	/** Video embed URL */
	videoUrl: string;
	/** Thumbnail image URL */
	thumbnail: string;
	/** Formatted duration (e.g., "24:35") */
	duration: string;
	/** Formatted publish date (e.g., "January 13, 2026 at 6:40 PM ET") */
	publishedDate: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// TRADE PLAN - Weekly Trade Plan Entries
// ═══════════════════════════════════════════════════════════════════════════

export type TradeBias = 'BULLISH' | 'BEARISH' | 'NEUTRAL';

export interface TradePlanEntry {
	/** Stock ticker symbol */
	ticker: string;
	/** Directional bias */
	bias: TradeBias;
	/** Entry price level */
	entry: string;
	/** First price target */
	target1: string;
	/** Second price target */
	target2: string;
	/** Third price target */
	target3: string;
	/** Runner target for letting profits run */
	runner: string;
	/** Stop loss level */
	stop: string;
	/** Options strike price */
	optionsStrike: string;
	/** Options expiration date */
	optionsExp: string;
	/** Trade notes/thesis */
	notes: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// QUICK STATS - Dashboard KPIs
// ═══════════════════════════════════════════════════════════════════════════

export interface QuickStats {
	/** Win rate as percentage (0-100) */
	winRate: number;
	/** Weekly profit display string (e.g., "+$4,850") */
	weeklyProfit: string;
	/** Number of currently active trades */
	activeTrades: number;
	/** Number of trades closed this week */
	closedThisWeek: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// VIDEO UPDATES - Latest Content Grid
// ═══════════════════════════════════════════════════════════════════════════

export interface VideoUpdate {
	/** Unique identifier */
	id: number;
	/** Video/content title */
	title: string;
	/** Formatted date string */
	date: string;
	/** Short description/excerpt */
	excerpt: string;
	/** Link to full content */
	href: string;
	/** Thumbnail image URL */
	image: string;
	/** Whether this is video content */
	isVideo: boolean;
	/** Formatted duration (e.g., "18:42") */
	duration: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// FILTER TYPES - UI State
// ═══════════════════════════════════════════════════════════════════════════

export type AlertFilter = 'all' | 'entry' | 'exit' | 'update';

// ═══════════════════════════════════════════════════════════════════════════
// API RESPONSE TYPES - Backend Data Shapes
// ═══════════════════════════════════════════════════════════════════════════

export interface ApiTrade {
	id: number;
	ticker: string;
	status: 'open' | 'closed';
	entry_price: number;
	exit_price: number | null;
	pnl_percent: number | null;
	entry_date: string;
	exit_date: string | null;
	direction: string;
	setup?: string;
	notes?: string;
}

export interface ApiWeeklyVideo {
	id: number;
	video_title: string;
	video_url: string;
	thumbnail_url: string | null;
	duration: string | null;
	published_at: string;
	week_title: string;
}

export interface PaginationState {
	total: number;
	limit: number;
	offset: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// UTILITY TYPES - State Management
// ═══════════════════════════════════════════════════════════════════════════

export type LoadingState = 'idle' | 'loading' | 'success' | 'error';

export interface ApiError {
	message: string;
	code?: string;
	status?: number;
}

export interface AsyncState<T> {
	data: T | null;
	isLoading: boolean;
	error: ApiError | null;
}

// ═══════════════════════════════════════════════════════════════════════════
// COMPONENT PROP TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface PerformanceSummaryProps {
	performance: WeeklyPerformance;
	closedTrades: ClosedTrade[];
	activePositions: ActivePosition[];
	isLoading?: boolean;
	isAdmin?: boolean;
	onClosePosition?: (position: ActivePosition) => void;
	onAddTrade?: () => void;
}

export interface ActivePositionCardProps {
	position: ActivePosition;
	isAdmin?: boolean;
	onClose?: (position: ActivePosition) => void;
}

export interface PerformanceCardProps {
	performance: ThirtyDayPerformance;
	isLoading?: boolean;
}

export interface TickerPillProps {
	trade: ClosedTrade;
}

// ═══════════════════════════════════════════════════════════════════════════
// STANDARDIZED BREAKPOINTS - Repository-Wide System
// ═══════════════════════════════════════════════════════════════════════════
//
// These align with Tailwind defaults and are used across all components.
// Import from '$lib/constants/breakpoints' for programmatic use.
//
// CSS Usage (max-width for desktop-first):
//   @media (max-width: 639px)  → Mobile phones (< sm)
//   @media (max-width: 767px)  → Small tablets (< md)
//   @media (max-width: 1023px) → Tablets (< lg)
//   @media (max-width: 1279px) → Small desktops (< xl)
//
// Standard values:
//   sm: 640px, md: 768px, lg: 1024px, xl: 1280px, 2xl: 1536px
// ═══════════════════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════════════════════
// TRADE ENUMS - Status and Type Definitions
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Current status of a trade in the system.
 * @description Tracks the lifecycle of a trade from open to completion.
 */
export type TradeStatus = 'open' | 'closed' | 'invalidated' | 'partial';

/**
 * Type of trading instrument being used.
 * @description Determines the instrument category for the trade.
 */
export type TradeType = 'stock' | 'option' | 'spread';

/**
 * Action being taken on a trade.
 * @description Indicates whether entering or exiting a position.
 */
export type TradeAction = 'buy' | 'sell' | 'buy_to_open' | 'sell_to_close' | 'buy_to_close' | 'sell_to_open';

/**
 * Type of option contract.
 * @description Specifies the option type for options trades.
 */
export type OptionType = 'call' | 'put';

/**
 * Order execution type.
 * @description Determines how the order should be executed.
 */
export type OrderType = 'market' | 'limit' | 'stop' | 'stop_limit';

// ═══════════════════════════════════════════════════════════════════════════
// API Response Types
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Generic API response wrapper.
 * @template T - The type of data contained in the response.
 * @description Standardized response format for all API endpoints.
 */
export interface ApiResponse<T> {
	/** Whether the API call was successful */
	success: boolean;
	/** The response data payload */
	data: T;
	/** Error message if success is false */
	error?: string;
	/** Pagination metadata for list responses */
	pagination?: PaginationMeta;
}

/**
 * Pagination metadata for paginated responses.
 * @description Contains all information needed for pagination controls.
 */
export interface PaginationMeta {
	/** Current page number (1-indexed) */
	page: number;
	/** Number of items per page */
	per_page: number;
	/** Total number of items across all pages */
	total: number;
	/** Total number of pages available */
	total_pages: number;
	/** Whether there is a next page */
	has_next: boolean;
	/** Whether there is a previous page */
	has_prev: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════
// Search Types
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Search filter parameters.
 * @description Filters applied to search queries across the dashboard.
 */
export interface SearchFilters {
	/** Content types to include in search results */
	types: ('alerts' | 'trades' | 'trade_plans')[];
	/** Filter by specific ticker symbol */
	ticker?: string;
	/** Start date for date range filter (ISO 8601) */
	from?: string;
	/** End date for date range filter (ISO 8601) */
	to?: string;
}

/**
 * Individual search result item.
 * @description Represents a single match from a search query.
 */
export interface SearchResult {
	/** Type of content that matched */
	type: 'alert' | 'trade' | 'trade_plan';
	/** Unique identifier of the matched item */
	id: number;
	/** Ticker symbol associated with the result */
	ticker: string;
	/** Title or name of the matched item */
	title: string;
	/** Relevance rank (lower is more relevant) */
	rank: number;
	/** Highlighted headline/excerpt showing match context */
	headline: string;
	/** When the item was created (ISO 8601) */
	created_at: string;
}

/**
 * Complete search response.
 * @description Full response from a search query including metadata.
 */
export interface SearchResponse {
	/** Array of matching results */
	results: SearchResult[];
	/** Total count of matches (may exceed results if paginated) */
	total_count: number;
	/** The original search query */
	query: string;
	/** Time taken to execute search in milliseconds */
	took_ms: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// Analytics Types
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Summary statistics for trading performance.
 * @description Comprehensive metrics for evaluating trading performance.
 */
export interface AnalyticsSummary {
	/** Total number of trades in the period */
	total_trades: number;
	/** Number of winning trades */
	wins: number;
	/** Number of losing trades */
	losses: number;
	/** Win rate as a decimal (0-1) */
	win_rate: number;
	/** Ratio of gross profits to gross losses */
	profit_factor: number;
	/** Average percentage gain on winning trades */
	avg_win: number;
	/** Average percentage loss on losing trades */
	avg_loss: number;
	/** Largest single winning trade percentage */
	largest_win: number;
	/** Largest single losing trade percentage */
	largest_loss: number;
	/** Maximum peak-to-trough decline percentage */
	max_drawdown: number;
	/** Average number of days positions are held */
	avg_holding_days: number;
	/** Current consecutive win/loss streak */
	current_streak: number;
	/** Type of current streak */
	streak_type: 'win' | 'loss';
}

/**
 * Performance metrics for a specific ticker.
 * @description Breakdown of trading performance by individual stock symbol.
 */
export interface TickerPerformance {
	/** Stock ticker symbol */
	ticker: string;
	/** Total trades on this ticker */
	total_trades: number;
	/** Winning trades on this ticker */
	wins: number;
	/** Losing trades on this ticker */
	losses: number;
	/** Win rate for this ticker (0-1) */
	win_rate: number;
	/** Total P&L percentage for this ticker */
	total_pnl: number;
	/** Average P&L per trade percentage */
	avg_pnl: number;
}

/**
 * Monthly return data point.
 * @description Performance metrics aggregated by calendar month.
 */
export interface MonthlyReturn {
	/** Month identifier (YYYY-MM format) */
	month: string;
	/** Total P&L percentage for the month */
	pnl: number;
	/** Number of trades closed in the month */
	trades: number;
	/** Win rate for the month (0-1) */
	win_rate: number;
}

/**
 * Point on the equity curve.
 * @description Represents cumulative P&L at a specific point in time.
 */
export interface EquityPoint {
	/** Date of the equity snapshot (ISO 8601) */
	date: string;
	/** Cumulative P&L percentage to date */
	cumulative_pnl: number;
	/** Trade ID that caused this equity change */
	trade_id: number;
}

/**
 * Drawdown period analysis.
 * @description Tracks periods of declining equity and recovery.
 */
export interface DrawdownPeriod {
	/** Start date of the drawdown (ISO 8601) */
	start_date: string;
	/** End date of the drawdown (ISO 8601) */
	end_date: string;
	/** Maximum drawdown percentage during this period */
	max_drawdown: number;
	/** Number of days to recover from drawdown */
	recovery_days: number;
}

/**
 * Complete analytics data for a trading room.
 * @description All analytics data needed for the room analytics dashboard.
 */
export interface RoomAnalytics {
	/** Summary performance statistics */
	summary: AnalyticsSummary;
	/** Performance breakdown by ticker */
	ticker_performance: TickerPerformance[];
	/** Monthly return history */
	monthly_returns: MonthlyReturn[];
	/** Equity curve data points */
	equity_curve: EquityPoint[];
	/** Historical drawdown periods */
	drawdown_periods: DrawdownPeriod[];
}

// ═══════════════════════════════════════════════════════════════════════════
// WebSocket Event Types
// ═══════════════════════════════════════════════════════════════════════════

/**
 * All possible WebSocket event types.
 * @description Union type of all events that can be received via WebSocket.
 */
export type WsEventType =
	| 'alert:created'
	| 'alert:updated'
	| 'alert:deleted'
	| 'trade:opened'
	| 'trade:closed'
	| 'trade:invalidated'
	| 'stats:updated'
	| 'heartbeat';

/**
 * Generic WebSocket message wrapper.
 * @template T - The type of data contained in the message.
 * @description Standardized format for all WebSocket messages.
 */
export interface WsMessage<T = unknown> {
	/** Type of event being broadcast */
	type: WsEventType;
	/** Event-specific data payload */
	data: T;
	/** When the event occurred (ISO 8601) */
	timestamp: string;
}

/**
 * Event data for alert creation.
 * @description Payload received when a new alert is published.
 */
export interface AlertCreatedEvent {
	/** Unique identifier of the new alert */
	id: number;
	/** Slug of the room the alert belongs to */
	room_slug: string;
	/** Type of alert */
	alert_type: AlertType;
	/** Stock ticker symbol */
	ticker: string;
	/** Alert title/headline */
	title: string;
	/** ThinkOrSwim share string (optional) */
	tos_string?: string;
	/** When the alert was published (ISO 8601) */
	published_at: string;
}

/**
 * Event data for trade closure.
 * @description Payload received when a trade is closed.
 */
export interface TradeClosedEvent {
	/** Unique identifier of the closed trade */
	id: number;
	/** Slug of the room the trade belongs to */
	room_slug: string;
	/** Stock ticker symbol */
	ticker: string;
	/** Final status of the trade */
	status: TradeStatus;
	/** Profit/loss in dollars (optional) */
	pnl?: number;
	/** Profit/loss as percentage (optional) */
	pnl_percent?: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// Export Types
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Options for exporting trade data.
 * @description Configuration for data export functionality.
 */
export interface ExportOptions {
	/** Output format for the export */
	format: 'csv' | 'pdf' | 'json';
	/** Optional date range filter for export */
	date_range?: {
		/** Start date (ISO 8601) */
		from: string;
		/** End date (ISO 8601) */
		to: string;
	};
	/** Include closed trades in export */
	include_closed?: boolean;
	/** Include open trades in export */
	include_open?: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════
// Form Types (for modals)
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Form data for creating/editing alerts.
 * @description Shape of data collected from alert creation forms.
 */
export interface AlertFormData {
	/** Stock ticker symbol */
	ticker: string;
	/** Alert title/headline */
	title: string;
	/** Type of alert being created */
	alert_type: AlertType;
	/** Detailed message/notes */
	message?: string;
	/** Type of trade instrument */
	trade_type?: TradeType;
	/** Action being taken */
	action?: TradeAction;
	/** Number of shares/contracts */
	quantity?: number;
	/** Option type for options trades */
	option_type?: OptionType;
	/** Strike price for options */
	strike?: number;
	/** Expiration date for options (ISO 8601) */
	expiration?: string;
	/** Order execution type */
	order_type?: OrderType;
	/** Limit price for limit orders */
	limit_price?: number;
}

/**
 * Form data for creating/editing trades.
 * @description Shape of data collected from trade entry forms.
 */
export interface TradeFormData {
	/** Stock ticker symbol */
	ticker: string;
	/** Type of trade instrument */
	trade_type: TradeType;
	/** Trade direction */
	direction: 'long' | 'short';
	/** Entry price of the trade */
	entry_price: number;
	/** Entry date (ISO 8601) */
	entry_date: string;
	/** Number of shares/contracts */
	quantity?: number;
	/** Stop loss price */
	stop?: number;
	/** Target price */
	target?: number;
	/** Trade notes/thesis */
	notes?: string;
}

/**
 * Form data for closing a trade.
 * @description Shape of data collected when closing a position.
 */
export interface CloseTradeFormData {
	/** Exit price of the trade */
	exit_price: number;
	/** Exit date (ISO 8601) */
	exit_date: string;
	/** Notes about the exit */
	notes?: string;
}

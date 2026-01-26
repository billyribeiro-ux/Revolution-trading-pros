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

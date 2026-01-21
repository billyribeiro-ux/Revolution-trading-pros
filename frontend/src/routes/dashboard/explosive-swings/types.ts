/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Explosive Swings Dashboard - Type Definitions
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description TypeScript interfaces for the Explosive Swings member dashboard
 * @version 4.0.0 - January 2026 - Nuclear Build Specification
 * @standards Apple Principal Engineer ICT 7+ Standards
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
}

// ═══════════════════════════════════════════════════════════════════════════
// ALERTS - Live Alerts Feed
// ═══════════════════════════════════════════════════════════════════════════

export type AlertType = 'ENTRY' | 'UPDATE' | 'EXIT';

export interface Alert {
	/** Unique identifier */
	id: string;
	/** Stock ticker symbol (uppercase) */
	ticker: string;
	/** Alert type determines visual styling */
	type: AlertType;
	/** Alert title/headline */
	title: string;
	/** Detailed alert description */
	description: string;
	/** When the alert was published */
	timestamp: Date;
	/** True if < 30 minutes old (triggers pulse animation) */
	isNew: boolean;
	/** Entry price for ENTRY alerts */
	entryPrice?: number;
	/** Target price for ENTRY alerts */
	targetPrice?: number;
	/** Stop loss price for ENTRY alerts */
	stopPrice?: number;
	/** Result percentage for EXIT alerts */
	resultPercent?: number;
	/** Additional notes or context */
	notes?: string;
	/** Whether this alert has an associated video */
	hasVideo?: boolean;
	/** URL to the video explanation */
	videoUrl?: string;
	/** ThinkOrSwim share string */
	tosString?: string;
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

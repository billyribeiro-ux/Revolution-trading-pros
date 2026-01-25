/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Explosive Swings - API Module
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Centralized API calls for the Explosive Swings dashboard
 * @version 1.0.0
 * @standards Error handling with typed responses
 */

import type {
	ApiTrade,
	ApiWeeklyVideo,
	PaginationState,
	QuickStats,
	TradePlanEntry
} from './types';

// ═══════════════════════════════════════════════════════════════════════════
// RESPONSE TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface AlertsResponse {
	alerts: FormattedAlert[];
	pagination: PaginationState;
}

export interface FormattedAlert {
	id: number;
	type: 'ENTRY' | 'EXIT' | 'UPDATE';
	ticker: string;
	title: string;
	time: string;
	message: string;
	isNew: boolean;
	notes: string;
	tosString?: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// API FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Fetch paginated alerts for a trading room
 */
export async function fetchAlerts(
	roomSlug: string,
	page: number,
	limit: number
): Promise<AlertsResponse> {
	const offset = (page - 1) * limit;
	const response = await fetch(
		`/api/alerts/${roomSlug}?limit=${limit}&offset=${offset}`,
		{ credentials: 'include' }
	);

	if (!response.ok) {
		throw new Error(`HTTP ${response.status}: Failed to fetch alerts`);
	}

	const data = await response.json();

	if (!data.success) {
		throw new Error(data.error || 'Failed to fetch alerts');
	}

	return {
		alerts: data.data.map(formatAlert),
		pagination: {
			total: data.total || data.data.length,
			limit,
			offset
		}
	};
}

/**
 * Fetch trade plan entries for a trading room
 */
export async function fetchTradePlan(roomSlug: string): Promise<TradePlanEntry[]> {
	const response = await fetch(`/api/trade-plans/${roomSlug}`, {
		credentials: 'include'
	});

	if (!response.ok) {
		throw new Error(`HTTP ${response.status}: Failed to fetch trade plan`);
	}

	const data = await response.json();

	if (!data.success) {
		throw new Error(data.error || 'Failed to fetch trade plan');
	}

	return data.data.map(formatTradePlanEntry);
}

/**
 * Fetch stats for a trading room
 */
export async function fetchStats(roomSlug: string): Promise<QuickStats> {
	const response = await fetch(`/api/stats/${roomSlug}`, {
		credentials: 'include'
	});

	if (!response.ok) {
		throw new Error(`HTTP ${response.status}: Failed to fetch stats`);
	}

	const data = await response.json();

	if (!data.success) {
		throw new Error(data.error || 'Failed to fetch stats');
	}

	return {
		winRate: data.data.win_rate,
		weeklyProfit: data.data.weekly_profit,
		activeTrades: data.data.active_trades,
		closedThisWeek: data.data.closed_this_week
	};
}

/**
 * Fetch all trades for a trading room
 */
export async function fetchAllTrades(roomSlug: string): Promise<ApiTrade[]> {
	const response = await fetch(`/api/trades/${roomSlug}?per_page=100`, {
		credentials: 'include'
	});

	if (!response.ok) {
		throw new Error(`HTTP ${response.status}: Failed to fetch trades`);
	}

	const data = await response.json();

	if (!data.success) {
		throw new Error(data.error || 'Failed to fetch trades');
	}

	return data.data;
}

/**
 * Fetch weekly video for a trading room
 */
export async function fetchWeeklyVideo(roomSlug: string): Promise<ApiWeeklyVideo | null> {
	const response = await fetch(`/api/weekly-video/${roomSlug}`, {
		credentials: 'include'
	});

	if (!response.ok) {
		throw new Error(`HTTP ${response.status}: Failed to fetch weekly video`);
	}

	const data = await response.json();

	if (!data.success || !data.data) {
		return null;
	}

	return data.data;
}

/**
 * Check if current user is admin
 */
export async function checkAdminStatus(): Promise<boolean> {
	const response = await fetch('/api/auth/me');

	if (!response.ok) {
		return false;
	}

	const data = await response.json();
	return data.is_admin === true || data.role === 'admin' || data.role === 'super_admin';
}

// ═══════════════════════════════════════════════════════════════════════════
// FORMATTERS - Transform API responses to internal types
// ═══════════════════════════════════════════════════════════════════════════

function formatAlert(apiAlert: Record<string, unknown>): FormattedAlert {
	return {
		id: apiAlert.id as number,
		type: apiAlert.alert_type as 'ENTRY' | 'EXIT' | 'UPDATE',
		ticker: apiAlert.ticker as string,
		title: apiAlert.title as string,
		time: formatTimeAgo(apiAlert.published_at as string),
		message: apiAlert.message as string,
		isNew: apiAlert.is_new as boolean,
		notes: (apiAlert.notes as string) || '',
		tosString: (apiAlert.tos_string as string) || undefined
	};
}

function formatTradePlanEntry(apiEntry: Record<string, unknown>): TradePlanEntry {
	return {
		ticker: apiEntry.ticker as string,
		bias: apiEntry.bias as 'BULLISH' | 'BEARISH' | 'NEUTRAL',
		entry: apiEntry.entry as string,
		target1: apiEntry.target1 as string,
		target2: apiEntry.target2 as string,
		target3: apiEntry.target3 as string,
		runner: apiEntry.runner as string,
		stop: apiEntry.stop as string,
		optionsStrike: (apiEntry.options_strike as string) || '-',
		optionsExp: (apiEntry.options_exp as string) || '-',
		notes: (apiEntry.notes as string) || ''
	};
}

function formatTimeAgo(dateString: string): string {
	const date = new Date(dateString);
	const now = new Date();
	const diffMs = now.getTime() - date.getTime();
	const diffMins = Math.floor(diffMs / 60000);
	const diffHours = Math.floor(diffMs / 3600000);
	const diffDays = Math.floor(diffMs / 86400000);

	if (diffMins < 60) return `${diffMins} min ago`;
	if (diffHours < 24) return `${diffHours}h ago`;
	if (diffDays === 0) return `Today at ${date.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' })}`;
	if (diffDays === 1) return `Yesterday at ${date.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' })}`;

	return `${date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })} at ${date.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' })}`;
}

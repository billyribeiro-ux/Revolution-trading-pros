/**
 * Axum Stats Domain Adapter — Server-Only
 *
 * @version 1.0.0
 */

import { axum } from './client';

// ═══════════════════════════════════════════════════════════════════════════
// Types — Raw Axum response shapes (snake_case)
// ═══════════════════════════════════════════════════════════════════════════

export interface AxumQuickStats {
	win_rate: number;
	weekly_profit: string;
	active_trades: number;
	closed_this_week: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// API Functions
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Fetch quick stats for a trading room.
 */
export async function fetchStats(roomSlug: string): Promise<AxumQuickStats> {
	return axum.get<AxumQuickStats>(`/api/stats/${roomSlug}`);
}

/**
 * Axum Trade Plans Domain Adapter — Server-Only
 *
 * @version 1.0.0
 */

import { axum } from './client';

// ═══════════════════════════════════════════════════════════════════════════
// Types — Raw Axum response shapes (snake_case)
// ═══════════════════════════════════════════════════════════════════════════

export interface AxumTradePlanEntry {
	ticker: string;
	bias: 'BULLISH' | 'BEARISH' | 'NEUTRAL';
	entry: string;
	target1: string;
	target2: string;
	target3: string;
	runner: string;
	stop: string;
	options_strike: string | null;
	options_exp: string | null;
	notes: string | null;
}

export interface AxumTradePlanCreatePayload {
	ticker: string;
	bias: 'BULLISH' | 'BEARISH' | 'NEUTRAL';
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
// API Functions
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Fetch trade plan entries for a trading room.
 */
export async function fetchTradePlan(roomSlug: string): Promise<AxumTradePlanEntry[]> {
	const data = await axum.get<AxumTradePlanEntry[] | { data: AxumTradePlanEntry[] }>(
		`/api/trade-plans/${roomSlug}`
	);

	return Array.isArray(data) ? data : (data?.data ?? []);
}

/**
 * Create a trade plan entry.
 */
export async function createTradePlanEntry(
	roomSlug: string,
	payload: AxumTradePlanCreatePayload
): Promise<AxumTradePlanEntry> {
	return axum.post<AxumTradePlanEntry>(`/api/trade-plans/${roomSlug}`, payload);
}

/**
 * Update a trade plan entry.
 */
export async function updateTradePlanEntry(
	roomSlug: string,
	entryId: number,
	payload: Partial<AxumTradePlanCreatePayload>
): Promise<AxumTradePlanEntry> {
	return axum.put<AxumTradePlanEntry>(`/api/trade-plans/${roomSlug}/${entryId}`, payload);
}

/**
 * Delete a trade plan entry.
 */
export async function deleteTradePlanEntry(roomSlug: string, entryId: number): Promise<void> {
	await axum.delete(`/api/trade-plans/${roomSlug}/${entryId}`);
}

/**
 * Axum Trades Domain Adapter — Server-Only
 *
 * Typed server-side calls to the Axum trades API.
 * Used by Remote Functions — never imported from client code.
 *
 * @version 1.0.0
 */

import { axum, AxumError } from './client';

// ═══════════════════════════════════════════════════════════════════════════
// Types — Raw Axum response shapes (snake_case)
// ═══════════════════════════════════════════════════════════════════════════

export interface AxumTrade {
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

export interface AxumTradeCreatePayload {
	ticker: string;
	trade_type: string;
	direction: 'long' | 'short';
	entry_price: number;
	entry_date: string;
	quantity?: number;
	stop?: number;
	target?: number;
	notes?: string;
}

export interface AxumTradeClosePayload {
	exit_price: number;
	exit_date: string;
	notes?: string;
}

export interface AxumTradeUpdatePayload {
	entry_price?: number;
	stop?: number;
	target?: number;
	notes?: string;
	status?: string;
	invalidation_reason?: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// API Functions
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Fetch all trades for a trading room.
 */
export async function fetchTrades(
	roomSlug: string,
	perPage: number = 100
): Promise<AxumTrade[]> {
	return axum.get<AxumTrade[]>(
		`/api/trades/${roomSlug}`,
		{ params: { per_page: perPage } }
	);
}

/**
 * Create a new trade.
 */
export async function createTrade(
	roomSlug: string,
	payload: AxumTradeCreatePayload
): Promise<AxumTrade> {
	return axum.post<AxumTrade>(`/api/trades/${roomSlug}`, payload);
}

/**
 * Close an existing trade.
 */
export async function closeTrade(
	roomSlug: string,
	tradeId: number,
	payload: AxumTradeClosePayload
): Promise<AxumTrade> {
	return axum.patch<AxumTrade>(`/api/trades/${roomSlug}/${tradeId}`, payload);
}

/**
 * Update a trade (entry price, stop, target, notes, status).
 */
export async function updateTrade(
	roomSlug: string,
	tradeId: number,
	payload: AxumTradeUpdatePayload
): Promise<AxumTrade> {
	return axum.patch<AxumTrade>(`/api/trades/${roomSlug}/${tradeId}`, payload);
}

/**
 * Delete a trade (admin only).
 */
export async function deleteTrade(tradeId: number): Promise<void> {
	await axum.delete(`/api/admin/trades/${tradeId}`);
}

/**
 * Fetch a single trade by ID.
 */
export async function fetchTrade(
	roomSlug: string,
	tradeId: number
): Promise<AxumTrade | null> {
	try {
		return await axum.get<AxumTrade>(`/api/trades/${roomSlug}/${tradeId}`);
	} catch (err) {
		if (err instanceof AxumError && err.isNotFound) return null;
		throw err;
	}
}

/**
 * ═══════════════════════════════════════════════════════════════════════════
 * Explosive Swings — Remote Functions (Commands)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Server-side command functions for mutations requiring auth.
 * These run on the server and are called from client components.
 *
 * @version 1.0.0
 * @boundary Remote Functions (command)
 */

import { error } from '@sveltejs/kit';
import { command, getRequestEvent } from '$app/server';
import { axumTrades, axumTradePlans, axumAuth } from '$lib/server/axum';
import {
	CreateTradeInputSchema,
	CloseTradeInputSchema,
	UpdateTradeInputSchema,
	DeleteTradeInputSchema,
	CreateTradePlanEntryInputSchema,
	UpdateTradePlanEntryInputSchema,
	DeleteTradePlanEntryInputSchema
} from '$lib/shared/schemas/trades';
import { getTradePlan, getStats, getTrades } from './data.remote';

// ═══════════════════════════════════════════════════════════════════════════
// Auth Guard
// ═══════════════════════════════════════════════════════════════════════════

async function requireAuth(): Promise<void> {
	const event = getRequestEvent();
	const token = event.locals.accessToken ?? event.cookies.get('rtp_access_token');
	if (!token) {
		error(401, 'Authentication required');
	}
}

async function requireAdmin(): Promise<void> {
	await requireAuth();
	const isAdmin = await axumAuth.checkAdminStatus();
	if (!isAdmin) {
		error(403, 'Admin access required');
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Trade Commands
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Create a new trade entry.
 */
export const createTrade = command(
	CreateTradeInputSchema,
	async ({ roomSlug, ...payload }) => {
		await requireAuth();

		const trade = await axumTrades.createTrade(roomSlug, payload);

		// Invalidate related queries
		getTrades({ roomSlug }).refresh();
		getStats({ roomSlug }).refresh();

		return trade;
	}
);

/**
 * Close an existing trade position.
 */
export const closeTrade = command(
	CloseTradeInputSchema,
	async ({ roomSlug, tradeId, ...payload }) => {
		await requireAuth();

		const trade = await axumTrades.closeTrade(roomSlug, tradeId, payload);

		// Invalidate related queries
		getTrades({ roomSlug }).refresh();
		getStats({ roomSlug }).refresh();

		return trade;
	}
);

/**
 * Update a trade (entry price, stop, target, notes, status).
 */
export const updateTrade = command(
	UpdateTradeInputSchema,
	async ({ roomSlug, tradeId, ...payload }) => {
		await requireAuth();

		const trade = await axumTrades.updateTrade(roomSlug, tradeId, payload);

		// Invalidate related queries
		getTrades({ roomSlug }).refresh();

		return trade;
	}
);

/**
 * Delete a trade (admin only).
 */
export const deleteTrade = command(
	DeleteTradeInputSchema,
	async ({ tradeId }) => {
		await requireAdmin();

		await axumTrades.deleteTrade(tradeId);

		// Note: Cannot refresh with specific roomSlug here since it's not in the input.
		// Caller should manually refresh queries after this command.
	}
);

// ═══════════════════════════════════════════════════════════════════════════
// Trade Plan Commands
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Create a trade plan entry.
 */
export const createTradePlanEntry = command(
	CreateTradePlanEntryInputSchema,
	async ({ roomSlug, ...payload }) => {
		await requireAuth();

		const entry = await axumTradePlans.createTradePlanEntry(roomSlug, payload);

		// Invalidate trade plan query
		getTradePlan({ roomSlug }).refresh();

		return entry;
	}
);

/**
 * Update a trade plan entry.
 */
export const updateTradePlanEntry = command(
	UpdateTradePlanEntryInputSchema,
	async ({ roomSlug, entryId, ...payload }) => {
		await requireAuth();

		const entry = await axumTradePlans.updateTradePlanEntry(roomSlug, entryId, payload);

		// Invalidate trade plan query
		getTradePlan({ roomSlug }).refresh();

		return entry;
	}
);

/**
 * Delete a trade plan entry.
 */
export const deleteTradePlanEntry = command(
	DeleteTradePlanEntryInputSchema,
	async ({ roomSlug, entryId }) => {
		await requireAuth();

		await axumTradePlans.deleteTradePlanEntry(roomSlug, entryId);

		// Invalidate trade plan query
		getTradePlan({ roomSlug }).refresh();
	}
);

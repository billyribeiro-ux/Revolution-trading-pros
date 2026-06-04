/**
 * Explosive Swings — Remote Functions (Commands)
 *
 * Server-side `command` mutations requiring auth. Called from the client via
 * `await command(args)` directly (kit ≥ 2.61; `.run()` was removed in 2.61).
 *
 * REFRESH MODEL: the consumer (page.state.svelte.ts) reads data imperatively
 * (`await query()` → assign to local `$state`), not reactively via `query.current`.
 * It re-fetches after every mutation (fetchAlerts/fetchAllTrades/fetchStats/
 * fetchTradePlan), which is the complete and correct refresh path for this
 * architecture. Server-side `query.refresh()` only updates a matching active
 * client query instance/cache key, so it was removed here (the previous
 * `getAlerts({page:1,limit:10}).refresh()` was also the wrong instance — the
 * client paginates with `currentPage`). To adopt June-2026 client-requested
 * single-flight mutations, retain a query instance in the consumer, call
 * `command(args).updates(getAlerts(...))`, and accept it on the server with
 * `requested(getAlerts, limit)` which now yields `{ arg, query }` entries.
 */

import * as v from 'valibot';
import { error } from '@sveltejs/kit';
import { command, getRequestEvent } from '$app/server';
import { axumAlerts, axumTrades, axumTradePlans, axumAuth } from '$lib/server/axum';
import {
	CreateTradeInputSchema,
	CloseTradeInputSchema,
	UpdateTradeInputSchema,
	DeleteTradeInputSchema,
	CreateTradePlanEntryInputSchema,
	UpdateTradePlanEntryInputSchema,
	DeleteTradePlanEntryInputSchema
} from '$lib/shared/schemas/trades';

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
// Alert Commands
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Save (create or update) an alert.
 */
export const saveAlert = command(
	v.object({
		roomSlug: v.pipe(v.string(), v.nonEmpty()),
		alertId: v.optional(v.pipe(v.number(), v.integer(), v.minValue(1))),
		alertData: v.record(v.string(), v.unknown())
	}),
	async ({ roomSlug, alertId, alertData }) => {
		await requireAuth();

		if (alertId) {
			await axumAlerts.updateAlert(roomSlug, alertId, alertData);
		} else {
			await axumAlerts.createAlert(roomSlug, alertData);
		}
	}
);

/**
 * Delete an alert.
 */
export const deleteAlertCommand = command(
	v.object({
		roomSlug: v.pipe(v.string(), v.nonEmpty()),
		alertId: v.pipe(v.number(), v.integer(), v.minValue(1))
	}),
	async ({ roomSlug, alertId }) => {
		await requireAuth();

		await axumAlerts.deleteAlert(roomSlug, alertId);
	}
);

// ═══════════════════════════════════════════════════════════════════════════
// Trade Commands
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Create a new trade entry.
 */
export const createTrade = command(CreateTradeInputSchema, async ({ roomSlug, ...payload }) => {
	await requireAuth();

	const trade = await axumTrades.createTrade(roomSlug, payload);

	return trade;
});

/**
 * Close an existing trade position.
 */
export const closeTrade = command(
	CloseTradeInputSchema,
	async ({ roomSlug, tradeId, ...payload }) => {
		await requireAuth();

		const trade = await axumTrades.closeTrade(roomSlug, tradeId, payload);

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

		return trade;
	}
);

/**
 * Delete a trade (admin only).
 */
export const deleteTrade = command(DeleteTradeInputSchema, async ({ tradeId }) => {
	await requireAdmin();

	await axumTrades.deleteTrade(tradeId);

	// Note: Cannot refresh with specific roomSlug here since it's not in the input.
	// Caller should manually refresh queries after this command.
});

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
	}
);

/**
 * Shared Valibot Schemas — Trades Domain
 *
 * @version 1.0.0
 */

import * as v from 'valibot';
import { RoomSlugSchema, IdSchema, PriceSchema, DateStringSchema } from './common';

/** Alerts fetch input */
export const FetchAlertsInputSchema = v.object({
	roomSlug: RoomSlugSchema,
	page: v.pipe(v.number(), v.integer(), v.minValue(1)),
	limit: v.pipe(v.number(), v.integer(), v.minValue(1), v.maxValue(100))
});

/** Trade plan fetch input */
export const FetchTradePlanInputSchema = v.object({
	roomSlug: RoomSlugSchema
});

/** Stats fetch input */
export const FetchStatsInputSchema = v.object({
	roomSlug: RoomSlugSchema
});

/** Trades fetch input */
export const FetchTradesInputSchema = v.object({
	roomSlug: RoomSlugSchema
});

/** Weekly video fetch input */
export const FetchWeeklyVideoInputSchema = v.object({
	roomSlug: RoomSlugSchema
});

/** Trade bias enum */
const TradeBiasSchema = v.picklist(['BULLISH', 'BEARISH', 'NEUTRAL']);

/** Trade direction enum */
const TradeDirectionSchema = v.picklist(['long', 'short']);

/** Trade type enum */
const TradeTypeSchema = v.picklist(['stock', 'option', 'spread']);

/** Create trade input */
export const CreateTradeInputSchema = v.object({
	roomSlug: RoomSlugSchema,
	ticker: v.pipe(v.string(), v.nonEmpty('Ticker is required')),
	trade_type: TradeTypeSchema,
	direction: TradeDirectionSchema,
	entry_price: PriceSchema,
	entry_date: DateStringSchema,
	quantity: v.optional(v.pipe(v.number(), v.integer(), v.minValue(1))),
	stop: v.optional(PriceSchema),
	target: v.optional(PriceSchema),
	notes: v.optional(v.string())
});

/** Close trade input */
export const CloseTradeInputSchema = v.object({
	roomSlug: RoomSlugSchema,
	tradeId: IdSchema,
	exit_price: PriceSchema,
	exit_date: DateStringSchema,
	notes: v.optional(v.string())
});

/** Update trade input */
export const UpdateTradeInputSchema = v.object({
	roomSlug: RoomSlugSchema,
	tradeId: IdSchema,
	entry_price: v.optional(PriceSchema),
	stop: v.optional(PriceSchema),
	target: v.optional(PriceSchema),
	notes: v.optional(v.string()),
	status: v.optional(v.string()),
	invalidation_reason: v.optional(v.string())
});

/** Delete trade input (admin) */
export const DeleteTradeInputSchema = v.object({
	tradeId: IdSchema
});

/** Create trade plan entry input */
export const CreateTradePlanEntryInputSchema = v.object({
	roomSlug: RoomSlugSchema,
	ticker: v.pipe(v.string(), v.nonEmpty('Ticker is required')),
	bias: TradeBiasSchema,
	entry: v.pipe(v.string(), v.nonEmpty()),
	target1: v.pipe(v.string(), v.nonEmpty()),
	target2: v.pipe(v.string(), v.nonEmpty()),
	target3: v.pipe(v.string(), v.nonEmpty()),
	runner: v.pipe(v.string(), v.nonEmpty()),
	stop: v.pipe(v.string(), v.nonEmpty()),
	options_strike: v.optional(v.string()),
	options_exp: v.optional(v.string()),
	notes: v.optional(v.string())
});

/** Update trade plan entry input */
export const UpdateTradePlanEntryInputSchema = v.object({
	roomSlug: RoomSlugSchema,
	entryId: IdSchema,
	ticker: v.optional(v.pipe(v.string(), v.nonEmpty())),
	bias: v.optional(TradeBiasSchema),
	entry: v.optional(v.pipe(v.string(), v.nonEmpty())),
	target1: v.optional(v.pipe(v.string(), v.nonEmpty())),
	target2: v.optional(v.pipe(v.string(), v.nonEmpty())),
	target3: v.optional(v.pipe(v.string(), v.nonEmpty())),
	runner: v.optional(v.pipe(v.string(), v.nonEmpty())),
	stop: v.optional(v.pipe(v.string(), v.nonEmpty())),
	options_strike: v.optional(v.string()),
	options_exp: v.optional(v.string()),
	notes: v.optional(v.string())
});

/** Delete trade plan entry input */
export const DeleteTradePlanEntryInputSchema = v.object({
	roomSlug: RoomSlugSchema,
	entryId: IdSchema
});

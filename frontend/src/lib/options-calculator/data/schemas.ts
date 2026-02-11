// ============================================================
// ZOD VALIDATION SCHEMAS — API Response Validation
// ============================================================

import { z } from 'zod';

/** Polygon.io snapshot quote response */
export const polygonSnapshotSchema = z
	.object({
		ticker: z
			.object({
				ticker: z.string().optional(),
				day: z
					.object({
						o: z.number(),
						h: z.number(),
						l: z.number(),
						c: z.number(),
						v: z.number()
					})
					.optional(),
				lastTrade: z
					.object({
						p: z.number(),
						t: z.number()
					})
					.optional(),
				prevDay: z
					.object({
						c: z.number()
					})
					.optional(),
				todaysChangePerc: z.number().optional()
			})
			.optional()
	})
	.passthrough();

/** Polygon.io options chain response */
export const polygonOptionsChainSchema = z
	.object({
		results: z.array(
			z.object({
				details: z.object({
					contract_type: z.enum(['call', 'put']),
					expiration_date: z.string(),
					strike_price: z.number(),
					ticker: z.string()
				}),
				greeks: z
					.object({
						delta: z.number().optional(),
						gamma: z.number().optional(),
						theta: z.number().optional(),
						vega: z.number().optional()
					})
					.optional(),
				implied_volatility: z.number().optional(),
				last_quote: z
					.object({
						ask: z.number().optional(),
						bid: z.number().optional(),
						midpoint: z.number().optional()
					})
					.optional(),
				open_interest: z.number().optional(),
				day: z
					.object({
						close: z.number().optional(),
						volume: z.number().optional()
					})
					.optional(),
				underlying_asset: z
					.object({
						price: z.number().optional(),
						ticker: z.string().optional()
					})
					.optional()
			})
		),
		next_url: z.string().optional()
	})
	.passthrough();

/** Polygon.io ticker search response */
export const polygonTickerSearchSchema = z
	.object({
		results: z.array(
			z.object({
				ticker: z.string(),
				name: z.string(),
				market: z.string(),
				primary_exchange: z.string().optional(),
				type: z.string().optional(),
				active: z.boolean().optional()
			})
		),
		count: z.number().optional()
	})
	.passthrough();

/** Tradier options chain response */
export const tradierOptionsChainSchema = z
	.object({
		options: z
			.object({
				option: z.array(
					z.object({
						symbol: z.string(),
						last: z.number().nullable().optional(),
						volume: z.number().optional(),
						bid: z.number().nullable().optional(),
						ask: z.number().nullable().optional(),
						strike: z.number(),
						greeks: z
							.object({
								delta: z.number(),
								gamma: z.number(),
								theta: z.number(),
								vega: z.number(),
								rho: z.number(),
								mid_iv: z.number()
							})
							.optional(),
						open_interest: z.number().optional(),
						expiration_date: z.string(),
						option_type: z.enum(['call', 'put']),
						root_symbol: z.string().optional()
					})
				)
			})
			.nullable()
	})
	.passthrough();

/** Tradier expirations response */
export const tradierExpirationsSchema = z
	.object({
		expirations: z
			.object({
				date: z.union([z.array(z.string()), z.string()])
			})
			.nullable()
	})
	.passthrough();

/** FRED API response (Treasury yields) */
export const fredSeriesSchema = z
	.object({
		observations: z.array(
			z.object({
				date: z.string(),
				value: z.string()
			})
		)
	})
	.passthrough();

/** Validated stock quote (our internal format) */
export const stockQuoteSchema = z.object({
	ticker: z.string().min(1).max(10),
	price: z.number().positive(),
	open: z.number().nonnegative(),
	high: z.number().nonnegative(),
	low: z.number().nonnegative(),
	close: z.number().nonnegative(),
	previousClose: z.number().nonnegative(),
	volume: z.number().nonnegative(),
	change: z.number(),
	changePercent: z.number(),
	timestamp: z.string(),
	source: z.string()
});

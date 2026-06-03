// ============================================================
// VALIBOT VALIDATION SCHEMAS — API Response Validation
// ============================================================

import * as v from 'valibot';

/** Polygon.io snapshot quote response */
export const polygonSnapshotSchema = v.looseObject({
	ticker: v.optional(
		v.looseObject({
			ticker: v.optional(v.string()),
			day: v.optional(
				v.object({
					o: v.number(),
					h: v.number(),
					l: v.number(),
					c: v.number(),
					v: v.number()
				})
			),
			lastTrade: v.optional(
				v.object({
					p: v.number(),
					t: v.number()
				})
			),
			prevDay: v.optional(
				v.object({
					c: v.number()
				})
			),
			todaysChangePerc: v.optional(v.number())
		})
	)
});

/** Polygon.io options chain response */
export const polygonOptionsChainSchema = v.looseObject({
	results: v.array(
		v.object({
			details: v.object({
				contract_type: v.picklist(['call', 'put']),
				expiration_date: v.string(),
				strike_price: v.number(),
				ticker: v.string()
			}),
			greeks: v.optional(
				v.object({
					delta: v.optional(v.number()),
					gamma: v.optional(v.number()),
					theta: v.optional(v.number()),
					vega: v.optional(v.number())
				})
			),
			implied_volatility: v.optional(v.number()),
			last_quote: v.optional(
				v.object({
					ask: v.optional(v.number()),
					bid: v.optional(v.number()),
					midpoint: v.optional(v.number())
				})
			),
			open_interest: v.optional(v.number()),
			day: v.optional(
				v.object({
					close: v.optional(v.number()),
					volume: v.optional(v.number())
				})
			),
			underlying_asset: v.optional(
				v.object({
					price: v.optional(v.number()),
					ticker: v.optional(v.string())
				})
			)
		})
	),
	next_url: v.optional(v.string())
});

/** Polygon.io ticker search response */
export const polygonTickerSearchSchema = v.looseObject({
	results: v.array(
		v.object({
			ticker: v.string(),
			name: v.string(),
			market: v.string(),
			primary_exchange: v.optional(v.string()),
			type: v.optional(v.string()),
			active: v.optional(v.boolean())
		})
	),
	count: v.optional(v.number())
});

/** Tradier options chain response */
export const tradierOptionsChainSchema = v.looseObject({
	options: v.nullable(
		v.object({
			option: v.array(
				v.object({
					symbol: v.string(),
					last: v.optional(v.nullable(v.number())),
					volume: v.optional(v.number()),
					bid: v.optional(v.nullable(v.number())),
					ask: v.optional(v.nullable(v.number())),
					strike: v.number(),
					greeks: v.optional(
						v.object({
							delta: v.number(),
							gamma: v.number(),
							theta: v.number(),
							vega: v.number(),
							rho: v.number(),
							mid_iv: v.number()
						})
					),
					open_interest: v.optional(v.number()),
					expiration_date: v.string(),
					option_type: v.picklist(['call', 'put']),
					root_symbol: v.optional(v.string())
				})
			)
		})
	)
});

/** Tradier expirations response */
export const tradierExpirationsSchema = v.looseObject({
	expirations: v.nullable(
		v.object({
			date: v.union([v.array(v.string()), v.string()])
		})
	)
});

/** FRED API response (Treasury yields) */
export const fredSeriesSchema = v.looseObject({
	observations: v.array(
		v.object({
			date: v.string(),
			value: v.string()
		})
	)
});

/** Validated stock quote (our internal format) */
export const stockQuoteSchema = v.object({
	ticker: v.pipe(v.string(), v.minLength(1), v.maxLength(10)),
	price: v.pipe(v.number(), v.gtValue(0)),
	open: v.pipe(v.number(), v.minValue(0)),
	high: v.pipe(v.number(), v.minValue(0)),
	low: v.pipe(v.number(), v.minValue(0)),
	close: v.pipe(v.number(), v.minValue(0)),
	previousClose: v.pipe(v.number(), v.minValue(0)),
	volume: v.pipe(v.number(), v.minValue(0)),
	change: v.number(),
	changePercent: v.number(),
	timestamp: v.string(),
	source: v.string()
});

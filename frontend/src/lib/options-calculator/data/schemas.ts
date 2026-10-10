// ============================================================
// VALIBOT VALIDATION SCHEMAS — API Response Validation
// Upstream schemas describe raw third-party payloads; the internal
// schemas below describe what our proxy routes return and what the
// cache persists. Each internal schema is checked against its type
// with `satisfies`, so the two cannot drift apart.
// ============================================================

import * as v from 'valibot';
import { DATA_PROVIDER_NAMES } from './types.js';
import type {
	DividendInfo,
	EarningsInfo,
	ExpirationList,
	HistoricalVolData,
	OHLCV,
	OptionQuote,
	OptionsChain,
	RiskFreeRateData,
	StockQuote,
	TickerSearchResult
} from './types.js';

/** A provider name (`DataProviderName`). */
export const providerNameSchema = v.picklist(DATA_PROVIDER_NAMES);

/**
 * An optional field as our types declare it (`field?: T`). JSON has no
 * `undefined`, so an upstream `null` is accepted and normalised away.
 */
function optionalField<T>(schema: v.GenericSchema<T>) {
	return v.optional(
		v.pipe(
			v.nullable(schema),
			v.transform((value) => value ?? undefined)
		)
	);
}

const optionalNumber = optionalField(v.number());
const optionalString = optionalField(v.string());

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

// ── Internal formats (proxy route responses + cache entries) ──

/**
 * Validated stock quote (our internal format). A non-positive price is
 * rejected: it would turn every Black-Scholes output into NaN.
 */
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
	marketCap: optionalNumber,
	timestamp: v.string(),
	source: providerNameSchema
}) satisfies v.GenericSchema<unknown, StockQuote>;

/** Single option contract quote */
export const optionQuoteSchema = v.object({
	symbol: v.string(),
	underlying: v.string(),
	type: v.picklist(['call', 'put']),
	strike: v.number(),
	expiration: v.string(),
	lastPrice: v.number(),
	bid: v.number(),
	ask: v.number(),
	mid: v.number(),
	volume: v.number(),
	openInterest: v.number(),
	impliedVolatility: v.number(),
	delta: optionalNumber,
	gamma: optionalNumber,
	theta: optionalNumber,
	vega: optionalNumber,
	rho: optionalNumber,
	inTheMoney: v.boolean(),
	source: providerNameSchema
}) satisfies v.GenericSchema<unknown, OptionQuote>;

/** Full options chain for a ticker + expiration */
export const optionsChainSchema = v.object({
	underlying: v.string(),
	underlyingPrice: v.number(),
	expiration: v.string(),
	calls: v.array(optionQuoteSchema),
	puts: v.array(optionQuoteSchema),
	timestamp: v.string(),
	source: providerNameSchema
}) satisfies v.GenericSchema<unknown, OptionsChain>;

/** Available expirations for a ticker */
export const expirationListSchema = v.object({
	underlying: v.string(),
	expirations: v.array(v.string()),
	source: providerNameSchema
}) satisfies v.GenericSchema<unknown, ExpirationList>;

/** Historical OHLCV series */
export const ohlcvListSchema = v.array(
	v.object({
		date: v.string(),
		open: v.number(),
		high: v.number(),
		low: v.number(),
		close: v.number(),
		volume: v.number()
	})
) satisfies v.GenericSchema<unknown, OHLCV[]>;

/** Ticker search results */
export const tickerSearchResultsSchema = v.array(
	v.object({
		ticker: v.string(),
		name: v.string(),
		type: v.picklist(['stock', 'etf', 'index', 'forex', 'crypto']),
		exchange: v.string(),
		primaryExchange: optionalString
	})
) satisfies v.GenericSchema<unknown, TickerSearchResult[]>;

/** Treasury yield curve */
export const riskFreeRateSchema = v.object({
	rate1M: v.number(),
	rate3M: v.number(),
	rate6M: v.number(),
	rate1Y: v.number(),
	rate2Y: v.number(),
	rate10Y: v.number(),
	date: v.string(),
	source: providerNameSchema
}) satisfies v.GenericSchema<unknown, RiskFreeRateData>;

/** Dividend info */
export const dividendInfoSchema = v.object({
	ticker: v.string(),
	annualDividend: v.number(),
	dividendYield: v.number(),
	exDividendDate: optionalString,
	paymentDate: optionalString,
	frequency: v.picklist(['annual', 'semi-annual', 'quarterly', 'monthly', 'none']),
	source: providerNameSchema
}) satisfies v.GenericSchema<unknown, DividendInfo>;

/** Earnings info */
export const earningsInfoSchema = v.object({
	ticker: v.string(),
	nextEarningsDate: optionalString,
	daysToEarnings: optionalNumber,
	expectedMove: optionalNumber,
	expectedMovePercent: optionalNumber,
	source: providerNameSchema
}) satisfies v.GenericSchema<unknown, EarningsInfo>;

/** Historical volatility data */
export const historicalVolSchema = v.object({
	ticker: v.string(),
	hv10: v.number(),
	hv20: v.number(),
	hv30: v.number(),
	hv60: v.number(),
	hv90: v.number(),
	ivPercentile: optionalNumber,
	ivRank: optionalNumber,
	source: providerNameSchema
}) satisfies v.GenericSchema<unknown, HistoricalVolData>;

// ============================================================
// MISPRICING DETECTION
// Compares a market quote's mid price with its Black-Scholes value.
// Shared by the options chain viewer and the mispricing alerts.
// ============================================================

import { priceOption } from './black-scholes.js';
import type { MispricingResult, OptionQuote } from '../data/types.js';

const MS_PER_DAY = 86_400_000;

/** Flag contracts whose mid differs from the Black-Scholes value by at least this (%). */
export const MISPRICING_THRESHOLD_PCT = 5;

/** Above this (%) a mispricing is "significant". */
export const SIGNIFICANT_MISPRICING_PCT = 15;

/** Market inputs for the reference price, besides the contract's own strike and expiration. */
export interface MispricingMarket {
	spotPrice: number;
	/** Used when the quote carries no implied volatility. */
	volatility: number;
	riskFreeRate: number;
	dividendYield: number;
}

/** Whole days from `now` (epoch ms) until `expiration` (`YYYY-MM-DD`); at least 1. */
export function daysUntilExpiration(expiration: string, now: number): number {
	return Math.max(1, Math.round((Date.parse(expiration) - now) / MS_PER_DAY));
}

/**
 * The quote's mispricing against Black-Scholes, or `null` when it is within
 * `MISPRICING_THRESHOLD_PCT` or either price is too small to compare.
 */
export function detectMispricing(
	option: OptionQuote,
	market: MispricingMarket,
	now: number
): MispricingResult | null {
	if (option.mid <= 0.01) return null;

	const theoretical = priceOption(
		{
			spotPrice: market.spotPrice,
			strikePrice: option.strike,
			volatility: option.impliedVolatility > 0 ? option.impliedVolatility : market.volatility,
			timeToExpiry: daysUntilExpiration(option.expiration, now) / 365,
			riskFreeRate: market.riskFreeRate,
			dividendYield: market.dividendYield
		},
		option.type
	);
	if (theoretical <= 0.01) return null;

	const diff = option.mid - theoretical;
	const diffPct = (diff / theoretical) * 100;
	if (Math.abs(diffPct) < MISPRICING_THRESHOLD_PCT) return null;

	return {
		option,
		theoreticalPrice: theoretical,
		marketPrice: option.mid,
		priceDifference: diff,
		priceDifferencePct: diffPct,
		direction: diff > 0 ? 'overpriced' : 'underpriced',
		significance: Math.abs(diffPct) > SIGNIFICANT_MISPRICING_PCT ? 'significant' : 'moderate'
	};
}

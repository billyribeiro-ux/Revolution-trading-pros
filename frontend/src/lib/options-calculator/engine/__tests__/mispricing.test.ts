import { describe, expect, it } from 'vitest';
import { priceOption } from '../black-scholes.js';
import {
	MISPRICING_THRESHOLD_PCT,
	daysUntilExpiration,
	detectMispricing,
	type MispricingMarket
} from '../mispricing.js';
import type { OptionQuote } from '../../data/types.js';

const NOW = Date.parse('2026-01-02T12:00:00Z');
const EXPIRATION = '2026-03-02';
const MARKET: MispricingMarket = {
	spotPrice: 100,
	volatility: 0.3,
	riskFreeRate: 0.04,
	dividendYield: 0
};

function quoteAt(mid: number, overrides: Partial<OptionQuote> = {}): OptionQuote {
	return {
		symbol: 'TEST260302C00100000',
		underlying: 'TEST',
		strike: 100,
		expiration: EXPIRATION,
		type: 'call',
		bid: mid,
		ask: mid,
		lastPrice: mid,
		mid,
		volume: 0,
		openInterest: 0,
		impliedVolatility: 0.25,
		inTheMoney: false,
		source: 'mock',
		...overrides
	};
}

const theoretical = priceOption(
	{
		...MARKET,
		strikePrice: 100,
		volatility: 0.25,
		timeToExpiry: daysUntilExpiration(EXPIRATION, NOW) / 365
	},
	'call'
);

describe('daysUntilExpiration', () => {
	it('rounds to whole days and never goes below one', () => {
		expect(daysUntilExpiration(EXPIRATION, NOW)).toBe(59);
		expect(daysUntilExpiration('2025-12-01', NOW)).toBe(1);
	});
});

describe('detectMispricing', () => {
	it('ignores quotes within the threshold', () => {
		expect(detectMispricing(quoteAt(theoretical * 1.01), MARKET, NOW)).toBeNull();
	});

	it('flags a quote above the threshold as overpriced, and a large one as significant', () => {
		const moderate = detectMispricing(quoteAt(theoretical * 1.1), MARKET, NOW);
		expect(moderate).toMatchObject({ direction: 'overpriced', significance: 'moderate' });
		expect(moderate?.priceDifferencePct).toBeCloseTo(10, 6);
		expect(moderate?.theoreticalPrice).toBeCloseTo(theoretical, 10);

		const significant = detectMispricing(quoteAt(theoretical * 0.8), MARKET, NOW);
		expect(significant).toMatchObject({ direction: 'underpriced', significance: 'significant' });
		expect(Math.abs(significant?.priceDifferencePct ?? 0)).toBeGreaterThan(
			MISPRICING_THRESHOLD_PCT
		);
	});

	it('falls back to the market volatility when the quote has no implied volatility', () => {
		const result = detectMispricing(quoteAt(1, { impliedVolatility: 0 }), MARKET, NOW);
		const fallback = priceOption(
			{ ...MARKET, strikePrice: 100, timeToExpiry: daysUntilExpiration(EXPIRATION, NOW) / 365 },
			'call'
		);
		expect(result?.theoreticalPrice).toBeCloseTo(fallback, 10);
	});

	it('skips quotes too cheap to compare', () => {
		expect(detectMispricing(quoteAt(0.01), MARKET, NOW)).toBeNull();
	});
});

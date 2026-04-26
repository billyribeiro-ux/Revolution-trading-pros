// ============================================================
// BLACK-SCHOLES PRICING ENGINE
// Generalized Black-Scholes-Merton with continuous dividend yield
// ============================================================

import { MATH } from './constants.js';
import type { BSInputs, BSPricingResult, OptionType } from './types.js';

/**
 * Standard normal cumulative distribution function (CDF).
 * Uses Abramowitz & Stegun approximation (error < 7.5e-8).
 */
export function normalCDF(x: number): number {
	if (x < -8) return 0;
	if (x > 8) return 1;

	const a1 = 0.254829592;
	const a2 = -0.284496736;
	const a3 = 1.421413741;
	const a4 = -1.453152027;
	const a5 = 1.061405429;
	const p = 0.3275911;

	const sign = x < 0 ? -1 : 1;
	const absX = Math.abs(x);
	const t = 1.0 / (1.0 + p * absX);
	const t2 = t * t;
	const t3 = t2 * t;
	const t4 = t3 * t;
	const t5 = t4 * t;

	const y = 1.0 - (a1 * t + a2 * t2 + a3 * t3 + a4 * t4 + a5 * t5) * Math.exp((-absX * absX) / 2);

	return 0.5 * (1.0 + sign * y);
}

/**
 * Standard normal probability density function (PDF).
 */
export function normalPDF(x: number): number {
	return MATH.INV_SQRT_2PI * Math.exp(-0.5 * x * x);
}

/**
 * Calculate d1 parameter of Black-Scholes formula.
 * d1 = [ln(S/K) + (r - q + σ²/2) * T] / (σ * √T)
 */
export function calculateD1(inputs: BSInputs): number {
	const { spotPrice, strikePrice, volatility, timeToExpiry, riskFreeRate, dividendYield } = inputs;
	const sqrtT = Math.sqrt(timeToExpiry);
	const volSqrtT = volatility * sqrtT;

	if (volSqrtT < MATH.EPSILON) {
		return spotPrice > strikePrice ? Infinity : -Infinity;
	}

	return (
		(Math.log(spotPrice / strikePrice) +
			(riskFreeRate - dividendYield + 0.5 * volatility * volatility) * timeToExpiry) /
		volSqrtT
	);
}

/**
 * Calculate d2 parameter of Black-Scholes formula.
 * d2 = d1 - σ * √T
 */
export function calculateD2(d1: number, volatility: number, timeToExpiry: number): number {
	return d1 - volatility * Math.sqrt(timeToExpiry);
}

/**
 * Black-Scholes-Merton pricing for European options with continuous dividend yield.
 *
 * Call: S * e^(-qT) * N(d1) - K * e^(-rT) * N(d2)
 * Put:  K * e^(-rT) * N(-d2) - S * e^(-qT) * N(-d1)
 */
export function price(inputs: BSInputs): BSPricingResult {
	const { spotPrice, strikePrice, volatility, timeToExpiry, riskFreeRate, dividendYield } = inputs;

	if (timeToExpiry <= 0) {
		const callIntrinsic = Math.max(spotPrice - strikePrice, 0);
		const putIntrinsic = Math.max(strikePrice - spotPrice, 0);
		return {
			callPrice: callIntrinsic,
			putPrice: putIntrinsic,
			d1: spotPrice > strikePrice ? Infinity : -Infinity,
			d2: spotPrice > strikePrice ? Infinity : -Infinity,
			parityCheck: 0
		};
	}

	const d1 = calculateD1(inputs);
	const d2 = calculateD2(d1, volatility, timeToExpiry);

	const discountFactor = Math.exp(-riskFreeRate * timeToExpiry);
	const dividendDiscount = Math.exp(-dividendYield * timeToExpiry);

	const callPrice =
		spotPrice * dividendDiscount * normalCDF(d1) - strikePrice * discountFactor * normalCDF(d2);

	const putPrice =
		strikePrice * discountFactor * normalCDF(-d2) - spotPrice * dividendDiscount * normalCDF(-d1);

	const parityLHS = callPrice - putPrice;
	const parityRHS = spotPrice * dividendDiscount - strikePrice * discountFactor;
	const parityCheck = Math.abs(parityLHS - parityRHS);

	return {
		callPrice: Math.max(callPrice, 0),
		putPrice: Math.max(putPrice, 0),
		d1,
		d2,
		parityCheck
	};
}

/**
 * Price a single option (call or put).
 */
export function priceOption(inputs: BSInputs, type: OptionType): number {
	const result = price(inputs);
	return type === 'call' ? result.callPrice : result.putPrice;
}

/**
 * Calculate intrinsic value.
 */
export function intrinsicValue(spotPrice: number, strikePrice: number, type: OptionType): number {
	if (type === 'call') return Math.max(spotPrice - strikePrice, 0);
	return Math.max(strikePrice - spotPrice, 0);
}

/**
 * Determine moneyness.
 */
export function moneyness(
	spotPrice: number,
	strikePrice: number,
	type: OptionType
): 'ITM' | 'ATM' | 'OTM' {
	const threshold = spotPrice * 0.005;
	const diff = Math.abs(spotPrice - strikePrice);

	if (diff < threshold) return 'ATM';
	if (type === 'call') return spotPrice > strikePrice ? 'ITM' : 'OTM';
	return spotPrice < strikePrice ? 'ITM' : 'OTM';
}

/**
 * Calculate breakeven price at expiration.
 */
export function breakeven(strikePrice: number, premium: number, type: OptionType): number {
	if (type === 'call') return strikePrice + premium;
	return strikePrice - premium;
}

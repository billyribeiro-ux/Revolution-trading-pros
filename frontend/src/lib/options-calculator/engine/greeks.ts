// ============================================================
// GREEKS ENGINE — First, Second, and Third Order
// All formulas use the Generalized BSM with continuous dividends
// ============================================================

import { normalCDF, normalPDF, calculateD1, calculateD2 } from './black-scholes.js';
import { MATH } from './constants.js';
import type {
	BSInputs,
	OptionType,
	FirstOrderGreeks,
	SecondOrderGreeks,
	AllGreeks
} from './types.js';

/**
 * Calculate ALL first-order Greeks for a given option.
 */
export function firstOrderGreeks(inputs: BSInputs, type: OptionType): FirstOrderGreeks {
	const { spotPrice, strikePrice, volatility, timeToExpiry, riskFreeRate, dividendYield } = inputs;

	if (timeToExpiry <= MATH.EPSILON) {
		const isITM = type === 'call' ? spotPrice > strikePrice : spotPrice < strikePrice;
		return {
			delta: isITM ? (type === 'call' ? 1 : -1) : 0,
			gamma: 0,
			theta: 0,
			vega: 0,
			rho: 0
		};
	}

	const d1 = calculateD1(inputs);
	const d2 = calculateD2(d1, volatility, timeToExpiry);
	const sqrtT = Math.sqrt(timeToExpiry);
	const discountFactor = Math.exp(-riskFreeRate * timeToExpiry);
	const dividendDiscount = Math.exp(-dividendYield * timeToExpiry);
	const nd1 = normalPDF(d1);

	// DELTA: ∂V/∂S
	let delta: number;
	if (type === 'call') {
		delta = dividendDiscount * normalCDF(d1);
	} else {
		delta = -dividendDiscount * normalCDF(-d1);
	}

	// GAMMA: ∂²V/∂S² — Same for calls and puts
	const gamma = (dividendDiscount * nd1) / (spotPrice * volatility * sqrtT);

	// THETA: ∂V/∂t (per calendar day — divide annual by 365)
	let theta: number;
	const thetaCommon = -(spotPrice * dividendDiscount * nd1 * volatility) / (2 * sqrtT);
	if (type === 'call') {
		theta =
			thetaCommon +
			dividendYield * spotPrice * dividendDiscount * normalCDF(d1) -
			riskFreeRate * strikePrice * discountFactor * normalCDF(d2);
	} else {
		theta =
			thetaCommon -
			dividendYield * spotPrice * dividendDiscount * normalCDF(-d1) +
			riskFreeRate * strikePrice * discountFactor * normalCDF(-d2);
	}
	theta = theta / MATH.CALENDAR_DAYS_PER_YEAR;

	// VEGA: ∂V/∂σ — Same for calls and puts. Returned per 1% move (divide by 100)
	const vega = (spotPrice * dividendDiscount * nd1 * sqrtT) / 100;

	// RHO: ∂V/∂r — Per 1% move (divide by 100)
	let rho: number;
	if (type === 'call') {
		rho = (strikePrice * timeToExpiry * discountFactor * normalCDF(d2)) / 100;
	} else {
		rho = -(strikePrice * timeToExpiry * discountFactor * normalCDF(-d2)) / 100;
	}

	return { delta, gamma, theta, vega, rho };
}

/**
 * Calculate ALL second-order Greeks.
 */
export function secondOrderGreeks(inputs: BSInputs, type: OptionType): SecondOrderGreeks {
	const { spotPrice, volatility, timeToExpiry, riskFreeRate, dividendYield } = inputs;

	if (timeToExpiry <= MATH.EPSILON) {
		return { charm: 0, vanna: 0, volga: 0, veta: 0, speed: 0, color: 0, zomma: 0 };
	}

	const d1 = calculateD1(inputs);
	const d2 = calculateD2(d1, volatility, timeToExpiry);
	const sqrtT = Math.sqrt(timeToExpiry);
	const dividendDiscount = Math.exp(-dividendYield * timeToExpiry);
	const nd1 = normalPDF(d1);

	// CHARM (Delta Decay): ∂Δ/∂t
	let charm: number;
	const charmCommon =
		(dividendDiscount *
			nd1 *
			(2 * (riskFreeRate - dividendYield) * timeToExpiry - d2 * volatility * sqrtT)) /
		(2 * timeToExpiry * volatility * sqrtT);
	if (type === 'call') {
		charm = -dividendYield * dividendDiscount * normalCDF(d1) + charmCommon;
	} else {
		charm = dividendYield * dividendDiscount * normalCDF(-d1) + charmCommon;
	}
	charm = charm / MATH.CALENDAR_DAYS_PER_YEAR;

	// VANNA: ∂Δ/∂σ = ∂Vega/∂S — Same for calls and puts
	const vanna = (-dividendDiscount * nd1 * d2) / volatility;

	// VOLGA (Vomma): ∂Vega/∂σ = ∂²V/∂σ² — Same for calls and puts
	const volga = (spotPrice * dividendDiscount * nd1 * sqrtT * d1 * d2) / volatility;

	// VETA: ∂Vega/∂t
	const veta =
		-spotPrice *
		dividendDiscount *
		nd1 *
		sqrtT *
		(dividendYield +
			((riskFreeRate - dividendYield) * d1) / (volatility * sqrtT) -
			(1 + d1 * d2) / (2 * timeToExpiry));
	const vetaDaily = veta / MATH.CALENDAR_DAYS_PER_YEAR;

	// SPEED: ∂Γ/∂S = ∂³V/∂S³
	const gamma = (dividendDiscount * nd1) / (spotPrice * volatility * sqrtT);
	const speed = -(gamma / spotPrice) * (d1 / (volatility * sqrtT) + 1);

	// COLOR: ∂Γ/∂t
	const color =
		((-dividendDiscount * nd1) / (2 * spotPrice * timeToExpiry * volatility * sqrtT)) *
		(2 * dividendYield * timeToExpiry +
			1 +
			(d1 * (2 * (riskFreeRate - dividendYield) * timeToExpiry - d2 * volatility * sqrtT)) /
				(volatility * sqrtT));
	const colorDaily = color / MATH.CALENDAR_DAYS_PER_YEAR;

	// ZOMMA: ∂Γ/∂σ
	const zomma = gamma * ((d1 * d2 - 1) / volatility);

	return {
		charm,
		vanna,
		volga,
		veta: vetaDaily,
		speed,
		color: colorDaily,
		zomma
	};
}

/**
 * Calculate ALL Greeks (first + second order) in one pass.
 */
export function allGreeks(inputs: BSInputs, type: OptionType): AllGreeks {
	return {
		first: firstOrderGreeks(inputs, type),
		second: secondOrderGreeks(inputs, type)
	};
}

/**
 * Calculate a specific Greek value across a range of strikes.
 */
export function greekAcrossStrikes(
	inputs: BSInputs,
	type: OptionType,
	greekName: string,
	strikes: number[]
): { strike: number; value: number }[] {
	return strikes.map((strike) => {
		const modifiedInputs = { ...inputs, strikePrice: strike };
		const greeks = allGreeks(modifiedInputs, type);

		let value = 0;
		if (greekName in greeks.first) {
			value = greeks.first[greekName as keyof FirstOrderGreeks];
		} else if (greekName in greeks.second) {
			value = greeks.second[greekName as keyof SecondOrderGreeks];
		}

		return { strike, value };
	});
}

/**
 * Calculate Greeks matrix: across strikes AND expiries.
 */
export function greeksMatrix(
	inputs: BSInputs,
	type: OptionType,
	greekName: string,
	strikes: number[],
	expiries: number[]
): { strike: number; expiry: number; value: number }[] {
	const results: { strike: number; expiry: number; value: number }[] = [];

	for (const strike of strikes) {
		for (const expiry of expiries) {
			const modifiedInputs = { ...inputs, strikePrice: strike, timeToExpiry: expiry };
			const greeks = allGreeks(modifiedInputs, type);

			let value = 0;
			if (greekName in greeks.first) {
				value = greeks.first[greekName as keyof FirstOrderGreeks];
			} else if (greekName in greeks.second) {
				value = greeks.second[greekName as keyof SecondOrderGreeks];
			}

			results.push({ strike, expiry, value });
		}
	}

	return results;
}

// ============================================================
// IMPLIED VOLATILITY SOLVER — Newton-Raphson Method
// ============================================================

import { priceOption } from './black-scholes.js';
import { firstOrderGreeks } from './greeks.js';
import { MATH } from './constants.js';
import type { BSInputs, OptionType, IVSolverResult } from './types.js';

/**
 * Solve for implied volatility using Newton-Raphson iteration.
 */
export function solveImpliedVolatility(
	marketPrice: number,
	inputs: BSInputs,
	type: OptionType
): IVSolverResult {
	if (marketPrice <= 0) {
		return { impliedVolatility: 0, iterations: 0, converged: false, error: Infinity };
	}

	let vol = inputs.volatility > 0 ? inputs.volatility : 0.25;

	for (let i = 0; i < MATH.NR_MAX_ITERATIONS; i++) {
		const iterations = i + 1;
		const testInputs: BSInputs = { ...inputs, volatility: vol };
		const bsPrice = priceOption(testInputs, type);
		const diff = bsPrice - marketPrice;

		if (Math.abs(diff) < MATH.NR_TOLERANCE) {
			return {
				impliedVolatility: vol,
				iterations,
				converged: true,
				error: Math.abs(diff)
			};
		}

		const greeks = firstOrderGreeks(testInputs, type);
		const vegaRaw = greeks.vega * 100;

		if (Math.abs(vegaRaw) < MATH.EPSILON) {
			break;
		}

		vol = vol - diff / vegaRaw;
		vol = Math.max(0.001, Math.min(vol, 10.0));
	}

	return bisectionIV(marketPrice, inputs, type);
}

/**
 * Bisection method fallback for IV solving.
 */
function bisectionIV(marketPrice: number, inputs: BSInputs, type: OptionType): IVSolverResult {
	let low = 0.001;
	let high = 5.0;
	let mid = 0.5;

	for (let i = 0; i < 200; i++) {
		const iterations = i + 1;
		mid = (low + high) / 2;
		const testInputs: BSInputs = { ...inputs, volatility: mid };
		const bsPrice = priceOption(testInputs, type);
		const diff = bsPrice - marketPrice;

		if (Math.abs(diff) < MATH.NR_TOLERANCE) {
			return {
				impliedVolatility: mid,
				iterations,
				converged: true,
				error: Math.abs(diff)
			};
		}

		if (diff > 0) {
			high = mid;
		} else {
			low = mid;
		}
	}

	return {
		impliedVolatility: mid,
		iterations: 200,
		converged: false,
		error: Math.abs(priceOption({ ...inputs, volatility: mid }, type) - marketPrice)
	};
}

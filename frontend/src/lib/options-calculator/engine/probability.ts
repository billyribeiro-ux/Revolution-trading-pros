// ============================================================
// PROBABILITY ENGINE
// ============================================================

import { normalCDF } from './black-scholes.js';
import type { BSInputs, OptionType, ProbabilityResult } from './types.js';

/**
 * Calculate comprehensive probability metrics for an option.
 */
export function calculateProbabilities(inputs: BSInputs, type: OptionType): ProbabilityResult {
	const { spotPrice, strikePrice, volatility, timeToExpiry, riskFreeRate, dividendYield } = inputs;

	const sqrtT = Math.sqrt(timeToExpiry);
	const volSqrtT = volatility * sqrtT;

	const d2 =
		(Math.log(spotPrice / strikePrice) +
			(riskFreeRate - dividendYield - 0.5 * volatility * volatility) * timeToExpiry) /
		volSqrtT;

	const probabilityITM = type === 'call' ? normalCDF(d2) : normalCDF(-d2);
	const probabilityOTM = 1 - probabilityITM;

	const absDist = Math.abs(Math.log(spotPrice / strikePrice));
	const drift = riskFreeRate - dividendYield - 0.5 * volatility * volatility;

	const probabilityOfTouching = Math.min(
		1,
		normalCDF((-absDist + drift * timeToExpiry) / volSqrtT) +
			Math.exp((2 * drift * absDist) / (volatility * volatility)) *
				normalCDF((-absDist - drift * timeToExpiry) / volSqrtT)
	);

	const expectedMove = spotPrice * volSqrtT;
	const expectedMovePercent = volSqrtT * 100;

	const oneSDRange: [number, number] = [
		spotPrice * Math.exp(-volSqrtT),
		spotPrice * Math.exp(volSqrtT)
	];

	const twoSDRange: [number, number] = [
		spotPrice * Math.exp(-2 * volSqrtT),
		spotPrice * Math.exp(2 * volSqrtT)
	];

	return {
		probabilityITM,
		probabilityOTM,
		probabilityOfTouching,
		expectedMove,
		expectedMovePercent,
		oneSDRange,
		twoSDRange
	};
}

// ============================================================
// MONTE CARLO SIMULATION ENGINE
// Geometric Brownian Motion with antithetic variates
// ============================================================

import { price as bsPrice } from './black-scholes.js';
import type { BSInputs, MonteCarloConfig, MonteCarloResult, MonteCarloPath } from './types.js';

/**
 * Box-Muller transform to generate standard normal random numbers.
 */
function boxMuller(): [number, number] {
	let u1 = 0;
	let u2 = 0;
	while (u1 === 0) u1 = Math.random();
	while (u2 === 0) u2 = Math.random();

	const r = Math.sqrt(-2.0 * Math.log(u1));
	const theta = 2.0 * Math.PI * u2;
	return [r * Math.cos(theta), r * Math.sin(theta)];
}

/**
 * Seeded pseudo-random number generator (Mulberry32).
 */
function createSeededRNG(seed: number): () => number {
	let s = seed | 0;
	return () => {
		s = (s + 0x6d2b79f5) | 0;
		let t = Math.imul(s ^ (s >>> 15), 1 | s);
		t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
		return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
	};
}

/**
 * Seeded Box-Muller transform.
 */
function seededBoxMuller(rng: () => number): [number, number] {
	let u1 = rng();
	let u2 = rng();
	while (u1 === 0) u1 = rng();
	while (u2 === 0) u2 = rng();

	const r = Math.sqrt(-2.0 * Math.log(u1));
	const theta = 2.0 * Math.PI * u2;
	return [r * Math.cos(theta), r * Math.sin(theta)];
}

/**
 * Run Monte Carlo simulation using Geometric Brownian Motion.
 * Uses antithetic variates for variance reduction.
 */
export function runMonteCarlo(
	inputs: BSInputs,
	config: MonteCarloConfig = { numPaths: 5000, numSteps: 100 }
): MonteCarloResult {
	const startTime = performance.now();

	const { spotPrice, strikePrice, volatility, timeToExpiry, riskFreeRate, dividendYield } = inputs;
	const { numPaths, numSteps, seed } = config;

	const dt = timeToExpiry / numSteps;
	const sqrtDt = Math.sqrt(dt);
	const drift = (riskFreeRate - dividendYield - 0.5 * volatility * volatility) * dt;
	const diffusion = volatility * sqrtDt;

	const timePoints = Array.from({ length: numSteps + 1 }, (_, i) => (i / numSteps) * timeToExpiry);

	const rng = seed !== undefined ? createSeededRNG(seed) : undefined;
	const generateNormal = rng ? () => seededBoxMuller(rng) : boxMuller;

	const paths: MonteCarloPath[] = [];
	const finalPrices: number[] = [];

	const halfPaths = Math.ceil(numPaths / 2);

	for (let p = 0; p < halfPaths; p++) {
		const prices1: number[] = [spotPrice];
		const prices2: number[] = [spotPrice];

		let S1 = spotPrice;
		let S2 = spotPrice;
		let max1 = spotPrice;
		let min1 = spotPrice;
		let max2 = spotPrice;
		let min2 = spotPrice;

		for (let step = 0; step < numSteps; step++) {
			const [z1] = generateNormal();

			S1 = S1 * Math.exp(drift + diffusion * z1);
			prices1.push(S1);
			max1 = Math.max(max1, S1);
			min1 = Math.min(min1, S1);

			S2 = S2 * Math.exp(drift + diffusion * -z1);
			prices2.push(S2);
			max2 = Math.max(max2, S2);
			min2 = Math.min(min2, S2);
		}

		paths.push({ id: p * 2, prices: prices1, finalPrice: S1, maxPrice: max1, minPrice: min1 });
		finalPrices.push(S1);

		if (paths.length < numPaths) {
			paths.push({
				id: p * 2 + 1,
				prices: prices2,
				finalPrice: S2,
				maxPrice: max2,
				minPrice: min2
			});
			finalPrices.push(S2);
		}
	}

	const sortedPrices = [...finalPrices].sort((a, b) => a - b);
	const n = sortedPrices.length;

	const percentile = (pct: number): number => {
		const idx = Math.floor(pct * n);
		return sortedPrices[Math.min(idx, n - 1)];
	};

	const mean = finalPrices.reduce((sum, pr) => sum + pr, 0) / n;
	const variance = finalPrices.reduce((sum, pr) => sum + (pr - mean) ** 2, 0) / (n - 1);
	const stdDev = Math.sqrt(variance);

	const discountFactor = Math.exp(-riskFreeRate * timeToExpiry);
	const callPayoffs = finalPrices.map((pr) => Math.max(pr - strikePrice, 0));
	const putPayoffs = finalPrices.map((pr) => Math.max(strikePrice - pr, 0));

	const expectedPayoffCall = discountFactor * (callPayoffs.reduce((s, v) => s + v, 0) / n);
	const expectedPayoffPut = discountFactor * (putPayoffs.reduce((s, v) => s + v, 0) / n);

	const bsResult = bsPrice(inputs);
	const aboveStrike = finalPrices.filter((pr) => pr > strikePrice).length;

	const computeTimeMs = performance.now() - startTime;

	return {
		paths: paths.slice(0, numPaths),
		timePoints,
		stats: {
			meanFinalPrice: mean,
			medianFinalPrice: percentile(0.5),
			stdDev,
			percentile5: percentile(0.05),
			percentile25: percentile(0.25),
			percentile75: percentile(0.75),
			percentile95: percentile(0.95),
			probabilityAboveStrike: aboveStrike / n,
			probabilityBelowStrike: 1 - aboveStrike / n,
			expectedPayoffCall,
			expectedPayoffPut,
			bsCallPrice: bsResult.callPrice,
			bsPutPrice: bsResult.putPrice
		},
		computeTimeMs
	};
}

/**
 * Lightweight Monte Carlo for real-time UI updates.
 */
export function runLightMonteCarlo(
	inputs: BSInputs,
	numPaths: number = 200,
	numSteps: number = 50
): MonteCarloResult {
	return runMonteCarlo(inputs, { numPaths, numSteps });
}

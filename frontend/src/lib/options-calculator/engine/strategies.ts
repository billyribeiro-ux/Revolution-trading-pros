// ============================================================
// MULTI-LEG STRATEGY ENGINE
// ============================================================

import { firstOrderGreeks } from './greeks.js';
import type { BSInputs, StrategyLeg, PayoffPoint } from './types.js';

/**
 * Calculate combined payoff at expiration for a set of strategy legs.
 */
export function calculateStrategyPayoff(legs: StrategyLeg[], priceRange: number[]): PayoffPoint[] {
	return priceRange.map((underlyingPrice) => {
		let totalPayoff = 0;
		let totalPremium = 0;

		for (const leg of legs) {
			const intrinsic =
				leg.type === 'call'
					? Math.max(underlyingPrice - leg.strike, 0)
					: Math.max(leg.strike - underlyingPrice, 0);

			totalPayoff += intrinsic * leg.position * leg.quantity;
			totalPremium += leg.premium * leg.position * leg.quantity;
		}

		const profit = totalPayoff + totalPremium;

		return {
			underlyingPrice,
			payoff: totalPayoff,
			profit
		};
	});
}

/**
 * Calculate combined Greeks for a multi-leg strategy.
 */
export function calculateStrategyGreeks(
	legs: StrategyLeg[],
	baseInputs: BSInputs
): { netDelta: number; netGamma: number; netTheta: number; netVega: number; netRho: number } {
	let netDelta = 0;
	let netGamma = 0;
	let netTheta = 0;
	let netVega = 0;
	let netRho = 0;

	for (const leg of legs) {
		const legInputs: BSInputs = {
			...baseInputs,
			strikePrice: leg.strike,
			timeToExpiry: leg.expiry
		};
		const greeks = firstOrderGreeks(legInputs, leg.type);

		netDelta += greeks.delta * leg.position * leg.quantity;
		netGamma += greeks.gamma * leg.position * leg.quantity;
		netTheta += greeks.theta * leg.position * leg.quantity;
		netVega += greeks.vega * leg.position * leg.quantity;
		netRho += greeks.rho * leg.position * leg.quantity;
	}

	return { netDelta, netGamma, netTheta, netVega, netRho };
}

/**
 * Find breakeven points for a strategy by scanning the payoff curve.
 */
export function findBreakevens(payoffPoints: PayoffPoint[]): number[] {
	const breakevens: number[] = [];

	for (let i = 1; i < payoffPoints.length; i++) {
		const prev = payoffPoints[i - 1];
		const curr = payoffPoints[i];

		if ((prev.profit >= 0 && curr.profit < 0) || (prev.profit < 0 && curr.profit >= 0)) {
			const ratio = Math.abs(prev.profit) / (Math.abs(prev.profit) + Math.abs(curr.profit));
			const be = prev.underlyingPrice + ratio * (curr.underlyingPrice - prev.underlyingPrice);
			breakevens.push(Math.round(be * 100) / 100);
		}
	}

	return breakevens;
}

/**
 * Calculate max profit and max loss from payoff curve.
 */
export function findMaxProfitLoss(payoffPoints: PayoffPoint[]): {
	maxProfit: number | 'unlimited';
	maxLoss: number | 'unlimited';
} {
	const profits = payoffPoints.map((p) => p.profit);
	const maxProfit = Math.max(...profits);
	const maxLoss = Math.min(...profits);

	const lastProfit = profits[profits.length - 1];
	const secondLastProfit = profits[profits.length - 2];
	const firstProfit = profits[0];
	const secondProfit = profits[1];

	const isUnlimitedUpside = lastProfit > secondLastProfit && lastProfit === maxProfit;
	const isUnlimitedDownside = firstProfit < secondProfit && firstProfit === maxLoss;

	return {
		maxProfit: isUnlimitedUpside ? 'unlimited' : maxProfit,
		maxLoss: isUnlimitedDownside ? 'unlimited' : maxLoss
	};
}

/**
 * Generate price range for payoff diagram.
 */
export function generatePriceRange(
	spotPrice: number,
	strikes: number[],
	points: number = 200
): number[] {
	const allPrices = [spotPrice, ...strikes];
	const minPrice = Math.min(...allPrices);
	const maxPrice = Math.max(...allPrices);
	const range = maxPrice - minPrice;
	const padding = Math.max(range * 0.5, spotPrice * 0.3);

	const low = Math.max(0.01, minPrice - padding);
	const high = maxPrice + padding;
	const step = (high - low) / points;

	return Array.from({ length: points + 1 }, (_, i) => low + i * step);
}

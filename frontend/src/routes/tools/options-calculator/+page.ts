import type { PageLoad } from './$types';

export const load: PageLoad = async () => {
	return {
		meta: {
			title: 'Black-Scholes Options Calculator | Revolution Trading Pros',
			description:
				'The most advanced free Black-Scholes options calculator. Real-time Greeks (Delta, Gamma, Theta, Vega, Rho + second-order), interactive payoff diagrams, 3D P&L surface, Monte Carlo simulation, multi-leg strategy builder, volatility smile analysis, and live market data integration. Built for professional traders.',
			keywords:
				'black scholes calculator, options calculator, greeks calculator, delta gamma theta vega, options pricing, implied volatility, options strategy builder, monte carlo simulation, options payoff diagram',
		},
	};
};

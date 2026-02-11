// ============================================================
// CONSTANTS & DEFAULT VALUES
// ============================================================

import type { BSInputs, InputFieldConfig, StrategyTemplate } from './types.js';

/** Mathematical constants */
export const MATH = {
	/** 1 / sqrt(2π) — used in normal PDF */
	INV_SQRT_2PI: 0.3989422804014327,
	/** sqrt(2π) */
	SQRT_2PI: 2.5066282746310002,
	/** Number of trading days per year */
	TRADING_DAYS_PER_YEAR: 252,
	/** Calendar days per year */
	CALENDAR_DAYS_PER_YEAR: 365,
	/** Convergence tolerance for Newton-Raphson */
	NR_TOLERANCE: 1e-8,
	/** Max iterations for Newton-Raphson */
	NR_MAX_ITERATIONS: 100,
	/** Very small number to avoid division by zero */
	EPSILON: 1e-10
} as const;

/** Default BS input values */
export const DEFAULT_INPUTS: BSInputs = {
	spotPrice: 100,
	strikePrice: 100,
	volatility: 0.25,
	timeToExpiry: 0.25,
	riskFreeRate: 0.05,
	dividendYield: 0.0
};

/** Input field configurations */
export const INPUT_FIELDS: InputFieldConfig[] = [
	{
		key: 'spotPrice',
		label: 'Spot Price',
		min: 0.01,
		max: 10000,
		step: 0.5,
		unit: '$',
		tooltip: 'Current market price of the underlying asset (S)',
		icon: 'trending-up'
	},
	{
		key: 'strikePrice',
		label: 'Strike Price',
		min: 0.01,
		max: 10000,
		step: 0.5,
		unit: '$',
		tooltip: 'The price at which the option can be exercised (K)',
		icon: 'target'
	},
	{
		key: 'volatility',
		label: 'Implied Volatility',
		min: 0.01,
		max: 3.0,
		step: 0.005,
		unit: '%',
		tooltip: 'Annualized implied volatility (σ). 0.25 = 25%. Measures expected price fluctuation.',
		icon: 'activity',
		displayMultiplier: 100,
		displayDecimals: 1
	},
	{
		key: 'timeToExpiry',
		label: 'Time to Expiry',
		min: 0.001,
		max: 5.0,
		step: 0.01,
		unit: 'years',
		tooltip: 'Time until option expiration in years (T). 0.25 = ~3 months, 0.0833 = ~1 month.',
		icon: 'clock',
		displayDecimals: 3
	},
	{
		key: 'riskFreeRate',
		label: 'Risk-Free Rate',
		min: 0.0,
		max: 0.2,
		step: 0.0025,
		unit: '%',
		tooltip:
			'Annual risk-free interest rate (r). Typically the Treasury yield matching expiration.',
		icon: 'landmark',
		displayMultiplier: 100,
		displayDecimals: 2
	},
	{
		key: 'dividendYield',
		label: 'Dividend Yield',
		min: 0.0,
		max: 0.15,
		step: 0.001,
		unit: '%',
		tooltip: 'Annual continuous dividend yield (q). Set to 0 for non-dividend stocks.',
		icon: 'coins',
		displayMultiplier: 100,
		displayDecimals: 2
	}
];

/** Strategy templates */
export const STRATEGY_TEMPLATES: StrategyTemplate[] = [
	{
		id: 'long-call',
		name: 'Long Call',
		description: 'Bullish bet with limited risk and unlimited upside',
		sentiment: 'bullish',
		riskProfile: 'defined',
		legs: [{ type: 'call', strike: 0, expiry: 0.25, position: 1, quantity: 1 }]
	},
	{
		id: 'long-put',
		name: 'Long Put',
		description: 'Bearish bet with limited risk and large downside profit potential',
		sentiment: 'bearish',
		riskProfile: 'defined',
		legs: [{ type: 'put', strike: 0, expiry: 0.25, position: 1, quantity: 1 }]
	},
	{
		id: 'covered-call',
		name: 'Covered Call',
		description: 'Own shares + sell call for income. Caps upside, reduces cost basis.',
		sentiment: 'neutral',
		riskProfile: 'undefined',
		legs: [{ type: 'call', strike: 5, expiry: 0.0833, position: -1, quantity: 1 }]
	},
	{
		id: 'bull-call-spread',
		name: 'Bull Call Spread',
		description: 'Buy lower strike call, sell higher strike call. Defined risk bullish play.',
		sentiment: 'bullish',
		riskProfile: 'defined',
		legs: [
			{ type: 'call', strike: 0, expiry: 0.25, position: 1, quantity: 1 },
			{ type: 'call', strike: 10, expiry: 0.25, position: -1, quantity: 1 }
		]
	},
	{
		id: 'bear-put-spread',
		name: 'Bear Put Spread',
		description: 'Buy higher strike put, sell lower strike put. Defined risk bearish play.',
		sentiment: 'bearish',
		riskProfile: 'defined',
		legs: [
			{ type: 'put', strike: 0, expiry: 0.25, position: 1, quantity: 1 },
			{ type: 'put', strike: -10, expiry: 0.25, position: -1, quantity: 1 }
		]
	},
	{
		id: 'iron-condor',
		name: 'Iron Condor',
		description: 'Sell OTM call spread + OTM put spread. Profit from low volatility / range.',
		sentiment: 'neutral',
		riskProfile: 'defined',
		legs: [
			{ type: 'put', strike: -15, expiry: 0.25, position: -1, quantity: 1 },
			{ type: 'put', strike: -20, expiry: 0.25, position: 1, quantity: 1 },
			{ type: 'call', strike: 15, expiry: 0.25, position: -1, quantity: 1 },
			{ type: 'call', strike: 20, expiry: 0.25, position: 1, quantity: 1 }
		]
	},
	{
		id: 'straddle',
		name: 'Long Straddle',
		description: 'Buy ATM call + ATM put. Profit from large move in either direction.',
		sentiment: 'neutral',
		riskProfile: 'defined',
		legs: [
			{ type: 'call', strike: 0, expiry: 0.25, position: 1, quantity: 1 },
			{ type: 'put', strike: 0, expiry: 0.25, position: 1, quantity: 1 }
		]
	},
	{
		id: 'strangle',
		name: 'Long Strangle',
		description: 'Buy OTM call + OTM put. Cheaper than straddle, needs bigger move.',
		sentiment: 'neutral',
		riskProfile: 'defined',
		legs: [
			{ type: 'call', strike: 10, expiry: 0.25, position: 1, quantity: 1 },
			{ type: 'put', strike: -10, expiry: 0.25, position: 1, quantity: 1 }
		]
	},
	{
		id: 'butterfly',
		name: 'Long Butterfly',
		description: 'Buy 1 lower, sell 2 middle, buy 1 upper. Max profit at middle strike.',
		sentiment: 'neutral',
		riskProfile: 'defined',
		legs: [
			{ type: 'call', strike: -10, expiry: 0.25, position: 1, quantity: 1 },
			{ type: 'call', strike: 0, expiry: 0.25, position: -1, quantity: 2 },
			{ type: 'call', strike: 10, expiry: 0.25, position: 1, quantity: 1 }
		]
	},
	{
		id: 'jade-lizard',
		name: 'Jade Lizard',
		description: 'Sell OTM put + sell call spread. No upside risk if premium > spread width.',
		sentiment: 'bullish',
		riskProfile: 'defined',
		legs: [
			{ type: 'put', strike: -10, expiry: 0.25, position: -1, quantity: 1 },
			{ type: 'call', strike: 10, expiry: 0.25, position: -1, quantity: 1 },
			{ type: 'call', strike: 15, expiry: 0.25, position: 1, quantity: 1 }
		]
	},
	{
		id: 'calendar-spread',
		name: 'Calendar Spread',
		description:
			'Sell near-term, buy longer-term at same strike. Profit from time decay differential.',
		sentiment: 'neutral',
		riskProfile: 'defined',
		legs: [
			{ type: 'call', strike: 0, expiry: 0.0833, position: -1, quantity: 1 },
			{ type: 'call', strike: 0, expiry: 0.25, position: 1, quantity: 1 }
		]
	},
	{
		id: 'ratio-spread',
		name: 'Ratio Call Spread',
		description: 'Buy 1 call, sell 2 higher calls. Can be done for credit. Undefined upside risk.',
		sentiment: 'bullish',
		riskProfile: 'undefined',
		legs: [
			{ type: 'call', strike: 0, expiry: 0.25, position: 1, quantity: 1 },
			{ type: 'call', strike: 10, expiry: 0.25, position: -1, quantity: 2 }
		]
	}
];

// ============================================================
// SCENARIO / WHAT-IF ENGINE
// ============================================================

import { price } from './black-scholes.js';
import { allGreeks } from './greeks.js';
import { calculateProbabilities } from './probability.js';
import type {
	BSInputs,
	OptionType,
	Scenario,
	ScenarioResult,
	TimeMachineSnapshot,
	SensitivityData
} from './types.js';

/** Pre-defined scenario templates */
export const SCENARIO_PRESETS: Scenario[] = [
	{
		id: 'gap-up-5',
		name: 'Gap Up 5%',
		description: 'Stock gaps up 5% overnight',
		adjustments: { spotPriceChangePct: 5 },
		color: '#00d4aa'
	},
	{
		id: 'gap-up-10',
		name: 'Gap Up 10%',
		description: 'Stock gaps up 10% (strong earnings beat)',
		adjustments: { spotPriceChangePct: 10 },
		color: '#00b894'
	},
	{
		id: 'gap-down-5',
		name: 'Gap Down 5%',
		description: 'Stock gaps down 5% overnight',
		adjustments: { spotPriceChangePct: -5 },
		color: '#ff4477'
	},
	{
		id: 'gap-down-10',
		name: 'Gap Down 10%',
		description: 'Stock gaps down 10% (earnings miss)',
		adjustments: { spotPriceChangePct: -10 },
		color: '#e74c3c'
	},
	{
		id: 'iv-crush-light',
		name: 'IV Crush (Light)',
		description: 'Implied volatility drops 15% after earnings',
		adjustments: { volatilityChange: -0.15 },
		color: '#f39c12'
	},
	{
		id: 'iv-crush-heavy',
		name: 'IV Crush (Heavy)',
		description: 'Implied volatility drops 35% after earnings',
		adjustments: { volatilityChange: -0.35 },
		color: '#e67e22'
	},
	{
		id: 'iv-expansion',
		name: 'IV Expansion',
		description: 'Implied volatility increases 20% (fear event)',
		adjustments: { volatilityChange: 0.2 },
		color: '#9b59b6'
	},
	{
		id: 'one-week-passes',
		name: '1 Week Passes',
		description: 'Time decay: 7 calendar days elapse',
		adjustments: { timeChange: -7 },
		color: '#3498db'
	},
	{
		id: 'one-month-passes',
		name: '1 Month Passes',
		description: 'Time decay: 30 calendar days elapse',
		adjustments: { timeChange: -30 },
		color: '#2980b9'
	},
	{
		id: 'rate-hike',
		name: 'Rate Hike 25bp',
		description: 'Fed raises rates by 25 basis points',
		adjustments: { rateChange: 0.0025 },
		color: '#1abc9c'
	},
	{
		id: 'earnings-bull',
		name: 'Earnings: Bull Case',
		description: 'Stock +8%, IV crush -30%',
		adjustments: { spotPriceChangePct: 8, volatilityChange: -0.3 },
		color: '#27ae60'
	},
	{
		id: 'earnings-bear',
		name: 'Earnings: Bear Case',
		description: 'Stock -8%, IV crush -25%',
		adjustments: { spotPriceChangePct: -8, volatilityChange: -0.25 },
		color: '#c0392b'
	},
	{
		id: 'black-swan',
		name: 'Black Swan',
		description: 'Stock -20%, IV +50%',
		adjustments: { spotPriceChangePct: -20, volatilityChange: 0.5 },
		color: '#2c3e50'
	},
	{
		id: 'melt-up',
		name: 'Melt Up',
		description: 'Stock +15%, IV -10%',
		adjustments: { spotPriceChangePct: 15, volatilityChange: -0.1 },
		color: '#00d4aa'
	}
];

/**
 * Apply a scenario's adjustments to base inputs.
 */
export function applyScenario(baseInputs: BSInputs, scenario: Scenario): BSInputs {
	const { adjustments } = scenario;
	let newSpot = baseInputs.spotPrice;
	let newVol = baseInputs.volatility;
	let newTime = baseInputs.timeToExpiry;
	let newRate = baseInputs.riskFreeRate;

	if (adjustments.spotPriceChangePct !== undefined) {
		newSpot = newSpot * (1 + adjustments.spotPriceChangePct / 100);
	}
	if (adjustments.spotPriceChange !== undefined) {
		newSpot = newSpot + adjustments.spotPriceChange;
	}
	if (adjustments.volatilityChange !== undefined) {
		newVol = Math.max(0.01, newVol * (1 + adjustments.volatilityChange));
	}
	if (adjustments.timeChange !== undefined) {
		newTime = Math.max(0.001, newTime + adjustments.timeChange / 365);
	}
	if (adjustments.rateChange !== undefined) {
		newRate = Math.max(0, newRate + adjustments.rateChange);
	}

	return {
		...baseInputs,
		spotPrice: Math.max(0.01, newSpot),
		volatility: newVol,
		timeToExpiry: newTime,
		riskFreeRate: newRate
	};
}

/**
 * Calculate scenario result compared to base case.
 */
export function calculateScenario(
	baseInputs: BSInputs,
	scenario: Scenario,
	type: OptionType
): ScenarioResult {
	const basePricing = price(baseInputs);
	const scenarioInputs = applyScenario(baseInputs, scenario);
	const scenarioPricing = price(scenarioInputs);
	const scenarioGreeks = allGreeks(scenarioInputs, type);

	return {
		scenario,
		inputs: scenarioInputs,
		callPrice: scenarioPricing.callPrice,
		putPrice: scenarioPricing.putPrice,
		callPriceChange: scenarioPricing.callPrice - basePricing.callPrice,
		putPriceChange: scenarioPricing.putPrice - basePricing.putPrice,
		callPriceChangePct:
			basePricing.callPrice > 0.001
				? ((scenarioPricing.callPrice - basePricing.callPrice) / basePricing.callPrice) * 100
				: 0,
		putPriceChangePct:
			basePricing.putPrice > 0.001
				? ((scenarioPricing.putPrice - basePricing.putPrice) / basePricing.putPrice) * 100
				: 0,
		greeks: scenarioGreeks
	};
}

/**
 * Calculate multiple scenarios at once.
 */
export function calculateScenarios(
	baseInputs: BSInputs,
	scenarios: Scenario[],
	type: OptionType
): ScenarioResult[] {
	return scenarios.map((scenario) => calculateScenario(baseInputs, scenario, type));
}

/**
 * Generate Time Machine snapshots from now to expiration.
 */
export function generateTimeMachineSnapshots(
	inputs: BSInputs,
	type: OptionType,
	numSnapshots: number = 50
): TimeMachineSnapshot[] {
	const totalDays = inputs.timeToExpiry * 365;
	const snapshots: TimeMachineSnapshot[] = [];

	for (let i = 0; i <= numSnapshots; i++) {
		const daysFromNow = (i / numSnapshots) * totalDays;
		const remainingTime = Math.max(0.001, inputs.timeToExpiry - daysFromNow / 365);

		const snapshotInputs: BSInputs = { ...inputs, timeToExpiry: remainingTime };
		const pricing = price(snapshotInputs);
		const greeks = allGreeks(snapshotInputs, type);
		const probs = calculateProbabilities(snapshotInputs, type);

		snapshots.push({
			daysFromNow,
			timeToExpiry: remainingTime,
			callPrice: pricing.callPrice,
			putPrice: pricing.putPrice,
			greeks,
			probabilities: probs
		});
	}

	return snapshots;
}

/**
 * Generate sensitivity data — how option price changes as one parameter varies.
 */
export function generateSensitivityData(inputs: BSInputs): SensitivityData[] {
	const parameters: {
		key: keyof BSInputs;
		label: string;
		rangePct: number;
		steps: number;
	}[] = [
		{ key: 'spotPrice', label: 'Spot Price', rangePct: 0.2, steps: 40 },
		{ key: 'volatility', label: 'Volatility', rangePct: 0.5, steps: 40 },
		{ key: 'timeToExpiry', label: 'Time to Expiry', rangePct: 0.8, steps: 40 },
		{ key: 'riskFreeRate', label: 'Interest Rate', rangePct: 1.0, steps: 40 },
		{ key: 'dividendYield', label: 'Dividend Yield', rangePct: 1.0, steps: 40 }
	];

	return parameters.map(({ key, label, rangePct, steps }) => {
		const baseValue = inputs[key];
		const low = Math.max(0.001, baseValue * (1 - rangePct));
		const high = baseValue * (1 + rangePct);
		const stepSize = (high - low) / steps;

		const range: number[] = [];
		const callPrices: number[] = [];
		const putPrices: number[] = [];

		for (let i = 0; i <= steps; i++) {
			const paramValue = low + i * stepSize;
			range.push(paramValue);
			const testInputs: BSInputs = { ...inputs, [key]: paramValue };
			const pricing = price(testInputs);
			callPrices.push(pricing.callPrice);
			putPrices.push(pricing.putPrice);
		}

		const basePricing = price(inputs);
		const bumpedInputs: BSInputs = { ...inputs, [key]: baseValue * 1.01 };
		const bumpedPricing = price(bumpedInputs);

		const callElasticity =
			basePricing.callPrice > 0.001
				? (bumpedPricing.callPrice - basePricing.callPrice) / basePricing.callPrice / 0.01
				: 0;

		return {
			parameter: key,
			label,
			baseValue,
			range,
			callPrices,
			putPrices,
			sensitivity: Math.abs(callElasticity)
		};
	});
}

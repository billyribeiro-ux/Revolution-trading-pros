/**
 * Strategy legs and shared setups on `CalculatorState`: editing a leg keeps its identity,
 * legs are priced at the current inputs (so the payoff, net premium and net delta are the
 * strategy's), and a shared setup restores everything it carries.
 */

import { describe, expect, it } from 'vitest';
import { CalculatorState } from '../calculator.svelte.js';
import { priceOption } from '../../engine/black-scholes.js';
import { decodeState, encodeState } from '../../utils/share-utils.js';
import type { StrategyLeg } from '../../engine/types.js';

const LONG_CALL: Omit<StrategyLeg, 'id'> = {
	type: 'call',
	strike: 100,
	expiry: 0.25,
	position: 1,
	quantity: 1,
	premium: 0
};

function strategyCalculator(...legs: Omit<StrategyLeg, 'id'>[]): CalculatorState {
	const calc = new CalculatorState();
	calc.calculatorMode = 'strategy';
	for (const leg of legs) calc.addStrategyLeg(leg);
	return calc;
}

describe('updateStrategyLeg', () => {
	it('replaces the leg in place, keeping its id and position in the list', () => {
		const calc = strategyCalculator(LONG_CALL, { ...LONG_CALL, strike: 110 });
		const [first, second] = calc.strategyLegs;

		calc.updateStrategyLeg({ ...first, strike: 450 });

		expect(calc.strategyLegs.map((leg) => leg.id)).toEqual([first.id, second.id]);
		expect(calc.strategyLegs[0].strike).toBe(450);
		expect(calc.strategyLegs[1]).toBe(second);
	});

	it('ignores a leg that is no longer in the strategy', () => {
		const calc = strategyCalculator(LONG_CALL);
		const before = calc.strategyLegs;

		calc.updateStrategyLeg({ ...before[0], id: 'gone', strike: 1 });

		expect(calc.strategyLegs).toEqual(before);
	});
});

describe('strategy pricing', () => {
	it('prices each leg at the current inputs, whatever premium it carries', () => {
		const calc = strategyCalculator({ ...LONG_CALL, premium: 123 });
		const expected = priceOption(
			{ ...calc.inputs, strikePrice: LONG_CALL.strike, timeToExpiry: LONG_CALL.expiry },
			'call'
		);

		expect(calc.pricedStrategyLegs[0].premium).toBeCloseTo(-expected, 10);
		expect(calc.strategyNetPremium).toBeCloseTo(-expected, 10);
	});

	it('nets premiums and delta across positions and quantities', () => {
		const calc = strategyCalculator(LONG_CALL, { ...LONG_CALL, strike: 110, position: -1 });
		const [long, short] = calc.pricedStrategyLegs;

		// A bull call spread is a debit: the long leg costs more than the short leg brings in.
		expect(calc.strategyNetPremium).toBeCloseTo(long.premium - short.premium, 10);
		expect(calc.strategyNetPremium).toBeLessThan(0);
		expect(calc.strategyGreeks.netDelta).toBeGreaterThan(0);
		expect(calc.strategyGreeks.netDelta).toBeLessThan(calc.callGreeks.first.delta);
	});

	it('includes the premium in the strategy payoff, so a long call can lose', () => {
		const calc = strategyCalculator(LONG_CALL);

		expect(calc.strategyMaxProfitLoss.maxLoss).toBeCloseTo(calc.strategyNetPremium, 6);
		expect(calc.strategyBreakevens).toHaveLength(1);
		expect(calc.strategyBreakevens[0]).toBeGreaterThan(LONG_CALL.strike);
	});

	it('quantity scales the net premium', () => {
		const calc = strategyCalculator(LONG_CALL);
		const single = calc.strategyNetPremium;

		calc.updateStrategyLeg({ ...calc.strategyLegs[0], quantity: 3 });

		expect(calc.strategyNetPremium).toBeCloseTo(single * 3, 10);
	});
});

describe('applyShareableState', () => {
	it('round-trips a share link', () => {
		const source = strategyCalculator(LONG_CALL, { ...LONG_CALL, type: 'put', position: -1 });
		source.spotPrice = 123.5;
		source.volatility = 0.42;
		source.optionType = 'put';
		source.activeTab = 'heatmap';
		source.activeTicker = 'SPY';

		const query = encodeState({
			inputs: source.inputs,
			optionType: source.optionType,
			mode: source.calculatorMode,
			activeTab: source.activeTab,
			ticker: source.activeTicker,
			strategyLegs: source.strategyLegs
		});
		const target = new CalculatorState();
		target.applyShareableState(decodeState(new URLSearchParams(query)));

		expect(target.inputs).toEqual(source.inputs);
		expect(target.optionType).toBe('put');
		expect(target.calculatorMode).toBe('strategy');
		expect(target.activeTab).toBe('heatmap');
		expect(target.activeTicker).toBe('SPY');
		expect(target.strategyLegs.map(({ id: _id, ...leg }) => leg)).toEqual(
			source.strategyLegs.map(({ id: _id, ...leg }) => ({ ...leg, premium: 0 }))
		);
		expect(new Set(target.strategyLegs.map((leg) => leg.id)).size).toBe(2);
	});

	it('leaves everything the shared state does not carry untouched', () => {
		const calc = strategyCalculator(LONG_CALL);
		const legs = calc.strategyLegs;
		const inputs = calc.inputs;

		calc.applyShareableState({});

		expect(calc.strategyLegs).toBe(legs);
		expect(calc.inputs).toEqual(inputs);
		expect(calc.calculatorMode).toBe('strategy');
	});
});

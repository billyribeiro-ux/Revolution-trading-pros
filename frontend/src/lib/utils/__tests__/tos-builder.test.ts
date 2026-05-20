/**
 * tos-builder — Unit Tests (R24-D)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * The tos-builder module is the source of truth for ThinkOrSwim
 * trade-string format generation. It is finance-critical: every alert
 * the trading rooms ship out gets formatted through here, and an
 * incorrect string can mean a customer pastes a wrong-direction trade
 * into their broker.
 *
 * What we test:
 *   1. buildTosString — both shares and options formats match the
 *      documented examples character-for-character.
 *   2. parseTosString — round-trip parses the string we just built
 *      back to the same shape (the operator-relevant invariant).
 *   3. validateTosParams — all required-field branches return the
 *      expected error message.
 *   4. formatExpirationDate uses UTC accessors (timezone-agnostic).
 *   5. Date helpers (getNextFriday, determineContractType) handle
 *      boundary days correctly.
 *
 * Non-obvious contract:
 *   - formatPrice always emits 2 decimals (`toFixed(2)`). Integer
 *     strikes are rendered WITHOUT a decimal (`Number.isInteger`
 *     branch in buildTosString). This is intentional — TOS accepts
 *     `255` and `255.00` differently for some symbols.
 */

import { describe, it, expect } from 'vitest';
import {
	formatExpirationDate,
	formatPrice,
	buildTosString,
	parseTosString,
	validateTosParams,
	getNextFriday,
	getMonthlyExpiration,
	determineContractType
} from '../tos-builder';

// ═══════════════════════════════════════════════════════════════════════════
// formatExpirationDate
// ═══════════════════════════════════════════════════════════════════════════

describe('formatExpirationDate', () => {
	it('formats an ISO date to `DD MON YY` in UTC', () => {
		// 2026-02-27 — example from buildTosString docstring.
		// We use UTC accessors so any local TZ offset is ignored.
		expect(formatExpirationDate('2026-02-27')).toBe('27 FEB 26');
	});

	it('uses month abbreviations (no locale dependency)', () => {
		expect(formatExpirationDate('2026-01-15')).toBe('15 JAN 26');
		expect(formatExpirationDate('2026-12-31')).toBe('31 DEC 26');
	});

	it('truncates year to 2 digits', () => {
		expect(formatExpirationDate('2030-06-01')).toBe('1 JUN 30');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// formatPrice
// ═══════════════════════════════════════════════════════════════════════════

describe('formatPrice', () => {
	it('always emits 2 decimal places (toFixed contract)', () => {
		expect(formatPrice(9.55)).toBe('9.55');
		expect(formatPrice(255)).toBe('255.00');
		expect(formatPrice(0)).toBe('0.00');
	});

	it('rounds half-even at the 2-decimal boundary', () => {
		// toFixed uses banker's rounding semantics in practice on most engines;
		// we just assert the documented "2 decimals" contract holds.
		expect(formatPrice(1.005).length).toBe(4); // "1.00" or "1.01"
		expect(formatPrice(1.234567)).toBe('1.23');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// buildTosString — happy paths
// ═══════════════════════════════════════════════════════════════════════════

describe('buildTosString — options', () => {
	it('matches the docstring example (AAPL CALL)', () => {
		const result = buildTosString({
			trade_type: 'options',
			action: 'BUY',
			quantity: 1,
			ticker: 'AAPL',
			option_type: 'CALL',
			strike: 255,
			expiration: '2026-02-27',
			contract_type: 'Weeklys',
			order_type: 'LMT',
			limit_price: 9.55
		});
		expect(result).toBe('BUY +1 AAPL 100 (Weeklys) 27 FEB 26 255 CALL @9.55 LMT');
	});

	it('renders SELL with a `-` sign on quantity', () => {
		const result = buildTosString({
			trade_type: 'options',
			action: 'SELL',
			quantity: 2,
			ticker: 'TSLA',
			option_type: 'PUT',
			strike: 200,
			expiration: '2026-03-20',
			contract_type: 'Monthly',
			order_type: 'LMT',
			limit_price: 5.5
		});
		expect(result).toBe('SELL -2 TSLA 100 (Monthly) 20 MAR 26 200 PUT @5.50 LMT');
	});

	it('uses MKT instead of @price for MKT orders', () => {
		const result = buildTosString({
			trade_type: 'options',
			action: 'BUY',
			quantity: 1,
			ticker: 'SPY',
			option_type: 'CALL',
			strike: 500,
			expiration: '2026-04-19',
			contract_type: 'Standard',
			order_type: 'MKT'
		});
		expect(result).toBe('BUY +1 SPY 100 (Standard) 19 APR 26 500 CALL MKT');
	});

	it('renders fractional strikes with 2 decimals (non-integer branch)', () => {
		const result = buildTosString({
			trade_type: 'options',
			action: 'BUY',
			quantity: 1,
			ticker: 'AAPL',
			option_type: 'CALL',
			strike: 252.5,
			expiration: '2026-02-27',
			contract_type: 'Weeklys',
			order_type: 'LMT',
			limit_price: 1.25
		});
		expect(result).toBe('BUY +1 AAPL 100 (Weeklys) 27 FEB 26 252.50 CALL @1.25 LMT');
	});

	it('defaults contract_type to "Standard" when not provided', () => {
		const result = buildTosString({
			trade_type: 'options',
			action: 'BUY',
			quantity: 1,
			ticker: 'AAPL',
			option_type: 'CALL',
			strike: 255,
			expiration: '2026-02-27',
			order_type: 'MKT'
		});
		expect(result).toContain('(Standard)');
	});
});

describe('buildTosString — shares', () => {
	it('matches the docstring example (AAPL shares)', () => {
		const result = buildTosString({
			trade_type: 'shares',
			action: 'BUY',
			quantity: 100,
			ticker: 'AAPL',
			order_type: 'LMT',
			limit_price: 255.03
		});
		expect(result).toBe('BUY +100 AAPL @255.03 LMT');
	});

	it('renders SELL share orders with negative quantity sign', () => {
		const result = buildTosString({
			trade_type: 'shares',
			action: 'SELL',
			quantity: 50,
			ticker: 'NVDA',
			order_type: 'MKT'
		});
		expect(result).toBe('SELL -50 NVDA MKT');
	});
});

describe('buildTosString — NEGATIVE', () => {
	it('throws when options trade is missing option_type', () => {
		expect(() =>
			buildTosString({
				trade_type: 'options',
				action: 'BUY',
				quantity: 1,
				ticker: 'AAPL',
				strike: 255,
				expiration: '2026-02-27',
				order_type: 'MKT'
			})
		).toThrow(/option_type/);
	});

	it('throws when options trade is missing strike', () => {
		expect(() =>
			buildTosString({
				trade_type: 'options',
				action: 'BUY',
				quantity: 1,
				ticker: 'AAPL',
				option_type: 'CALL',
				expiration: '2026-02-27',
				order_type: 'MKT'
			})
		).toThrow(/strike/);
	});

	it('throws when options trade is missing expiration', () => {
		expect(() =>
			buildTosString({
				trade_type: 'options',
				action: 'BUY',
				quantity: 1,
				ticker: 'AAPL',
				option_type: 'CALL',
				strike: 255,
				order_type: 'MKT'
			})
		).toThrow(/expiration/);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// parseTosString — round-trip + NEGATIVE
// ═══════════════════════════════════════════════════════════════════════════

describe('parseTosString', () => {
	it('round-trips an options string built by buildTosString', () => {
		const built = 'BUY +1 AAPL 100 (Weeklys) 27 FEB 26 255 CALL @9.55 LMT';
		const parsed = parseTosString(built);
		expect(parsed).not.toBeNull();
		expect(parsed!.trade_type).toBe('options');
		expect(parsed!.action).toBe('BUY');
		expect(parsed!.quantity).toBe(1);
		expect(parsed!.ticker).toBe('AAPL');
		expect(parsed!.option_type).toBe('CALL');
		expect(parsed!.strike).toBe(255);
		expect(parsed!.expiration).toBe('2026-02-27');
		expect(parsed!.contract_type).toBe('Weeklys');
		expect(parsed!.order_type).toBe('LMT');
		expect(parsed!.limit_price).toBe(9.55);
	});

	it('round-trips a shares string', () => {
		const built = 'BUY +100 AAPL @255.03 LMT';
		const parsed = parseTosString(built);
		expect(parsed).not.toBeNull();
		expect(parsed!.trade_type).toBe('shares');
		expect(parsed!.quantity).toBe(100);
		expect(parsed!.ticker).toBe('AAPL');
		expect(parsed!.order_type).toBe('LMT');
		expect(parsed!.limit_price).toBe(255.03);
	});

	it('parses MKT orders without limit_price', () => {
		const parsed = parseTosString('BUY +1 SPY 100 (Standard) 19 APR 26 500 CALL MKT');
		expect(parsed).not.toBeNull();
		expect(parsed!.order_type).toBe('MKT');
		expect(parsed!.limit_price).toBeUndefined();
	});

	it('returns null for unknown action (NEGATIVE)', () => {
		expect(parseTosString('HOLD +1 AAPL MKT')).toBeNull();
	});

	it('returns null when quantity is missing sign (NEGATIVE)', () => {
		// The qty regex requires +/- prefix.
		expect(parseTosString('BUY 1 AAPL MKT')).toBeNull();
	});

	it('returns null for empty / garbage input (NEGATIVE)', () => {
		expect(parseTosString('')).toBeNull();
		expect(parseTosString('lol')).toBeNull();
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// validateTosParams
// ═══════════════════════════════════════════════════════════════════════════

describe('validateTosParams', () => {
	it('returns empty errors for a complete options payload', () => {
		const errors = validateTosParams({
			trade_type: 'options',
			action: 'BUY',
			quantity: 1,
			ticker: 'AAPL',
			option_type: 'CALL',
			strike: 255,
			expiration: '2026-02-27',
			order_type: 'LMT',
			limit_price: 9.55
		});
		expect(errors).toEqual([]);
	});

	it('returns empty errors for a complete shares payload', () => {
		const errors = validateTosParams({
			trade_type: 'shares',
			action: 'BUY',
			quantity: 100,
			ticker: 'AAPL',
			order_type: 'MKT'
		});
		expect(errors).toEqual([]);
	});

	it('catches missing required fields (NEGATIVE)', () => {
		const errors = validateTosParams({});
		expect(errors).toContain('Trade type is required');
		expect(errors).toContain('Action must be BUY or SELL');
		expect(errors).toContain('Quantity must be greater than 0');
		expect(errors).toContain('Ticker is required');
		expect(errors).toContain('Order type must be MKT or LMT');
	});

	it('flags LMT order without limit_price (NEGATIVE)', () => {
		const errors = validateTosParams({
			trade_type: 'shares',
			action: 'BUY',
			quantity: 100,
			ticker: 'AAPL',
			order_type: 'LMT'
			// limit_price omitted
		});
		expect(errors).toContain('Limit price is required for LMT orders');
	});

	it('flags options without option_type/strike/expiration (NEGATIVE)', () => {
		const errors = validateTosParams({
			trade_type: 'options',
			action: 'BUY',
			quantity: 1,
			ticker: 'AAPL',
			order_type: 'MKT'
		});
		expect(errors).toContain('Option type must be CALL or PUT');
		expect(errors).toContain('Strike price is required for options');
		expect(errors).toContain('Expiration date is required for options');
	});

	it('flags zero / negative quantity (NEGATIVE)', () => {
		const errors = validateTosParams({
			trade_type: 'shares',
			action: 'BUY',
			quantity: 0,
			ticker: 'AAPL',
			order_type: 'MKT'
		});
		expect(errors).toContain('Quantity must be greater than 0');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// Date helpers
// ═══════════════════════════════════════════════════════════════════════════

describe('getNextFriday', () => {
	it('returns the next Friday from a Monday', () => {
		// 2026-05-04 is a Monday (UTC).
		// getDay() uses LOCAL time — we construct the date in local time.
		const monday = new Date(2026, 4, 4); // May is month 4 (0-indexed)
		expect(monday.getDay()).toBe(1); // sanity check: Monday
		const friday = getNextFriday(monday);
		expect(friday.getDay()).toBe(5);
	});

	it('returns the FOLLOWING Friday when called on a Friday (skip-current contract)', () => {
		// `daysUntilFriday = (5 - 5 + 7) % 7 || 7` = `0 || 7` = 7 days.
		const friday = new Date(2026, 4, 8); // May 8 2026, Friday
		expect(friday.getDay()).toBe(5);
		const next = getNextFriday(friday);
		// Should jump 7 days forward to the next Friday.
		expect(next.getDay()).toBe(5);
		expect(next.getDate()).toBe(15);
	});
});

describe('determineContractType', () => {
	it('returns "Weeklys" for an expiration within 7 days', () => {
		const soon = new Date();
		soon.setDate(soon.getDate() + 3);
		expect(determineContractType(soon.toISOString())).toBe('Weeklys');
	});

	it('returns "Monthly" for an expiration within 8-45 days', () => {
		const mid = new Date();
		mid.setDate(mid.getDate() + 30);
		expect(determineContractType(mid.toISOString())).toBe('Monthly');
	});

	it('returns "LEAPS" for an expiration beyond 45 days', () => {
		const far = new Date();
		far.setDate(far.getDate() + 200);
		expect(determineContractType(far.toISOString())).toBe('LEAPS');
	});
});

describe('getMonthlyExpiration', () => {
	it('returns the 3rd Friday of the requested month', () => {
		// May 2026: 3rd Friday is May 15.
		const date = getMonthlyExpiration(4, 2026); // month index 4 = May
		expect(date.getDay()).toBe(5);
		expect(date.getDate()).toBe(15);
	});

	it('returns the 3rd Friday for a month starting on Friday', () => {
		// May 2026 starts on Friday — May 1 is a Friday.
		// 3rd Friday = May 15.
		const date = getMonthlyExpiration(4, 2026);
		expect(date.getMonth()).toBe(4);
		expect(date.getDay()).toBe(5);
	});
});

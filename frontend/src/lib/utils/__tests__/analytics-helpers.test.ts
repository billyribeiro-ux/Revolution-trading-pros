/**
 * analytics-helpers — Unit Tests (R26-D)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * `src/lib/utils/analytics-helpers.ts` is the formatting + math layer for
 * the admin Analytics surface. The functions are mostly pure but every
 * "small" formatter pins a user-visible contract:
 *
 *   - formatNumber: thresholds K=1e3 / M=1e6 / B=1e9, exact-boundary
 *     behaviour locked because the admin dashboard renders thousands of
 *     these and a "999 -> 1K" jitter at the boundary is visually loud.
 *   - formatCurrency: en-US locale, USD by default, NO forced decimals on
 *     whole-dollar amounts (the dashboard renders "$1,234" not "$1,234.00").
 *   - formatDuration: cascading s/m/h/d cutoffs at 60/3600/86400.
 *   - formatRelativeTime: "just now" / "Nm ago" / "Nh ago" / "Nd ago" /
 *     "Nw ago" / "Nmo ago" / "Ny ago" — using a frozen `now`.
 *   - calculateGrowthRate / calculateCompoundGrowthRate: division-by-zero
 *     branches MUST not return Infinity (they must return 0 or 100).
 *   - getTrendColor / getChangeColor: branching on `isPositive` flips
 *     green/red — invariant the dashboard uses to colour "churn" vs
 *     "revenue" correctly.
 *   - isValidEventName / sanitizeEventProperties: PII / cardinality
 *     defenders — accepting an empty key or a 100-char nonsense name
 *     would silently widen the analytics warehouse schema.
 */

import { describe, expect, it, beforeEach, afterEach, vi } from 'vitest';
import {
	formatNumber,
	formatCurrency,
	formatPercentage,
	formatDuration,
	formatRelativeTime,
	PERIOD_OPTIONS,
	getPeriodDates,
	getTrendColor,
	getChangeColor,
	getPercentileColor,
	generateChartColors,
	interpolateColor,
	calculateGrowthRate,
	calculateCompoundGrowthRate,
	smoothData,
	detectAnomalies,
	isValidEventName,
	sanitizeEventProperties
} from '../analytics-helpers';

// ═══════════════════════════════════════════════════════════════════════════
// formatNumber — thresholds and exact boundaries
// ═══════════════════════════════════════════════════════════════════════════

describe('formatNumber', () => {
	it('returns plain integer string under 1000', () => {
		// CONTRACT: 999 should NOT flip to "1K" — exact-threshold guard.
		expect(formatNumber(999)).toBe('999');
		expect(formatNumber(0)).toBe('0');
	});

	it('switches to K at the 1000 boundary', () => {
		expect(formatNumber(1000)).toBe('1K');
		expect(formatNumber(2500, 1)).toBe('2.5K');
	});

	it('switches to M at the 1_000_000 boundary', () => {
		expect(formatNumber(1_000_000)).toBe('1M');
		expect(formatNumber(2_500_000, 1)).toBe('2.5M');
	});

	it('switches to B at the 1_000_000_000 boundary', () => {
		expect(formatNumber(1_000_000_000)).toBe('1B');
		expect(formatNumber(7_400_000_000, 2)).toBe('7.40B');
	});

	it('respects the decimals argument under 1000 (no K/M/B suffix yet)', () => {
		// The implementation calls `.toFixed(decimals)` in every branch
		// including the plain branch, so `formatNumber(42, 2)` -> "42.00".
		expect(formatNumber(42, 2)).toBe('42.00');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// formatCurrency / formatPercentage
// ═══════════════════════════════════════════════════════════════════════════

describe('formatCurrency', () => {
	it('formats whole dollars with NO trailing decimals (dashboard convention)', () => {
		expect(formatCurrency(1234)).toBe('$1,234');
	});

	it('uses USD by default; honours the currency override', () => {
		const eur = formatCurrency(99, 'EUR');
		// Intl uses €99 in en-US locale; the symbol is locale-dependent
		// (€ for EUR). Pin just the digits + currency-aware-ness:
		expect(eur).toContain('99');
		expect(eur).not.toContain('$'); // it's NOT USD
	});

	it('keeps up to 2 decimals when present', () => {
		expect(formatCurrency(12.34)).toBe('$12.34');
	});
});

describe('formatPercentage', () => {
	it('appends % and respects decimals', () => {
		expect(formatPercentage(12.345)).toBe('12.3%');
		expect(formatPercentage(12.345, 2)).toBe('12.35%');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// formatDuration — cascading thresholds at 60 / 3600 / 86400
// ═══════════════════════════════════════════════════════════════════════════

describe('formatDuration', () => {
	it('uses seconds under 60', () => {
		expect(formatDuration(0)).toBe('0s');
		expect(formatDuration(59)).toBe('59s');
	});

	it('switches to minutes at 60', () => {
		expect(formatDuration(60)).toBe('1m');
		expect(formatDuration(3599)).toBe('60m'); // 59.98m -> rounded 60
	});

	it('switches to hours at 3600', () => {
		expect(formatDuration(3600)).toBe('1h');
		expect(formatDuration(7200)).toBe('2h');
	});

	it('switches to days at 86400', () => {
		expect(formatDuration(86400)).toBe('1d');
		expect(formatDuration(172800)).toBe('2d');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// formatRelativeTime — freeze `now` for determinism
// ═══════════════════════════════════════════════════════════════════════════

describe('formatRelativeTime', () => {
	beforeEach(() => {
		vi.useFakeTimers();
		// Pin: 2026-05-20T12:00:00Z
		vi.setSystemTime(new Date('2026-05-20T12:00:00Z'));
	});

	afterEach(() => {
		vi.useRealTimers();
	});

	it('returns "just now" for <60s ago', () => {
		expect(formatRelativeTime(new Date('2026-05-20T11:59:30Z'))).toBe('just now');
	});

	it('returns Nm for <60m ago', () => {
		// 5 min ago
		expect(formatRelativeTime(new Date('2026-05-20T11:55:00Z'))).toBe('5m ago');
	});

	it('returns Nh for <24h ago', () => {
		// 3h ago
		expect(formatRelativeTime(new Date('2026-05-20T09:00:00Z'))).toBe('3h ago');
	});

	it('returns Nd for <7d ago', () => {
		// 2 days ago
		expect(formatRelativeTime(new Date('2026-05-18T12:00:00Z'))).toBe('2d ago');
	});

	it('returns Nw for <30d ago', () => {
		// 14 days ago = 2w ago (floor(14/7))
		expect(formatRelativeTime(new Date('2026-05-06T12:00:00Z'))).toBe('2w ago');
	});

	it('returns Nmo for <365d ago', () => {
		// 60 days ago = 2mo
		expect(formatRelativeTime(new Date('2026-03-21T12:00:00Z'))).toBe('2mo ago');
	});

	it('returns Ny for >=365d ago', () => {
		// 400 days ago = 1y
		const d = new Date('2026-05-20T12:00:00Z');
		d.setDate(d.getDate() - 400);
		expect(formatRelativeTime(d)).toBe('1y ago');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// PERIOD_OPTIONS / getPeriodDates
// ═══════════════════════════════════════════════════════════════════════════

describe('PERIOD_OPTIONS + getPeriodDates', () => {
	it('PERIOD_OPTIONS includes the standard set used by the dashboard', () => {
		const values = PERIOD_OPTIONS.map((p) => p.value);
		expect(values).toEqual(['1d', '7d', '14d', '30d', '60d', '90d', '180d', '365d']);
	});

	it('getPeriodDates(7d) sets start exactly 7 days before end', () => {
		const { start, end } = getPeriodDates('7d');
		const days = (end.getTime() - start.getTime()) / (1000 * 60 * 60 * 24);
		// setDate() preserves intra-day time exactly; expect ~7.0 days
		// (tolerate the 1ms drift from the two new Date() calls).
		expect(Math.round(days)).toBe(7);
	});

	it('falls back to 30 days for an unknown period code', () => {
		const { start, end } = getPeriodDates('garbage');
		const days = (end.getTime() - start.getTime()) / (1000 * 60 * 60 * 24);
		expect(Math.round(days)).toBe(30);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// Trend / Change color helpers — isPositive flips green/red
// ═══════════════════════════════════════════════════════════════════════════

describe('getTrendColor', () => {
	it('"stable" is always gray (isPositive irrelevant)', () => {
		expect(getTrendColor('stable')).toBe('text-gray-400');
		expect(getTrendColor('stable', false)).toBe('text-gray-400');
	});

	it('"up" + isPositive=true is green; "up" + isPositive=false is red', () => {
		// CONTRACT pin: this is the invariant the churn-vs-revenue chart
		// relies on. If revenue's `up`, it's green. If churn's `up`, it's
		// red (because growing churn is BAD).
		expect(getTrendColor('up', true)).toBe('text-green-400');
		expect(getTrendColor('up', false)).toBe('text-red-400');
	});

	it('"down" + isPositive=true is red; "down" + isPositive=false is green', () => {
		expect(getTrendColor('down', true)).toBe('text-red-400');
		expect(getTrendColor('down', false)).toBe('text-green-400');
	});
});

describe('getChangeColor', () => {
	it('returns gray when |change| < 0.1 (noise-floor band)', () => {
		expect(getChangeColor(0)).toBe('text-gray-400');
		expect(getChangeColor(0.05)).toBe('text-gray-400');
		expect(getChangeColor(-0.05)).toBe('text-gray-400');
	});

	it('positive change is green when isPositive=true', () => {
		expect(getChangeColor(5, true)).toBe('text-green-400');
	});

	it('positive change is red when isPositive=false (churn-style metric)', () => {
		expect(getChangeColor(5, false)).toBe('text-red-400');
	});
});

describe('getPercentileColor', () => {
	it('maps 90+ to green-500 (top-tier)', () => {
		expect(getPercentileColor(95)).toBe('text-green-500');
	});

	it('maps 75-89 to green-400', () => {
		expect(getPercentileColor(80)).toBe('text-green-400');
	});

	it('maps 50-74 to yellow-400', () => {
		expect(getPercentileColor(60)).toBe('text-yellow-400');
	});

	it('maps 25-49 to orange-400', () => {
		expect(getPercentileColor(30)).toBe('text-orange-400');
	});

	it('maps <25 to red-400', () => {
		expect(getPercentileColor(10)).toBe('text-red-400');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// generateChartColors / interpolateColor
// ═══════════════════════════════════════════════════════════════════════════

describe('generateChartColors', () => {
	it('returns the first N base colors when N <= 8', () => {
		const colors = generateChartColors(3);
		expect(colors).toHaveLength(3);
		expect(colors[0]).toBe('#fbbf24');
		expect(colors[1]).toBe('#60a5fa');
	});

	it('generates additional HSL colors when N > 8', () => {
		const colors = generateChartColors(12);
		expect(colors).toHaveLength(12);
		// First 8 are hex from the base palette
		expect(colors[0]).toMatch(/^#[0-9a-f]{6}$/i);
		// Last is an HSL-generated one
		expect(colors[11]).toMatch(/^hsl\(\d+(\.\d+)?, 70%, 60%\)$/);
	});
});

describe('interpolateColor', () => {
	it('returns the start color when factor=0', () => {
		// rgb(255,0,0) at factor 0 = the same as #ff0000 -> rgb(255, 0, 0)
		expect(interpolateColor('#ff0000', '#0000ff', 0)).toBe('rgb(255, 0, 0)');
	});

	it('returns the end color when factor=1', () => {
		expect(interpolateColor('#ff0000', '#0000ff', 1)).toBe('rgb(0, 0, 255)');
	});

	it('midpoint factor=0.5 produces a 50/50 mix', () => {
		// rgb(128, 0, 128) (within rounding)
		expect(interpolateColor('#ff0000', '#0000ff', 0.5)).toBe('rgb(128, 0, 128)');
	});

	it('falls back to color1 when either input is not a valid hex', () => {
		expect(interpolateColor('not-a-color', '#0000ff', 0.5)).toBe('not-a-color');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// calculateGrowthRate / calculateCompoundGrowthRate
// ═══════════════════════════════════════════════════════════════════════════

describe('calculateGrowthRate', () => {
	it('returns the standard ((current - previous) / previous) * 100', () => {
		expect(calculateGrowthRate(120, 100)).toBe(20);
		expect(calculateGrowthRate(50, 100)).toBe(-50);
	});

	it('returns 100 when previous=0 and current>0 (instead of Infinity)', () => {
		// CONTRACT: division-by-zero must NOT leak Infinity to the UI.
		expect(calculateGrowthRate(50, 0)).toBe(100);
	});

	it('returns 0 when both previous and current are 0', () => {
		expect(calculateGrowthRate(0, 0)).toBe(0);
	});
});

describe('calculateCompoundGrowthRate', () => {
	it('returns 0 for arrays shorter than 2', () => {
		expect(calculateCompoundGrowthRate([])).toBe(0);
		expect(calculateCompoundGrowthRate([100])).toBe(0);
	});

	it('returns 0 when the first value is 0 (zero base)', () => {
		expect(calculateCompoundGrowthRate([0, 50, 100])).toBe(0);
	});

	it('returns the compound rate for a simple doubling sequence', () => {
		// values: [100, 200] — 1 period, rate = (200/100)^(1/1) - 1 = 1.0 -> 100%
		expect(calculateCompoundGrowthRate([100, 200])).toBeCloseTo(100, 4);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// smoothData / detectAnomalies
// ═══════════════════════════════════════════════════════════════════════════

describe('smoothData', () => {
	it('returns the input unchanged when data shorter than the window', () => {
		// windowSize default = 3; len 2 < 3 -> passthrough
		const data = [1, 2];
		expect(smoothData(data)).toBe(data); // same reference (the implementation returns the original)
	});

	it('returns a moving-average smoothed series for longer inputs', () => {
		// data: [1, 2, 3, 4, 5], window=3
		// i=0: window {0..1} -> avg(1,2)=1.5
		// i=1: window {0..2} -> avg(1,2,3)=2
		// i=2: window {1..3} -> avg(2,3,4)=3
		// i=3: window {2..4} -> avg(3,4,5)=4
		// i=4: window {3..4} -> avg(4,5)=4.5
		const result = smoothData([1, 2, 3, 4, 5]);
		expect(result).toEqual([1.5, 2, 3, 4, 4.5]);
	});
});

describe('detectAnomalies', () => {
	it('flags only the outlier when one value is >2σ from the mean', () => {
		// 9 clustered ones + 1 large outlier => mean=10.9, stddev~29.7,
		// 2σ ~ 59.4; |100-10.9|=89.1 > 59.4 -> the outlier IS flagged.
		const flags = detectAnomalies([1, 1, 1, 1, 1, 1, 1, 1, 1, 100]);
		expect(flags[9]).toBe(true);
		expect(flags.slice(0, 9).every((f) => f === false)).toBe(true);
	});

	it('flags none when all values are within threshold', () => {
		const flags = detectAnomalies([10, 10, 10, 10]);
		expect(flags.every((f) => f === false)).toBe(true);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// isValidEventName / sanitizeEventProperties
// ═══════════════════════════════════════════════════════════════════════════

describe('isValidEventName', () => {
	it('accepts lowercase + digits + underscore names up to 100 chars', () => {
		expect(isValidEventName('signup_completed')).toBe(true);
		expect(isValidEventName('a1_b2_c3')).toBe(true);
	});

	it('rejects names with uppercase letters', () => {
		expect(isValidEventName('signupCompleted')).toBe(false);
	});

	it('rejects names with spaces, hyphens, or other punctuation', () => {
		expect(isValidEventName('signup completed')).toBe(false);
		expect(isValidEventName('signup-completed')).toBe(false);
		expect(isValidEventName('signup.completed')).toBe(false);
	});

	it('rejects names longer than 100 characters', () => {
		expect(isValidEventName('a'.repeat(101))).toBe(false);
	});

	it('rejects the empty string', () => {
		// Regex `^[a-z0-9_]+$` requires at least 1 char, so '' is rejected.
		expect(isValidEventName('')).toBe(false);
	});
});

describe('sanitizeEventProperties', () => {
	it('drops keys with disallowed characters', () => {
		const out = sanitizeEventProperties({
			valid_key: 1,
			'invalid-key': 2,
			NotLowercase: 3
		});
		expect(out).toHaveProperty('valid_key', 1);
		expect(out).not.toHaveProperty('invalid-key');
		expect(out).not.toHaveProperty('NotLowercase');
	});

	it('truncates strings to 1000 chars', () => {
		const longStr = 'a'.repeat(2000);
		const out = sanitizeEventProperties({ key: longStr });
		expect((out.key as string).length).toBe(1000);
	});

	it('passes numbers and booleans through unchanged', () => {
		const out = sanitizeEventProperties({ count: 42, flag: true });
		expect(out.count).toBe(42);
		expect(out.flag).toBe(true);
	});

	it('coerces null/undefined to null (not "null")', () => {
		const out = sanitizeEventProperties({ a: null, b: undefined });
		expect(out.a).toBeNull();
		expect(out.b).toBeNull();
	});

	it('JSON-stringifies objects/arrays and truncates to 1000', () => {
		const out = sanitizeEventProperties({ obj: { hello: 'world' }, arr: [1, 2, 3] });
		expect(out.obj).toBe('{"hello":"world"}');
		expect(out.arr).toBe('[1,2,3]');
	});
});

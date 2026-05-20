/**
 * datetime utilities — Unit Tests (R25-D)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * The datetime module is a small helper layer over `Intl.DateTimeFormat` +
 * raw `Date` arithmetic. It powers:
 *   - "Posted 2 hours ago" labels on blog/article cards
 *   - "Today" / "Tomorrow" badges on trading-room schedule UI
 *   - ISO normalizers feeding analytics events
 *
 * The contracts under test:
 *   - `formatDate` accepts Date / string / number and overrideable options.
 *   - `formatRelativeTime` BUCKETS into the natural language ranges
 *     just-now / minutes / hours / days / weeks / months / years and
 *     pluralizes correctly. Refactors that flip the ordering (e.g. moving
 *     the `< 60` minute branch above the `< 60` second branch) would
 *     break the user-visible label.
 *   - `isToday` / `isPast` / `isFuture` — boundary semantics around
 *     midnight + `Date.now()`.
 *   - `addDays` — returns a NEW Date, doesn't mutate input. A common bug
 *     in this codebase is `setDate(d.getDate() + n)` returning the same
 *     reference, which then ripples into Svelte $state and re-renders
 *     unrelated UI.
 *   - `toISOString` — string/number/Date inputs all land in UTC ISO.
 *
 * We mock `Date.now()` and the `Date` constructor where needed to pin
 * "now" — without that, relative-time / isToday tests are race-prone
 * across midnight boundaries on slow CI.
 */

import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import {
	addDays,
	formatDate,
	formatRelativeTime,
	isFuture,
	isPast,
	isToday,
	toISOString
} from '../datetime';

// ═══════════════════════════════════════════════════════════════════════════
// formatDate
// ═══════════════════════════════════════════════════════════════════════════

describe('formatDate', () => {
	it('formats a Date with the default long-month / numeric-day options', () => {
		// 2026-05-20 picked so month/day are unambiguous across en-US.
		const d = new Date('2026-05-20T12:00:00Z');
		const out = formatDate(d);
		// Locale en-US with month: 'long' yields "May 20, 2026".
		expect(out).toMatch(/^May 20, 2026$/);
	});

	it('accepts an ISO string and a numeric timestamp interchangeably', () => {
		const iso = '2026-05-20T12:00:00Z';
		const ms = new Date(iso).getTime();
		expect(formatDate(iso)).toBe(formatDate(ms));
	});

	it('honours caller-supplied options (override month/day)', () => {
		const d = new Date('2026-05-20T12:00:00Z');
		// Short month + 2-digit day → "May 20" is the en-US output.
		const out = formatDate(d, { month: 'short', day: '2-digit', year: undefined });
		// We only assert structure here — exact spacing varies between
		// ICU versions, so check that "May" and "20" both appear.
		expect(out).toContain('May');
		expect(out).toContain('20');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// formatRelativeTime
// ═══════════════════════════════════════════════════════════════════════════

describe('formatRelativeTime', () => {
	const FIXED_NOW = new Date('2026-05-20T12:00:00Z').getTime();

	beforeEach(() => {
		vi.useFakeTimers();
		vi.setSystemTime(FIXED_NOW);
	});

	afterEach(() => {
		vi.useRealTimers();
	});

	it('returns "just now" for diffs under 60 seconds', () => {
		expect(formatRelativeTime(FIXED_NOW - 30_000)).toBe('just now');
	});

	it('returns minute-precision string with correct singular/plural', () => {
		expect(formatRelativeTime(FIXED_NOW - 60_000)).toBe('1 minute ago');
		expect(formatRelativeTime(FIXED_NOW - 5 * 60_000)).toBe('5 minutes ago');
	});

	it('returns hour-precision string', () => {
		expect(formatRelativeTime(FIXED_NOW - 60 * 60_000)).toBe('1 hour ago');
		expect(formatRelativeTime(FIXED_NOW - 23 * 60 * 60_000)).toBe('23 hours ago');
	});

	it('returns day-precision string under a week', () => {
		expect(formatRelativeTime(FIXED_NOW - 24 * 60 * 60_000)).toBe('1 day ago');
		expect(formatRelativeTime(FIXED_NOW - 6 * 24 * 60 * 60_000)).toBe('6 days ago');
	});

	it('returns week-precision string under 4 weeks', () => {
		expect(formatRelativeTime(FIXED_NOW - 7 * 24 * 60 * 60_000)).toBe('1 week ago');
		expect(formatRelativeTime(FIXED_NOW - 3 * 7 * 24 * 60 * 60_000)).toBe('3 weeks ago');
	});

	it('returns month-precision (≥30 days, <12 months)', () => {
		// 31 days → 1 month
		expect(formatRelativeTime(FIXED_NOW - 31 * 24 * 60 * 60_000)).toBe('1 month ago');
		// 6 months ≈ 180 days
		expect(formatRelativeTime(FIXED_NOW - 180 * 24 * 60 * 60_000)).toBe('6 months ago');
	});

	it('returns year-precision when ≥365 days', () => {
		expect(formatRelativeTime(FIXED_NOW - 365 * 24 * 60 * 60_000)).toBe('1 year ago');
		expect(formatRelativeTime(FIXED_NOW - 3 * 365 * 24 * 60 * 60_000)).toBe('3 years ago');
	});

	it('accepts a Date and a string interchangeably with the numeric branch', () => {
		const ago = new Date(FIXED_NOW - 60_000);
		expect(formatRelativeTime(ago)).toBe('1 minute ago');
		expect(formatRelativeTime(ago.toISOString())).toBe('1 minute ago');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// isToday / isPast / isFuture
// ═══════════════════════════════════════════════════════════════════════════

describe('isToday', () => {
	const FIXED_NOW = new Date('2026-05-20T12:00:00Z').getTime();

	beforeEach(() => {
		vi.useFakeTimers();
		vi.setSystemTime(FIXED_NOW);
	});

	afterEach(() => {
		vi.useRealTimers();
	});

	it('returns true for "now"', () => {
		expect(isToday(FIXED_NOW)).toBe(true);
	});

	it('returns false for yesterday', () => {
		expect(isToday(FIXED_NOW - 24 * 60 * 60_000)).toBe(false);
	});

	it('returns false for tomorrow', () => {
		expect(isToday(FIXED_NOW + 24 * 60 * 60_000)).toBe(false);
	});
});

describe('isPast / isFuture', () => {
	const FIXED_NOW = new Date('2026-05-20T12:00:00Z').getTime();

	beforeEach(() => {
		vi.useFakeTimers();
		vi.setSystemTime(FIXED_NOW);
	});

	afterEach(() => {
		vi.useRealTimers();
	});

	it('isPast: true for any timestamp before now', () => {
		expect(isPast(FIXED_NOW - 1)).toBe(true);
		expect(isPast(FIXED_NOW - 365 * 24 * 60 * 60_000)).toBe(true);
	});

	it('isPast: false at exactly "now" (strict <)', () => {
		// Documented contract: strict less-than (`getTime() < Date.now()`),
		// so "now itself" is NOT past.
		expect(isPast(FIXED_NOW)).toBe(false);
	});

	it('isFuture: true for any timestamp after now', () => {
		expect(isFuture(FIXED_NOW + 1)).toBe(true);
		expect(isFuture(FIXED_NOW + 365 * 24 * 60 * 60_000)).toBe(true);
	});

	it('isFuture: false at exactly "now" (strict >)', () => {
		expect(isFuture(FIXED_NOW)).toBe(false);
	});

	it('isPast and isFuture are mutually exclusive', () => {
		const sample = FIXED_NOW - 5_000;
		expect(isPast(sample) && isFuture(sample)).toBe(false);
		expect(isPast(sample) || isFuture(sample)).toBe(true);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// addDays — must return NEW Date (not mutate input)
// ═══════════════════════════════════════════════════════════════════════════

describe('addDays', () => {
	it('returns a Date n days in the future', () => {
		const input = new Date('2026-05-20T12:00:00Z');
		const out = addDays(input, 5);
		expect(out.getUTCDate()).toBe(25);
		expect(out.getUTCMonth()).toBe(4); // May (0-indexed)
	});

	it('handles negative day count (subtracts)', () => {
		const input = new Date('2026-05-20T12:00:00Z');
		const out = addDays(input, -5);
		expect(out.getUTCDate()).toBe(15);
	});

	it('does NOT mutate the input Date', () => {
		// This is the load-bearing invariant. A refactor that drops the
		// `new Date(date)` clone (e.g. inlining setDate on the input)
		// would re-introduce a bug we've shipped twice on this stack.
		const input = new Date('2026-05-20T12:00:00Z');
		const originalTime = input.getTime();
		addDays(input, 10);
		expect(input.getTime()).toBe(originalTime);
	});

	it('crosses month boundaries correctly', () => {
		const input = new Date('2026-05-30T12:00:00Z');
		const out = addDays(input, 5);
		expect(out.getUTCMonth()).toBe(5); // June
		expect(out.getUTCDate()).toBe(4);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// toISOString
// ═══════════════════════════════════════════════════════════════════════════

describe('toISOString', () => {
	it('formats a Date to ISO 8601 UTC', () => {
		const d = new Date('2026-05-20T12:00:00Z');
		expect(toISOString(d)).toBe('2026-05-20T12:00:00.000Z');
	});

	it('accepts a string input', () => {
		expect(toISOString('2026-05-20T12:00:00Z')).toBe('2026-05-20T12:00:00.000Z');
	});

	it('accepts a numeric timestamp', () => {
		const ms = new Date('2026-05-20T12:00:00Z').getTime();
		expect(toISOString(ms)).toBe('2026-05-20T12:00:00.000Z');
	});
});

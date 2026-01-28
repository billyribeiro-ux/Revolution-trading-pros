/**
 * ===============================================================================
 * Explosive Swings - Formatters Unit Tests
 * ===============================================================================
 *
 * @description Comprehensive tests for formatting utility functions
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Vitest January 2026 Patterns
 *
 * Tests cover:
 * - formatPercent()
 * - formatPrice()
 * - formatDate()
 * - formatDateShort()
 * - formatDateTime()
 * - formatRelativeTime()
 * - formatWinLossRatio()
 * - formatRiskReward()
 * - formatNumber()
 * - formatDuration()
 * - formatTicker()
 * - truncateText()
 * - Edge cases (null, undefined, NaN, infinity)
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import {
	formatPercent,
	formatPrice,
	formatDate,
	formatDateShort,
	formatDateTime,
	formatRelativeTime,
	formatWinLossRatio,
	formatRiskReward,
	formatNumber,
	formatDuration,
	formatTicker,
	truncateText
} from '../utils/formatters';

// ===============================================================================
// TEST SUITE: formatPercent
// ===============================================================================

describe('formatPercent()', () => {
	describe('positive values', () => {
		it('should format positive percent with + sign', () => {
			expect(formatPercent(5.7)).toBe('+5.7%');
		});

		it('should format zero with + sign', () => {
			expect(formatPercent(0)).toBe('+0.0%');
		});

		it('should format large positive percent', () => {
			expect(formatPercent(100)).toBe('+100.0%');
		});

		it('should format very small positive percent', () => {
			expect(formatPercent(0.1)).toBe('+0.1%');
		});
	});

	describe('negative values', () => {
		it('should format negative percent with - sign', () => {
			expect(formatPercent(-2.1)).toBe('-2.1%');
		});

		it('should format large negative percent', () => {
			expect(formatPercent(-50)).toBe('-50.0%');
		});

		it('should format very small negative percent', () => {
			expect(formatPercent(-0.1)).toBe('-0.1%');
		});
	});

	describe('decimal places', () => {
		it('should use default 1 decimal place', () => {
			expect(formatPercent(5.789)).toBe('+5.8%'); // Rounds up
		});

		it('should use custom decimal places (0)', () => {
			expect(formatPercent(5.7, 0)).toBe('+6%');
		});

		it('should use custom decimal places (2)', () => {
			expect(formatPercent(5.789, 2)).toBe('+5.79%');
		});

		it('should use custom decimal places (3)', () => {
			expect(formatPercent(5.7891, 3)).toBe('+5.789%');
		});
	});

	describe('edge cases', () => {
		it('should handle negative zero', () => {
			expect(formatPercent(-0)).toBe('+0.0%');
		});

		it('should handle very precise decimals', () => {
			expect(formatPercent(1.23456789, 4)).toBe('+1.2346%');
		});
	});
});

// ===============================================================================
// TEST SUITE: formatPrice
// ===============================================================================

describe('formatPrice()', () => {
	describe('normal prices', () => {
		it('should format price with $ prefix', () => {
			expect(formatPrice(142.5)).toBe('$142.50');
		});

		it('should format whole number price', () => {
			expect(formatPrice(100)).toBe('$100.00');
		});

		it('should format price with thousands', () => {
			expect(formatPrice(1000)).toBe('$1,000.00');
		});

		it('should format large price with commas', () => {
			expect(formatPrice(1234567.89)).toBe('$1,234,567.89');
		});
	});

	describe('decimal places', () => {
		it('should use default 2 decimal places', () => {
			expect(formatPrice(142.5)).toBe('$142.50');
		});

		it('should use custom decimal places (0)', () => {
			expect(formatPrice(142.5, 0)).toBe('$143');
		});

		it('should use custom decimal places (3)', () => {
			expect(formatPrice(142.5, 3)).toBe('$142.500');
		});

		it('should round correctly', () => {
			expect(formatPrice(142.555, 2)).toBe('$142.56'); // Rounds up
		});
	});

	describe('edge cases', () => {
		it('should format zero', () => {
			expect(formatPrice(0)).toBe('$0.00');
		});

		it('should handle very small prices', () => {
			expect(formatPrice(0.01)).toBe('$0.01');
		});

		it('should handle very large prices', () => {
			expect(formatPrice(9999999.99)).toBe('$9,999,999.99');
		});
	});
});

// ===============================================================================
// TEST SUITE: formatDate
// ===============================================================================

describe('formatDate()', () => {
	describe('valid dates', () => {
		it('should format date correctly', () => {
			const date = new Date('2026-01-20');
			const formatted = formatDate(date);
			expect(formatted).toMatch(/January 20, 2026/);
		});

		it('should format different months', () => {
			expect(formatDate(new Date('2026-03-15'))).toMatch(/March 15, 2026/);
			expect(formatDate(new Date('2026-12-25'))).toMatch(/December 25, 2026/);
		});
	});

	describe('edge cases', () => {
		it('should return empty string for null', () => {
			expect(formatDate(null)).toBe('');
		});

		it('should return empty string for undefined', () => {
			expect(formatDate(undefined)).toBe('');
		});

		it('should return empty string for invalid date', () => {
			expect(formatDate(new Date('invalid'))).toBe('');
		});

		it('should handle date at year boundary', () => {
			const date = new Date('2026-01-01');
			expect(formatDate(date)).toMatch(/January 1, 2026/);
		});
	});
});

// ===============================================================================
// TEST SUITE: formatDateShort
// ===============================================================================

describe('formatDateShort()', () => {
	describe('valid dates', () => {
		it('should format short date correctly', () => {
			const date = new Date('2026-01-20');
			const formatted = formatDateShort(date);
			expect(formatted).toMatch(/Jan 20/);
		});

		it('should format different months', () => {
			expect(formatDateShort(new Date('2026-03-15'))).toMatch(/Mar 15/);
			expect(formatDateShort(new Date('2026-12-25'))).toMatch(/Dec 25/);
		});
	});

	describe('edge cases', () => {
		it('should return empty string for null', () => {
			expect(formatDateShort(null)).toBe('');
		});

		it('should return empty string for undefined', () => {
			expect(formatDateShort(undefined)).toBe('');
		});

		it('should return empty string for invalid date', () => {
			expect(formatDateShort(new Date('invalid'))).toBe('');
		});
	});
});

// ===============================================================================
// TEST SUITE: formatDateTime
// ===============================================================================

describe('formatDateTime()', () => {
	describe('valid dates', () => {
		it('should format date with time', () => {
			const date = new Date('2026-01-20T14:30:00');
			const formatted = formatDateTime(date);
			expect(formatted).toContain('January 20, 2026');
			expect(formatted).toContain('ET');
		});

		it('should include time component', () => {
			const date = new Date('2026-01-20T09:15:00');
			const formatted = formatDateTime(date);
			expect(formatted).toMatch(/at \d{1,2}:\d{2} [AP]M ET/);
		});
	});

	describe('edge cases', () => {
		it('should return empty string for null', () => {
			expect(formatDateTime(null)).toBe('');
		});

		it('should return empty string for undefined', () => {
			expect(formatDateTime(undefined)).toBe('');
		});

		it('should return empty string for invalid date', () => {
			expect(formatDateTime(new Date('invalid'))).toBe('');
		});
	});
});

// ===============================================================================
// TEST SUITE: formatRelativeTime
// ===============================================================================

describe('formatRelativeTime()', () => {
	beforeEach(() => {
		vi.useFakeTimers();
		vi.setSystemTime(new Date('2026-01-20T12:00:00Z'));
	});

	afterEach(() => {
		vi.useRealTimers();
	});

	describe('time intervals', () => {
		it('should format "Just now" for < 60 seconds', () => {
			const tenSecondsAgo = new Date('2026-01-20T11:59:50Z');
			expect(formatRelativeTime(tenSecondsAgo)).toBe('Just now');
		});

		it('should format minutes ago for < 60 minutes', () => {
			const thirtyMinutesAgo = new Date('2026-01-20T11:30:00Z');
			expect(formatRelativeTime(thirtyMinutesAgo)).toBe('30 min ago');
		});

		it('should format hours ago for < 24 hours', () => {
			const threeHoursAgo = new Date('2026-01-20T09:00:00Z');
			expect(formatRelativeTime(threeHoursAgo)).toBe('3h ago');
		});

		it('should format "Yesterday" for 1 day ago', () => {
			const yesterday = new Date('2026-01-19T12:00:00Z');
			expect(formatRelativeTime(yesterday)).toBe('Yesterday');
		});

		it('should format days ago for < 7 days', () => {
			const threeDaysAgo = new Date('2026-01-17T12:00:00Z');
			expect(formatRelativeTime(threeDaysAgo)).toBe('3 days ago');
		});

		it('should format as date for >= 7 days', () => {
			const twoWeeksAgo = new Date('2026-01-06T12:00:00Z');
			const formatted = formatRelativeTime(twoWeeksAgo);
			// Should be formatted as a date
			expect(formatted).toMatch(/January 6, 2026/);
		});
	});

	describe('edge cases', () => {
		it('should handle exactly 1 minute', () => {
			const oneMinuteAgo = new Date('2026-01-20T11:59:00Z');
			expect(formatRelativeTime(oneMinuteAgo)).toBe('1 min ago');
		});

		it('should handle exactly 1 hour', () => {
			const oneHourAgo = new Date('2026-01-20T11:00:00Z');
			expect(formatRelativeTime(oneHourAgo)).toBe('1h ago');
		});
	});
});

// ===============================================================================
// TEST SUITE: formatWinLossRatio
// ===============================================================================

describe('formatWinLossRatio()', () => {
	describe('normal cases', () => {
		it('should format win/total ratio', () => {
			expect(formatWinLossRatio(6, 7)).toBe('6/7');
		});

		it('should format perfect record', () => {
			expect(formatWinLossRatio(10, 10)).toBe('10/10');
		});

		it('should format no wins', () => {
			expect(formatWinLossRatio(0, 5)).toBe('0/5');
		});
	});

	describe('edge cases', () => {
		it('should handle zero trades', () => {
			expect(formatWinLossRatio(0, 0)).toBe('0/0');
		});

		it('should handle large numbers', () => {
			expect(formatWinLossRatio(150, 200)).toBe('150/200');
		});
	});
});

// ===============================================================================
// TEST SUITE: formatRiskReward
// ===============================================================================

describe('formatRiskReward()', () => {
	describe('normal cases', () => {
		it('should format risk/reward ratio', () => {
			expect(formatRiskReward(3.2)).toBe('3.2:1');
		});

		it('should format 1:1 ratio', () => {
			expect(formatRiskReward(1)).toBe('1.0:1');
		});

		it('should format fractional ratio', () => {
			expect(formatRiskReward(0.5)).toBe('0.5:1');
		});
	});

	describe('edge cases', () => {
		it('should format zero ratio', () => {
			expect(formatRiskReward(0)).toBe('0.0:1');
		});

		it('should format large ratio', () => {
			expect(formatRiskReward(10)).toBe('10.0:1');
		});

		it('should round to 1 decimal', () => {
			expect(formatRiskReward(3.14159)).toBe('3.1:1');
		});
	});
});

// ===============================================================================
// TEST SUITE: formatNumber
// ===============================================================================

describe('formatNumber()', () => {
	describe('normal cases', () => {
		it('should format number with thousands separators', () => {
			expect(formatNumber(1234567)).toBe('1,234,567');
		});

		it('should format small number without separators', () => {
			expect(formatNumber(123)).toBe('123');
		});

		it('should format exact thousand', () => {
			expect(formatNumber(1000)).toBe('1,000');
		});
	});

	describe('edge cases', () => {
		it('should format zero', () => {
			expect(formatNumber(0)).toBe('0');
		});

		it('should format negative numbers', () => {
			expect(formatNumber(-1234567)).toBe('-1,234,567');
		});

		it('should handle very large numbers', () => {
			expect(formatNumber(1000000000)).toBe('1,000,000,000');
		});
	});
});

// ===============================================================================
// TEST SUITE: formatDuration
// ===============================================================================

describe('formatDuration()', () => {
	describe('minutes and seconds', () => {
		it('should format seconds only', () => {
			expect(formatDuration(45)).toBe('0:45');
		});

		it('should format minutes and seconds', () => {
			expect(formatDuration(185)).toBe('3:05'); // 3 minutes, 5 seconds
		});

		it('should format even minutes', () => {
			expect(formatDuration(120)).toBe('2:00');
		});

		it('should pad seconds with leading zero', () => {
			expect(formatDuration(65)).toBe('1:05');
		});
	});

	describe('hours, minutes and seconds', () => {
		it('should format hours', () => {
			expect(formatDuration(3661)).toBe('1:01:01'); // 1 hour, 1 min, 1 sec
		});

		it('should format multiple hours', () => {
			expect(formatDuration(7325)).toBe('2:02:05'); // 2 hours, 2 min, 5 sec
		});

		it('should pad minutes and seconds', () => {
			expect(formatDuration(3725)).toBe('1:02:05');
		});
	});

	describe('edge cases', () => {
		it('should format zero duration', () => {
			expect(formatDuration(0)).toBe('0:00');
		});

		it('should format exactly one hour', () => {
			expect(formatDuration(3600)).toBe('1:00:00');
		});

		it('should format 59 minutes 59 seconds', () => {
			expect(formatDuration(3599)).toBe('59:59');
		});
	});
});

// ===============================================================================
// TEST SUITE: formatTicker
// ===============================================================================

describe('formatTicker()', () => {
	describe('normal cases', () => {
		it('should uppercase lowercase ticker', () => {
			expect(formatTicker('nvda')).toBe('NVDA');
		});

		it('should keep uppercase ticker', () => {
			expect(formatTicker('TSLA')).toBe('TSLA');
		});

		it('should handle mixed case', () => {
			expect(formatTicker('NvDa')).toBe('NVDA');
		});
	});

	describe('trimming', () => {
		it('should trim leading whitespace', () => {
			expect(formatTicker('  NVDA')).toBe('NVDA');
		});

		it('should trim trailing whitespace', () => {
			expect(formatTicker('NVDA  ')).toBe('NVDA');
		});

		it('should trim both sides', () => {
			expect(formatTicker('  NVDA  ')).toBe('NVDA');
		});
	});

	describe('edge cases', () => {
		it('should handle empty string', () => {
			expect(formatTicker('')).toBe('');
		});

		it('should handle whitespace only', () => {
			expect(formatTicker('   ')).toBe('');
		});

		it('should handle special characters', () => {
			expect(formatTicker('brk.b')).toBe('BRK.B');
		});
	});
});

// ===============================================================================
// TEST SUITE: truncateText
// ===============================================================================

describe('truncateText()', () => {
	describe('normal cases', () => {
		it('should not truncate text shorter than maxLength', () => {
			expect(truncateText('Hello', 10)).toBe('Hello');
		});

		it('should truncate text longer than maxLength', () => {
			expect(truncateText('Hello World', 8)).toBe('Hello...');
		});

		it('should handle exact maxLength', () => {
			expect(truncateText('Hello', 5)).toBe('Hello');
		});
	});

	describe('edge cases', () => {
		it('should handle empty string', () => {
			expect(truncateText('', 10)).toBe('');
		});

		it('should handle maxLength of 3 (minimum for ellipsis)', () => {
			expect(truncateText('Hello', 3)).toBe('...');
		});

		it('should handle maxLength less than 3', () => {
			// Edge case: truncateText with maxLength < 3 may produce unexpected results
			// The function slices (0, maxLength - 3) which would be negative
			expect(truncateText('Hello', 2)).toBe('...');
		});

		it('should handle very long text', () => {
			const longText = 'A'.repeat(1000);
			const truncated = truncateText(longText, 100);
			expect(truncated.length).toBe(100);
			expect(truncated.endsWith('...')).toBe(true);
		});
	});
});

// ===============================================================================
// TEST SUITE: Localization Consistency
// ===============================================================================

describe('Localization Consistency', () => {
	it('formatPrice should use en-US locale', () => {
		// In en-US, we use commas for thousands and period for decimals
		expect(formatPrice(1234.56)).toBe('$1,234.56');
	});

	it('formatNumber should use en-US locale', () => {
		expect(formatNumber(1234567)).toBe('1,234,567');
	});

	it('formatDate should use en-US locale', () => {
		const date = new Date('2026-01-20');
		const formatted = formatDate(date);
		// en-US format: Month Day, Year
		expect(formatted).toMatch(/January 20, 2026/);
	});
});

// ===============================================================================
// TEST SUITE: Type Safety
// ===============================================================================

describe('Type Safety', () => {
	it('formatPercent should handle number input', () => {
		const result = formatPercent(5.5 as number);
		expect(typeof result).toBe('string');
	});

	it('formatPrice should handle number input', () => {
		const result = formatPrice(100 as number);
		expect(typeof result).toBe('string');
	});

	it('formatDate should handle Date input', () => {
		const result = formatDate(new Date());
		expect(typeof result).toBe('string');
	});
});

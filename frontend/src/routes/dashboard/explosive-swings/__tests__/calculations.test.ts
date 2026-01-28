/**
 * ===============================================================================
 * Explosive Swings - Calculations Unit Tests
 * ===============================================================================
 *
 * @description Comprehensive tests for calculation utility functions
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Vitest January 2026 Patterns
 *
 * Tests cover:
 * - calculateProgress()
 * - calculatePercentChange()
 * - calculateRiskReward()
 * - calculateRiskRewardFromPercentages()
 * - calculateWinRate()
 * - calculateAverage()
 * - calculateExpectancy()
 * - isAlertNew()
 * - getPositionStatusColor()
 * - Edge cases (zero, negative, null, undefined)
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import {
	calculateProgress,
	calculatePercentChange,
	calculateRiskReward,
	calculateRiskRewardFromPercentages,
	calculateWinRate,
	calculateAverage,
	calculateExpectancy,
	isAlertNew,
	getPositionStatusColor
} from '../utils/calculations';

// ===============================================================================
// TEST SUITE: calculateProgress
// ===============================================================================

describe('calculateProgress()', () => {
	describe('normal cases', () => {
		it('should calculate 0% progress when at entry price', () => {
			const progress = calculateProgress(100, 100, 120);
			expect(progress).toBe(0);
		});

		it('should calculate 50% progress when halfway to target', () => {
			const progress = calculateProgress(100, 110, 120);
			expect(progress).toBe(50);
		});

		it('should calculate 100% progress when at target', () => {
			const progress = calculateProgress(100, 120, 120);
			expect(progress).toBe(100);
		});

		it('should calculate correct progress for arbitrary values', () => {
			// Entry: 142.50, Current: 145.00, Target: 148.00
			// Distance to target: 5.50, Current distance: 2.50
			// Progress: (2.50 / 5.50) * 100 = 45.45%
			const progress = calculateProgress(142.5, 145.0, 148.0);
			expect(progress).toBeCloseTo(45.45, 1);
		});

		it('should handle bearish positions (target below entry)', () => {
			// Short position: Entry 125, Current 123, Target 120
			// Progress should be positive (moving toward target)
			const progress = calculateProgress(125, 123, 120);
			expect(progress).toBe(40); // (2/5) * 100 = 40%
		});
	});

	describe('clamping behavior', () => {
		it('should clamp to 0 when below entry (long position)', () => {
			// Entry: 100, Current: 95, Target: 120
			// Negative progress should be clamped to 0
			const progress = calculateProgress(100, 95, 120);
			expect(progress).toBe(0);
		});

		it('should clamp to 100 when above target (long position)', () => {
			// Entry: 100, Current: 130, Target: 120
			// Progress above 100% should be clamped
			const progress = calculateProgress(100, 130, 120);
			expect(progress).toBe(100);
		});

		it('should clamp to 0 when above entry (short position)', () => {
			// Short: Entry 125, Current 127, Target 120
			// Moving away from target, should clamp to 0
			const progress = calculateProgress(125, 127, 120);
			expect(progress).toBe(0);
		});

		it('should clamp to 100 when below target (short position)', () => {
			// Short: Entry 125, Current 118, Target 120
			// Past target, should clamp to 100
			const progress = calculateProgress(125, 118, 120);
			expect(progress).toBe(100);
		});
	});

	describe('edge cases', () => {
		it('should return 0 when entry equals target (no movement possible)', () => {
			const progress = calculateProgress(100, 100, 100);
			expect(progress).toBe(0);
		});

		it('should handle very small price differences', () => {
			const progress = calculateProgress(100.0, 100.001, 100.002);
			expect(progress).toBe(50);
		});

		it('should handle very large numbers', () => {
			const progress = calculateProgress(1000000, 1500000, 2000000);
			expect(progress).toBe(50);
		});

		it('should handle decimal precision correctly', () => {
			const progress = calculateProgress(142.5, 143.125, 148.0);
			expect(progress).toBeCloseTo(11.36, 1);
		});
	});
});

// ===============================================================================
// TEST SUITE: calculatePercentChange
// ===============================================================================

describe('calculatePercentChange()', () => {
	describe('normal cases', () => {
		it('should calculate positive percent change', () => {
			const change = calculatePercentChange(100, 110);
			expect(change).toBe(10);
		});

		it('should calculate negative percent change', () => {
			const change = calculatePercentChange(100, 90);
			expect(change).toBe(-10);
		});

		it('should return 0 when prices are equal', () => {
			const change = calculatePercentChange(100, 100);
			expect(change).toBe(0);
		});

		it('should calculate correct change for real trading scenarios', () => {
			// Entry: $142.50, Exit: $148.50
			const change = calculatePercentChange(142.5, 148.5);
			expect(change).toBeCloseTo(4.21, 2);
		});

		it('should handle large percent changes', () => {
			const change = calculatePercentChange(100, 200);
			expect(change).toBe(100);
		});
	});

	describe('edge cases', () => {
		it('should return 0 when from price is 0 (avoid division by zero)', () => {
			const change = calculatePercentChange(0, 100);
			expect(change).toBe(0);
		});

		it('should handle negative starting price', () => {
			// Not typical in trading but function should handle it
			const change = calculatePercentChange(-100, -80);
			expect(change).toBe(-20); // (-80 - (-100)) / -100 * 100 = 20 / -100 * 100 = -20
		});

		it('should handle very small numbers', () => {
			const change = calculatePercentChange(0.0001, 0.0002);
			expect(change).toBe(100);
		});

		it('should handle decimal precision', () => {
			const change = calculatePercentChange(142.5, 143.8);
			expect(change).toBeCloseTo(0.91, 2);
		});
	});
});

// ===============================================================================
// TEST SUITE: calculateRiskReward
// ===============================================================================

describe('calculateRiskReward()', () => {
	describe('normal cases', () => {
		it('should calculate 1:1 risk/reward', () => {
			// Entry: 100, Target: 110, Stop: 90
			// Reward: 10, Risk: 10 = 1:1
			const rr = calculateRiskReward(100, 110, 90);
			expect(rr).toBe(1);
		});

		it('should calculate 2:1 risk/reward', () => {
			// Entry: 100, Target: 120, Stop: 90
			// Reward: 20, Risk: 10 = 2:1
			const rr = calculateRiskReward(100, 120, 90);
			expect(rr).toBe(2);
		});

		it('should calculate 3:1 risk/reward', () => {
			// Entry: 142.50, Target: 157.50, Stop: 137.50
			// Reward: 15, Risk: 5 = 3:1
			const rr = calculateRiskReward(142.5, 157.5, 137.5);
			expect(rr).toBe(3);
		});

		it('should calculate fractional risk/reward', () => {
			// Entry: 100, Target: 105, Stop: 90
			// Reward: 5, Risk: 10 = 0.5:1
			const rr = calculateRiskReward(100, 105, 90);
			expect(rr).toBe(0.5);
		});

		it('should handle bearish positions correctly', () => {
			// Short: Entry: 125, Target: 115, Stop: 130
			// Reward: |115-125| = 10, Risk: |125-130| = 5 = 2:1
			const rr = calculateRiskReward(125, 115, 130);
			expect(rr).toBe(2);
		});
	});

	describe('edge cases', () => {
		it('should return 0 when stop equals entry (no risk)', () => {
			const rr = calculateRiskReward(100, 120, 100);
			expect(rr).toBe(0);
		});

		it('should handle equal entry, target, and stop', () => {
			const rr = calculateRiskReward(100, 100, 100);
			expect(rr).toBe(0);
		});

		it('should handle very small differences', () => {
			const rr = calculateRiskReward(100.0, 100.02, 99.99);
			expect(rr).toBe(2); // 0.02 / 0.01 = 2
		});
	});
});

// ===============================================================================
// TEST SUITE: calculateRiskRewardFromPercentages
// ===============================================================================

describe('calculateRiskRewardFromPercentages()', () => {
	describe('normal cases', () => {
		it('should calculate R/R from win/loss percentages', () => {
			// Avg Win: 6%, Avg Loss: 2% = 3:1
			const rr = calculateRiskRewardFromPercentages(6, 2);
			expect(rr).toBe(3);
		});

		it('should handle typical trading ratios', () => {
			// Avg Win: 5.7%, Avg Loss: 2.1%
			const rr = calculateRiskRewardFromPercentages(5.7, 2.1);
			expect(rr).toBeCloseTo(2.71, 2);
		});

		it('should handle 1:1 ratio', () => {
			const rr = calculateRiskRewardFromPercentages(5, 5);
			expect(rr).toBe(1);
		});
	});

	describe('edge cases', () => {
		it('should return 0 when avg loss is 0', () => {
			const rr = calculateRiskRewardFromPercentages(5, 0);
			expect(rr).toBe(0);
		});

		it('should handle negative avg loss input (takes absolute value)', () => {
			const rr = calculateRiskRewardFromPercentages(6, -2);
			expect(rr).toBe(3);
		});

		it('should handle negative avg win input (takes absolute value)', () => {
			const rr = calculateRiskRewardFromPercentages(-6, 2);
			expect(rr).toBe(3);
		});

		it('should handle both negative inputs', () => {
			const rr = calculateRiskRewardFromPercentages(-6, -2);
			expect(rr).toBe(3);
		});
	});
});

// ===============================================================================
// TEST SUITE: calculateWinRate
// ===============================================================================

describe('calculateWinRate()', () => {
	describe('normal cases', () => {
		it('should calculate 100% win rate', () => {
			const winRate = calculateWinRate(10, 10);
			expect(winRate).toBe(100);
		});

		it('should calculate 0% win rate', () => {
			const winRate = calculateWinRate(0, 10);
			expect(winRate).toBe(0);
		});

		it('should calculate 50% win rate', () => {
			const winRate = calculateWinRate(5, 10);
			expect(winRate).toBe(50);
		});

		it('should calculate typical win rate', () => {
			// 6 wins out of 7 trades = 85.71%
			const winRate = calculateWinRate(6, 7);
			expect(winRate).toBeCloseTo(85.71, 2);
		});
	});

	describe('edge cases', () => {
		it('should return 0 when total trades is 0', () => {
			const winRate = calculateWinRate(0, 0);
			expect(winRate).toBe(0);
		});

		it('should handle single winning trade', () => {
			const winRate = calculateWinRate(1, 1);
			expect(winRate).toBe(100);
		});

		it('should handle single losing trade', () => {
			const winRate = calculateWinRate(0, 1);
			expect(winRate).toBe(0);
		});
	});
});

// ===============================================================================
// TEST SUITE: calculateAverage
// ===============================================================================

describe('calculateAverage()', () => {
	describe('normal cases', () => {
		it('should calculate average of positive numbers', () => {
			const avg = calculateAverage([2, 4, 6, 8, 10]);
			expect(avg).toBe(6);
		});

		it('should calculate average with decimals', () => {
			const avg = calculateAverage([5.2, 4.8, 6.1, 3.9]);
			expect(avg).toBe(5);
		});

		it('should handle single value', () => {
			const avg = calculateAverage([42]);
			expect(avg).toBe(42);
		});

		it('should handle typical trading gains', () => {
			const gains = [8.2, 5.1, 4.8, 6.3, 3.9, -2.1];
			const avg = calculateAverage(gains);
			expect(avg).toBeCloseTo(4.37, 2);
		});
	});

	describe('edge cases', () => {
		it('should return 0 for empty array', () => {
			const avg = calculateAverage([]);
			expect(avg).toBe(0);
		});

		it('should handle negative numbers', () => {
			const avg = calculateAverage([-5, -10, -15]);
			expect(avg).toBe(-10);
		});

		it('should handle mix of positive and negative', () => {
			const avg = calculateAverage([10, -5, 5, -10]);
			expect(avg).toBe(0);
		});

		it('should handle zero values', () => {
			const avg = calculateAverage([0, 0, 0]);
			expect(avg).toBe(0);
		});
	});
});

// ===============================================================================
// TEST SUITE: calculateExpectancy
// ===============================================================================

describe('calculateExpectancy()', () => {
	describe('normal cases', () => {
		it('should calculate positive expectancy', () => {
			// Win rate: 60% (0.6), Avg Win: 10%, Avg Loss: 5%
			// Expectancy = (0.6 * 10) - (0.4 * 5) = 6 - 2 = 4%
			const expectancy = calculateExpectancy(0.6, 10, 5);
			expect(expectancy).toBe(4);
		});

		it('should calculate negative expectancy', () => {
			// Win rate: 30% (0.3), Avg Win: 5%, Avg Loss: 10%
			// Expectancy = (0.3 * 5) - (0.7 * 10) = 1.5 - 7 = -5.5%
			const expectancy = calculateExpectancy(0.3, 5, 10);
			expect(expectancy).toBe(-5.5);
		});

		it('should calculate zero expectancy (breakeven)', () => {
			// Win rate: 50%, Avg Win: 10%, Avg Loss: 10%
			// Expectancy = (0.5 * 10) - (0.5 * 10) = 0
			const expectancy = calculateExpectancy(0.5, 10, 10);
			expect(expectancy).toBe(0);
		});

		it('should calculate realistic trading expectancy', () => {
			// Win rate: 82% (0.82), Avg Win: 5.7%, Avg Loss: 2.1%
			// Expectancy = (0.82 * 5.7) - (0.18 * 2.1) = 4.674 - 0.378 = 4.296%
			const expectancy = calculateExpectancy(0.82, 5.7, 2.1);
			expect(expectancy).toBeCloseTo(4.296, 2);
		});
	});

	describe('edge cases', () => {
		it('should handle 100% win rate', () => {
			const expectancy = calculateExpectancy(1, 10, 5);
			expect(expectancy).toBe(10);
		});

		it('should handle 0% win rate', () => {
			const expectancy = calculateExpectancy(0, 10, 5);
			expect(expectancy).toBe(-5);
		});

		it('should handle zero average win', () => {
			const expectancy = calculateExpectancy(0.5, 0, 5);
			expect(expectancy).toBe(-2.5);
		});

		it('should handle zero average loss', () => {
			const expectancy = calculateExpectancy(0.5, 10, 0);
			expect(expectancy).toBe(5);
		});
	});
});

// ===============================================================================
// TEST SUITE: isAlertNew
// ===============================================================================

describe('isAlertNew()', () => {
	beforeEach(() => {
		vi.useFakeTimers();
		vi.setSystemTime(new Date('2026-01-20T12:00:00Z'));
	});

	afterEach(() => {
		vi.useRealTimers();
	});

	describe('default threshold (30 minutes)', () => {
		it('should return true for alert created 5 minutes ago', () => {
			const fiveMinutesAgo = new Date('2026-01-20T11:55:00Z');
			expect(isAlertNew(fiveMinutesAgo)).toBe(true);
		});

		it('should return true for alert created 29 minutes ago', () => {
			const twentyNineMinutesAgo = new Date('2026-01-20T11:31:00Z');
			expect(isAlertNew(twentyNineMinutesAgo)).toBe(true);
		});

		it('should return false for alert created 31 minutes ago', () => {
			const thirtyOneMinutesAgo = new Date('2026-01-20T11:29:00Z');
			expect(isAlertNew(thirtyOneMinutesAgo)).toBe(false);
		});

		it('should return false for alert created yesterday', () => {
			const yesterday = new Date('2026-01-19T12:00:00Z');
			expect(isAlertNew(yesterday)).toBe(false);
		});
	});

	describe('custom threshold', () => {
		it('should use custom threshold of 60 minutes', () => {
			const fiftyMinutesAgo = new Date('2026-01-20T11:10:00Z');
			expect(isAlertNew(fiftyMinutesAgo, 60)).toBe(true);
		});

		it('should use custom threshold of 15 minutes', () => {
			const twentyMinutesAgo = new Date('2026-01-20T11:40:00Z');
			expect(isAlertNew(twentyMinutesAgo, 15)).toBe(false);
		});

		it('should handle very small threshold', () => {
			const oneMinuteAgo = new Date('2026-01-20T11:59:00Z');
			expect(isAlertNew(oneMinuteAgo, 2)).toBe(true);
		});
	});

	describe('edge cases', () => {
		it('should return true for alert created just now', () => {
			const now = new Date('2026-01-20T12:00:00Z');
			expect(isAlertNew(now)).toBe(true);
		});

		it('should handle exactly 30 minutes', () => {
			const exactlyThirtyMinutes = new Date('2026-01-20T11:30:00Z');
			expect(isAlertNew(exactlyThirtyMinutes)).toBe(false);
		});

		it('should handle future dates (returns true)', () => {
			const futureDate = new Date('2026-01-20T13:00:00Z');
			expect(isAlertNew(futureDate)).toBe(true);
		});
	});
});

// ===============================================================================
// TEST SUITE: getPositionStatusColor
// ===============================================================================

describe('getPositionStatusColor()', () => {
	describe('ENTRY status', () => {
		it('should return entry badge class regardless of unrealized percent', () => {
			expect(getPositionStatusColor(5, 'ENTRY')).toBe('badge-entry');
			expect(getPositionStatusColor(-2, 'ENTRY')).toBe('badge-entry');
			expect(getPositionStatusColor(null, 'ENTRY')).toBe('badge-entry');
		});
	});

	describe('WATCHING status', () => {
		it('should return watching badge class regardless of unrealized percent', () => {
			expect(getPositionStatusColor(5, 'WATCHING')).toBe('badge-watching');
			expect(getPositionStatusColor(-2, 'WATCHING')).toBe('badge-watching');
			expect(getPositionStatusColor(null, 'WATCHING')).toBe('badge-watching');
		});
	});

	describe('ACTIVE status', () => {
		it('should return profit badge when unrealized percent is positive', () => {
			expect(getPositionStatusColor(5.5, 'ACTIVE')).toBe('badge-active-profit');
		});

		it('should return profit badge when unrealized percent is zero', () => {
			expect(getPositionStatusColor(0, 'ACTIVE')).toBe('badge-active-profit');
		});

		it('should return loss badge when unrealized percent is negative', () => {
			expect(getPositionStatusColor(-2.5, 'ACTIVE')).toBe('badge-active-loss');
		});

		it('should return loss badge when unrealized percent is null', () => {
			expect(getPositionStatusColor(null, 'ACTIVE')).toBe('badge-active-loss');
		});
	});

	describe('default case', () => {
		it('should return default badge class for unknown status', () => {
			expect(getPositionStatusColor(5, 'UNKNOWN' as any)).toBe('badge-default');
		});
	});
});

// ===============================================================================
// TEST SUITE: Numerical Precision
// ===============================================================================

describe('Numerical Precision', () => {
	it('should handle floating point precision in calculateProgress', () => {
		// Typical floating point issue: 0.1 + 0.2 !== 0.3
		const progress = calculateProgress(0.1, 0.2, 0.3);
		expect(progress).toBeCloseTo(50, 5);
	});

	it('should handle floating point precision in calculatePercentChange', () => {
		const change = calculatePercentChange(0.1, 0.3);
		expect(change).toBeCloseTo(200, 5);
	});

	it('should handle very small numbers in calculateRiskReward', () => {
		const rr = calculateRiskReward(0.0001, 0.0003, 0.00005);
		expect(rr).toBeCloseTo(3.33, 2);
	});

	it('should handle very large numbers in calculateAverage', () => {
		const avg = calculateAverage([1e10, 2e10, 3e10]);
		expect(avg).toBe(2e10);
	});
});

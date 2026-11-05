import { expect, it } from 'vite-plus/test';
import {
	accountActivityLabel,
	cohortLabel,
	recordedInstant,
	recordedUnits,
	recordedUsdCents,
	recordedWallClock
} from './profile-display.js';
it('keeps legacy wall-clock values raw rather than applying the browser timezone', () => {
	expect(recordedWallClock('2026-11-01T01:30:00')).toBe(
		'2026-11-01T01:30:00 (timezone not recorded)'
	);
	expect(recordedWallClock('300000-01-01T00:00:00')).toContain('300000-01-01T00:00:00');
	expect(recordedWallClock(null)).toBe('Not recorded');
	expect(recordedInstant('infinity')).toBe('infinity (recorded source value)');
	expect(recordedInstant('2026-11-05T12:00:00-05:00')).toBe(
		recordedInstant('2026-11-05T17:00:00Z')
	);
});
it('retains cents, refunds, large decimal precision and missing currency without inventing dollars', () => {
	expect(recordedUsdCents(7501)).toBe('USD 75.01');
	expect(recordedUsdCents(0)).toBe('USD 0.00');
	expect(recordedUnits('9007199254740993.01', 'EUR')).toBe('EUR 9,007,199,254,740,993.01');
	expect(recordedUnits('12.50', null)).toBe('Currency not recorded 12.50');
	expect(recordedUnits(null, 'USD')).toBe('Not recorded');
	expect(recordedUnits('0.001', 'EUR')).toBe('EUR 0.001');
});
it('keeps current membership cohort separate from account restrictions and unknown activity', () => {
	expect(cohortLabel('active')).toBe('Active');
	expect(accountActivityLabel(false)).toBe('Inactive account');
	expect(accountActivityLabel(null)).toBe('Account activity not recorded');
});

import { describe, it, expect } from 'vite-plus/test';
import {
	parseProfileTagCatalog,
	parseProfileTagAssignment,
	parseProfileTagRemoval,
	parseMembershipAction,
	operationalExpiry,
	extendedRecordedExpiry,
	memberNumericIdentity
} from './member-profile-actions.js';
describe('recorded member actions', () => {
	it('rejects an empty successful catalog when saved rows are claimed', () => {
		expect(() =>
			parseProfileTagCatalog(
				{ tags: [], meta: { current_page: 1, per_page: 25, total: 1, total_pages: 1 } },
				1,
				''
			)
		).toThrow();
	});
	it('requires exact saved tag identity and actual assignment counts', () => {
		const ack = {
			assigned: 0,
			already_assigned: 1,
			requested: 1,
			tags: [{ id: 3, name: 'VIP', slug: 'vip' }]
		};
		expect(parseProfileTagAssignment(ack, '3')).toEqual({ assigned: 0, already_assigned: 1 });
		expect(() => parseProfileTagAssignment({ ...ack, assigned: 1 }, '3')).toThrow();
		expect(() => parseProfileTagAssignment(ack, '4')).toThrow();
	});
	it('checks member and tag identity for an idempotent removal', () => {
		expect(
			parseProfileTagRemoval({ removed: false, user_id: 2, tag_id: 3 }, '2', '3').removed
		).toBe(false);
		expect(() =>
			parseProfileTagRemoval({ removed: true, user_id: 4, tag_id: 3 }, '2', '3')
		).toThrow();
	});
	it('requires persisted membership identity and requested expiry', () => {
		const ack = {
			membership: {
				id: 9,
				user_id: 2,
				plan_id: 3,
				expires_at: '2027-01-01T00:00:00',
				status: 'active'
			}
		};
		expect(parseMembershipAction(ack, '2', { id: '9', expiresAt: '2027-01-01T00:00:00' }).id).toBe(
			'9'
		);
		expect(() => parseMembershipAction(ack, '2', { expiresAt: '2028-01-01T00:00:00' })).toThrow();
	});
	it('never fabricates an extension base for missing, infinite or unsupported dates', () => {
		for (const raw of [null, 'infinity', '123456-01-01T00:00:00'])
			expect(extendedRecordedExpiry(raw, 30)).toBe('');
		expect(extendedRecordedExpiry('2026-11-01T03:00:00', 7)).toBe('2026-11-08T03:00');
		expect(() => operationalExpiry('2026-02-30T10:00')).toThrow();
	});
	it('rejects unsafe legacy numeric action identities without rounding', () => {
		expect(memberNumericIdentity('12')).toBe(12);
		expect(() => memberNumericIdentity('9007199254740993')).toThrow();
	});
});

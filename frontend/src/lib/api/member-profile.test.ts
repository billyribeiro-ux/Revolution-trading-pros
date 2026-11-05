// @vitest-environment node
import { expect, it, vi } from 'vite-plus/test';
import {
	formatMemberInstant,
	formatMemberRecordedAmount,
	formatMemberRecordedTimestamp,
	memberProfileApi,
	parseMemberEmails,
	parseMemberMemberships,
	parseMemberNoteCreated,
	parseMemberNoteRemoved,
	parseMemberNotes,
	parseMemberOrders,
	parseMemberProfile,
	parseMemberTimeline
} from './member-profile';
const memberId = 9;
const naive = '2020-11-01T01:30:00.123456',
	instant = '2026-11-05T20:00:00.123456Z';
const savedNote = {
	id: 17,
	user_id: memberId,
	content: 'Actual note\nsecond line',
	created_by: 2,
	created_by_name: 'Recorded admin',
	created_at: instant,
	updated_at: instant
};
const membership = {
	id: 11,
	user_id: memberId,
	plan_id: 3,
	plan_name: 'Saved plan',
	plan_slug: 'saved-plan',
	status: 'expired',
	starts_at: naive,
	expires_at: naive,
	cancelled_at: null,
	trial_ends_at: null,
	grace_period_end: null,
	current_period_start: null,
	current_period_end: null,
	cancel_at_period_end: null,
	created_at: naive,
	updated_at: null,
	deleted_at: null,
	payment_provider: null,
	stripe_subscription_id: null,
	catalog_price_units: 120,
	catalog_currency: null,
	billing_cycle: 'annual',
	interval_count: 12,
	total_paid: null,
	next_payment_at: null,
	unmeasured_reason: 'No charged snapshot or next-charge promise'
};
const order = {
	id: 12,
	user_id: memberId,
	order_number: 'Recorded-001',
	status: 'paid',
	currency: 'USD',
	total_units: '123.45',
	refund_units: '0.00',
	net_recorded_units: '123.45',
	created_at: naive,
	completed_at: null,
	refunded_at: null
};
function history(kind: string, entries: unknown[] = []) {
	return {
		schema_version: 1,
		metrics_scope: `member_${kind}`,
		member_id: memberId,
		pagination: { total: entries.length, per_page: 25, current_page: 1, last_page: 1 },
		timestamp_basis: 'Actual declared source basis',
		[kind]: entries
	};
}
function timeline() {
	return {
		schema_version: 1,
		metrics_scope: 'member_recorded_timeline',
		member_id: memberId,
		data: [
			{
				id: 'membership:11',
				type: 'membership',
				title: 'Membership record created',
				date: naive,
				timestamp_basis: 'recorded without timezone',
				meta: { current_status: 'expired' }
			}
		],
		total: 1,
		pagination: { total: 1, per_page: 25, current_page: 1, last_page: 1 },
		coverage: 'Creation fact; not historical status change'
	};
}
function profile() {
	return {
		schema_version: 1,
		metrics_scope: 'member_profile',
		member_id: memberId,
		evaluated_at: instant,
		member: {
			id: memberId,
			name: null,
			first_name: null,
			last_name: null,
			email: 'actual@example.test',
			role: null,
			avatar: null,
			email_verified: false,
			created_at: naive,
			updated_at: null,
			joined_at: naive,
			status: 'active',
			active_subscriptions_count: 1,
			current_plan: 'Another active plan',
			current_product: null,
			last_login: null,
			tags: ['Recorded tag'],
			tag_records: [{ id: 4, name: 'Recorded tag', slug: 'recorded-tag' }],
			account: {
				is_active: null,
				mfa_enabled: null,
				recorded_restriction: 'suspended',
				reason: 'Recorded reason',
				ban_reason: null,
				banned_until: null
			},
			recorded_age_days: null
		},
		recorded_order_value: {
			currency: 'USD',
			net_cents: 12345,
			order_count: 1,
			excluded_non_usd_orders: 0,
			excluded_unknown_currency_orders: 0,
			scope: 'recorded_orders_less_recorded_refunds',
			limitations: 'Not processor settlement or complete invoice revenue'
		},
		memberships: history('memberships', [membership]),
		orders: history('orders', [order]),
		notes: history('notes', [savedNote]),
		activity: history('activity'),
		emails: {
			...history('emails'),
			coverage: 'Recorded user_id only',
			observations: 'Provider receipt observations; not human opens'
		},
		timeline: timeline(),
		engagement: {
			score: null,
			reason: 'Unmeasured',
			observations: {
				linked_email_receipts: 0,
				stored_open_signals: 0,
				stored_click_signals: 0,
				scope: 'Recorded receipt signals'
			}
		},
		timestamp_basis: 'Legacy wall-clock values are unqualified',
		billing_coverage: 'Catalog prices are not charged totals'
	};
}
it('preserves nullable account/profile facts, actual cents and unavailable financial/engagement measurements', () => {
	const result = parseMemberProfile(profile(), memberId);
	expect(result.member.name).toBeNull();
	expect(result.member.status).toBe('active');
	expect(result.member.account.recorded_restriction).toBe('suspended');
	expect(result.member.account.is_active).toBeNull();
	expect(result.engagement.score).toBeNull();
	expect(result.recorded_order_value.net_cents).toBe(12345);
	expect(result.memberships.memberships[0].catalog_currency).toBeNull();
	expect(result.memberships.memberships[0].total_paid).toBeNull();
	expect(result.memberships.memberships[0].next_payment_at).toBeNull();
});
it('rejects mismatched member identity in the profile, history envelope and linked row', () => {
	const value = profile();
	expect(() => parseMemberProfile({ ...value, member_id: 10 }, 9)).toThrow();
	expect(() =>
		parseMemberProfile({ ...value, notes: { ...value.notes, member_id: 10 } }, 9)
	).toThrow();
	expect(() => parseMemberNotes(history('notes', [{ ...savedNote, user_id: 10 }]), 9)).toThrow();
});
it('rejects old asserted success shapes and guessed LTV/paid/next-charge facts', () => {
	expect(() => parseMemberProfile({ member: { id: 9 }, stats: { total_spent: 0 } }, 9)).toThrow();
	expect(() =>
		parseMemberMemberships(history('memberships', [{ ...membership, total_paid: 0 }]), 9)
	).toThrow();
	expect(() =>
		parseMemberMemberships(
			history('memberships', [{ ...membership, next_payment_at: membership.expires_at }]),
			9
		)
	).toThrow();
	expect(() =>
		parseMemberProfile({ ...profile(), engagement: { ...profile().engagement, score: 0 } }, 9)
	).toThrow();
});
it('retains decimal units as exact strings and leaves unknown currency unlabelled as dollars', () => {
	const result = parseMemberOrders(
		history('orders', [{ ...order, total_units: '12345678.91', currency: null }]),
		9
	);
	expect(result.orders[0].total_units).toBe('12345678.91');
	expect(formatMemberRecordedAmount(result.orders[0].total_units, null)).toBe(
		'12345678.91 units (currency unrecorded)'
	);
	expect(() =>
		parseMemberOrders(history('orders', [{ ...order, total_units: 123.45 }]), 9)
	).toThrow();
});
it('never converts an ambiguous historical timestamp into browser-local or invented UTC time', () => {
	expect(formatMemberRecordedTimestamp(naive)).toBe(
		'2020-11-01 01:30:00.123456 (recorded without timezone)'
	);
	expect(formatMemberRecordedTimestamp(null)).toBe('Not recorded');
	expect(formatMemberInstant(instant)).toContain('UTC');
	expect(() =>
		parseMemberMemberships(history('memberships', [{ ...membership, starts_at: instant }]), 9)
	).toThrow();
});
it('requires actual provider ID and acceptance instant before accepting an accepted email outcome', () => {
	const receipt = {
		id: 19,
		user_id: 9,
		recipient: 'actual@example.test',
		subject: 'Recorded subject',
		recorded_status: 'sent',
		outcome: 'unknown',
		provider: null,
		provider_message_id: null,
		sent_at: null,
		queued_at: null,
		created_at: naive,
		opened_at: naive,
		clicked_at: null,
		bounced_at: null,
		error: null
	};
	const value = {
		...history('emails', [receipt]),
		coverage: 'Recorded user_id',
		observations: 'Stored signals'
	};
	expect(parseMemberEmails(value, 9).emails[0].outcome).toBe('unknown');
	expect(() =>
		parseMemberEmails({ ...value, emails: [{ ...receipt, outcome: 'accepted' }] }, 9)
	).toThrow();
});
it('preserves timeline creation meaning and validates source-specific timestamp representations', () => {
	expect(parseMemberTimeline(timeline(), 9).data[0].title).toBe('Membership record created');
	expect(() =>
		parseMemberTimeline(
			{ ...timeline(), data: [{ ...timeline().data[0], timestamp_basis: 'recorded UTC instant' }] },
			9
		)
	).toThrow();
	expect(() => parseMemberTimeline({ ...timeline(), total: 2 }, 9)).toThrow();
});
it('requires exact saved-note content/identities and accepts an explicit idempotent removal acknowledgment', () => {
	const value = {
		schema_version: 1,
		metrics_scope: 'member_note_mutation',
		member_id: 9,
		note: savedNote
	};
	expect(parseMemberNoteCreated(value, 9, savedNote.content).id).toBe('17');
	expect(() => parseMemberNoteCreated(value, 9, 'Different content')).toThrow();
	expect(() => parseMemberNoteCreated({ message: 'Saved' }, 9, savedNote.content)).toThrow();
	expect(
		parseMemberNoteRemoved(
			{
				schema_version: 1,
				metrics_scope: 'member_note_mutation',
				member_id: 9,
				note_id: 17,
				removed: false
			},
			9,
			17
		).removed
	).toBe(false);
	expect(() =>
		parseMemberNoteRemoved(
			{
				schema_version: 1,
				metrics_scope: 'member_note_mutation',
				member_id: 9,
				note_id: 18,
				removed: true
			},
			9,
			17
		)
	).toThrow();
});
it('reads through both canonical aliases without caching and rejects malformed200', async () => {
	const fetcher = vi
		.fn<typeof fetch>()
		.mockResolvedValueOnce(Response.json(profile()))
		.mockResolvedValueOnce(Response.json({ member: { id: 9 } }));
	expect((await memberProfileApi.get(9, { family: 'members', fetcher })).member_id).toBe('9');
	expect(fetcher).toHaveBeenNthCalledWith(
		1,
		'/api/admin/members/9',
		expect.objectContaining({ method: 'GET', cache: 'no-store', credentials: 'same-origin' })
	);
	await expect(memberProfileApi.get(9, { fetcher })).rejects.toThrow('invalid or unconfirmed');
	expect(fetcher).toHaveBeenCalledTimes(2);
});
it('keeps action filtering and pagination exact, rejecting unsupported filters before a request', async () => {
	const value = {
			...history('activity'),
			pagination: { total: 0, per_page: 10, current_page: 2, last_page: 1 }
		},
		fetcher = vi.fn<typeof fetch>().mockResolvedValue(Response.json(value));
	await memberProfileApi.activity(
		9,
		{ page: 2, per_page: 10, action: 'member_note_created' },
		{ fetcher }
	);
	expect(fetcher.mock.calls[0][0]).toBe(
		'/api/admin/member-management/9/activity?page=2&per_page=10&action=member_note_created'
	);
	expect(() => memberProfileApi.notes(9, { action: 'made-up' }, { fetcher })).toThrow(
		'activity filter'
	);
	expect(fetcher).toHaveBeenCalledTimes(1);
});
it('attempts a note mutation once, preserves a real failure and recovers through a separate explicit read', async () => {
	const fetcher = vi
		.fn<typeof fetch>()
		.mockResolvedValueOnce(Response.json({ message: 'Audit unavailable' }, { status: 503 }))
		.mockResolvedValueOnce(Response.json(history('notes', [savedNote])));
	await expect(memberProfileApi.addNote(9, savedNote.content, { fetcher })).rejects.toMatchObject({
		status: 503,
		message: 'Audit unavailable'
	});
	expect(fetcher).toHaveBeenCalledTimes(1);
	expect((await memberProfileApi.notes(9, {}, { fetcher })).notes[0].id).toBe('17');
	expect(fetcher).toHaveBeenCalledTimes(2);
});
it('rejects invalid notes without a request and checks deletion acknowledgment rather than mere2xx', async () => {
	const fetcher = vi.fn<typeof fetch>().mockResolvedValue(Response.json({ message: 'Removed' }));
	await expect(memberProfileApi.addNote(9, '\0', { fetcher })).rejects.toThrow('nonempty note');
	expect(fetcher).not.toHaveBeenCalled();
	await expect(memberProfileApi.removeNote(9, 17, { fetcher })).rejects.toMatchObject({
		status: 200,
		outcome: 'unconfirmed',
		reconcile_history_before_retry: true
	});
	expect(fetcher).toHaveBeenCalledTimes(1);
});

it('rejects malformed empty successful histories and accepts an actual empty page beyond the last page', () => {
	const value = history('notes', [savedNote]);
	expect(() => parseMemberNotes({ ...value, notes: [] }, 9)).toThrow();
	expect(() => parseMemberProfile({ ...profile(), notes: { ...value, notes: [] } }, 9)).toThrow();
	expect(
		parseMemberNotes(
			{
				...value,
				notes: [],
				pagination: { total: 1, per_page: 25, current_page: 2, last_page: 1 }
			},
			9
		).notes
	).toEqual([]);
	expect(() =>
		parseMemberNotes(
			{
				...value,
				pagination: { total: 1, per_page: 100, current_page: Number.MAX_SAFE_INTEGER, last_page: 1 }
			},
			9
		)
	).toThrow();
});
it('preserves wide UTC, BCE and infinite recorded timestamps without inventing a browser date', () => {
	const wide = '120000-11-05T20:00:00.123456Z';
	const result = parseMemberNotes(
		history('notes', [{ ...savedNote, created_at: wide, updated_at: null }]),
		9
	);
	expect(result.notes[0].created_at).toBe(wide);
	expect(formatMemberInstant(wide)).toBe('120000-11-05 20:00:00.123456Z (recorded UTC instant)');
	const ancient = '0001-11-01T01:30:00 BC';
	expect(
		parseMemberMemberships(
			history('memberships', [{ ...membership, starts_at: ancient, expires_at: 'infinity' }]),
			9
		).memberships[0].starts_at
	).toBe(ancient);
	expect(formatMemberRecordedTimestamp(ancient)).toContain('BC (recorded without timezone)');
	expect(formatMemberRecordedTimestamp('infinity')).toBe('infinity (recorded without timezone)');
});
it('does not accept provider acknowledgment against an incompatible recorded status', () => {
	const receipt = {
		id: 19,
		user_id: 9,
		recipient: 'actual@example.test',
		subject: 'Subject',
		recorded_status: 'failed',
		outcome: 'accepted',
		provider: null,
		provider_message_id: 'real-recorded-id',
		sent_at: instant,
		queued_at: null,
		created_at: naive,
		opened_at: null,
		clicked_at: null,
		bounced_at: null,
		error: null
	};
	expect(() =>
		parseMemberEmails(
			{
				...history('emails', [receipt]),
				coverage: 'Recorded user_id',
				observations: 'Stored signals'
			},
			9
		)
	).toThrow();
});

it('keeps mutation uncertainty after a JSON503, network failure and malformed2xx without retrying', async () => {
	for (const make of [
		() =>
			Promise.resolve(
				Response.json(
					{
						message: 'Audit unavailable',
						outcome: 'unconfirmed',
						reconcile_history_before_retry: true
					},
					{ status: 503 }
				)
			),
		() => Promise.reject(new Error('network failure')),
		() => Promise.resolve(Response.json({ message: 'Saved' }))
	]) {
		const fetcher = vi.fn<typeof fetch>().mockImplementation(make);
		await expect(memberProfileApi.addNote(9, savedNote.content, { fetcher })).rejects.toMatchObject(
			{ outcome: 'unconfirmed', reconcile_history_before_retry: true }
		);
		expect(fetcher).toHaveBeenCalledTimes(1);
	}
});
it('preserves explicit server reconciliation flags and keeps clear4xx validation rejections distinct', async () => {
	const fetcher = vi
		.fn<typeof fetch>()
		.mockResolvedValueOnce(
			Response.json(
				{
					message: 'Unconfirmed conflict',
					outcome: 'unconfirmed',
					reconcile_history_before_retry: true
				},
				{ status: 409 }
			)
		)
		.mockResolvedValueOnce(Response.json({ message: 'Invalid note' }, { status: 422 }));
	await expect(memberProfileApi.removeNote(9, 17, { fetcher })).rejects.toMatchObject({
		status: 409,
		outcome: 'unconfirmed',
		reconcile_history_before_retry: true
	});
	await expect(memberProfileApi.addNote(9, savedNote.content, { fetcher })).rejects.toMatchObject({
		status: 422,
		outcome: null,
		reconcile_history_before_retry: false
	});
	expect(fetcher).toHaveBeenCalledTimes(2);
});

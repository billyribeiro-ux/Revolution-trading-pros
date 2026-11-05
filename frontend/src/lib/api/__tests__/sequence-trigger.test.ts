import { expect, it } from 'vite-plus/test';
import { sequenceTriggerHistory } from '../sequence-trigger.js';
const entry = () => ({
	id: '4',
	event_id: '6',
	sequence_id: '42',
	revision_id: '34ad48b0-615b-49e8-86ee-c0d9f77206ee',
	kind: 'custom_event',
	status: 'enrolled',
	contact_id: '9',
	tracker_id: '12',
	happened_at: '2026-11-05T00:00:00Z',
	attempts: 1,
	last_error: null,
	processed_at: '2026-11-05T00:00:01Z'
});
const fixture = () => ({
	sequence_id: '42',
	metrics_scope: 'sequence_trigger',
	data: [entry()],
	meta: { current_page: 2, per_page: 20, total: 21, total_pages: 2 }
});
it('keeps actual recorded trigger outcomes, frozen revisions and pagination without treating a captured event as delivery', () => {
	expect(sequenceTriggerHistory(fixture(), '42')).toMatchObject({
		page: 2,
		total: 21,
		data: [{ status: 'enrolled', event_id: '6', tracker_id: '12', attempts: 1 }]
	});
	for (const status of ['pending', 'skipped', 'cancelled', 'failed', 'already_enrolled'])
		expect(
			sequenceTriggerHistory(
				{
					...fixture(),
					data: [{ ...entry(), status, tracker_id: null, contact_id: null, processed_at: null }]
				},
				'42'
			).data[0]?.status
		).toBe(status);
});
it('rejects malformed or cross-sequence records, invented statuses and inconsistent pagination', () => {
	for (const patch of [
		{ sequence_id: '43' },
		{ metrics_scope: 'legacy_totals' },
		{ meta: { current_page: 1, per_page: 20, total: 21, total_pages: 1 } },
		{ data: [entry(), entry()] },
		{ data: [{ ...entry(), status: 'delivered' }] },
		{ data: [{ ...entry(), revision_id: 'wrong' }] },
		{ data: [{ ...entry(), happened_at: 'wrong' }] }
	])
		expect(() => sequenceTriggerHistory({ ...fixture(), ...patch }, '42')).toThrow();
});

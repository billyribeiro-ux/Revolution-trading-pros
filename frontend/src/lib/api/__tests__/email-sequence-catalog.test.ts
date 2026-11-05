import { expect, it } from 'vite-plus/test';
import {
	catalogSequence,
	catalogSequenceEnvelope,
	catalogSequencePage,
	catalogSequenceStats,
	sequenceCatalogWrite,
	sequenceEmailWrite
} from '../email-sequence-catalog.js';
const key = '34ad48b0-615b-49e8-86ee-c0d9f77206ee';
const fixture = () => ({
	id: '42',
	title: 'Saved sequence',
	description: null,
	status: 'draft',
	trigger_type: 'manual',
	trigger_config: {},
	metrics_scope: 'sequence_outbox',
	definition_error: null,
	historical_steps: [],
	emails_count: 1,
	subscribers_count: 0,
	created_at: '2026-11-05T00:00:00Z',
	updated_at: '2026-11-05T00:00:00Z',
	emails: [
		{
			id: key,
			sequence_id: '42',
			title: 'First',
			email_subject: 'Actual subject',
			email_body: '<p>Saved source</p>',
			text_body: null,
			position: 0,
			status: 'active',
			delay_value: 1,
			delay_unit: 'minutes',
			conditions: [{ type: 'not_opened', value: null, previous_email_id: null }]
		}
	]
});
it('adapts canonical identities and preserves a legacy review error without inventing an empty sequence', () => {
	const saved = catalogSequence(fixture(), '42');
	expect(saved.name).toBe('Saved sequence');
	expect(saved.emails[0]?.step_number).toBe(1);
	expect(saved.emails[0]?.conditions?.[0]?.previous_email_id).toBeUndefined();
	const legacy = catalogSequence({
		...fixture(),
		definition_error: 'Review stored steps',
		emails: [],
		emails_count: null,
		historical_steps: [{ subject: 'Original legacy source' }]
	});
	expect(legacy.definition_error).toBe('Review stored steps');
	expect(legacy.historical_steps).toEqual([{ subject: 'Original legacy source' }]);
	for (const patch of [
		{ id: '43' },
		{ metrics_scope: 'legacy_counters' },
		{ success: false },
		{ emails_count: 2 },
		{ subscribers_count: -1 },
		{ description: {} },
		{ emails: [{ ...fixture().emails[0], sequence_id: '99' }] }
	])
		expect(() => catalogSequence({ ...fixture(), ...patch }, '42')).toThrow();
});
it('requires actual pagination and rejects explicit failure even with a plausible record', () => {
	expect(
		catalogSequencePage({
			data: [fixture()],
			meta: { total: 21, current_page: 2, per_page: 20, last_page: 2 }
		}).total
	).toBe(21);
	expect(() =>
		catalogSequencePage({
			data: [],
			meta: { total: 21, current_page: 2, per_page: 20, last_page: 1 }
		})
	).toThrow('pagination');
	expect(() => catalogSequenceEnvelope({ success: false, sequence: fixture() }, '42')).toThrow(
		'not confirmed'
	);
});
it('round-trips native step numbers as checked canonical positions and prevents cross-parent or measurement writes', () => {
	expect(
		sequenceCatalogWrite({
			name: 'New',
			emails: [
				{
					id: key,
					sequence_id: '',
					step_number: 1,
					name: 'First',
					subject: 'Subject',
					html_content: '<p>Source</p>',
					delay_value: 0,
					delay_unit: 'minutes'
				}
			]
		})
	).toMatchObject({
		title: 'New',
		emails: [{ id: key, position: 0, title: 'First', email_subject: 'Subject', status: 'active' }]
	});
	expect(() => sequenceEmailWrite({ sequence_id: '43', name: 'Wrong' }, '42')).toThrow(
		'different sequence'
	);
	expect(() => sequenceCatalogWrite({ subscribers_count: 999 })).toThrow('authoring');
	expect(() =>
		sequenceCatalogWrite(
			{
				emails: [
					{
						id: key,
						sequence_id: '42',
						step_number: 2,
						name: 'Wrong position',
						subject: 'Subject',
						html_content: 'Body',
						delay_value: 0,
						delay_unit: 'minutes'
					}
				]
			},
			'42'
		)
	).toThrow('order');
});
it('keeps unavailable rate denominators and revenue unknown and validates receipt count relationships', () => {
	const raw = {
		sequence_id: '42',
		metrics_scope: 'sequence_outbox',
		subscribers: 0,
		active_subscribers: 0,
		completed: 0,
		total_sent: 0,
		total_opened: 0,
		total_clicked: 0,
		total_delivered: 0,
		total_bounced: 0,
		total_complained: 0,
		total_unsubscribed: 0,
		open_rate: null,
		click_rate: null,
		revenue: { amount: null, currency: null }
	};
	expect(catalogSequenceStats(raw, '42')).toMatchObject({
		sent: 0,
		open_rate: null,
		bounce_rate: null,
		completion_rate: null,
		revenue: null
	});
	expect(() => catalogSequenceStats({ ...raw, sequence_id: '43' }, '42')).toThrow('identity');
	expect(() => catalogSequenceStats({ ...raw, total_opened: 1 }, '42')).toThrow('counts');
	expect(() =>
		catalogSequenceStats({ ...raw, subscribers: 1, active_subscribers: 1, completed: 1 }, '42')
	).toThrow('counts');
});

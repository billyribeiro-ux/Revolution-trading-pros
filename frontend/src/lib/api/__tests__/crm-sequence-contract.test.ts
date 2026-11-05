import { afterEach, expect, it, vi } from 'vite-plus/test';
const transport = vi.hoisted(() => ({
	get: vi.fn(),
	post: vi.fn(),
	put: vi.fn(),
	delete: vi.fn()
}));
vi.mock('../client.svelte', () => ({ apiClient: transport }));
import { CrmAPI } from '../crm.js';
import {
	crmSequence,
	crmSequenceEnvelope,
	crmSequenceTracker,
	sequenceMailer
} from '../crm-sequence-contract.js';
const mailId = '34ad48b0-615b-49e8-86ee-c0d9f77206ee';
const at = '2026-11-05T00:00:00Z';
function fixture() {
	return {
		id: '42',
		title: 'Saved sequence',
		slug: 'saved-sequence',
		description: null,
		authoring_version: 4,
		definition_error: null,
		historical_steps: [],
		last_error: null,
		trigger_type: 'manual',
		trigger_config: {},
		design_template: 'raw_html',
		settings: {},
		subscriber_settings: {},
		status: 'paused',
		metrics_scope: 'sequence_outbox',
		emails_count: 1,
		subscribers_count: 1,
		total_sent: 1,
		total_opened: 0,
		total_clicked: 0,
		total_revenue: null,
		currency: null,
		created_at: at,
		updated_at: at,
		emails: [
			{
				id: mailId,
				sequence_id: '42',
				position: 0,
				title: 'First',
				email_subject: 'Actual subject',
				email_pre_header: '',
				email_body: '<p>Saved source</p>',
				text_body: null,
				delay: 60,
				delay_value: 1,
				delay_unit: 'minutes',
				status: 'active',
				settings: sequenceMailer(undefined),
				conditions: [],
				sent_count: 1,
				open_count: 0,
				click_count: 0,
				unsubscribe_count: 0,
				created_at: at,
				updated_at: at
			}
		]
	};
}
const stats = () => ({
	sequence_id: '42',
	emails: 1,
	subscribers: 1,
	active_subscribers: 1,
	completed: 0,
	total_sent: 1,
	total_delivered: 1,
	total_opened: 0,
	total_clicked: 0,
	total_bounced: 0,
	total_complained: 0,
	total_unsubscribed: 0,
	failed: 0,
	unknown: 0,
	queued: 0,
	processing: 0,
	open_rate: 0,
	click_rate: 0,
	revenue: { amount: null, currency: null },
	metrics_scope: 'sequence_outbox'
});
afterEach(() => vi.resetAllMocks());
it('preserves recorded revision, nullable legacy count, unmeasured revenue and frozen mail identity', () => {
	expect(crmSequence(fixture(), '42')).toMatchObject({
		authoring_version: 4,
		emails_count: 1,
		total_revenue: null,
		settings: { mailer_settings: { is_custom: false } },
		emails: [{ id: mailId, delay: 60, text_body: null }]
	});
	expect(
		crmSequence({
			...fixture(),
			definition_error: 'Review legacy source',
			emails_count: null,
			emails: [],
			historical_steps: [{ original: 'Original source' }]
		})
	).toMatchObject({
		definition_error: 'Review legacy source',
		emails_count: null,
		historical_steps: [{ original: 'Original source' }]
	});
	expect(() =>
		crmSequenceEnvelope({ sequence: fixture(), stats: { ...stats(), sequence_id: '43' } }, '42')
	).toThrow('identity');
	expect(() =>
		crmSequence({ ...fixture(), emails: [{ ...fixture().emails[0], delay: 1 }] })
	).toThrow('delay');
	expect(() =>
		crmSequence({ ...fixture(), emails: [{ ...fixture().emails[0], open_count: 2 }] })
	).toThrow('receipt');
});
it('retains deleted contact enrollment identity and rejects cross-sequence or malformed history records', () => {
	const tracker = {
		id: '5',
		sequence_id: '42',
		contact_id: null,
		contact_email: 'fixture@example.test',
		revision_id: mailId,
		status: 'completed',
		last_error: null,
		started_at: at,
		created_at: at,
		updated_at: at,
		emails_sent: 1,
		emails_opened: 0,
		emails_clicked: 0,
		notes: [{ action: 'provider_accepted', at }]
	};
	expect(crmSequenceTracker(tracker, '42')).toMatchObject({
		contact_id: null,
		contact_email: 'fixture@example.test',
		status: 'completed'
	});
	expect(() => crmSequenceTracker(tracker, '43')).toThrow('identity');
	expect(() =>
		crmSequenceTracker(
			{ ...tracker, notes: [{ action: 'provider_accepted', at: 'invalid' }] },
			'42'
		)
	).toThrow('timestamp');
});
it('requires confirmed action identity and configures each write as one attempt while reads retain actual pagination', async () => {
	const api = new CrmAPI();
	transport.get.mockResolvedValue({
		data: [fixture()],
		meta: { total: 21, current_page: 2, per_page: 20, last_page: 2 }
	});
	expect((await api.getSequences({ page: 2 })).meta).toMatchObject({
		total: 21,
		current_page: 2,
		total_pages: 2
	});
	expect(transport.get).toHaveBeenCalledWith(
		'/admin/crm/sequences',
		expect.objectContaining({
			preserveEnvelope: true,
			cache: { enabled: false },
			params: { page: 2 }
		})
	);
	transport.put.mockResolvedValue({ ...fixture(), id: '43' });
	await expect(api.updateSequence('42', { status: 'paused', expected_version: 4 })).rejects.toThrow(
		'identity'
	);
	expect(transport.put).toHaveBeenCalledExactlyOnceWith(
		'/admin/crm/sequences/42',
		{ status: 'paused', expected_version: 4 },
		expect.objectContaining({ retry: { attempts: 1, condition: expect.any(Function) } })
	);
	expect(transport.put.mock.calls[0]?.[2]?.retry.condition()).toBe(false);
	transport.delete.mockResolvedValue({ deleted: true, id: '43' });
	await expect(api.deleteSequence('42')).rejects.toThrow('not confirmed');
	transport.post.mockResolvedValue({ sequence_id: '43', subscribed_count: 1 });
	await expect(api.subscribeToSequence('42', ['2'])).rejects.toThrow('did not match');
	transport.post.mockResolvedValue({ sequence_id: '42', subscribed_count: 2 });
	await expect(api.subscribeToSequence('42', ['2'])).rejects.toThrow('exceeded');
	await expect(api.deleteSequence('../settings')).rejects.toThrow('identity');
	expect(transport.delete).toHaveBeenCalledTimes(1);
});

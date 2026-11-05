import { beforeEach } from 'vite-plus/test';
import { authorizedCrmRequestFixture } from '#lib/crm/__tests__/request-fixture.js';
let releaseCrmFixture: () => void = () => undefined;
beforeEach(() => {
	releaseCrmFixture = authorizedCrmRequestFixture();
});
afterEach(() => releaseCrmFixture());
import { afterEach, expect, it, vi } from 'vite-plus/test';
vi.mock('../client.svelte', () => ({ apiClient: { get: vi.fn(), post: vi.fn() } }));
import { CrmAPI } from '../crm.js';
import {
	contactRecord,
	contactFormWrite,
	contactPage,
	contactTimeline,
	contactTags,
	contactMembership,
	contactMessage,
	contactMessageActivity,
	contactEmails,
	contactRequest
} from '../crm-contact-contract.js';
const at = '2026-11-05T12:00:00Z',
	requestId = 'd503dff2-c847-41d5-bcf6-76dcb217b8e0';
function record() {
	return {
		id: 42,
		user_id: null,
		owner_id: null,
		email: 'saved@example.test',
		full_name: 'saved@example.test',
		first_name: null,
		last_name: null,
		phone: null,
		company: null,
		job_title: null,
		status: null,
		source: null,
		lifecycle_stage: null,
		tags: null,
		custom_fields: null,
		created_at: null,
		updated_at: null,
		last_contacted_at: null,
		last_activity_at: null,
		score: null,
		lead_score: null,
		recorded_score: 99,
		historical_score: 100,
		score_assessed_at: null,
		scoring: {
			status: 'not_calculated',
			score: null,
			max_score: 100,
			evaluated_at: null,
			policy_hash: null,
			contributions: [],
			reason: 'No current assessment',
			scope: 'configured_contact_rules'
		},
		facts: {
			scope: 'recorded_contact_links',
			deals: { total: 0, open: 0, won: 0, activities: 0, notes: 0 },
			email: {
				scope: 'crm_outbox',
				accepted: 0,
				open_signals: 0,
				click_signals: 0,
				human_engagement: null
			},
			orders: null,
			unmeasured: {
				health_score: null,
				engagement_score: null,
				value_score: null,
				lifetime_value: null,
				human_email_opens: null
			}
		}
	};
}
const meta = { current_page: 1, per_page: 25, total: 1, last_page: 1 };
const intent = {
	request_id: requestId,
	expected_email: 'saved@example.test',
	subject: 'Exact subject',
	body_html: '<p>Exact creative</p>'
};
function receipt(status: 'accepted' | 'unknown' = 'unknown') {
	return {
		success: status === 'accepted',
		data: {
			contact_id: '42',
			request_id: requestId,
			source_subject: intent.subject.trim(),
			source_hash: 'a'.repeat(64),
			template_id: null,
			metrics_scope: 'contact_message',
			created_at: at,
			submissions: [
				{
					id: '8',
					recipient: intent.expected_email,
					subject: intent.subject,
					provider: 'local-smtp',
					provider_message_id: null,
					status,
					error: status === 'unknown' ? 'Connection lost after submission' : null,
					queued_at: at,
					accepted_at: status === 'accepted' ? at : null
				}
			]
		}
	};
}
afterEach(() => {
	vi.unstubAllGlobals();
	vi.clearAllMocks();
});
it('omits GET bodies and rejects accidental GET payloads before issuing a request', async () => {
	const transport = vi.fn<typeof fetch>().mockResolvedValue(Response.json({ data: [] }));
	vi.stubGlobal('fetch', transport);
	await expect(contactRequest('', 'GET', {})).rejects.toThrow(
		'GET contact request cannot contain a body'
	);
	expect(transport).not.toHaveBeenCalled();
	await contactRequest('');
	expect(transport).toHaveBeenCalledOnce();
	expect(transport.mock.calls[0]?.[1]).not.toHaveProperty('body');
});
it('preserves untouched unknown fields and legacy status when saving an unrelated edit', () => {
	const initial = {
		email: 'actual@example.test',
		first_name: '',
		last_name: '',
		phone: '',
		company: '',
		job_title: '',
		status: 'legacy_unknown',
		lifecycle_stage: '' as const
	};
	expect(contactFormWrite({ ...initial, job_title: 'Analyst' }, initial)).toEqual({
		job_title: 'Analyst'
	});
	expect(contactFormWrite({ ...initial, lifecycle_stage: 'customer' }, initial)).toEqual({
		lifecycle_stage: 'customer'
	});
	expect(
		contactFormWrite(
			{ ...initial, lifecycle_stage: '' },
			{ ...initial, lifecycle_stage: 'customer' }
		)
	).toEqual({ lifecycle_stage: null });
	expect(contactFormWrite({ ...initial, phone: '' }, { ...initial, phone: '123' })).toEqual({
		phone: ''
	});
});
it('preserves unknown facts and raw score provenance without synthesizing current measurements', () => {
	const value = contactRecord(record(), '42', true);
	expect(value.lead_score).toBeNull();
	expect(value.recorded_score).toBe(99);
	expect(value.status).toBeNull();
	expect(value.lifecycle_stage).toBeNull();
	expect(value.facts?.unmeasured.health_score).toBeNull();
	expect(value.facts?.email.open_signals).toBe(0);
	expect(() => contactRecord({ ...record(), score: 99, lead_score: 99 })).toThrow();
	expect(() => contactRecord({ ...record(), id: 9007199254740992 })).toThrow();
	expect(() => contactRecord(record(), '43')).toThrow();
	expect(() => contactRecord({ ...record(), lifecycle_stage: 'inferred' })).toThrow();
	expect(() =>
		contactRecord(
			{
				...record(),
				facts: {
					...record().facts,
					unmeasured: { ...record().facts.unmeasured, lifetime_value: 0 }
				}
			},
			'42',
			true
		)
	).toThrow();
	expect(contactRecord({ ...record(), lifecycle_stage: 'customer' }).lifecycle_stage).toBe(
		'customer'
	);
});
it('validates all page records independently, stable event identifiers and actual coverage', () => {
	const page = contactPage(
		{
			data: [record(), { ...record(), id: 43 }],
			meta: { current_page: 1, per_page: 25, total: 2, total_pages: 1 }
		},
		contactRecord
	);
	expect(page.data.map((row) => row.id)).toEqual(['42', '43']);
	const timeline = {
		contact_id: '42',
		scope: 'recorded_contact_links',
		data: [
			{
				id: 'contact-change:7',
				type: 'contact_updated',
				category: 'contact',
				title: 'Contact changed',
				description: null,
				metadata: {},
				occurred_at: at,
				created_by: null
			}
		],
		meta,
		coverage: {
			sources: ['contact_changes'],
			human_email_engagement: null,
			unlinked_address_history: null
		}
	};
	expect(contactTimeline(timeline, '42').data[0]?.id).toBe('contact-change:7');
	expect(() => contactTimeline({ ...timeline, contact_id: '43' }, '42')).toThrow();
	expect(() => contactTimeline({ ...timeline, meta: { ...meta, total: 50 } }, '42')).toThrow();
});
it('keeps unregistered tags and requires exact membership identity/count acknowledgements', () => {
	expect(
		contactTags(
			{
				contact_id: '42',
				data: [{ id: null, name: 'Legacy label', color: null, definition_status: 'unregistered' }]
			},
			'42'
		)[0]?.id
	).toBeNull();
	expect(
		contactMembership({ contact_id: '42', tag_id: '9', applied_count: 0 }, '42', 'tag', '9', 'add')
	).toBe(0);
	for (const value of [
		{ contact_id: '43', tag_id: '9', applied_count: 1 },
		{ contact_id: '42', tag_id: '10', applied_count: 1 },
		{ contact_id: '42', tag_id: '9', applied_count: 2 },
		{ success: true }
	])
		expect(() => contactMembership(value, '42', 'tag', '9', 'add')).toThrow();
});
it('shows valid uncertain provider receipts on non-2xx and never retries a submission automatically', async () => {
	const transport = vi
		.fn<typeof fetch>()
		.mockImplementation(async () => Response.json(receipt(), { status: 502 }));
	vi.stubGlobal('fetch', transport);
	expect((await new CrmAPI().sendContactMessage('42', intent)).submissions[0]?.status).toBe(
		'unknown'
	);
	expect(transport).toHaveBeenCalledOnce();
	expect(transport).toHaveBeenLastCalledWith(
		'/api/admin/crm/contacts/42/send-email',
		expect.objectContaining({
			method: 'POST',
			body: JSON.stringify(intent),
			redirect: 'error',
			cache: 'no-store'
		})
	);
	transport.mockRejectedValue(new Error('Controlled disconnect'));
	await expect(new CrmAPI().sendContactMessage('42', intent)).rejects.toThrow(
		/Refresh saved state/
	);
	expect(transport).toHaveBeenCalledTimes(2);
	expect(contactMessage(receipt('accepted'), '42', intent).submissions[0]?.accepted_at).toBe(at);
	expect(() =>
		contactMessage(receipt(), '42', { ...intent, expected_email: 'another@example.test' })
	).toThrow();
	expect(() => contactMessage({ ...receipt(), success: true }, '42', intent)).toThrow();
});
it('does not turn malformed successful CRUD or panel responses into saved records or empty history', async () => {
	vi.stubGlobal(
		'fetch',
		vi.fn<typeof fetch>().mockImplementation(async () => Response.json({ success: true }))
	);
	await expect(new CrmAPI().createContact({ email: 'saved@example.test' })).rejects.toThrow();
	await expect(new CrmAPI().deleteContact('42')).rejects.toThrow();
	await expect(new CrmAPI().addContactNote('42', 'Actual note')).rejects.toThrow();
	await expect(new CrmAPI().getContactNotes('42')).rejects.toThrow();
	expect(() =>
		contactEmails(
			{ contact_id: '42', data: [{ id: 'personal:other', contact_id: '43' }], meta },
			'42'
		)
	).toThrow();
});
it('reconciles refreshed message outcomes against the frozen recipient, template and saved source hash', () => {
	const hash = 'a'.repeat(64),
		row = {
			id: `personal:${requestId}`,
			contact_id: '42',
			request_id: requestId,
			source_subject: intent.subject,
			source_hash: hash,
			template_id: null,
			recipient: intent.expected_email,
			receipt_id: '8',
			subject: 'Actual rendered subject',
			status: 'accepted',
			scope: 'contact_message',
			provider: 'local-smtp',
			provider_message_id: 'actual-receipt',
			queued_at: at,
			sent_at: at,
			accepted_at: at,
			opened_at: null,
			clicked_at: null,
			bounced_at: null,
			error: null,
			campaign_name: null
		};
	const activity = contactEmails({ contact_id: '42', data: [row], meta }, '42').data[0];
	if (!activity) throw new Error('Missing confirmed activity');
	expect(() => contactMessageActivity(activity, intent, hash)).not.toThrow();
	for (const change of [
		{ source_hash: 'b'.repeat(64) },
		{ source_subject: 'Different frozen subject' },
		{ recipient: 'different@example.test' },
		{ template_id: '9' },
		{ id: 'personal:another' }
	])
		expect(() => contactMessageActivity({ ...activity, ...change }, intent, hash)).toThrow(
			/frozen message intent/
		);
});

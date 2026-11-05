import { expect, it } from 'vite-plus/test';
import {
	campaignId,
	emailCampaignEnvelope,
	emailCampaignPage,
	emailCampaignWrite,
	emailCampaignAction,
	emailCampaignDeleted
} from '../email-campaign-catalog.js';
const fixture = () => ({
	id: 9,
	name: 'Saved campaign',
	subject: 'Actual subject',
	from_name: '',
	from_email: '',
	reply_to: null,
	template_id: 3,
	segment_id: null,
	created_by: null,
	status: 'draft',
	created_at: '2026-11-05T00:00:00Z',
	updated_at: '2026-11-05T00:00:00Z',
	scheduled_at: null,
	sent_at: null,
	ab_test_config: null,
	total_recipients: 0,
	sent_count: 0,
	opened_count: 0,
	clicked_count: 0,
	bounced_count: 0,
	unsubscribed_count: 0,
	open_rate: null,
	click_rate: null,
	html_content: '<p>Stored source</p>',
	plain_text_content: null,
	preview_text: '',
	type: 'broadcast',
	tags: []
});
it('adapts actual native identities and saved creative while preserving unknown delivery measures', () => {
	const actual = emailCampaignEnvelope({ success: true, data: fixture() }, 9);
	expect(actual.id).toBe('9');
	expect(actual.template_id).toBe('3');
	expect(actual.created_by).toBeNull();
	expect(actual.html_content).toBe('<p>Stored source</p>');
	expect(actual.stats).toMatchObject({
		sent: 0,
		open_rate: null,
		click_rate: null,
		delivered: null,
		complained: null
	});
	const page = emailCampaignPage({
		success: true,
		data: [fixture()],
		meta: { total: 21, current_page: 2, last_page: 2, per_page: 20 }
	});
	expect(page.page).toBe(2);
	expect(page.total).toBe(21);
	expect(page.per_page).toBe(20);
	expect(page.campaigns[0]?.id).toBe('9');
});
it('rejects wrong identities, unconfirmed mutations and missing native metadata', () => {
	expect(() => emailCampaignEnvelope({ success: true, data: fixture() }, 10)).toThrow(/identity/);
	expect(() =>
		emailCampaignEnvelope({ success: true, data: { ...fixture(), html_content: undefined } })
	).toThrow(/content/);
	expect(() =>
		emailCampaignPage({
			success: true,
			data: [fixture()],
			meta: { total: 1, current_page: 1, last_page: 1 }
		})
	).toThrow(/size/);
	expect(() => emailCampaignAction({ success: true, data: fixture() }, 9, 'submit')).toThrow(
		/confirmed/
	);
	expect(() => emailCampaignAction({ success: true, data: fixture() }, 9, 'pause')).toThrow(
		/confirmed/
	);
	expect(() => emailCampaignDeleted({ success: true }, 9)).toThrow(/confirmed/);
	for (const invalid of ['0', '01', '9/inject', '9007199254740992'])
		expect(() => campaignId(invalid)).toThrow(/identifier/);
});
it('writes native nullable references and requested times without forwarding read-only fields', () => {
	expect(
		emailCampaignWrite({
			name: 'Edited',
			template_id: '9',
			segment_id: null,
			send_at: null,
			ab_test_config: null
		})
	).toEqual({
		name: 'Edited',
		template_id: 9,
		segment_id: null,
		scheduled_at: null,
		ab_test_config: null
	});
	expect(() => emailCampaignWrite({ status: 'sent' })).toThrow(/delivery actions/);
	expect(() => emailCampaignWrite({})).toThrow(/authoring/);
});
it('rejects unsupported legacy partial-cohort experiments rather than silently sending the full audience', () => {
	expect(() =>
		emailCampaignWrite({
			subject: 'A',
			ab_test: {
				id: 'old-id',
				status: 'running',
				winning_metric: 'open_rate',
				test_percentage: 50,
				variants: [
					{ id: 'a', name: 'A', subject: 'A', allocation: 50 },
					{ id: 'b', name: 'B', subject: 'B', allocation: 50 }
				]
			}
		})
	).toThrow(/subject A\/B/);
	expect(emailCampaignWrite({ ab_test: null })).toEqual({ ab_test_config: null });
});

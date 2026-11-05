import { expect, it } from 'vite-plus/test';
import { campaignForm, campaignPatch, campaignWrite, emptyCampaignForm } from './campaign-form.js';
import { campaignRecord } from './campaign-contract.js';
const original = () =>
	campaignRecord({
		id: 17,
		name: 'Saved',
		subject: 'Frozen subject',
		from_name: '',
		from_email: '',
		reply_to: null,
		template_id: 3,
		segment_id: 4,
		created_by: null,
		status: 'paused',
		created_at: '2026-11-05T00:00:00Z',
		updated_at: '2026-11-05T00:00:00Z',
		scheduled_at: '2026-11-06T12:34:56.123Z',
		planned_at: '2026-11-06T12:34:56.123Z',
		sent_at: null,
		ab_test_config: { enabled: true, subject_b: 'Frozen B', split_percentage: 40 },
		total_recipients: 2,
		sent_count: 1,
		opened_count: 0,
		clicked_count: 0,
		bounced_count: 0,
		unsubscribed_count: 0,
		open_rate: 0,
		click_rate: 0
	});
it('keeps unchanged references and date precision out of an edit request', () => {
	const saved = original(),
		form = campaignForm(saved);
	expect(campaignPatch(saved, form)).toEqual({});
	form.name = 'Renamed';
	expect(campaignPatch(saved, form)).toEqual({ name: 'Renamed' });
	expect(campaignWrite(form).scheduled_at).toBe(saved.planned_at);
});
it('sends deliberate reference and time clearing without replacing unrelated creative', () => {
	const saved = original(),
		form = campaignForm(saved);
	form.templateId = '9';
	form.segmentId = '';
	form.scheduledFor = '';
	expect(campaignPatch(saved, form)).toEqual({
		template_id: 9,
		segment_id: null,
		scheduled_at: null
	});
	form.useABTest = false;
	expect(campaignPatch(saved, form).ab_test_config).toBeNull();
});
it('rejects unsafe reference precision and invalid requested dates before a write', () => {
	const form = emptyCampaignForm();
	form.templateId = '9007199254740992';
	expect(() => campaignWrite(form)).toThrow(/reference/);
	form.templateId = '1';
	form.scheduledFor = 'invalid';
	expect(() => campaignWrite(form)).toThrow(/time/);
});

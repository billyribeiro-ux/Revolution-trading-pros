import { expect, it } from 'vite-plus/test';
import {
	campaignDeleted,
	campaignEnvelope,
	campaignPage,
	campaignRecord,
	campaignStats
} from '../campaign-contract.js';
const fixture = () => ({
	id: 42,
	name: 'Persisted draft',
	subject: 'Actual subject',
	from_name: '',
	from_email: '',
	reply_to: null,
	template_id: null,
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
	click_rate: null
});
it('preserves unmeasured rates and unknown creator instead of inventing them', () => {
	const draft = campaignRecord(fixture());
	expect(draft.open_rate).toBeNull();
	expect(draft.click_rate).toBeNull();
	expect(draft.created_by).toBeNull();
});
it('rejects unsuccessful or malformed success responses and wrong mutation identities', () => {
	expect(() => campaignEnvelope({ success: false, data: fixture() })).toThrow('not confirmed');
	expect(() => campaignEnvelope({ success: true, data: fixture() }, 43)).toThrow('identity');
	for (const patch of [
		{ id: Number.MAX_SAFE_INTEGER + 1 },
		{ sent_count: -1 },
		{ status: 'success' },
		{ open_rate: '30' },
		{ ab_test_config: { enabled: true, variant_a: { subject: 'A', open_rate: '30' } } }
	])
		expect(() => campaignRecord({ ...fixture(), ...patch })).toThrow();
});
it('requires actual pagination and identity-specific delete acknowledgments', () => {
	expect(() => campaignPage({ success: true, data: [fixture()], meta: { total: 1 } })).toThrow(
		'pagination'
	);
	expect(
		campaignPage({
			success: true,
			data: [fixture()],
			meta: { total: 21, current_page: 2, last_page: 2 }
		}).meta
	).toEqual({ total: 21, current_page: 2, last_page: 2 });
	expect(() => campaignDeleted({ success: true }, 42)).toThrow('not confirmed');
	expect(() => campaignDeleted({ success: true, deleted_id: 43 }, 42)).toThrow('not confirmed');
	expect(() => campaignDeleted({ success: true, deleted_id: 42 }, 42)).not.toThrow();
});
it('rejects missing or invalid statistics while preserving an unmeasured rate', () => {
	const stats = {
		total_campaigns: 1,
		total_sent: 0,
		total_opened: 0,
		total_clicked: 0,
		total_bounced: 0,
		total_unsubscribed: 0,
		drafts: 1,
		scheduled: 0,
		sending: 0,
		sent: 0,
		avg_open_rate: null,
		avg_click_rate: null,
		avg_bounce_rate: null,
		recent: { campaigns: 1, sent: 0, opened: 0, clicked: 0 }
	};
	expect(campaignStats({ success: true, data: stats }).avg_open_rate).toBeNull();
	expect(() =>
		campaignStats({ success: true, data: { ...stats, total_sent: undefined } })
	).toThrow();
	expect(() => campaignStats({ success: true, data: { ...stats, avg_open_rate: 101 } })).toThrow();
});

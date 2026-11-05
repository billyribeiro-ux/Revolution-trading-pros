import { expect, it } from 'vite-plus/test';
import { campaignTestEnvelope, campaignTestHistory } from '../campaign-test.js';
const requestId = '34ad48b0-615b-49e8-86ee-c0d9f77206ee';
const fixture = () => ({
	success: true,
	data: {
		campaign_id: 42,
		metrics_scope: 'native_campaign_test',
		request_id: requestId,
		authoring_version: 3,
		variant: 'b',
		created_at: '2026-11-05T00:00:00Z',
		submissions: [
			{
				id: 12,
				recipient: 'sample@example.test',
				provider: 'smtp',
				provider_message_id: '<actual@example.test>',
				status: 'accepted',
				error: null,
				queued_at: '2026-11-05T00:00:00Z',
				accepted_at: '2026-11-05T00:00:01Z'
			}
		]
	}
});
it('requires the actual request, campaign, recipients and provider acknowledgment', () => {
	expect(campaignTestEnvelope(fixture(), 42, requestId, ['SAMPLE@example.test']).variant).toBe('b');
	expect(() => campaignTestEnvelope(fixture(), 43)).toThrow('identity');
	expect(() => campaignTestEnvelope(fixture(), 42, 'different')).toThrow('identity');
	expect(() => campaignTestEnvelope(fixture(), 42, requestId, ['other@example.test'])).toThrow(
		'recipients'
	);
	for (const provider_message_id of [null, '', ' ']) {
		const value = fixture();
		expect(() =>
			campaignTestEnvelope(
				{
					...value,
					data: {
						...value.data,
						submissions: [{ ...value.data.submissions[0], provider_message_id }]
					}
				},
				42
			)
		).toThrow('unconfirmed');
	}
});
it('preserves rejected, unknown and processing outcomes without reporting successful delivery', () => {
	for (const status of ['rejected', 'unknown', 'processing', 'unavailable']) {
		const value = fixture();
		const batch = {
			...value.data,
			submissions: [
				{ ...value.data.submissions[0], status, provider_message_id: null, accepted_at: null }
			]
		};
		expect(campaignTestEnvelope({ success: false, data: batch }, 42).submissions[0].status).toBe(
			status
		);
		expect(() => campaignTestEnvelope({ success: true, data: batch }, 42)).toThrow('success');
	}
});
it('rejects incomplete, duplicate or unrelated receipt history and permits a true empty history', () => {
	const value = fixture();
	const history = (requests: unknown[]) => ({
		success: true,
		data: { campaign_id: 42, metrics_scope: 'native_campaign_test', requests }
	});
	expect(campaignTestHistory(history([]), 42)).toEqual([]);
	expect(campaignTestHistory(history([value.data]), 42, requestId)).toHaveLength(1);
	expect(() => campaignTestHistory(history([value.data, value.data]), 42)).toThrow('Duplicate');
	expect(() => campaignTestHistory(history([{ ...value.data, submissions: [] }]), 42)).toThrow();
	expect(() =>
		campaignTestHistory(history([{ ...value.data, metrics_scope: 'recorded_outbox' }]), 42)
	).toThrow('scope');
});

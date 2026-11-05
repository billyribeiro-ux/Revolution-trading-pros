import { afterEach, expect, it, vi } from 'vite-plus/test';
import { sequenceTestEnvelope, sequenceTestHistory, sendSequenceTest } from '../sequence-test.js';
const intent = {
	email: 'sample@example.test',
	email_id: '18afd4a8-d2eb-46c7-852c-6b788b414dec',
	request_id: '34ad48b0-615b-49e8-86ee-c0d9f77206ee',
	expected_version: 3
};
const fixture = () => ({
	success: true,
	data: {
		sequence_id: '42',
		metrics_scope: 'sequence_test',
		email_id: intent.email_id,
		request_id: intent.request_id,
		authoring_version: 3,
		created_at: '2026-11-05T00:00:00Z',
		submissions: [
			{
				id: 12,
				recipient: intent.email,
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
afterEach(() => vi.unstubAllGlobals());
it('requires exact sequence, email, request, version and recipient identities plus actual acceptance', () => {
	expect(sequenceTestEnvelope(fixture(), '42', intent).submissions[0]?.status).toBe('accepted');
	for (const patch of [
		{ email_id: intent.request_id },
		{ request_id: intent.email_id },
		{ expected_version: 4 },
		{ email: 'other@example.test' }
	])
		expect(() => sequenceTestEnvelope(fixture(), '42', { ...intent, ...patch })).toThrow();
	expect(() => sequenceTestEnvelope(fixture(), '43')).toThrow('identity');
	const value = fixture();
	expect(() =>
		sequenceTestEnvelope(
			{
				...value,
				data: {
					...value.data,
					submissions: [{ ...value.data.submissions[0], provider_message_id: null }]
				}
			},
			'42'
		)
	).toThrow('unconfirmed');
});
it('preserves each real non-accepted state and rejects contradictory success or unrelated history', () => {
	for (const status of ['rejected', 'unknown', 'unavailable', 'processing']) {
		const data = fixture().data,
			batch = {
				...data,
				submissions: [
					{ ...data.submissions[0], status, provider_message_id: null, accepted_at: null }
				]
			};
		expect(sequenceTestEnvelope({ success: false, data: batch }, '42').submissions[0]?.status).toBe(
			status
		);
		expect(() => sequenceTestEnvelope({ success: true, data: batch }, '42')).toThrow('success');
	}
	const history = (requests: unknown[]) => ({
		success: true,
		data: { sequence_id: '42', metrics_scope: 'sequence_test', requests }
	});
	expect(sequenceTestHistory(history([]), '42')).toEqual([]);
	expect(sequenceTestHistory(history([fixture().data]), '42', intent.request_id)).toHaveLength(1);
	expect(() => sequenceTestHistory(history([fixture().data, fixture().data]), '42')).toThrow(
		'history'
	);
	expect(() => sequenceTestHistory(history([fixture().data]), '42', intent.email_id)).toThrow(
		'history'
	);
});
it('sends once, preserves rejected receipts on HTTP502 and never retries ambiguous network failures', async () => {
	const data = fixture().data,
		fetcher = vi.fn().mockResolvedValue(
			new Response(
				JSON.stringify({
					success: false,
					data: {
						...data,
						submissions: [
							{
								...data.submissions[0],
								status: 'rejected',
								provider_message_id: null,
								accepted_at: null
							}
						]
					}
				}),
				{ status: 502 }
			)
		);
	vi.stubGlobal('fetch', fetcher);
	expect((await sendSequenceTest('42', intent)).submissions[0]?.status).toBe('rejected');
	expect(fetcher).toHaveBeenCalledTimes(1);
	expect(JSON.parse(fetcher.mock.calls[0]?.[1]?.body)).toEqual(intent);
	fetcher.mockReset().mockRejectedValue(new Error('Fixture network uncertainty'));
	await expect(sendSequenceTest('42', intent)).rejects.toThrow('uncertainty');
	expect(fetcher).toHaveBeenCalledTimes(1);
});

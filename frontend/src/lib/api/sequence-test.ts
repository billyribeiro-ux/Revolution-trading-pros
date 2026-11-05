import { sequenceId, sequenceEmailId } from './email-sequence-catalog.js';
import type { CampaignTestReceipt } from './campaign-test.js';

/** Explicit QA submissions never enroll contacts or enter sequence delivery metrics. */
export interface SequenceTestBatch {
	sequence_id: string;
	email_id: string;
	request_id: string;
	authoring_version: number;
	metrics_scope: 'sequence_test';
	created_at: string;
	submissions: CampaignTestReceipt[];
}
export interface SequenceTestIntent {
	email: string;
	email_id: string;
	request_id: string;
	expected_version: number;
}
function object(value: unknown): value is Record<string, unknown> {
	return value !== null && typeof value === 'object' && !Array.isArray(value);
}
function positive(value: unknown): value is number {
	return typeof value === 'number' && Number.isSafeInteger(value) && value > 0;
}
function timestamp(value: unknown): value is string {
	return typeof value === 'string' && Number.isFinite(Date.parse(value));
}
export function sequenceTestBatch(
	value: unknown,
	id: string,
	intent?: SequenceTestIntent
): SequenceTestBatch {
	id = sequenceId(id);
	if (
		!object(value) ||
		value.sequence_id !== id ||
		value.metrics_scope !== 'sequence_test' ||
		!positive(value.authoring_version) ||
		!timestamp(value.created_at) ||
		!Array.isArray(value.submissions) ||
		value.submissions.length !== 1
	)
		throw new Error('Invalid sequence test identity or scope');
	const emailId = sequenceEmailId(value.email_id),
		requestId = sequenceEmailId(value.request_id);
	if (
		intent &&
		(emailId !== sequenceEmailId(intent.email_id) ||
			requestId !== sequenceEmailId(intent.request_id) ||
			value.authoring_version !== intent.expected_version)
	)
		throw new Error('Sequence test does not match the submitted intent');
	const receipt: unknown = value.submissions[0];
	if (
		!object(receipt) ||
		!positive(receipt.id) ||
		typeof receipt.recipient !== 'string' ||
		!/^[^\s@]+@[^\s@]+$/.test(receipt.recipient) ||
		!['smtp', 'postmark'].includes(String(receipt.provider)) ||
		!['accepted', 'rejected', 'unavailable', 'unknown', 'processing'].includes(
			String(receipt.status)
		) ||
		!timestamp(receipt.queued_at) ||
		(receipt.accepted_at !== null && !timestamp(receipt.accepted_at)) ||
		(receipt.error !== null && typeof receipt.error !== 'string') ||
		(receipt.provider_message_id !== null &&
			(typeof receipt.provider_message_id !== 'string' || !receipt.provider_message_id.trim())) ||
		(receipt.status === 'accepted' && (!receipt.provider_message_id || !receipt.accepted_at))
	)
		throw new Error('Invalid sequence test receipt; acceptance is unconfirmed');
	if (intent && receipt.recipient !== intent.email.trim().toLowerCase())
		throw new Error('Sequence test recipient does not match');
	return { ...value, email_id: emailId, request_id: requestId } as unknown as SequenceTestBatch;
}
export function sequenceTestEnvelope(
	value: unknown,
	id: string,
	intent?: SequenceTestIntent
): SequenceTestBatch {
	if (!object(value) || typeof value.success !== 'boolean')
		throw new Error('Sequence test outcome is unconfirmed');
	const batch = sequenceTestBatch(value.data, id, intent);
	if (value.success !== batch.submissions.every((receipt) => receipt.status === 'accepted'))
		throw new Error('Sequence test success does not match its receipt');
	return batch;
}
export function sequenceTestHistory(
	value: unknown,
	id: string,
	requestId?: string
): SequenceTestBatch[] {
	if (
		!object(value) ||
		value.success !== true ||
		!object(value.data) ||
		value.data.sequence_id !== id ||
		value.data.metrics_scope !== 'sequence_test' ||
		!Array.isArray(value.data.requests) ||
		value.data.requests.length > 20
	)
		throw new Error('Sequence test history is unavailable');
	const batches = value.data.requests.map((value) => sequenceTestBatch(value, id));
	if (
		batches.some(
			(batch, index) =>
				(requestId !== undefined && batch.request_id !== sequenceEmailId(requestId)) ||
				batches.findIndex((other) => other.request_id === batch.request_id) !== index
		) ||
		new Set(batches.flatMap((batch) => batch.submissions.map((receipt) => receipt.id))).size !==
			batches.length
	)
		throw new Error('Invalid sequence test request in history');
	return batches;
}
function failure(value: unknown, fallback: string): Error {
	if (object(value)) {
		if (typeof value.message === 'string') return new Error(value.message);
		if (typeof value.error === 'string') return new Error(value.error);
	}
	return new Error(fallback);
}
export async function getSequenceTestHistory(
	id: string,
	requestId?: string
): Promise<SequenceTestBatch[]> {
	id = sequenceId(id);
	const query = requestId ? `?request_id=${sequenceEmailId(requestId)}` : '';
	const response = await fetch(`/api/admin/crm/sequences/${id}/tests${query}`, {
		credentials: 'include',
		cache: 'no-store'
	});
	const value: unknown = await response.json().catch(() => null);
	if (!response.ok) throw failure(value, 'Sequence test history is unavailable');
	return sequenceTestHistory(value, id, requestId);
}
export async function sendSequenceTest(
	id: string,
	intent: SequenceTestIntent
): Promise<SequenceTestBatch> {
	id = sequenceId(id);
	if (!positive(intent.expected_version) || !/^[^\s@]+@[^\s@]+$/.test(intent.email.trim()))
		throw new Error('Specify a saved version and an explicit test recipient');
	const body = {
		...intent,
		email: intent.email.trim().toLowerCase(),
		email_id: sequenceEmailId(intent.email_id),
		request_id: sequenceEmailId(intent.request_id)
	};
	// One explicit request. A replay uses exactly the same durable intent.
	const response = await fetch(`/api/admin/crm/sequences/${id}/test`, {
		method: 'POST',
		credentials: 'include',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(body)
	});
	const value: unknown = await response.json().catch(() => null);
	if (![200, 202, 502, 503].includes(response.status) || !object(value) || !('data' in value))
		throw failure(value, 'Sequence test outcome is unconfirmed; check saved receipts');
	return sequenceTestEnvelope(value, id, body);
}

/** Actual QA receipts, separate from campaign audience/delivery metrics. */
export interface CampaignTestReceipt {
	id: number;
	recipient: string;
	provider: string;
	provider_message_id: string | null;
	status: 'accepted' | 'rejected' | 'unavailable' | 'unknown' | 'processing';
	error: string | null;
	queued_at: string;
	accepted_at: string | null;
}
export interface CampaignTestBatch {
	campaign_id: number;
	metrics_scope: 'native_campaign_test';
	request_id: string;
	authoring_version: number;
	variant: 'a' | 'b';
	created_at: string;
	submissions: CampaignTestReceipt[];
}
export interface CampaignTestOptions {
	request_id?: string;
	variant?: 'a' | 'b';
	expected_version?: number;
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
function uuid(value: unknown): value is string {
	return (
		typeof value === 'string' &&
		/^[0-9a-f]{8}(?:-[0-9a-f]{4}){3}-[0-9a-f]{12}$/i.test(value) &&
		!/^0{8}(?:-0{4}){3}-0{12}$/.test(value)
	);
}
export function campaignTestBatch(
	value: unknown,
	campaignId: number,
	requestId?: string,
	recipients?: string[]
): CampaignTestBatch {
	if (
		!object(value) ||
		value.campaign_id !== campaignId ||
		value.metrics_scope !== 'native_campaign_test' ||
		!uuid(value.request_id) ||
		(requestId !== undefined && value.request_id !== requestId) ||
		!positive(value.authoring_version) ||
		!['a', 'b'].includes(String(value.variant)) ||
		!timestamp(value.created_at) ||
		!Array.isArray(value.submissions) ||
		value.submissions.length < 1 ||
		value.submissions.length > 5
	)
		throw new Error('Invalid campaign test request identity or scope');
	const seen: string[] = [];
	const ids: number[] = [];
	for (const receipt of value.submissions) {
		if (
			!object(receipt) ||
			!positive(receipt.id) ||
			ids.includes(receipt.id) ||
			typeof receipt.recipient !== 'string' ||
			!/^[^\s@]+@[^\s@]+$/.test(receipt.recipient) ||
			seen.includes(receipt.recipient) ||
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
			throw new Error('Invalid campaign test receipt; provider acceptance is unconfirmed');
		seen.push(receipt.recipient);
		ids.push(receipt.id);
	}
	if (
		recipients &&
		(recipients.length !== seen.length ||
			recipients.some((email) => !seen.includes(email.trim().toLowerCase())))
	)
		throw new Error('Campaign test receipt recipients do not match');
	return value as unknown as CampaignTestBatch;
}
export function campaignTestEnvelope(
	value: unknown,
	campaignId: number,
	requestId?: string,
	recipients?: string[]
): CampaignTestBatch {
	if (!object(value) || typeof value.success !== 'boolean')
		throw new Error('Campaign test outcome is unconfirmed');
	const batch = campaignTestBatch(value.data, campaignId, requestId, recipients);
	if (value.success !== batch.submissions.every((receipt) => receipt.status === 'accepted'))
		throw new Error('Campaign test success does not match its receipts');
	return batch;
}
export function campaignTestHistory(
	value: unknown,
	campaignId: number,
	requestId?: string
): CampaignTestBatch[] {
	if (
		!object(value) ||
		value.success !== true ||
		!object(value.data) ||
		value.data.campaign_id !== campaignId ||
		value.data.metrics_scope !== 'native_campaign_test' ||
		!Array.isArray(value.data.requests) ||
		value.data.requests.length > 20
	)
		throw new Error('Campaign test history is unavailable');
	const requests = value.data.requests.map((batch) =>
		campaignTestBatch(batch, campaignId, requestId)
	);
	if (
		requests.some(
			(batch, index) =>
				requests.findIndex((other) => other.request_id === batch.request_id) !== index
		)
	)
		throw new Error('Duplicate campaign test request in history');
	return requests;
}

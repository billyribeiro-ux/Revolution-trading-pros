/** Adapt the secondary dashboard's string identities to the native campaign API. */
import { campaignReport } from './campaign-report.js';
import type { RecordedCampaignEmailStats } from './email.js';
import type { EmailCampaign } from './email.js';
import {
	campaignEnvelope,
	campaignPage,
	campaignRecord,
	campaignDeleted
} from './campaign-contract.js';
function record(value: unknown): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		throw new Error('Invalid email campaign response');
	return value as Record<string, unknown>;
}
export function campaignId(value: unknown): number {
	if (
		!['string', 'number'].includes(typeof value) ||
		!/^[1-9]\d*$/.test(String(value)) ||
		!Number.isSafeInteger(Number(value))
	)
		throw new Error('Invalid campaign identifier');
	return Number(value);
}
function emailCampaign(value: unknown): EmailCampaign {
	const native = campaignRecord(value),
		row = record(value);
	if (row.metrics_scope !== 'recorded_outbox')
		throw new Error('Invalid campaign measurement scope');
	if (
		typeof row.html_content !== 'string' ||
		(row.plain_text_content !== null && typeof row.plain_text_content !== 'string') ||
		typeof row.preview_text !== 'string' ||
		!['broadcast', 'automation', 'sequence', 'transactional'].includes(String(row.type)) ||
		!Array.isArray(row.tags) ||
		!row.tags.every((tag: unknown) => typeof tag === 'string')
	)
		throw new Error('Invalid saved campaign content');
	const sent = native.sent_count,
		delivered = native.delivery_stats?.delivered ?? null,
		complained = native.delivery_stats?.complained ?? null;
	return {
		id: String(native.id),
		name: native.name,
		subject: native.subject,
		from_name: native.from_name,
		from_email: native.from_email,
		reply_to: native.reply_to,
		template_id: native.template_id === null ? null : String(native.template_id),
		segment_id: native.segment_id === null ? null : String(native.segment_id),
		html_content: row.html_content,
		plain_text_content: row.plain_text_content as string | null,
		preview_text: row.preview_text,
		status: native.status,
		type: row.type as EmailCampaign['type'],
		tags: row.tags as string[],
		send_at: native.planned_at ?? native.scheduled_at,
		sent_at: native.sent_at,
		created_by: native.created_by,
		created_at: native.created_at,
		updated_at: native.updated_at,
		ab_test_config: native.ab_test_config,
		stats: {
			sent,
			delivered,
			opened: native.opened_count,
			clicked: native.clicked_count,
			bounced: native.bounced_count,
			unsubscribed: native.unsubscribed_count,
			complained,
			open_rate: native.open_rate ?? null,
			click_rate: native.click_rate ?? null,
			bounce_rate: sent > 0 ? (100 * native.bounced_count) / sent : null,
			complaint_rate: sent > 0 && complained !== null ? (100 * complained) / sent : null
		}
	};
}
export function emailCampaignEnvelope(value: unknown, expected?: number): EmailCampaign {
	campaignEnvelope(value, expected);
	return emailCampaign(record(value).data);
}
export function emailCampaignPage(value: unknown) {
	const native = campaignPage(value),
		raw = record(value),
		meta = record(raw.meta);
	if (
		typeof meta.per_page !== 'number' ||
		!Number.isSafeInteger(meta.per_page) ||
		meta.per_page < 1 ||
		meta.per_page > 100
	)
		throw new Error('Invalid email campaign page size');
	if (!Array.isArray(raw.data)) throw new Error('Invalid email campaign list');
	return {
		campaigns: raw.data.map(emailCampaign),
		total: native.meta.total,
		page: native.meta.current_page,
		per_page: meta.per_page
	};
}
export function emailCampaignDeleted(value: unknown, id: number) {
	campaignDeleted(value, id);
}
export function emailCampaignAction(value: unknown, id: number, action: 'submit' | 'pause') {
	const saved = campaignEnvelope(value, id);
	if (
		(action === 'pause' && saved.status !== 'paused') ||
		(action === 'submit' &&
			(saved.total_recipients < 1 ||
				!['scheduled', 'sending', 'sent', 'paused', 'failed'].includes(saved.status)))
	)
		throw new Error('Campaign action was not confirmed');
}
export function emailCampaignWrite(data: Partial<EmailCampaign>): Record<string, unknown> {
	const out: Record<string, unknown> = {};
	for (const key of [
		'id',
		'status',
		'stats',
		'created_by',
		'created_at',
		'updated_at',
		'sent_at'
	] as const)
		if (data[key] !== undefined)
			throw new Error('Send authoring fields only; use campaign delivery actions to change status');
	for (const key of [
		'name',
		'subject',
		'preview_text',
		'from_name',
		'from_email',
		'reply_to',
		'html_content',
		'plain_text_content',
		'type',
		'tags',
		'ab_test_config'
	] as const)
		if (data[key] !== undefined) out[key] = data[key];
	for (const key of ['template_id', 'segment_id'] as const)
		if (data[key] !== undefined)
			out[key] = data[key] === null || data[key] === '' ? null : campaignId(data[key]);
	if (data.send_at !== undefined) out.scheduled_at = data.send_at;
	if (data.ab_test !== undefined) {
		if (data.ab_test_config !== undefined)
			throw new Error('Provide one campaign A/B configuration');
		const ab = data.ab_test;
		if (ab === null) {
			out.ab_test_config = null;
			return out;
		}
		if (
			ab.test_percentage !== 100 ||
			ab.winning_metric === 'conversion_rate' ||
			ab.variants.length !== 2 ||
			ab.status !== 'running' ||
			ab.winner_id ||
			ab.variants.some((v) => v.from_name !== undefined || v.html_content !== undefined) ||
			ab.variants[0]?.subject !== data.subject
		)
			throw new Error('Use the saved subject A/B configuration for this campaign');
		out.ab_test_config = {
			enabled: true,
			subject_b: ab.variants[1]?.subject,
			split_percentage: ab.variants[0]?.allocation
		};
	}
	if (!Object.keys(out).length) throw new Error('Specify campaign authoring changes');
	return out;
}

export function emailCampaignReportStats(value: unknown, id: number): RecordedCampaignEmailStats {
	const report = campaignReport(value, id),
		m = report.metrics,
		r = report.rates;
	return {
		sent: m.sent,
		delivered: m.delivered,
		opened: m.unique_opens,
		clicked: m.unique_clicks,
		bounced: m.bounced,
		complained: m.complained,
		unsubscribed: m.unsubscribed,
		open_rate: r.unique_open_rate,
		click_rate: r.unique_click_rate,
		bounce_rate: r.bounce_rate,
		complaint_rate: m.sent > 0 ? (100 * m.complained) / m.sent : null
	};
}

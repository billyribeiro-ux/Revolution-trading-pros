import type { Campaign, CampaignStats } from './campaigns.js';
function object(value: unknown): value is Record<string, unknown> {
	return value !== null && typeof value === 'object' && !Array.isArray(value);
}
function count(value: unknown, min = 0): value is number {
	return typeof value === 'number' && Number.isSafeInteger(value) && value >= min;
}
function rate(value: unknown) {
	return (
		value === null ||
		(typeof value === 'number' && Number.isFinite(value) && value >= 0 && value <= 100)
	);
}
function successful(value: unknown): Record<string, unknown> {
	if (!object(value) || value.success !== true || value.error)
		throw new Error('Campaign operation was not confirmed');
	return value;
}
export function campaignRecord(value: unknown): Campaign {
	if (
		!object(value) ||
		!count(value.id, 1) ||
		![
			'draft',
			'scheduled',
			'sending',
			'sent',
			'paused',
			'failed',
			'cancelled',
			'archived'
		].includes(String(value.status))
	)
		throw new Error('Invalid campaign identity or status');
	for (const key of ['name', 'subject', 'from_name', 'from_email'] as const)
		if (typeof value[key] !== 'string') throw new Error('Invalid campaign authoring response');
	for (const key of ['template_id', 'segment_id', 'created_by'] as const)
		if (value[key] !== null && !count(value[key], 1)) throw new Error('Invalid campaign reference');
	for (const key of [
		'total_recipients',
		'sent_count',
		'opened_count',
		'clicked_count',
		'bounced_count',
		'unsubscribed_count'
	] as const)
		if (!count(value[key])) throw new Error('Invalid recorded campaign counts');
	for (const key of ['created_at', 'updated_at'] as const)
		if (typeof value[key] !== 'string' || !Number.isFinite(Date.parse(value[key])))
			throw new Error('Invalid campaign timestamps');
	for (const key of ['scheduled_at', 'sent_at'] as const)
		if (
			value[key] !== null &&
			(typeof value[key] !== 'string' || !Number.isFinite(Date.parse(value[key])))
		)
			throw new Error('Invalid campaign timestamps');
	if (value.reply_to !== null && typeof value.reply_to !== 'string')
		throw new Error('Invalid campaign reply-to');
	for (const key of ['open_rate', 'click_rate', 'bounce_rate', 'unsubscribe_rate'] as const)
		if (value[key] !== undefined && !rate(value[key])) throw new Error('Invalid campaign rate');
	if (value.ab_test_config !== null) {
		const ab = value.ab_test_config;
		if (
			!object(ab) ||
			typeof ab.enabled !== 'boolean' ||
			(ab.subject_b != null && typeof ab.subject_b !== 'string') ||
			(ab.split_percentage != null &&
				(!count(ab.split_percentage, 1) || ab.split_percentage > 99)) ||
			(ab.winner != null && ab.winner !== 'a' && ab.winner !== 'b')
		)
			throw new Error('Invalid campaign A/B configuration');
		for (const key of ['variant_a', 'variant_b'] as const)
			if (
				ab[key] != null &&
				(!object(ab[key]) || typeof ab[key].subject !== 'string' || !rate(ab[key].open_rate))
			)
				throw new Error('Invalid campaign variant measurements');
	}
	for (const key of ['template', 'creator'] as const)
		if (
			value[key] != null &&
			(!object(value[key]) || !count(value[key].id, 1) || typeof value[key].name !== 'string')
		)
			throw new Error('Invalid campaign relation');
	return value as unknown as Campaign;
}
export function campaignEnvelope(value: unknown, expected?: number): Campaign {
	const record = campaignRecord(successful(value).data);
	if (expected !== undefined && record.id !== expected)
		throw new Error('Campaign response identity does not match the requested record');
	return record;
}
export function campaignPage(value: unknown) {
	const page = successful(value),
		meta = page.meta;
	if (
		!Array.isArray(page.data) ||
		!object(meta) ||
		!count(meta.total) ||
		!count(meta.current_page, 1) ||
		!count(meta.last_page, 1)
	)
		throw new Error('Invalid campaign pagination');
	return {
		data: page.data.map(campaignRecord),
		meta: { total: meta.total, current_page: meta.current_page, last_page: meta.last_page }
	};
}
export function campaignStats(value: unknown): CampaignStats {
	const stats = successful(value).data;
	if (!object(stats) || !object(stats.recent)) throw new Error('Invalid campaign statistics');
	for (const key of [
		'total_campaigns',
		'total_sent',
		'total_opened',
		'total_clicked',
		'total_bounced',
		'total_unsubscribed',
		'drafts',
		'scheduled',
		'sending',
		'sent'
	] as const)
		if (!count(stats[key])) throw new Error('Invalid recorded campaign statistics');
	for (const key of ['avg_open_rate', 'avg_click_rate', 'avg_bounce_rate'] as const)
		if (!rate(stats[key])) throw new Error('Invalid campaign rate');
	for (const key of ['campaigns', 'sent', 'opened', 'clicked'] as const)
		if (!count(stats.recent[key])) throw new Error('Invalid recent campaign statistics');
	return stats as unknown as CampaignStats;
}
export function campaignDeleted(value: unknown, id: number): void {
	if (successful(value).deleted_id !== id) throw new Error('Campaign deletion was not confirmed');
}

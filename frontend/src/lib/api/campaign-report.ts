export interface CampaignReport {
	metrics_scope: 'recorded_outbox';
	last_error: string | null;
	pause_reason: string | null;
	delivery_stats: Record<
		'total' | 'queued' | 'processing' | 'failed' | 'unknown' | 'skipped',
		number
	>;
	id: number;
	name: string;
	subject: string;
	status: string;
	sent_at: string | null;
	scheduled_at: string | null;
	created_at: string;
	segment_name: string | null;
	template_name: string | null;
	metrics: {
		sent: number;
		delivered: number;
		opened: number;
		unique_opens: number;
		clicked: number;
		unique_clicks: number;
		bounced: number;
		soft_bounced: number | null;
		hard_bounced: number | null;
		unsubscribed: number;
		complained: number;
	};
	rates: {
		delivery_rate: number | null;
		open_rate: number | null;
		unique_open_rate: number | null;
		click_rate: number | null;
		unique_click_rate: number | null;
		click_to_open_rate: number | null;
		bounce_rate: number | null;
		unsubscribe_rate: number | null;
	};
	engagement_over_time: Array<{
		hour: number;
		opens: number;
		clicks: number;
	}>;
	top_links: Array<{
		url: string;
		clicks: number;
		unique_clicks: number;
	}>;
	device_breakdown: {
		desktop: number;
		mobile: number;
		tablet: number;
		unknown: number;
	};
	email_client_breakdown: Array<{
		client: string;
		count: number;
		percentage: number;
	}>;
	geographic_distribution: Array<{
		country: string;
		country_code: string;
		opens: number;
		clicks: number;
	}>;
	ab_test_results: {
		enabled: boolean;
		winner: 'a' | 'b' | null;
		variant_a: {
			subject: string;
			sent: number;
			opens: number;
			open_rate: number | null;
			clicks: number;
			click_rate: number | null;
		};
		variant_b: {
			subject: string;
			sent: number;
			opens: number;
			open_rate: number | null;
			clicks: number;
			click_rate: number | null;
		};
	} | null;
}

function record(value: unknown): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		throw new Error('Invalid recorded campaign report');
	return value as Record<string, unknown>;
}
function count(value: unknown, minimum = 0): value is number {
	return typeof value === 'number' && Number.isSafeInteger(value) && value >= minimum;
}
function rate(value: unknown, limit = 100): boolean {
	return (
		value === null ||
		(typeof value === 'number' && Number.isFinite(value) && value >= 0 && value <= limit)
	);
}
function counts(value: unknown, keys: string[]) {
	const data = record(value);
	for (const key of keys)
		if (!count(data[key])) throw new Error('Invalid recorded campaign counts');
	return data;
}
function rows(value: unknown, max: number): Record<string, unknown>[] {
	if (!Array.isArray(value) || value.length > max)
		throw new Error('Invalid campaign report observations');
	return value.map(record);
}
export function campaignReport(value: unknown, expected: number): CampaignReport {
	const envelope = record(value);
	if (envelope.success !== true || envelope.error)
		throw new Error('Campaign report was not confirmed');
	const data = record(envelope.data);
	if (!count(data.id, 1) || data.id !== expected || data.metrics_scope !== 'recorded_outbox')
		throw new Error('Campaign report identity or measurement scope does not match');
	for (const key of ['name', 'subject', 'status', 'created_at'])
		if (typeof data[key] !== 'string') throw new Error('Invalid campaign report identity');
	if (!Number.isFinite(Date.parse(String(data.created_at))))
		throw new Error('Invalid campaign report timestamp');
	for (const key of ['sent_at', 'scheduled_at'])
		if (
			data[key] !== null &&
			(typeof data[key] !== 'string' || !Number.isFinite(Date.parse(data[key])))
		)
			throw new Error('Invalid campaign report timestamp');
	for (const key of ['segment_name', 'template_name', 'last_error', 'pause_reason'])
		if (data[key] !== null && typeof data[key] !== 'string')
			throw new Error('Invalid campaign report relation');
	counts(data.metrics, [
		'sent',
		'delivered',
		'opened',
		'unique_opens',
		'clicked',
		'unique_clicks',
		'bounced',
		'unsubscribed',
		'complained'
	]);
	const metrics = record(data.metrics);
	for (const key of ['soft_bounced', 'hard_bounced'])
		if (metrics[key] !== null && !count(metrics[key]))
			throw new Error('Invalid campaign bounce classification');
	counts(data.delivery_stats, ['total', 'queued', 'processing', 'failed', 'unknown', 'skipped']);
	const rates = record(data.rates);
	for (const key of [
		'delivery_rate',
		'open_rate',
		'unique_open_rate',
		'click_rate',
		'unique_click_rate',
		'click_to_open_rate',
		'bounce_rate',
		'unsubscribe_rate'
	])
		if (!rate(rates[key], key === 'click_to_open_rate' ? Infinity : 100))
			throw new Error('Invalid campaign report rate');
	const hours = [] as unknown[];
	for (const row of rows(data.engagement_over_time, 48)) {
		counts(row, ['hour', 'opens', 'clicks']);
		if (Number(row.hour) >= 48 || hours.includes(row.hour))
			throw new Error('Invalid campaign report timeline');
		hours.push(row.hour);
	}
	const urls = [] as unknown[];
	for (const row of rows(data.top_links, 100)) {
		counts(row, ['clicks', 'unique_clicks']);
		if (typeof row.url !== 'string' || urls.includes(row.url))
			throw new Error('Invalid campaign report link');
		const url = new URL(row.url);
		if (!['http:', 'https:'].includes(url.protocol) || url.username || url.password)
			throw new Error('Invalid campaign report link');
		urls.push(row.url);
	}
	counts(data.device_breakdown, ['desktop', 'mobile', 'tablet', 'unknown']);
	for (const row of rows(data.email_client_breakdown, 100)) {
		if (
			typeof row.client !== 'string' ||
			!count(row.count) ||
			typeof row.percentage !== 'number' ||
			!rate(row.percentage)
		)
			throw new Error('Invalid recorded email client');
	}
	for (const row of rows(data.geographic_distribution, 100)) {
		counts(row, ['opens', 'clicks']);
		if (
			typeof row.country !== 'string' ||
			typeof row.country_code !== 'string' ||
			!/^[A-Z]{2}$/.test(row.country_code)
		)
			throw new Error('Invalid recorded campaign geography');
	}
	if (data.ab_test_results !== null) {
		const ab = record(data.ab_test_results);
		if (ab.enabled !== true || ab.winner !== null)
			throw new Error('Invalid recorded campaign experiment');
		for (const key of ['variant_a', 'variant_b']) {
			const variant = counts(ab[key], ['sent', 'opens', 'clicks']);
			if (
				typeof variant.subject !== 'string' ||
				!rate(variant.open_rate) ||
				!rate(variant.click_rate)
			)
				throw new Error('Invalid recorded campaign variant');
		}
	}
	return data as unknown as CampaignReport;
}
export function formatCampaignRate(value: number | null, digits = 1): string {
	return value === null ? '—' : `${value.toFixed(digits)}%`;
}

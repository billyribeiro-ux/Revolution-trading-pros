import { SvelteDate } from 'svelte/reactivity';
import type { EmailAnalytics } from './email.js';
import type { CampaignAnalytics } from './campaigns.js';
function record(value: unknown): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		throw new Error('Invalid recorded email analytics');
	return value as Record<string, unknown>;
}
function count(value: unknown): value is number {
	return typeof value === 'number' && Number.isSafeInteger(value) && value >= 0;
}
function rate(value: unknown): boolean {
	return (
		value === null ||
		(typeof value === 'number' && Number.isFinite(value) && value >= 0 && value <= 100)
	);
}
function rows(value: unknown, max: number): Record<string, unknown>[] {
	if (!Array.isArray(value) || value.length > max)
		throw new Error('Invalid email analytics observations');
	return value.map(record);
}
function counts(value: unknown, keys: string[]): Record<string, unknown> {
	const data = record(value);
	for (const key of keys)
		if (!count(data[key])) throw new Error('Invalid recorded email analytics counts');
	return data;
}
function date(value: unknown): value is string {
	if (typeof value !== 'string' || !/^\d{4}-\d{2}-\d{2}$/.test(value)) return false;
	const timestamp = Date.parse(`${value}T00:00:00Z`);
	return (
		Number.isFinite(timestamp) && new SvelteDate(timestamp).toISOString().slice(0, 10) === value
	);
}
function windowOf(data: Record<string, unknown>) {
	const period = data.period;
	if (typeof period !== 'string' || !['7d', '30d', '90d', '365d'].includes(period))
		throw new Error('Invalid email reporting period');
	const window = record(data.window);
	if (window.timezone !== 'UTC') throw new Error('Invalid email reporting timezone');
	const stamps = ['start', 'end', 'previous_start'].map((key) =>
		typeof window[key] === 'string' ? Date.parse(window[key]) : NaN
	);
	if (
		stamps.some((value) => !Number.isFinite(value)) ||
		stamps[0] === undefined ||
		stamps[1] === undefined ||
		stamps[2] === undefined
	)
		throw new Error('Invalid email reporting timestamps');
	const duration = Number.parseInt(period) * 86400000;
	if (stamps[1] - stamps[0] !== duration || stamps[0] - stamps[2] !== duration)
		throw new Error('Invalid email reporting window');
	return window;
}
export function emailAnalytics(value: unknown): EmailAnalytics {
	const envelope = record(value);
	if (envelope.success !== true || envelope.error)
		throw new Error('Email analytics were not confirmed');
	const data = counts(envelope.analytics, [
		'campaigns',
		'new_campaigns',
		'emails_sent',
		'total_opens',
		'total_clicks',
		'unlisted_client_observations'
	]);
	if (data.metrics_scope !== 'native_campaign_outbox')
		throw new Error('Invalid email analytics scope');
	windowOf(data);
	for (const key of ['average_open_rate', 'average_click_rate'])
		if (!rate(data[key]) || (data.emails_sent === 0 && data[key] !== null))
			throw new Error('Invalid email acceptance cohort rate');
	const comparison = record(data.comparison);
	for (const key of [
		'emails_sent_change_pct',
		'open_rate_change_points',
		'click_rate_change_points'
	])
		if (
			comparison[key] !== null &&
			(typeof comparison[key] !== 'number' || !Number.isFinite(comparison[key]))
		)
			throw new Error('Invalid measured email comparison');
	const top = rows(data.top_campaigns, 10).map((row) => {
		counts(row, ['id', 'sent', 'opens']);
		if (
			Number(row.id) < 1 ||
			Number(row.sent) < 1 ||
			typeof row.name !== 'string' ||
			!rate(row.open_rate) ||
			row.open_rate === null
		)
			throw new Error('Invalid ranked campaign observations');
		return { ...row, id: String(row.id) };
	});
	let opens = 0,
		clicks = 0;
	const dates: string[] = [];
	for (const row of rows(data.engagement_trend, 366)) {
		if (!date(row.date) || dates.includes(row.date))
			throw new Error('Invalid email observation date');
		counts(row, ['opens', 'clicks']);
		dates.push(row.date);
		opens += Number(row.opens);
		clicks += Number(row.clicks);
	}
	if (opens !== data.total_opens || clicks !== data.total_clicks)
		throw new Error('Email observations do not match recorded totals');
	const devices = counts(data.device_breakdown, ['desktop', 'mobile', 'tablet', 'unknown']);
	if (
		Object.values(devices).reduce<number>((total, value) => total + Number(value), 0) !==
		data.total_opens
	)
		throw new Error('Device observations do not match recorded opens');
	const clients = record(data.client_breakdown);
	if (
		Object.keys(clients).length > 100 ||
		!Object.values(clients).every(count) ||
		Object.values(clients).reduce<number>((total, value) => total + Number(value), 0) +
			Number(data.unlisted_client_observations) !==
			data.total_opens
	)
		throw new Error('Client observations do not match recorded opens');
	return { ...data, top_campaigns: top } as unknown as EmailAnalytics;
}
export function campaignAnalytics(value: unknown, expected: number): CampaignAnalytics {
	const envelope = record(value);
	if (envelope.success !== true || envelope.error)
		throw new Error('Campaign analytics were not confirmed');
	const data = record(envelope.data),
		campaign = counts(data.campaign, [
			'id',
			'sent_count',
			'opened_count',
			'clicked_count',
			'bounced_count',
			'unsubscribed_count'
		]);
	if (
		campaign.id !== expected ||
		typeof campaign.name !== 'string' ||
		data.metrics_scope !== 'recorded_outbox'
	)
		throw new Error('Campaign analytics identity or scope does not match');
	for (const key of ['open_rate', 'click_rate', 'bounce_rate'])
		if (!rate(campaign[key]) || (campaign.sent_count === 0 && campaign[key] !== null))
			throw new Error('Invalid campaign analytics rate');
	windowOf(data);
	for (const row of rows(data.hourly_stats, 18000)) {
		counts(row, ['hour', 'count']);
		if (
			!date(row.date) ||
			Number(row.hour) > 23 ||
			!['opened', 'clicked'].includes(String(row.event_type))
		)
			throw new Error('Invalid hourly campaign observations');
	}
	const devices = rows(data.device_stats, 4);
	if (
		devices.length !== 4 ||
		!['desktop', 'mobile', 'tablet', 'unknown'].every(
			(kind) => devices.filter((row) => row.device_type === kind && count(row.count)).length === 1
		)
	)
		throw new Error('Invalid campaign device observations');
	for (const row of rows(data.top_links, 100)) {
		if (typeof row.link_url !== 'string' || !count(row.clicks))
			throw new Error('Invalid recorded campaign link');
		const url = new URL(row.link_url);
		if (!['http:', 'https:'].includes(url.protocol) || url.username || url.password)
			throw new Error('Invalid recorded campaign link');
	}
	for (const row of rows(data.geo_stats, 100))
		if (typeof row.country !== 'string' || !count(row.count))
			throw new Error('Invalid campaign geography');
	return data as unknown as CampaignAnalytics;
}

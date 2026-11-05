import { expect, it } from 'vite-plus/test';
import { emailAnalytics, campaignAnalytics } from '../email-analytics.js';
const window = {
	start: '2026-10-06T00:00:00Z',
	end: '2026-11-05T00:00:00Z',
	previous_start: '2026-09-06T00:00:00Z',
	timezone: 'UTC'
};
const fixture = () => ({
	success: true,
	analytics: {
		metrics_scope: 'native_campaign_outbox',
		period: '30d',
		window,
		campaigns: 0,
		new_campaigns: 1,
		emails_sent: 2,
		total_opens: 2,
		total_clicks: 1,
		average_open_rate: 50,
		average_click_rate: 50,
		comparison: {
			emails_sent_change_pct: null,
			open_rate_change_points: null,
			click_rate_change_points: null
		},
		top_campaigns: [{ id: 9, name: 'Actual campaign', sent: 2, opens: 1, open_rate: 50 }],
		engagement_trend: [{ date: '2026-11-04', opens: 2, clicks: 1 }],
		device_breakdown: { desktop: 0, mobile: 2, tablet: 0, unknown: 0 },
		client_breakdown: { 'Fixture Mail': 2 },
		unlisted_client_observations: 0
	}
});
it('preserves recorded signal totals, unique cohort rates, unknown comparisons and actual campaign identities', () => {
	const actual = emailAnalytics(fixture());
	expect(actual.total_opens).toBe(2);
	expect(actual.average_open_rate).toBe(50);
	expect(actual.comparison.open_rate_change_points).toBeNull();
	expect(actual.top_campaigns[0]?.id).toBe('9');
	const empty = {
		...fixture(),
		analytics: {
			...fixture().analytics,
			emails_sent: 0,
			average_open_rate: null,
			average_click_rate: null,
			top_campaigns: []
		}
	};
	expect(emailAnalytics(empty).average_open_rate).toBeNull();
});
it('rejects malformed successes, fabricated scopes and observations that do not match their totals', () => {
	for (const patch of [
		{ metrics_scope: 'historical' },
		{ average_open_rate: 101 },
		{ total_opens: 3 },
		{ device_breakdown: { desktop: 0, mobile: 0, tablet: 0, unknown: 0 } },
		{ unlisted_client_observations: 1 }
	])
		expect(() =>
			emailAnalytics({ ...fixture(), analytics: { ...fixture().analytics, ...patch } })
		).toThrow();
	expect(() => emailAnalytics({ success: true, analytics: {} })).toThrow();
});
it('rejects local-calendar intervals that drift by an hour across daylight-saving transitions', () => {
	expect(() =>
		emailAnalytics({
			...fixture(),
			analytics: {
				...fixture().analytics,
				window: {
					...window,
					start: '2026-10-05T23:00:00-04:00',
					end: '2026-11-04T23:00:00-05:00',
					previous_start: '2026-09-05T23:00:00-04:00'
				}
			}
		})
	).toThrow(/window/);
});
it('requires matching campaign identities and safe links on bounded hourly analytics', () => {
	const data = {
		metrics_scope: 'recorded_outbox',
		period: '30d',
		window,
		campaign: {
			id: 9,
			name: 'Actual campaign',
			sent_count: 2,
			opened_count: 1,
			clicked_count: 1,
			bounced_count: 0,
			unsubscribed_count: 0,
			open_rate: 50,
			click_rate: 50,
			bounce_rate: 0
		},
		hourly_stats: [{ date: '2026-11-04', hour: 12, event_type: 'opened', count: 2 }],
		device_stats: [
			{ device_type: 'desktop', count: 0 },
			{ device_type: 'mobile', count: 2 },
			{ device_type: 'tablet', count: 0 },
			{ device_type: 'unknown', count: 0 }
		],
		top_links: [{ link_url: 'https://offer.example.test/actual', clicks: 1 }],
		geo_stats: [{ country: 'United States', count: 2 }]
	};
	expect(campaignAnalytics({ success: true, data }, 9).campaign.opened_count).toBe(1);
	expect(() => campaignAnalytics({ success: true, data }, 10)).toThrow(/identity/);
	expect(() =>
		campaignAnalytics(
			{
				success: true,
				data: { ...data, top_links: [{ link_url: 'javascript:alert(1)', clicks: 1 }] }
			},
			9
		)
	).toThrow(/link/);
});

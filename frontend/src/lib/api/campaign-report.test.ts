import { describe, expect, it } from 'vite-plus/test';
import { campaignReport, formatCampaignRate } from './campaign-report.js';
function empty() {
	return {
		success: true,
		data: {
			id: 14,
			name: 'Saved draft',
			subject: 'Draft',
			status: 'draft',
			created_at: '2026-11-05T01:00:00Z',
			sent_at: null,
			scheduled_at: null,
			segment_name: null,
			template_name: null,
			metrics_scope: 'recorded_outbox',
			last_error: null,
			pause_reason: null,
			delivery_stats: { total: 0, queued: 0, processing: 0, failed: 0, unknown: 0, skipped: 0 },
			metrics: {
				sent: 0,
				delivered: 0,
				opened: 0,
				unique_opens: 0,
				clicked: 0,
				unique_clicks: 0,
				bounced: 0,
				soft_bounced: null,
				hard_bounced: null,
				unsubscribed: 0,
				complained: 0
			},
			rates: {
				delivery_rate: null,
				open_rate: null,
				unique_open_rate: null,
				click_rate: null,
				unique_click_rate: null,
				click_to_open_rate: null,
				bounce_rate: null,
				unsubscribe_rate: null
			},
			engagement_over_time: [],
			top_links: [],
			device_breakdown: { desktop: 0, mobile: 0, tablet: 0, unknown: 0 },
			email_client_breakdown: [],
			geographic_distribution: [],
			ab_test_results: null
		}
	};
}
describe('recorded campaign reports', () => {
	it('retains empty recorded observations and unknown denominators without inventing results', () => {
		const actual = campaignReport(empty(), 14);
		expect(actual.metrics.sent).toBe(0);
		expect(actual.rates.open_rate).toBeNull();
		expect(actual.top_links).toEqual([]);
		expect(actual.ab_test_results).toBeNull();
		expect(formatCampaignRate(null)).toBe('—');
		expect(formatCampaignRate(0)).toBe('0.0%');
	});
	it('rejects mismatched identities, historical metrics and malformed success', () => {
		expect(() => campaignReport(empty(), 15)).toThrow(/identity/);
		const data = empty();
		data.data.metrics_scope = 'historical';
		expect(() => campaignReport(data, 14)).toThrow(/scope/);
		expect(() => campaignReport({ success: true, data: {} }, 14)).toThrow();
		expect(() => campaignReport({ success: false, data: empty().data }, 14)).toThrow(/confirmed/);
	});
	it('rejects unsafe observation URLs and incomplete numerical reports', () => {
		const unsafe = {
			...empty(),
			data: {
				...empty().data,
				top_links: [{ url: 'javascript:alert(1)', clicks: 1, unique_clicks: 1 }]
			}
		};
		expect(() => campaignReport(unsafe, 14)).toThrow(/link/);
		const bad = {
			...empty(),
			data: { ...empty().data, metrics: { ...empty().data.metrics, sent: -1 } }
		};
		expect(() => campaignReport(bad, 14)).toThrow(/counts/);
	});
	it('permits measured click-to-open ratios above one hundred when clicks have no matching open signal', () => {
		const data = {
			...empty(),
			data: { ...empty().data, rates: { ...empty().data.rates, click_to_open_rate: 200 } }
		};
		expect(campaignReport(data, 14).rates.click_to_open_rate).toBe(200);
	});
});

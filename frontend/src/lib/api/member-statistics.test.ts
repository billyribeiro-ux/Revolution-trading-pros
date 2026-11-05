import { describe, expect, it, vi } from 'vite-plus/test';
import {
	memberFactsQuery,
	parseMemberStats,
	parseRecordedOrderValue,
	parseServiceCatalog,
	parseServiceMembers,
	readMemberFacts
} from './member-statistics';
const money = {
	currency: 'USD',
	net_cents: 12345,
	order_count: 2,
	excluded_non_usd_orders: 1,
	excluded_unknown_currency_orders: 1,
	scope: 'recorded_orders_less_recorded_refunds',
	limitations: 'Recorded orders; not processor settlement.'
};
const plan = {
	id: 17,
	name: 'Recorded plan',
	type: 'membership_plan',
	members_count: 1,
	price_units: 120,
	currency: null,
	billing_cycle: 'annual',
	interval_count: 12,
	is_active: false
};
const pagination = { total: 1, per_page: 25, current_page: 1, last_page: 1 };
function stats() {
	return {
		schema_version: 1,
		scope: 'current_non_deleted_members',
		evaluated_at: '2026-11-05T12:00:00Z',
		recorded_timestamp_basis: 'Legacy timestamps without verified timezone',
		period: {
			timezone: 'UTC',
			current_month_start: '2026-11-01T00:00:00',
			comparison: 'month_to_date_vs_previous_full_month'
		},
		overview: {
			total_members: 4,
			new_this_month: 1,
			new_last_month: 0,
			growth_rate: null,
			growth_rate_reason: 'No denominator'
		},
		subscriptions: {
			active: 1,
			trial: 1,
			churned: 1,
			never_subscribed: 1,
			active_membership_records: 2,
			churn_rate: null,
			churn_rate_reason: 'No historic census'
		},
		revenue: {
			recorded_net_usd_order_value: money,
			mrr_cents: null,
			mrr_reason: 'No charged snapshot',
			avg_ltv_cents: null,
			avg_ltv_reason: 'No lifetime coverage',
			catalog_monthly_estimate: {
				value: 10,
				unit: 'catalog_price_unit',
				currency: null,
				included_membership_records: 1,
				excluded_membership_records: 1,
				scope: 'current_active_membership_records_at_mutable_catalog_prices',
				reason: 'Currency not recorded'
			}
		},
		top_services: [{ id: 17, name: 'Recorded plan', type: 'membership_plan', members_count: 1 }],
		growth_trend: [{ month: '2026-11', members: 4, new: 1 }],
		growth_scope: 'Current nondeleted registrations',
		growth_timestamp_coverage: { unknown_members: 0, future_members: 0 }
	};
}
function service() {
	return {
		schema_version: 1,
		scope: 'plan_local_membership_cohort',
		cohort_rule: 'Current active then trial then latest effective local record',
		service: plan,
		stats: {
			total_members: 1,
			active_members: 0,
			trial_members: 0,
			cancelled_members: 0,
			expired_members: 1,
			other_members: 0,
			whole_member_recorded_net_usd_order_value: money
		},
		pagination,
		members: [
			{
				id: 9,
				name: null,
				email: 'actual@example.test',
				first_name: null,
				last_name: null,
				avatar: null,
				status: 'active',
				status_label: 'Active',
				joined_at: null,
				total_spent: 123.45,
				active_subscriptions_count: 1,
				current_plan: 'Another plan',
				current_product: null,
				service_status: 'expired',
				service_membership_id: 12
			}
		]
	};
}
describe('current member facts boundaries', () => {
	it('keeps cents, unknown billing metrics and nominal catalog units distinct', () => {
		const result = parseMemberStats(stats());
		expect(result.revenue.recorded_net_usd_order_value.net_cents).toBe(12345);
		expect(result.revenue.mrr_cents).toBeNull();
		expect(result.revenue.catalog_monthly_estimate.currency).toBeNull();
		expect(result.overview.growth_rate).toBeNull();
		expect(result.subscriptions.active_membership_records).toBe(2);
	});
	it('rejects old dollar-shaped fake success and inconsistent cohorts', () => {
		expect(() =>
			parseMemberStats({ overview: { total_members: 0 }, revenue: { mrr: 0 } })
		).toThrow();
		const value = stats();
		value.subscriptions.active = 10;
		expect(() => parseMemberStats(value)).toThrow();
		expect(() => parseRecordedOrderValue({ ...money, currency: 'EUR' })).toThrow();
		expect(() => parseRecordedOrderValue({ ...money, net_cents: -1 })).toThrow();
	});
	it('preserves off-page selected plans and actual inactive state', () => {
		const result = parseServiceCatalog(
			{
				schema_version: 1,
				scope: 'membership_plan_catalog',
				services: [],
				selected: plan,
				pagination: { ...pagination, total: 0 }
			},
			17
		);
		expect(result.selected?.is_active).toBe(false);
		expect(() => parseServiceCatalog({ ...result, selected: { ...plan, id: 18 } }, 17)).toThrow();
	});
	it('keeps plan-local expired and global active status independent with nullable recorded profile facts', () => {
		const result = parseServiceMembers(service(), 17);
		expect(result.members[0].service_status).toBe('expired');
		expect(result.members[0].status).toBe('active');
		expect(result.members[0].joined_at).toBeNull();
		expect(() => parseServiceMembers(service(), 18)).toThrow();
		const malformed = service();
		malformed.stats.total_members = 2;
		expect(() => parseServiceMembers(malformed, 17)).toThrow();
	});
	it('preserves exact status/search/page and plan selection query values', () => {
		expect(memberFactsQuery({ search: 'A&B', status: 'expired', page: 2, per_page: 1 })).toBe(
			'search=A%26B&status=expired&page=2&per_page=1'
		);
		expect(memberFactsQuery({ selected_id: 17, search: '', page: 1 })).toBe(
			'selected_id=17&page=1'
		);
	});
	it('makes one network-only read and rejects malformed successful replies', async () => {
		const fetcher = vi
			.fn<typeof fetch>()
			.mockResolvedValue(Response.json({ overview: { total_members: 0 } }));
		await expect(
			readMemberFacts('/api/admin/members/stats', parseMemberStats, fetcher)
		).rejects.toThrow('invalid member facts');
		expect(fetcher).toHaveBeenCalledTimes(1);
		expect(fetcher).toHaveBeenCalledWith(
			'/api/admin/members/stats',
			expect.objectContaining({ method: 'GET', cache: 'no-store', credentials: 'include' })
		);
	});
	it('surfaces a genuine failure and allows a separate explicit recovery read', async () => {
		const fetcher = vi
			.fn<typeof fetch>()
			.mockResolvedValueOnce(
				Response.json({ message: 'Recorded facts unavailable' }, { status: 503 })
			)
			.mockResolvedValueOnce(Response.json(stats()));
		await expect(
			readMemberFacts('/api/admin/members/stats', parseMemberStats, fetcher)
		).rejects.toThrow('Recorded facts unavailable');
		expect(fetcher).toHaveBeenCalledTimes(1);
		expect(
			(await readMemberFacts('/api/admin/members/stats', parseMemberStats, fetcher)).overview
				.total_members
		).toBe(4);
		expect(fetcher).toHaveBeenCalledTimes(2);
	});
});

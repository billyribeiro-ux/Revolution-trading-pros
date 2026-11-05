/** Checked current-member statistics and membership-plan boundaries. Reads always use the network. */
export type JsonRecord = Record<string, unknown>;
function invalid(): never {
	throw new Error('The server returned invalid member facts. Refresh to try again.');
}
function object(value: unknown): JsonRecord {
	if (value === null || typeof value !== 'object' || Array.isArray(value)) return invalid();
	return value as JsonRecord;
}
function text(value: unknown): string {
	return typeof value === 'string' ? value : invalid();
}
function count(value: unknown): number {
	return typeof value === 'number' && Number.isSafeInteger(value) && value >= 0 ? value : invalid();
}
function id(value: unknown): number {
	const parsed = count(value);
	return parsed > 0 ? parsed : invalid();
}
function numeric(value: unknown): number {
	return typeof value === 'number' && Number.isFinite(value) ? value : invalid();
}
function nullable<T>(value: unknown, parse: (value: unknown) => T): T | null {
	return value === null ? null : parse(value);
}
function literal<T extends string | number | null>(value: unknown, expected: T): T {
	return value === expected ? expected : invalid();
}
function list<T>(value: unknown, parse: (value: unknown) => T): T[] {
	return Array.isArray(value) ? value.map(parse) : invalid();
}
function timestamp(value: unknown): string {
	const parsed = text(value);
	if (!/^\d{4}-\d{2}-\d{2}T/.test(parsed) || !Number.isFinite(Date.parse(parsed))) return invalid();
	return parsed;
}
function boolean(value: unknown): boolean {
	return typeof value === 'boolean' ? value : invalid();
}
export function parseRecordedOrderValue(value: unknown) {
	const v = object(value);
	return {
		currency: literal(v.currency, 'USD'),
		net_cents: count(v.net_cents),
		order_count: count(v.order_count),
		excluded_non_usd_orders: count(v.excluded_non_usd_orders),
		excluded_unknown_currency_orders: count(v.excluded_unknown_currency_orders),
		scope: literal(v.scope, 'recorded_orders_less_recorded_refunds'),
		limitations: text(v.limitations)
	};
}
export type RecordedOrderValue = ReturnType<typeof parseRecordedOrderValue>;
function topService(value: unknown) {
	const v = object(value);
	return {
		id: id(v.id),
		name: text(v.name),
		type: literal(v.type, 'membership_plan'),
		members_count: count(v.members_count)
	};
}
export function parseMemberStats(value: unknown) {
	const v = object(value),
		overview = object(v.overview),
		subscriptions = object(v.subscriptions),
		revenue = object(v.revenue),
		catalog = object(revenue.catalog_monthly_estimate),
		period = object(v.period);
	const cohorts = {
		active: count(subscriptions.active),
		trial: count(subscriptions.trial),
		churned: count(subscriptions.churned),
		never_subscribed: count(subscriptions.never_subscribed),
		active_membership_records: count(subscriptions.active_membership_records),
		churn_rate: literal(subscriptions.churn_rate, null),
		churn_rate_reason: text(subscriptions.churn_rate_reason)
	};
	const total = count(overview.total_members);
	if (
		cohorts.active + cohorts.trial + cohorts.churned + cohorts.never_subscribed !== total ||
		cohorts.active_membership_records < cohorts.active
	)
		return invalid();
	return {
		schema_version: literal(v.schema_version, 1),
		scope: literal(v.scope, 'current_non_deleted_members'),
		evaluated_at: timestamp(v.evaluated_at),
		recorded_timestamp_basis: text(v.recorded_timestamp_basis),
		period: {
			timezone: literal(period.timezone, 'UTC'),
			current_month_start: timestamp(period.current_month_start),
			comparison: literal(period.comparison, 'month_to_date_vs_previous_full_month')
		},
		overview: {
			total_members: total,
			new_this_month: count(overview.new_this_month),
			new_last_month: count(overview.new_last_month),
			growth_rate: nullable(overview.growth_rate, numeric),
			growth_rate_reason: nullable(overview.growth_rate_reason, text)
		},
		subscriptions: cohorts,
		revenue: {
			recorded_net_usd_order_value: parseRecordedOrderValue(revenue.recorded_net_usd_order_value),
			mrr_cents: literal(revenue.mrr_cents, null),
			mrr_reason: text(revenue.mrr_reason),
			avg_ltv_cents: literal(revenue.avg_ltv_cents, null),
			avg_ltv_reason: text(revenue.avg_ltv_reason),
			catalog_monthly_estimate: {
				value: nullable(catalog.value, numeric),
				unit: literal(catalog.unit, 'catalog_price_unit'),
				currency: literal(catalog.currency, null),
				included_membership_records: count(catalog.included_membership_records),
				excluded_membership_records: count(catalog.excluded_membership_records),
				scope: literal(
					catalog.scope,
					'current_active_membership_records_at_mutable_catalog_prices'
				),
				reason: text(catalog.reason)
			}
		},
		top_services: list(v.top_services, topService),
		growth_trend: list(v.growth_trend, (entry) => {
			const p = object(entry),
				month = text(p.month);
			if (!/^\d{4}-\d{2}$/.test(month)) return invalid();
			return { month, members: count(p.members), new: count(p.new) };
		}),
		growth_scope: text(v.growth_scope),
		growth_timestamp_coverage: {
			unknown_members: count(object(v.growth_timestamp_coverage).unknown_members),
			future_members: count(object(v.growth_timestamp_coverage).future_members)
		}
	};
}
export type MemberStats = ReturnType<typeof parseMemberStats>;
export function parseMemberService(value: unknown) {
	const v = object(value);
	return {
		...topService(v),
		price_units: numeric(v.price_units),
		currency: literal(v.currency, null),
		billing_cycle: text(v.billing_cycle),
		interval_count: nullable(v.interval_count, count),
		is_active: nullable(v.is_active, boolean)
	};
}
export type Service = ReturnType<typeof parseMemberService>;
export function parseMemberPagination(value: unknown) {
	const v = object(value);
	const page = {
		total: count(v.total),
		per_page: id(v.per_page),
		current_page: id(v.current_page),
		last_page: id(v.last_page)
	};
	if (page.last_page !== Math.max(1, Math.ceil(page.total / page.per_page))) return invalid();
	return page;
}
export function parseServiceCatalog(value: unknown, selectedId?: number) {
	const v = object(value),
		selected = nullable(v.selected, parseMemberService),
		services = list(v.services, parseMemberService),
		pagination = parseMemberPagination(v.pagination);
	if (
		(selectedId !== undefined && selected?.id !== selectedId) ||
		new Set(services.map((s) => s.id)).size !== services.length ||
		services.length > pagination.per_page
	)
		return invalid();
	return {
		schema_version: literal(v.schema_version, 1),
		scope: literal(v.scope, 'membership_plan_catalog'),
		services,
		selected,
		pagination
	};
}
export const SERVICE_STATUSES = [
	'active',
	'trial',
	'cancelled',
	'expired',
	'pending',
	'past_due',
	'paused',
	'scheduled',
	'other'
] as const;
export type ServiceStatus = (typeof SERVICE_STATUSES)[number];
export interface ServiceMemberFilters {
	search?: string;
	status?: ServiceStatus | '';
	page?: number;
	per_page?: number;
}
function serviceStatus(value: unknown): ServiceStatus {
	return SERVICE_STATUSES.find((s) => s === value) ?? invalid();
}
function globalStatus(value: unknown) {
	return (
		(['active', 'trial', 'churned', 'never_subscribed'] as const).find((s) => s === value) ??
		invalid()
	);
}
function serviceMember(value: unknown) {
	const v = object(value);
	return {
		id: id(v.id),
		name: nullable(v.name, text),
		email: text(v.email),
		first_name: nullable(v.first_name, text),
		last_name: nullable(v.last_name, text),
		avatar: nullable(v.avatar, text),
		status: globalStatus(v.status),
		status_label: text(v.status_label),
		joined_at: nullable(v.joined_at, timestamp),
		total_spent: numeric(v.total_spent),
		active_subscriptions_count: count(v.active_subscriptions_count),
		current_plan: nullable(v.current_plan, text),
		current_product: nullable(v.current_product, text),
		service_status: serviceStatus(v.service_status),
		service_membership_id: id(v.service_membership_id)
	};
}
export type ServiceMemberRecord = ReturnType<typeof serviceMember>;
export function parseServiceMembers(value: unknown, serviceId: number) {
	const v = object(value),
		service = parseMemberService(v.service),
		s = object(v.stats),
		pagination = parseMemberPagination(v.pagination),
		members = list(v.members, serviceMember);
	const stats = {
		total_members: count(s.total_members),
		active_members: count(s.active_members),
		trial_members: count(s.trial_members),
		cancelled_members: count(s.cancelled_members),
		expired_members: count(s.expired_members),
		other_members: count(s.other_members),
		whole_member_recorded_net_usd_order_value: parseRecordedOrderValue(
			s.whole_member_recorded_net_usd_order_value
		)
	};
	if (
		service.id !== serviceId ||
		stats.total_members !== pagination.total ||
		stats.active_members +
			stats.trial_members +
			stats.cancelled_members +
			stats.expired_members +
			stats.other_members !==
			stats.total_members ||
		members.length > pagination.per_page ||
		new Set(members.map((m) => m.id)).size !== members.length
	)
		return invalid();
	return {
		schema_version: literal(v.schema_version, 1),
		scope: literal(v.scope, 'plan_local_membership_cohort'),
		cohort_rule: text(v.cohort_rule),
		service,
		stats,
		members,
		pagination
	};
}
export type ServiceMembers = ReturnType<typeof parseServiceMembers>;
export interface ServiceCatalogFilters {
	search?: string;
	page?: number;
	per_page?: number;
	selected_id?: number;
}
export function memberFactsQuery(
	filters: ServiceMemberFilters | ServiceCatalogFilters = {}
): string {
	const params = new URLSearchParams();
	for (const [key, value] of Object.entries(filters))
		if (value !== undefined && value !== '') params.set(key, String(value));
	return params.toString();
}
export async function readMemberFacts<T>(
	path: string,
	parse: (value: unknown) => T,
	fetcher: typeof fetch = fetch,
	signal?: AbortSignal
): Promise<T> {
	const requestSignal = signal
		? AbortSignal.any([signal, AbortSignal.timeout(15_000)])
		: AbortSignal.timeout(15_000);
	const response = await fetcher(path, {
		method: 'GET',
		credentials: 'include',
		cache: 'no-store',
		headers: { Accept: 'application/json' },
		signal: requestSignal
	});
	let data: unknown;
	try {
		data = await response.json();
	} catch {
		throw new Error(
			response.ok
				? 'The server returned an unreadable member response.'
				: `Member request failed (${response.status}).`
		);
	}
	if (!response.ok) {
		const v = data && typeof data === 'object' ? (data as JsonRecord) : {};
		throw new Error(
			typeof v.message === 'string' ? v.message : `Member request failed (${response.status}).`
		);
	}
	return parse(data);
}

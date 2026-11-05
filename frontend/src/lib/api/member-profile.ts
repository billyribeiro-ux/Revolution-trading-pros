/** Factual member profile. Money, account restrictions and legacy wall-clock dates stay distinct. */
import { parseRecordedOrderValue } from './member-statistics.js';
type RecordValue = Record<string, unknown>;
export type MemberJson =
	| null
	| boolean
	| number
	| string
	| MemberJson[]
	| { [key: string]: MemberJson };
function invalid(): never {
	throw new Error('The member service returned an invalid or unconfirmed response');
}
function object(value: unknown): RecordValue {
	return value !== null && typeof value === 'object' && !Array.isArray(value)
		? (value as RecordValue)
		: invalid();
}
function text(value: unknown): string {
	return typeof value === 'string' ? value : invalid();
}
function number(value: unknown): number {
	return typeof value === 'number' && Number.isFinite(value) ? value : invalid();
}
function integer(value: unknown): number {
	const n = number(value);
	return Number.isSafeInteger(n) ? n : invalid();
}
function count(value: unknown): number {
	const n = integer(value);
	return n >= 0 ? n : invalid();
}
function bool(value: unknown): boolean {
	return typeof value === 'boolean' ? value : invalid();
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
export function memberIdentity(value: unknown): string {
	if (typeof value === 'number')
		return Number.isSafeInteger(value) && value > 0 ? String(value) : invalid();
	return typeof value === 'string' &&
		/^[1-9]\d{0,18}$/.test(value) &&
		BigInt(value) <= 9223372036854775807n
		? value
		: invalid();
}
function sameIdentity(value: unknown, expected: string): string {
	const actual = memberIdentity(value);
	return actual === expected ? actual : invalid();
}
/** Validate the representation without converting an unqualified historical value into an instant. */
function legacyTimestamp(value: unknown): string {
	const raw = text(value);
	return /^\d{4,6}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d{1,6})?(?: BC)?$/.test(raw) ||
		raw === 'infinity' ||
		raw === '-infinity'
		? raw
		: invalid();
}
/** Preserve wide/ancient qualified source values without JavaScript Date coercion. */
function instant(value: unknown): string {
	const raw = text(value);
	return /^\d{4,6}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d{1,6})?(?:Z|[+-]\d{2}:\d{2})(?: BC)?$/.test(
		raw
	) ||
		raw === 'infinity' ||
		raw === '-infinity'
		? raw
		: invalid();
}
function decimal(value: unknown): string {
	const raw = text(value);
	return /^-?\d+(?:\.\d+)?$/.test(raw) && raw.length <= 128 ? raw : invalid();
}
function json(value: unknown, depth = 0): MemberJson {
	if (depth > 32) return invalid();
	if (value === null || typeof value === 'string' || typeof value === 'boolean') return value;
	if (typeof value === 'number') return number(value);
	if (Array.isArray(value)) return value.map((v) => json(v, depth + 1));
	const row = object(value);
	return Object.fromEntries(Object.entries(row).map(([key, v]) => [key, json(v, depth + 1)]));
}
export interface MemberHistoryQuery {
	page?: number;
	per_page?: number;
	action?: string;
}
export type MemberHistoryKind =
	| 'memberships'
	| 'orders'
	| 'notes'
	| 'activity'
	| 'emails'
	| 'timeline';
export type MemberRouteFamily = 'members' | 'member-management';
function pagination(value: unknown, expected?: MemberHistoryQuery) {
	const v = object(value),
		result = {
			total: count(v.total),
			per_page: count(v.per_page),
			current_page: count(v.current_page),
			last_page: count(v.last_page)
		};
	if (
		!Number.isSafeInteger((result.current_page - 1) * result.per_page) ||
		result.per_page < 1 ||
		result.per_page > 100 ||
		result.current_page < 1 ||
		result.last_page !== Math.max(1, Math.ceil(result.total / result.per_page)) ||
		(expected?.page !== undefined && result.current_page !== expected.page) ||
		(expected?.per_page !== undefined && result.per_page !== expected.per_page)
	)
		return invalid();
	return result;
}
function envelope(value: unknown, member: string, scope: string, expected?: MemberHistoryQuery) {
	const row = object(value);
	literal(row.schema_version, 1);
	literal(row.metrics_scope, scope);
	sameIdentity(row.member_id, member);
	return {
		row,
		pagination: pagination(row.pagination, expected),
		timestamp_basis: text(row.timestamp_basis)
	};
}
function rows<T extends { id: string }>(
	value: unknown,
	parse: (value: unknown) => T,
	page: ReturnType<typeof pagination>
): T[] {
	const result = list(value, parse);
	if (
		result.length !==
			Math.min(page.per_page, Math.max(0, page.total - (page.current_page - 1) * page.per_page)) ||
		new Set(result.map((r) => r.id)).size !== result.length
	)
		return invalid();
	return result;
}
function membership(value: unknown, member: string) {
	const v = object(value);
	return {
		id: memberIdentity(v.id),
		user_id: sameIdentity(v.user_id, member),
		plan_id: memberIdentity(v.plan_id),
		plan_name: nullable(v.plan_name, text),
		plan_slug: nullable(v.plan_slug, text),
		status: nullable(v.status, text),
		starts_at: nullable(v.starts_at, legacyTimestamp),
		expires_at: nullable(v.expires_at, legacyTimestamp),
		cancelled_at: nullable(v.cancelled_at, legacyTimestamp),
		trial_ends_at: nullable(v.trial_ends_at, legacyTimestamp),
		grace_period_end: nullable(v.grace_period_end, legacyTimestamp),
		current_period_start: nullable(v.current_period_start, legacyTimestamp),
		current_period_end: nullable(v.current_period_end, legacyTimestamp),
		cancel_at_period_end: nullable(v.cancel_at_period_end, bool),
		created_at: nullable(v.created_at, legacyTimestamp),
		updated_at: nullable(v.updated_at, legacyTimestamp),
		deleted_at: nullable(v.deleted_at, instant),
		payment_provider: nullable(v.payment_provider, text),
		stripe_subscription_id: nullable(v.stripe_subscription_id, text),
		catalog_price_units: nullable(v.catalog_price_units, number),
		catalog_currency: literal(v.catalog_currency, null),
		billing_cycle: nullable(v.billing_cycle, text),
		interval_count: nullable(v.interval_count, integer),
		total_paid: literal(v.total_paid, null),
		next_payment_at: literal(v.next_payment_at, null),
		unmeasured_reason: text(v.unmeasured_reason)
	};
}
function order(value: unknown, member: string) {
	const v = object(value),
		net = decimal(v.net_recorded_units);
	if (net.startsWith('-')) return invalid();
	return {
		id: memberIdentity(v.id),
		user_id: sameIdentity(v.user_id, member),
		order_number: nullable(v.order_number, text),
		status: nullable(v.status, text),
		currency: nullable(v.currency, text),
		total_units: decimal(v.total_units),
		refund_units: nullable(v.refund_units, decimal),
		net_recorded_units: net,
		created_at: nullable(v.created_at, legacyTimestamp),
		completed_at: nullable(v.completed_at, legacyTimestamp),
		refunded_at: nullable(v.refunded_at, legacyTimestamp)
	};
}
function note(value: unknown, member: string) {
	const v = object(value);
	return {
		id: memberIdentity(v.id),
		user_id: sameIdentity(v.user_id, member),
		content: text(v.content),
		created_by: nullable(v.created_by, memberIdentity),
		created_by_name: nullable(v.created_by_name, text),
		created_at: nullable(v.created_at, instant),
		updated_at: nullable(v.updated_at, instant)
	};
}
function activity(value: unknown, member: string) {
	const v = object(value);
	return {
		id: memberIdentity(v.id),
		user_id: sameIdentity(v.user_id, member),
		action: text(v.action),
		description: nullable(v.description, text),
		metadata: json(v.metadata),
		ip_address: nullable(v.ip_address, text),
		user_agent: nullable(v.user_agent, text),
		created_at: nullable(v.created_at, instant)
	};
}
function email(value: unknown, member: string) {
	const v = object(value),
		outcome =
			(['accepted', 'rejected', 'unavailable', 'queued', 'unknown'] as const).find(
				(item) => item === v.outcome
			) ?? invalid(),
		sent_at = nullable(v.sent_at, instant),
		provider_message_id = nullable(v.provider_message_id, text);
	if (
		outcome === 'accepted' &&
		(!sent_at ||
			!provider_message_id?.trim() ||
			!['sent', 'delivered', 'opened', 'clicked', 'bounced'].includes(text(v.recorded_status)))
	)
		return invalid();
	return {
		id: memberIdentity(v.id),
		user_id: sameIdentity(v.user_id, member),
		recipient: text(v.recipient),
		subject: text(v.subject),
		recorded_status: text(v.recorded_status),
		outcome,
		provider: nullable(v.provider, text),
		provider_message_id,
		sent_at,
		queued_at: nullable(v.queued_at, instant),
		created_at: nullable(v.created_at, legacyTimestamp),
		opened_at: nullable(v.opened_at, legacyTimestamp),
		clicked_at: nullable(v.clicked_at, legacyTimestamp),
		bounced_at: nullable(v.bounced_at, legacyTimestamp),
		error: nullable(v.error, text)
	};
}
export function parseMemberMemberships(
	value: unknown,
	memberId: string | number,
	expected?: MemberHistoryQuery
) {
	const member = memberIdentity(memberId),
		e = envelope(value, member, 'member_memberships', expected);
	return {
		schema_version: 1 as const,
		metrics_scope: 'member_memberships' as const,
		member_id: member,
		pagination: e.pagination,
		timestamp_basis: e.timestamp_basis,
		memberships: rows(e.row.memberships, (v) => membership(v, member), e.pagination)
	};
}
export function parseMemberOrders(
	value: unknown,
	memberId: string | number,
	expected?: MemberHistoryQuery
) {
	const member = memberIdentity(memberId),
		e = envelope(value, member, 'member_orders', expected);
	return {
		schema_version: 1 as const,
		metrics_scope: 'member_orders' as const,
		member_id: member,
		pagination: e.pagination,
		timestamp_basis: e.timestamp_basis,
		orders: rows(e.row.orders, (v) => order(v, member), e.pagination)
	};
}
export function parseMemberNotes(
	value: unknown,
	memberId: string | number,
	expected?: MemberHistoryQuery
) {
	const member = memberIdentity(memberId),
		e = envelope(value, member, 'member_notes', expected);
	return {
		schema_version: 1 as const,
		metrics_scope: 'member_notes' as const,
		member_id: member,
		pagination: e.pagination,
		timestamp_basis: e.timestamp_basis,
		notes: rows(e.row.notes, (v) => note(v, member), e.pagination)
	};
}
export function parseMemberActivity(
	value: unknown,
	memberId: string | number,
	expected?: MemberHistoryQuery
) {
	const member = memberIdentity(memberId),
		e = envelope(value, member, 'member_activity', expected);
	return {
		schema_version: 1 as const,
		metrics_scope: 'member_activity' as const,
		member_id: member,
		pagination: e.pagination,
		timestamp_basis: e.timestamp_basis,
		activity: rows(e.row.activity, (v) => activity(v, member), e.pagination)
	};
}
export function parseMemberEmails(
	value: unknown,
	memberId: string | number,
	expected?: MemberHistoryQuery
) {
	const member = memberIdentity(memberId),
		e = envelope(value, member, 'member_emails', expected);
	return {
		schema_version: 1 as const,
		metrics_scope: 'member_emails' as const,
		member_id: member,
		pagination: e.pagination,
		timestamp_basis: e.timestamp_basis,
		coverage: text(e.row.coverage),
		observations: text(e.row.observations),
		emails: rows(e.row.emails, (v) => email(v, member), e.pagination)
	};
}
export function parseMemberTimeline(
	value: unknown,
	memberId: string | number,
	expected?: MemberHistoryQuery
) {
	const v = object(value),
		member = memberIdentity(memberId),
		page = pagination(v.pagination, expected);
	literal(v.schema_version, 1);
	literal(v.metrics_scope, 'member_recorded_timeline');
	sameIdentity(v.member_id, member);
	if (count(v.total) !== page.total) return invalid();
	const data = rows(
		v.data,
		(entry) => {
			const r = object(entry),
				id = text(r.id),
				basis = text(r.timestamp_basis);
			if (
				!/^(?:activity|order|membership|cancellation|note):[1-9]\d*$/.test(id) ||
				!['recorded UTC instant', 'recorded without timezone'].includes(basis)
			)
				return invalid();
			return {
				id,
				type: text(r.type),
				title: text(r.title),
				date: nullable(r.date, basis === 'recorded UTC instant' ? instant : legacyTimestamp),
				timestamp_basis: basis,
				meta: json(r.meta)
			};
		},
		page
	);
	return {
		schema_version: 1 as const,
		metrics_scope: 'member_recorded_timeline' as const,
		member_id: member,
		data,
		total: page.total,
		pagination: page,
		coverage: text(v.coverage)
	};
}
function memberRecord(value: unknown, member: string) {
	const v = object(value),
		a = object(v.account),
		status =
			(['active', 'trial', 'churned', 'never_subscribed'] as const).find((s) => s === v.status) ??
			invalid(),
		tags = list(v.tags, text),
		tag_records = list(v.tag_records, (entry) => {
			const t = object(entry);
			return { id: memberIdentity(t.id), name: text(t.name), slug: text(t.slug) };
		});
	if (new Set(tag_records.map((t) => t.id)).size !== tag_records.length) return invalid();
	return {
		id: sameIdentity(v.id, member),
		name: nullable(v.name, text),
		first_name: nullable(v.first_name, text),
		last_name: nullable(v.last_name, text),
		email: text(v.email),
		role: nullable(v.role, text),
		avatar: nullable(v.avatar, text),
		email_verified: bool(v.email_verified),
		created_at: nullable(v.created_at, legacyTimestamp),
		updated_at: nullable(v.updated_at, legacyTimestamp),
		joined_at: nullable(v.joined_at, legacyTimestamp),
		status,
		active_subscriptions_count: count(v.active_subscriptions_count),
		current_plan: nullable(v.current_plan, text),
		current_product: nullable(v.current_product, text),
		last_login: nullable(v.last_login, legacyTimestamp),
		tags,
		tag_records,
		account: {
			is_active: nullable(a.is_active, bool),
			mfa_enabled: nullable(a.mfa_enabled, bool),
			recorded_restriction: nullable(a.recorded_restriction, text),
			reason: nullable(a.reason, text),
			ban_reason: nullable(a.ban_reason, text),
			banned_until: nullable(a.banned_until, instant)
		},
		recorded_age_days: nullable(v.recorded_age_days, count)
	};
}
export function parseMemberProfile(value: unknown, memberId: string | number) {
	const v = object(value),
		member = memberIdentity(memberId),
		engagement = object(v.engagement),
		observations = object(engagement.observations),
		first = { page: 1, per_page: 25 };
	literal(v.schema_version, 1);
	literal(v.metrics_scope, 'member_profile');
	sameIdentity(v.member_id, member);
	const signals = {
		linked_email_receipts: count(observations.linked_email_receipts),
		stored_open_signals: count(observations.stored_open_signals),
		stored_click_signals: count(observations.stored_click_signals),
		scope: text(observations.scope)
	};
	if (
		signals.stored_open_signals > signals.linked_email_receipts ||
		signals.stored_click_signals > signals.linked_email_receipts
	)
		return invalid();
	return {
		schema_version: 1 as const,
		metrics_scope: 'member_profile' as const,
		member_id: member,
		evaluated_at: instant(v.evaluated_at),
		member: memberRecord(v.member, member),
		recorded_order_value: parseRecordedOrderValue(v.recorded_order_value),
		memberships: parseMemberMemberships(v.memberships, member, first),
		orders: parseMemberOrders(v.orders, member, first),
		notes: parseMemberNotes(v.notes, member, first),
		activity: parseMemberActivity(v.activity, member, first),
		emails: parseMemberEmails(v.emails, member, first),
		timeline: parseMemberTimeline(v.timeline, member, first),
		engagement: {
			score: literal(engagement.score, null),
			reason: text(engagement.reason),
			observations: signals
		},
		timestamp_basis: text(v.timestamp_basis),
		billing_coverage: text(v.billing_coverage)
	};
}
export type MemberProfile = ReturnType<typeof parseMemberProfile>;
export type MemberProfileRecord = MemberProfile['member'];
export type MemberProfileNote = ReturnType<typeof note>;
export type MemberMembership = ReturnType<typeof membership>;
export type MemberOrderRecord = ReturnType<typeof order>;
export type MemberEmailRecord = ReturnType<typeof email>;
export function parseMemberNoteCreated(value: unknown, memberId: string | number, content: string) {
	const v = object(value),
		member = memberIdentity(memberId);
	literal(v.schema_version, 1);
	literal(v.metrics_scope, 'member_note_mutation');
	sameIdentity(v.member_id, member);
	const saved = note(v.note, member);
	if (saved.content !== content || !saved.created_by || !saved.created_at) return invalid();
	return saved;
}
export function parseMemberNoteRemoved(
	value: unknown,
	memberId: string | number,
	noteId: string | number
) {
	const v = object(value),
		member = memberIdentity(memberId);
	literal(v.schema_version, 1);
	literal(v.metrics_scope, 'member_note_mutation');
	sameIdentity(v.member_id, member);
	return {
		member_id: member,
		note_id: sameIdentity(v.note_id, memberIdentity(noteId)),
		removed: bool(v.removed)
	};
}
export class MemberProfileRequestError extends Error {
	readonly outcome: string | null;
	readonly reconcile_history_before_retry: boolean;
	constructor(
		message: string,
		readonly status: number,
		details: { outcome?: string | null; reconcile_history_before_retry?: boolean } = {}
	) {
		super(message);
		this.name = 'MemberProfileRequestError';
		this.outcome = details.outcome ?? null;
		this.reconcile_history_before_retry = details.reconcile_history_before_retry === true;
	}
}
function base(member: string | number, family: MemberRouteFamily): string {
	return `/api/admin/${family}/${memberIdentity(member)}`;
}
function query(filters: MemberHistoryQuery, kind: MemberHistoryKind): string {
	const page = filters.page ?? 1,
		size = filters.per_page ?? 25;
	if (
		!Number.isSafeInteger(page) ||
		page < 1 ||
		!Number.isSafeInteger(size) ||
		size < 1 ||
		size > 100 ||
		(page - 1) * size > Number.MAX_SAFE_INTEGER
	)
		throw new Error('Invalid member history pagination');
	if (
		filters.action !== undefined &&
		(kind !== 'activity' ||
			!filters.action.trim() ||
			filters.action.length > 100 ||
			Array.from(filters.action).some((c) => c.codePointAt(0) === 0 || /\p{Cc}/u.test(c)))
	)
		throw new Error('Invalid recorded activity filter');
	const params = new URLSearchParams({ page: String(page), per_page: String(size) });
	if (filters.action !== undefined) params.set('action', filters.action);
	return params.toString();
}
export async function memberProfileRequest<T>(
	path: string,
	parse: (v: unknown) => T,
	options: {
		method?: 'GET' | 'POST' | 'PUT' | 'DELETE';
		body?: unknown;
		signal?: AbortSignal;
		fetcher?: typeof fetch;
	} = {}
): Promise<T> {
	const signal = options.signal
			? AbortSignal.any([options.signal, AbortSignal.timeout(15_000)])
			: AbortSignal.timeout(15_000),
		method = options.method ?? 'GET';
	const init: RequestInit = {
		method,
		cache: 'no-store',
		credentials: 'same-origin',
		redirect: 'error',
		headers: { Accept: 'application/json', 'Content-Type': 'application/json' },
		signal
	};
	if (options.body !== undefined) {
		if (method === 'GET') throw new Error('A member history read cannot contain a body');
		init.body = JSON.stringify(options.body);
	}
	const mutation = method !== 'GET';
	const uncertain = { outcome: 'unconfirmed', reconcile_history_before_retry: true };
	let response: Response;
	try {
		response = await (options.fetcher ?? fetch)(path, init);
	} catch {
		throw new MemberProfileRequestError(
			mutation
				? 'The change outcome is unconfirmed. Refresh its saved history before retrying.'
				: 'Member history could not be read. Retry to fetch saved records.',
			0,
			mutation ? uncertain : {}
		);
	}
	let value: unknown;
	try {
		value = await response.json();
	} catch {
		throw new MemberProfileRequestError(
			'The member service returned an unreadable response',
			response.status,
			mutation && (response.ok || response.status >= 500) ? uncertain : {}
		);
	}
	if (!response.ok) {
		const v = value && typeof value === 'object' ? (value as RecordValue) : {};
		const serverOutcome = typeof v.outcome === 'string' ? v.outcome : null;
		const reconcile =
			v.reconcile_history_before_retry === true || (mutation && response.status >= 500);
		throw new MemberProfileRequestError(
			typeof v.message === 'string'
				? v.message
				: typeof v.error === 'string'
					? v.error
					: `Member request failed (${response.status})`,
			response.status,
			{
				outcome: serverOutcome ?? (reconcile ? 'unconfirmed' : null),
				reconcile_history_before_retry: reconcile
			}
		);
	}
	try {
		return parse(value);
	} catch (cause) {
		if (!mutation) throw cause;
		throw new MemberProfileRequestError(
			'The change outcome is unconfirmed. Refresh its saved history before retrying.',
			response.status,
			uncertain
		);
	}
}
const request = memberProfileRequest;
export const memberProfileApi = {
	get(
		member: string | number,
		options: { family?: MemberRouteFamily; signal?: AbortSignal; fetcher?: typeof fetch } = {}
	) {
		return request(
			base(member, options.family ?? 'member-management'),
			(v) => parseMemberProfile(v, member),
			options
		);
	},
	memberships(
		member: string | number,
		filters: MemberHistoryQuery = {},
		options: { family?: MemberRouteFamily; signal?: AbortSignal; fetcher?: typeof fetch } = {}
	) {
		return request(
			`${base(member, options.family ?? 'member-management')}/memberships?${query(filters, 'memberships')}`,
			(v) =>
				parseMemberMemberships(v, member, {
					page: filters.page ?? 1,
					per_page: filters.per_page ?? 25
				}),
			options
		);
	},
	orders(
		member: string | number,
		filters: MemberHistoryQuery = {},
		options: { family?: MemberRouteFamily; signal?: AbortSignal; fetcher?: typeof fetch } = {}
	) {
		return request(
			`${base(member, options.family ?? 'member-management')}/orders?${query(filters, 'orders')}`,
			(v) =>
				parseMemberOrders(v, member, { page: filters.page ?? 1, per_page: filters.per_page ?? 25 }),
			options
		);
	},
	notes(
		member: string | number,
		filters: MemberHistoryQuery = {},
		options: { family?: MemberRouteFamily; signal?: AbortSignal; fetcher?: typeof fetch } = {}
	) {
		return request(
			`${base(member, options.family ?? 'member-management')}/notes?${query(filters, 'notes')}`,
			(v) =>
				parseMemberNotes(v, member, { page: filters.page ?? 1, per_page: filters.per_page ?? 25 }),
			options
		);
	},
	activity(
		member: string | number,
		filters: MemberHistoryQuery = {},
		options: { family?: MemberRouteFamily; signal?: AbortSignal; fetcher?: typeof fetch } = {}
	) {
		return request(
			`${base(member, options.family ?? 'member-management')}/activity?${query(filters, 'activity')}`,
			(v) =>
				parseMemberActivity(v, member, {
					page: filters.page ?? 1,
					per_page: filters.per_page ?? 25
				}),
			options
		);
	},
	emails(
		member: string | number,
		filters: MemberHistoryQuery = {},
		options: { family?: MemberRouteFamily; signal?: AbortSignal; fetcher?: typeof fetch } = {}
	) {
		return request(
			`${base(member, options.family ?? 'member-management')}/emails?${query(filters, 'emails')}`,
			(v) =>
				parseMemberEmails(v, member, { page: filters.page ?? 1, per_page: filters.per_page ?? 25 }),
			options
		);
	},
	timeline(
		member: string | number,
		filters: MemberHistoryQuery = {},
		options: { family?: MemberRouteFamily; signal?: AbortSignal; fetcher?: typeof fetch } = {}
	) {
		return request(
			`${base(member, options.family ?? 'member-management')}/timeline?${query(filters, 'timeline')}`,
			(v) =>
				parseMemberTimeline(v, member, {
					page: filters.page ?? 1,
					per_page: filters.per_page ?? 25
				}),
			options
		);
	},
	addNote(
		member: string | number,
		content: string,
		options: { family?: MemberRouteFamily; signal?: AbortSignal; fetcher?: typeof fetch } = {}
	) {
		if (
			!content.trim() ||
			Array.from(content).length > 10000 ||
			new TextEncoder().encode(content).length > 40000 ||
			/[^\P{Cc}\n\r\t]/u.test(content)
		)
			return Promise.reject(
				new Error('Provide a nonempty note within 10000 characters, without control characters')
			);
		return request(
			`${base(member, options.family ?? 'member-management')}/notes`,
			(v) => parseMemberNoteCreated(v, member, content),
			{ ...options, method: 'POST', body: { content } }
		);
	},
	removeNote(
		member: string | number,
		noteId: string | number,
		options: { family?: MemberRouteFamily; signal?: AbortSignal; fetcher?: typeof fetch } = {}
	) {
		return request(
			`${base(member, options.family ?? 'member-management')}/notes/${memberIdentity(noteId)}`,
			(v) => parseMemberNoteRemoved(v, member, noteId),
			{ ...options, method: 'DELETE' }
		);
	}
};
/** Keep the recorded wall-clock string: never silently parse it as the browser's local zone or UTC. */
export function formatMemberRecordedTimestamp(value: string | null): string {
	return value === null
		? 'Not recorded'
		: `${legacyTimestamp(value).replace('T', ' ')} (recorded without timezone)`;
}
export function formatMemberInstant(value: string | null): string {
	if (value === null) return 'Not recorded';
	const raw = instant(value);
	if (raw === 'infinity' || raw === '-infinity') return `${raw} (unbounded recorded instant)`;
	return `${raw.replace('T', ' ')} (${/(?:Z|\+00:00)(?: BC)?$/.test(raw) ? 'recorded UTC instant' : 'recorded instant with explicit offset'})`;
}
/** Display recorded nominal units without float conversion or inventing an unknown currency. */
export function formatMemberRecordedAmount(value: string, currency: string | null): string {
	const units = decimal(value);
	return currency?.trim()
		? `${units} ${currency} (recorded)`
		: `${units} units (currency unrecorded)`;
}

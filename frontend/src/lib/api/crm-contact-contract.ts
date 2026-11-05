import { assertCrmClientRequest } from '#lib/crm/request-gate.js';
import { notifyCrmDenial } from '#lib/crm/denial.js';
import type { JsonValue } from './_types.js';
import type { ContactEmailActivity } from '#lib/crm/contact-types.js';
import type { ContactFormFields, ContactWrite } from '#lib/crm/contact-types.js';
import {
	lifecycleStages,
	type ContactRecord,
	type ContactFacts,
	type ContactScoring,
	type ContactPage,
	type ContactQuery,
	type ContactTimelinePage,
	type ContactNote,
	type ContactTag,
	type ContactListMembership,
	type ContactMessageReceipt,
	type ContactMessageIntent
} from '#lib/crm/contact-types.js';

function fail(): never {
	throw new Error('The contact service returned an invalid or unconfirmed response');
}
function object(value: unknown): Record<string, unknown> {
	return value !== null && typeof value === 'object' && !Array.isArray(value)
		? (value as Record<string, unknown>)
		: fail();
}
function text(value: unknown): string {
	return typeof value === 'string' ? value : fail();
}
function nullableText(value: unknown): string | null {
	return value === null ? null : text(value);
}
function numeric(value: unknown): number {
	return typeof value === 'number' && Number.isFinite(value) ? value : fail();
}
function nullableNumber(value: unknown): number | null {
	return value === null ? null : numeric(value);
}
function count(value: unknown): number {
	const result = numeric(value);
	return Number.isSafeInteger(result) && result >= 0 ? result : fail();
}
function date(value: unknown): string {
	const result = text(value);
	return Number.isFinite(Date.parse(result)) ? result : fail();
}
function nullableDate(value: unknown): string | null {
	return value === null ? null : date(value);
}
function json(value: unknown): value is JsonValue {
	return (
		value === null ||
		typeof value === 'string' ||
		typeof value === 'boolean' ||
		(typeof value === 'number' && Number.isFinite(value)) ||
		(Array.isArray(value)
			? value.every(json)
			: typeof value === 'object' && value !== null && Object.values(value).every(json))
	);
}
function jsonObject(value: unknown): Record<string, JsonValue> {
	const result = object(value);
	return Object.values(result).every(json) ? (result as Record<string, JsonValue>) : fail();
}
export function contactId(value: unknown): string {
	if (typeof value === 'number')
		return Number.isSafeInteger(value) && value > 0 ? String(value) : fail();
	return typeof value === 'string' &&
		/^[1-9]\d{0,18}$/.test(value) &&
		BigInt(value) <= 9223372036854775807n
		? value
		: fail();
}
export function contactNumericId(value: unknown): number {
	const result = Number(contactId(value));
	return Number.isSafeInteger(result) ? result : fail();
}
/** Editing an unrelated field must preserve unknown/null profile facts. */
export function contactFormWrite(
	fields: ContactFormFields,
	initial: ContactFormFields | null
): ContactWrite {
	const payload: ContactWrite = {};
	for (const key of ['email', 'first_name', 'last_name', 'phone', 'company', 'job_title'] as const)
		if (!initial || fields[key] !== initial[key]) payload[key] = fields[key];
	if ((!initial || fields.status !== initial.status) && fields.status)
		payload.status = fields.status;
	if (!initial || fields.lifecycle_stage !== initial.lifecycle_stage)
		payload.lifecycle_stage = fields.lifecycle_stage || null;
	return payload;
}
function author(value: unknown) {
	if (value === null) return null;
	const row = object(value);
	return { id: contactId(row.id), name: text(row.name) };
}
function scoring(value: unknown): ContactScoring {
	const row = object(value),
		status = text(row.status);
	if (
		![
			'calculated',
			'not_calculated',
			'stale',
			'disabled',
			'unconfigured',
			'invalid_policy'
		].includes(status) ||
		row.scope !== 'configured_contact_rules' ||
		!Array.isArray(row.contributions)
	)
		return fail();
	const score = nullableNumber(row.score),
		max = nullableNumber(row.max_score);
	if (
		(status === 'calculated') !== (score !== null) ||
		(max !== null && max <= 0) ||
		(score !== null && (score < 0 || max === null || score > max))
	)
		return fail();
	return {
		status: status as ContactScoring['status'],
		scope: 'configured_contact_rules',
		score,
		max_score: max,
		evaluated_at: nullableDate(row.evaluated_at),
		policy_hash: nullableText(row.policy_hash),
		reason: nullableText(row.reason),
		contributions: row.contributions.map((value) => {
			const rule = object(value);
			if (typeof rule.matched !== 'boolean') return fail();
			return {
				rule_id: text(rule.rule_id),
				matched: rule.matched,
				points: numeric(rule.points),
				applied_points: numeric(rule.applied_points)
			};
		})
	};
}
function facts(value: unknown): ContactFacts {
	const row = object(value),
		deals = object(row.deals),
		email = object(row.email),
		unmeasured = object(row.unmeasured);
	if (
		row.scope !== 'recorded_contact_links' ||
		email.scope !== 'crm_outbox' ||
		email.human_engagement !== null ||
		['health_score', 'engagement_score', 'value_score', 'lifetime_value', 'human_email_opens'].some(
			(key) => unmeasured[key] !== null
		)
	)
		return fail();
	const orders = row.orders === null ? null : object(row.orders);
	if (orders && orders.scope !== 'linked_member_orders') return fail();
	return {
		scope: 'recorded_contact_links',
		deals: {
			total: count(deals.total),
			open: count(deals.open),
			won: count(deals.won),
			activities: count(deals.activities),
			notes: count(deals.notes)
		},
		email: {
			scope: 'crm_outbox',
			accepted: count(email.accepted),
			open_signals: count(email.open_signals),
			click_signals: count(email.click_signals),
			human_engagement: null
		},
		orders: orders
			? {
					scope: 'linked_member_orders',
					total: count(orders.total),
					completed: count(orders.completed),
					refunded: count(orders.refunded)
				}
			: null,
		unmeasured: {
			health_score: null,
			engagement_score: null,
			value_score: null,
			lifetime_value: null,
			human_email_opens: null
		}
	};
}
export function contactRecord(value: unknown, expected?: string, detail = false): ContactRecord {
	let row = object(value);
	if ('data' in row) {
		if (row.success === false || row.error) return fail();
		row = object(row.data);
	}
	const id = contactId(row.id),
		assessment = scoring(row.scoring);
	if (expected && id !== contactId(expected)) return fail();
	const stage = row.lifecycle_stage;
	if (stage !== null && !lifecycleStages.includes(stage as (typeof lifecycleStages)[number]))
		return fail();
	const tags = row.tags;
	if (tags !== null && (!Array.isArray(tags) || !tags.every((value) => typeof value === 'string')))
		return fail();
	const result: ContactRecord = {
		id,
		user_id: row.user_id === null ? null : contactId(row.user_id),
		owner_id: row.owner_id === null ? null : contactId(row.owner_id),
		email: text(row.email),
		full_name: text(row.full_name),
		first_name: nullableText(row.first_name),
		last_name: nullableText(row.last_name),
		phone: nullableText(row.phone),
		company: nullableText(row.company),
		job_title: nullableText(row.job_title),
		status: nullableText(row.status),
		source: nullableText(row.source),
		lifecycle_stage: stage as ContactRecord['lifecycle_stage'],
		tags: tags as string[] | null,
		custom_fields: row.custom_fields === null ? null : jsonObject(row.custom_fields),
		created_at: nullableDate(row.created_at),
		updated_at: nullableDate(row.updated_at),
		last_contacted_at: nullableDate(row.last_contacted_at),
		last_activity_at: nullableDate(row.last_activity_at),
		score: nullableNumber(row.score),
		lead_score: nullableNumber(row.lead_score),
		recorded_score: nullableNumber(row.recorded_score),
		historical_score: nullableNumber(row.historical_score),
		score_assessed_at: nullableDate(row.score_assessed_at),
		scoring: assessment
	};
	if (result.score !== assessment.score || result.lead_score !== assessment.score) return fail();
	if (detail || row.facts !== undefined) result.facts = facts(row.facts);
	return result;
}
export function contactPage<T>(
	value: unknown,
	read: (value: unknown) => T,
	lastPage = false
): ContactPage<T> {
	const row = object(value),
		meta = object(row.meta);
	if (row.success === false || row.error || !Array.isArray(row.data)) return fail();
	const total = count(meta.total),
		page = count(meta.current_page),
		size = count(meta.per_page),
		pages = count(lastPage ? meta.last_page : meta.total_pages);
	if (
		page < 1 ||
		size < 1 ||
		size > 100 ||
		pages !== Math.max(1, Math.ceil(total / size)) ||
		row.data.length > size ||
		row.data.length > total
	)
		return fail();
	return {
		data: row.data.map((value) => read(value)),
		meta: { current_page: page, per_page: size, total, total_pages: pages }
	};
}
export function contactTimeline(value: unknown, expected: string): ContactTimelinePage {
	const row = object(value),
		coverage = object(row.coverage);
	if (
		contactId(row.contact_id) !== contactId(expected) ||
		row.scope !== 'recorded_contact_links' ||
		coverage.human_email_engagement !== null ||
		coverage.unlinked_address_history !== null ||
		!Array.isArray(coverage.sources) ||
		!coverage.sources.every((value) => typeof value === 'string')
	)
		return fail();
	return {
		...contactPage(
			value,
			(value) => {
				const event = object(value);
				return {
					id: text(event.id),
					type: text(event.type),
					category: text(event.category),
					title: text(event.title),
					description: nullableText(event.description),
					metadata: jsonObject(event.metadata),
					occurred_at: date(event.occurred_at),
					created_by: author(event.created_by)
				};
			},
			true
		),
		contact_id: contactId(row.contact_id),
		scope: 'recorded_contact_links',
		coverage: {
			sources: coverage.sources as string[],
			human_email_engagement: null,
			unlinked_address_history: null
		}
	};
}
export function contactNote(value: unknown): ContactNote {
	const row = object(value);
	return {
		id: contactId(row.id),
		content: text(row.content),
		created_at: date(row.created_at),
		created_by: author(row.created_by)
	};
}
export function contactOwned(value: unknown, expected: string): Record<string, unknown> {
	const row = object(value);
	if (row.success === false || row.error || contactId(row.contact_id) !== contactId(expected))
		return fail();
	return row;
}
export function contactTags(value: unknown, expected: string): ContactTag[] {
	const row = contactOwned(value, expected);
	if (!Array.isArray(row.data)) return fail();
	return row.data.map((value) => {
		const tag = object(value),
			status = tag.definition_status;
		if (
			!['registered', 'unregistered'].includes(text(status)) ||
			(status === 'registered') !== (tag.id !== null)
		)
			return fail();
		return {
			id: tag.id === null ? null : contactId(tag.id),
			name: text(tag.name),
			color: nullableText(tag.color),
			definition_status: status as ContactTag['definition_status']
		};
	});
}
export function contactLists(value: unknown, expected: string): ContactPage<ContactListMembership> {
	contactOwned(value, expected);
	return contactPage(
		value,
		(value) => {
			const row = object(value);
			return {
				id: contactId(row.id),
				name: text(row.name),
				subscribed_at: date(row.subscribed_at)
			};
		},
		true
	);
}
export function contactMembership(
	value: unknown,
	expected: string,
	kind: 'tag' | 'list',
	id: string,
	action: 'add' | 'remove'
): number {
	const row = contactOwned(value, expected);
	if (contactId(row[`${kind}_id`]) !== contactId(id)) return fail();
	const result = count(
		row[action === 'remove' ? 'removed_count' : kind === 'tag' ? 'applied_count' : 'added_count']
	);
	return result <= 1 ? result : fail();
}
export function contactMessage(
	value: unknown,
	expected: string,
	intent: ContactMessageIntent
): ContactMessageReceipt {
	const envelope = object(value),
		row = object(envelope.data);
	if (
		typeof envelope.success !== 'boolean' ||
		contactId(row.contact_id) !== contactId(expected) ||
		row.request_id !== intent.request_id ||
		row.source_subject !== intent.subject.trim() ||
		typeof row.source_hash !== 'string' ||
		!/^[0-9a-f]{64}$/.test(row.source_hash) ||
		row.template_id !== (intent.template_id === undefined ? null : String(intent.template_id)) ||
		row.metrics_scope !== 'contact_message' ||
		!Array.isArray(row.submissions) ||
		row.submissions.length === 0
	)
		return fail();
	const submissions = row.submissions.map((value) => {
		const item = object(value),
			status = text(item.status);
		if (
			!['accepted', 'rejected', 'unknown', 'processing', 'unavailable'].includes(status) ||
			item.recipient !== intent.expected_email ||
			typeof item.subject !== 'string'
		)
			return fail();
		return {
			id: contactId(item.id),
			recipient: text(item.recipient),
			subject: text(item.subject),
			provider: text(item.provider),
			provider_message_id: nullableText(item.provider_message_id),
			status: status as ContactMessageReceipt['submissions'][number]['status'],
			error: nullableText(item.error),
			queued_at: date(item.queued_at),
			accepted_at: nullableDate(item.accepted_at)
		};
	});
	if (envelope.success !== submissions.every((item) => item.status === 'accepted')) return fail();
	return {
		contact_id: contactId(row.contact_id),
		request_id: text(row.request_id),
		source_subject: text(row.source_subject),
		source_hash: text(row.source_hash),
		template_id: row.template_id === null ? null : contactId(row.template_id),
		metrics_scope: 'contact_message',
		created_at: date(row.created_at),
		submissions
	};
}
export function contactEmails(value: unknown, expected: string): ContactPage<ContactEmailActivity> {
	contactOwned(value, expected);
	return contactPage(
		value,
		(value) => {
			const row = object(value),
				status = text(row.status),
				scope = text(row.scope);
			if (
				contactId(row.contact_id) !== contactId(expected) ||
				![
					'queued',
					'processing',
					'accepted',
					'rejected',
					'unknown',
					'unavailable',
					'skipped'
				].includes(status) ||
				!['contact_message', 'crm_outbox', 'linked_member_email_log'].includes(scope)
			)
				return fail();
			return {
				id: text(row.id),
				contact_id: contactId(row.contact_id),
				request_id: nullableText(row.request_id),
				source_subject: nullableText(row.source_subject),
				source_hash: nullableText(row.source_hash),
				template_id: row.template_id === null ? null : contactId(row.template_id),
				recipient: nullableText(row.recipient),
				receipt_id: row.receipt_id === null ? null : contactId(row.receipt_id),
				subject: text(row.subject),
				status: status as ContactEmailActivity['status'],
				scope: scope as ContactEmailActivity['scope'],
				provider: nullableText(row.provider),
				provider_message_id: nullableText(row.provider_message_id),
				queued_at: nullableDate(row.queued_at),
				sent_at: nullableDate(row.sent_at),
				accepted_at: nullableDate(row.accepted_at),
				opened_at: nullableDate(row.opened_at),
				clicked_at: nullableDate(row.clicked_at),
				bounced_at: nullableDate(row.bounced_at),
				error: nullableText(row.error),
				campaign_name: nullableText(row.campaign_name)
			};
		},
		true
	);
}
export function contactMessageActivity(
	activity: ContactEmailActivity,
	intent: ContactMessageIntent,
	sourceHash?: string
): void {
	if (
		activity.scope !== 'contact_message' ||
		activity.id !== `personal:${intent.request_id}` ||
		activity.request_id !== intent.request_id ||
		activity.recipient !== intent.expected_email ||
		activity.source_subject !== intent.subject.trim() ||
		activity.template_id !==
			(intent.template_id === undefined ? null : String(intent.template_id)) ||
		!activity.source_hash ||
		!/^[0-9a-f]{64}$/.test(activity.source_hash) ||
		(sourceHash !== undefined && activity.source_hash !== sourceHash)
	)
		throw new Error('The saved message outcome does not match this frozen message intent');
}
export function contactQuery(value?: ContactQuery): string {
	const params = new URLSearchParams();
	for (const [key, item] of Object.entries(value ?? {}))
		if (item !== undefined && item !== '') params.set(key, String(item));
	return params.size ? `?${params}` : '';
}
/** Exactly one attempt. Failed or ambiguous writes stay visible until explicitly refreshed. */
export async function contactRequest(
	path: string,
	method: 'GET' | 'POST' | 'PUT' | 'DELETE' = 'GET',
	body?: unknown,
	allowReceipt = false
): Promise<unknown> {
	if (method === 'GET' && body !== undefined)
		throw new Error('A GET contact request cannot contain a body');
	const init: RequestInit = {
		method,
		headers: { 'Content-Type': 'application/json' },
		credentials: 'same-origin',
		redirect: 'error',
		cache: 'no-store',
		signal: AbortSignal.timeout(30000)
	};
	if (method !== 'GET' && body !== undefined) init.body = JSON.stringify(body);
	assertCrmClientRequest(`/api/admin/crm/contacts${path}`, method);
	let response: Response;
	try {
		response = await fetch(`/api/admin/crm/contacts${path}`, init);
	} catch {
		throw new Error(
			'The contact service could not confirm the request. Refresh saved state before retrying.'
		);
	}
	notifyCrmDenial(response.status);
	let value: unknown;
	try {
		value = await response.json();
	} catch {
		return fail();
	}
	if (
		!response.ok &&
		!(
			allowReceipt &&
			[502, 503].includes(response.status) &&
			value !== null &&
			typeof value === 'object' &&
			'data' in value
		)
	) {
		const row = object(value);
		throw new Error(
			(typeof row.error === 'string'
				? row.error
				: typeof row.message === 'string'
					? row.message
					: `Contact request failed (${response.status})`
			).slice(0, 500)
		);
	}
	return value;
}

/** Adapt canonical sequence records without relabeling legacy totals as receipts. */
import type { EmailSequence, SequenceEmail, SequenceCondition, SequenceStats } from './email.js';
import type { JsonValue } from './_types';
function record(value: unknown): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		throw new Error('Invalid sequence response');
	return value as Record<string, unknown>;
}
function integer(value: unknown, min = 0): number {
	if (typeof value !== 'number' || !Number.isSafeInteger(value) || value < min)
		throw new Error('Invalid recorded sequence counts');
	return value;
}
function text(row: Record<string, unknown>, key: string): string {
	if (typeof row[key] !== 'string') throw new Error('Invalid sequence content');
	return row[key];
}
function timestamp(row: Record<string, unknown>, key: string): string {
	const value = text(row, key);
	if (!Number.isFinite(Date.parse(value))) throw new Error('Invalid sequence timestamp');
	return value;
}
function json(value: unknown, depth = 0): value is JsonValue {
	if (depth > 64) return false;
	if (value === null || typeof value === 'string' || typeof value === 'boolean') return true;
	if (typeof value === 'number') return Number.isFinite(value);
	if (Array.isArray(value)) return value.every((child) => json(child, depth + 1));
	return (
		!!value &&
		typeof value === 'object' &&
		Object.values(value).every((child) => json(child, depth + 1))
	);
}
export function sequenceId(value: unknown): string {
	if (typeof value !== 'string' || !/^[1-9]\d{0,18}$/.test(value))
		throw new Error('Invalid sequence identity');
	return value;
}
export function sequenceEmailId(value: unknown): string {
	if (
		typeof value !== 'string' ||
		!/^[0-9a-f]{8}(?:-[0-9a-f]{4}){3}-[0-9a-f]{12}$/i.test(value) ||
		/^0{8}(?:-0{4}){3}-0{12}$/.test(value)
	)
		throw new Error('Invalid sequence email identity');
	return value.toLowerCase();
}
export function sequenceConditions(value: unknown): SequenceCondition[] {
	if (!Array.isArray(value)) throw new Error('Invalid sequence conditions');
	return value.map((value): SequenceCondition => {
		const condition = record(value);
		if (
			!['opened', 'clicked', 'not_opened', 'not_clicked', 'tag', 'custom'].includes(
				String(condition.type)
			) ||
			(condition.value !== undefined && !json(condition.value))
		)
			throw new Error('Invalid sequence condition');
		return {
			type: condition.type as SequenceCondition['type'],
			value: condition.value as JsonValue | undefined,
			previous_email_id:
				condition.previous_email_id == null
					? undefined
					: sequenceEmailId(condition.previous_email_id)
		};
	});
}
export function catalogSequenceEmail(value: unknown, parent: string): SequenceEmail {
	const row = record(value),
		id = sequenceEmailId(row.id);
	if (
		row.sequence_id !== parent ||
		!['minutes', 'hours', 'days', 'weeks'].includes(String(row.delay_unit)) ||
		!['draft', 'active', 'paused'].includes(String(row.status))
	)
		throw new Error('Sequence email does not match its parent or delay contract');
	if (row.text_body != null && typeof row.text_body !== 'string')
		throw new Error('Invalid sequence text alternative');
	const conditions = sequenceConditions(row.conditions);
	return {
		id,
		sequence_id: parent,
		step_number: integer(row.position) + 1,
		name: text(row, 'title'),
		subject: text(row, 'email_subject'),
		html_content: text(row, 'email_body'),
		plain_text_content: typeof row.text_body === 'string' ? row.text_body : undefined,
		delay_value: integer(row.delay_value),
		delay_unit: row.delay_unit as SequenceEmail['delay_unit'],
		conditions
	};
}
export function catalogSequence(value: unknown, expected?: string): EmailSequence {
	const row = record(value),
		id = sequenceId(row.id);
	if (row.success === false || row.error) throw new Error('Sequence operation was not confirmed');
	if ((expected !== undefined && id !== expected) || row.metrics_scope !== 'sequence_outbox')
		throw new Error('Invalid sequence identity or measurement scope');
	if (
		!['active', 'paused', 'draft', 'completed'].includes(String(row.status)) ||
		!['manual', 'signup', 'purchase', 'tag_added', 'custom_event'].includes(
			String(row.trigger_type)
		) ||
		!Array.isArray(row.emails) ||
		!json(row.trigger_config) ||
		!json(row.historical_steps) ||
		(row.description != null && typeof row.description !== 'string') ||
		(row.definition_error !== null && typeof row.definition_error !== 'string')
	)
		throw new Error('Invalid saved sequence definition');
	const emails = row.emails.map((value) => catalogSequenceEmail(value, id));
	if (
		emails.some(
			(mail, index) =>
				mail.step_number !== index + 1 ||
				emails.findIndex((other) => other.id === mail.id) !== index
		)
	)
		throw new Error('Invalid sequence email order or duplicate identity');
	if (
		row.definition_error === null
			? integer(row.emails_count) !== emails.length
			: row.emails_count !== null
	)
		throw new Error('Sequence email count does not match its reviewed definition');
	return {
		id,
		name: text(row, 'title'),
		description: typeof row.description === 'string' ? row.description : undefined,
		trigger_type: row.trigger_type as EmailSequence['trigger_type'],
		trigger_config: row.trigger_config,
		status: row.status as EmailSequence['status'],
		emails,
		subscribers_count: integer(row.subscribers_count),
		created_at: timestamp(row, 'created_at'),
		updated_at: timestamp(row, 'updated_at'),
		definition_error: row.definition_error as string | null,
		historical_steps: row.historical_steps
	};
}
export function catalogSequencePage(value: unknown) {
	const row = record(value),
		meta = record(row.meta);
	if (row.success === false || row.error) throw new Error('Sequence operation was not confirmed');
	if (!Array.isArray(row.data)) throw new Error('Invalid sequence list');
	const sequences = row.data.map((value) => catalogSequence(value));
	const total = integer(meta.total),
		page = integer(meta.current_page, 1),
		perPage = integer(meta.per_page, 1),
		last = integer(meta.last_page, 1);
	if (
		perPage > 100 ||
		sequences.length > perPage ||
		total < sequences.length ||
		last !== Math.max(1, Math.ceil(total / perPage))
	)
		throw new Error('Invalid sequence pagination');
	return { sequences, total, page, per_page: perPage, last_page: last };
}
export function catalogSequenceEnvelope(value: unknown, expected?: string) {
	const row = record(value);
	if (row.success === false || row.error) throw new Error('Sequence operation was not confirmed');
	return catalogSequence(row.sequence ?? value, expected);
}
export function sequenceEmailWrite(
	data: Partial<SequenceEmail>,
	parent?: string
): Record<string, unknown> {
	if (data.sequence_id && data.sequence_id !== parent)
		throw new Error('Sequence email belongs to a different sequence');
	const out: Record<string, unknown> = {};
	if (data.id !== undefined) out.id = sequenceEmailId(data.id);
	for (const [from, to] of [
		['name', 'title'],
		['subject', 'email_subject'],
		['html_content', 'email_body'],
		['plain_text_content', 'text_body'],
		['delay_value', 'delay_value'],
		['delay_unit', 'delay_unit'],
		['conditions', 'conditions']
	] as const)
		if (data[from] !== undefined) out[to] = data[from];
	if (data.stats !== undefined) throw new Error('Sequence statistics are read only');
	if (data.step_number !== undefined) out.position = integer(data.step_number, 1) - 1;
	return out;
}
export function sequenceCatalogWrite(
	data: Partial<EmailSequence>,
	parent?: string
): Record<string, unknown> {
	for (const key of [
		'id',
		'stats',
		'subscribers_count',
		'created_at',
		'updated_at',
		'definition_error',
		'historical_steps'
	] as const)
		if (data[key] !== undefined) throw new Error('Send sequence authoring fields only');
	const out: Record<string, unknown> = {};
	if (data.name !== undefined) out.title = data.name;
	for (const key of ['description', 'trigger_type', 'trigger_config', 'status'] as const)
		if (data[key] !== undefined) out[key] = data[key];
	if (data.emails !== undefined)
		out.emails = data.emails.map((mail, index) => {
			if (mail.step_number !== undefined && mail.step_number !== index + 1)
				throw new Error('Sequence email order does not match its step numbers');
			return { ...sequenceEmailWrite(mail, parent), status: 'active' };
		});
	if (!Object.keys(out).length) throw new Error('Specify sequence authoring changes');
	return out;
}
export function catalogSequenceStats(value: unknown, id: string): SequenceStats {
	const row = record(value);
	if (row.sequence_id !== id || row.metrics_scope !== 'sequence_outbox')
		throw new Error('Sequence statistics identity or scope did not match');
	const sent = integer(row.total_sent),
		subscribers = integer(row.subscribers),
		completed = integer(row.completed);
	const revenue = record(row.revenue);
	if (
		revenue.amount !== null ||
		revenue.currency !== null ||
		(sent === 0 && (row.open_rate !== null || row.click_rate !== null))
	)
		throw new Error('Unmeasured sequence rates or revenue must remain unavailable');
	const rate = (value: unknown): number | null => {
		if (value === null) return null;
		if (typeof value !== 'number' || !Number.isFinite(value) || value < 0 || value > 100)
			throw new Error('Invalid recorded sequence rate');
		return value;
	};
	const bounced = integer(row.total_bounced),
		complained = integer(row.total_complained);
	const active = integer(row.active_subscribers),
		opened = integer(row.total_opened),
		clicked = integer(row.total_clicked),
		delivered = integer(row.total_delivered),
		unsubscribed = integer(row.total_unsubscribed);
	if (
		completed + active > subscribers ||
		[bounced, complained, opened, clicked, delivered, unsubscribed].some((count) => count > sent)
	)
		throw new Error('Sequence receipt or completion counts do not match');
	return {
		sent,
		delivered,
		opened,
		clicked,
		bounced,
		complained,
		unsubscribed,
		open_rate: rate(row.open_rate),
		click_rate: rate(row.click_rate),
		bounce_rate: sent > 0 ? (100 * bounced) / sent : null,
		complaint_rate: sent > 0 ? (100 * complained) / sent : null,
		revenue: null,
		active_subscribers: active,
		completed_subscribers: completed,
		completion_rate: subscribers > 0 ? (100 * completed) / subscribers : null
	};
}

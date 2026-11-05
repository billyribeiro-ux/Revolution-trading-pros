import type {
	EmailSequence,
	SequenceMail,
	MailerSettings,
	SequenceStats,
	SequenceTracker
} from '#lib/crm/types.js';
import {
	catalogSequence,
	catalogSequenceEmail,
	catalogSequenceStats,
	sequenceEmailId,
	sequenceId
} from './email-sequence-catalog.js';
import type { JsonValue, PaginatedResponse } from './_types.js';

function object(value: unknown): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		throw new Error('Invalid saved sequence response');
	const row = value as Record<string, unknown>;
	if (row.success === false || row.error) throw new Error('Sequence operation was not confirmed');
	return row;
}
function count(value: unknown, minimum = 0): number {
	if (typeof value !== 'number' || !Number.isSafeInteger(value) || value < minimum)
		throw new Error('Invalid recorded sequence count');
	return value;
}
function text(value: unknown): string {
	if (typeof value !== 'string') throw new Error('Invalid saved sequence text');
	return value;
}
function nullable(value: unknown): string | null {
	return value == null ? null : text(value);
}
function timestamp(value: unknown): string {
	const result = text(value);
	if (!Number.isFinite(Date.parse(result))) throw new Error('Invalid saved sequence timestamp');
	return result;
}
export function sequenceMailer(value: unknown): MailerSettings {
	if (value === undefined)
		return {
			from_name: '',
			from_email: '',
			reply_to_name: '',
			reply_to_email: '',
			is_custom: false
		};
	const row = object(value);
	if (typeof row.is_custom !== 'boolean') throw new Error('Invalid saved sequence sender');
	return {
		from_name: text(row.from_name),
		from_email: text(row.from_email),
		reply_to_name: text(row.reply_to_name),
		reply_to_email: text(row.reply_to_email),
		is_custom: row.is_custom
	};
}
export function crmSequenceMail(value: unknown, parent: string, expected?: string): SequenceMail {
	const common = catalogSequenceEmail(value, parent),
		row = object(value);
	if (expected !== undefined && common.id !== expected)
		throw new Error('Saved sequence email identity did not match');
	const sent = count(row.sent_count),
		opened = count(row.open_count),
		clicked = count(row.click_count),
		unsubscribed = count(row.unsubscribe_count);
	if ([opened, clicked, unsubscribed].some((value) => value > sent))
		throw new Error('Invalid sequence receipt counts');
	const delayValue = count(row.delay_value),
		multipliers = { minutes: 60, hours: 3600, days: 86400, weeks: 604800 };
	if (count(row.delay) !== delayValue * multipliers[common.delay_unit])
		throw new Error('Invalid saved sequence delay');
	return {
		id: common.id,
		sequence_id: parent,
		title: common.name,
		email_subject: common.subject,
		email_pre_header: text(row.email_pre_header),
		email_body: common.html_content,
		text_body: nullable(row.text_body),
		conditions: common.conditions ?? [],
		delay: count(row.delay),
		delay_value: delayValue,
		delay_unit: common.delay_unit,
		status: row.status as SequenceMail['status'],
		settings: sequenceMailer(row.settings),
		position: common.step_number - 1,
		sent_count: sent,
		open_count: opened,
		click_count: clicked,
		unsubscribe_count: unsubscribed,
		revenue: null,
		created_at: timestamp(row.created_at),
		updated_at: timestamp(row.updated_at),
		open_rate: sent ? (100 * opened) / sent : null,
		click_rate: sent ? (100 * clicked) / sent : null
	};
}
export function crmSequence(value: unknown, expected?: string): EmailSequence {
	const common = catalogSequence(value, expected),
		row = object(value);
	const settings = object(row.settings);
	if (!Array.isArray(row.emails)) throw new Error('Invalid saved sequence emails');
	object(row.subscriber_settings);
	return {
		id: common.id,
		title: common.name,
		slug: text(row.slug),
		status: common.status,
		description: common.description,
		authoring_version: count(row.authoring_version, 1),
		definition_error: common.definition_error ?? null,
		historical_steps: common.historical_steps ?? null,
		last_error: nullable(row.last_error),
		trigger_type: common.trigger_type,
		trigger_config: common.trigger_config ?? {},
		design_template: text(row.design_template),
		settings: { mailer_settings: sequenceMailer(settings.mailer_settings) },
		subscriber_settings: row.subscriber_settings as EmailSequence['subscriber_settings'],
		emails_count: row.emails_count === null ? null : count(row.emails_count),
		subscribers_count: common.subscribers_count,
		total_sent: count(row.total_sent),
		total_opened: count(row.total_opened),
		total_clicked: count(row.total_clicked),
		total_revenue: null,
		currency: null,
		created_at: common.created_at,
		updated_at: common.updated_at,
		emails: row.emails.map((mail) => crmSequenceMail(mail, common.id))
	};
}
export function crmSequenceStats(value: unknown, parent: string): SequenceStats {
	const common = catalogSequenceStats(value, parent),
		row = object(value);
	return {
		sequence_id: parent,
		emails: row.emails === null ? null : count(row.emails),
		subscribers: count(row.subscribers),
		active_subscribers: common.active_subscribers,
		completed: common.completed_subscribers,
		total_sent: common.sent,
		total_opened: common.opened,
		total_clicked: common.clicked,
		total_delivered: common.delivered,
		total_bounced: common.bounced,
		total_complained: common.complained,
		total_unsubscribed: common.unsubscribed,
		failed: count(row.failed),
		unknown: count(row.unknown),
		queued: count(row.queued),
		processing: count(row.processing),
		open_rate: common.open_rate,
		click_rate: common.click_rate,
		revenue: { amount: null, currency: null }
	};
}
export function crmSequenceEnvelope(value: unknown, expected: string) {
	const row = object(value);
	return {
		sequence: crmSequence(row.sequence, expected),
		stats: crmSequenceStats(row.stats, expected)
	};
}
export function sequencePage<T>(value: unknown, read: (value: unknown) => T): PaginatedResponse<T> {
	const row = object(value),
		meta = object(row.meta);
	if (!Array.isArray(row.data)) throw new Error('Invalid sequence list');
	const total = count(meta.total),
		page = count(meta.current_page, 1),
		size = count(meta.per_page, 1),
		last = count(meta.last_page, 1);
	if (
		size > 100 ||
		row.data.length > size ||
		total < row.data.length ||
		last !== Math.max(1, Math.ceil(total / size))
	)
		throw new Error('Invalid sequence pagination');
	return {
		data: row.data.map((value) => read(value)),
		meta: { total, current_page: page, per_page: size, total_pages: last }
	};
}
export function crmSequenceTracker(value: unknown, parent: string): SequenceTracker {
	const row = object(value),
		status = row.status;
	if (
		row.sequence_id !== parent ||
		!['active', 'paused', 'cancelled', 'completed', 'failed'].includes(String(status))
	)
		throw new Error('Invalid enrollment identity or status');
	if (!Array.isArray(row.notes)) throw new Error('Invalid enrollment history');
	const notes = row.notes.map((value) => {
		const note = object(value);
		return {
			action: text(note.action),
			at: timestamp(note.at),
			data: note.data as Record<string, unknown> | undefined
		};
	});
	return {
		id: sequenceId(row.id),
		sequence_id: parent,
		contact_id: row.contact_id === null ? null : sequenceId(row.contact_id),
		contact_email: text(row.contact_email),
		revision_id: sequenceEmailId(row.revision_id),
		status: status as SequenceTracker['status'],
		last_error: nullable(row.last_error),
		notes,
		started_at: timestamp(row.started_at),
		created_at: timestamp(row.created_at),
		updated_at: timestamp(row.updated_at),
		emails_sent: count(row.emails_sent),
		emails_opened: count(row.emails_opened),
		emails_clicked: count(row.emails_clicked),
		next_execution_at: row.next_execution_at == null ? undefined : timestamp(row.next_execution_at),
		completed_at: row.completed_at == null ? undefined : timestamp(row.completed_at),
		last_executed_at: row.last_executed_at == null ? undefined : timestamp(row.last_executed_at),
		last_sequence_mail_id:
			row.last_sequence_mail_id == null ? undefined : sequenceEmailId(row.last_sequence_mail_id),
		next_sequence_mail_id:
			row.next_sequence_mail_id == null ? undefined : sequenceEmailId(row.next_sequence_mail_id)
	};
}
export function sequenceAcknowledgement(value: unknown, id: string, key: string): number {
	const row = object(value);
	if (row.sequence_id !== id) throw new Error('Sequence acknowledgement did not match');
	return count(row[key]);
}
export function sequenceDeletion(value: unknown, id: string, parent?: string): void {
	const row = object(value);
	if (row.deleted !== true || row.id !== id || (parent !== undefined && row.sequence_id !== parent))
		throw new Error('Sequence deletion was not confirmed');
}
export function sequenceMailList(value: unknown, parent: string): SequenceMail[] {
	if (!Array.isArray(value)) throw new Error('Invalid sequence email list');
	const result = value.map((mail) => crmSequenceMail(mail, parent));
	if (
		result.some(
			(mail, index) =>
				mail.position !== index || result.findIndex((other) => other.id === mail.id) !== index
		)
	)
		throw new Error('Invalid saved sequence order');
	return result;
}
export interface SequenceMailWrite {
	id?: string;
	title: string;
	email_subject: string;
	email_pre_header: string;
	email_body: string;
	text_body: string | null;
	delay_value: number;
	delay_unit: SequenceMail['delay_unit'];
	status: SequenceMail['status'];
	settings: MailerSettings;
	conditions: NonNullable<SequenceMail['conditions']>;
}
export interface SequenceWrite {
	title?: string;
	description?: string;
	slug?: string;
	status?: EmailSequence['status'];
	expected_version?: number;
	design_template?: string;
	settings?: EmailSequence['settings'];
	subscriber_settings?: EmailSequence['subscriber_settings'] | Record<string, JsonValue>;
	trigger_type?: EmailSequence['trigger_type'];
	trigger_config?: JsonValue;
	emails?: SequenceMailWrite[];
}

import { assertCrmClientRequest } from '#lib/crm/request-gate.js';
import { notifyCrmDenial } from '#lib/crm/denial.js';
import { sequenceId, sequenceEmailId } from './email-sequence-catalog.js';
export interface SequenceTriggerRecord {
	id: string;
	event_id: string;
	sequence_id: string;
	revision_id: string;
	kind: 'signup' | 'purchase' | 'tag_added' | 'custom_event';
	happened_at: string;
	status: 'pending' | 'enrolled' | 'already_enrolled' | 'skipped' | 'cancelled' | 'failed';
	contact_id: string | null;
	tracker_id: string | null;
	attempts: number;
	last_error: string | null;
	processed_at: string | null;
}
function object(value: unknown): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		throw new Error('Invalid sequence trigger history');
	return value as Record<string, unknown>;
}
function count(value: unknown, min = 0): number {
	if (typeof value !== 'number' || !Number.isSafeInteger(value) || value < min)
		throw new Error('Invalid sequence trigger count');
	return value;
}
function timestamp(value: unknown): string {
	if (typeof value !== 'string' || !Number.isFinite(Date.parse(value)))
		throw new Error('Invalid sequence trigger timestamp');
	return value;
}
export function sequenceTriggerHistory(value: unknown, id: string) {
	const row = object(value),
		meta = object(row.meta);
	if (
		row.sequence_id !== id ||
		row.metrics_scope !== 'sequence_trigger' ||
		row.success === false ||
		row.error ||
		!Array.isArray(row.data)
	)
		throw new Error('Sequence trigger history identity or scope did not match');
	const data = row.data.map((value): SequenceTriggerRecord => {
		const entry = object(value);
		if (
			entry.sequence_id !== id ||
			!['signup', 'purchase', 'tag_added', 'custom_event'].includes(String(entry.kind)) ||
			!['pending', 'enrolled', 'already_enrolled', 'skipped', 'cancelled', 'failed'].includes(
				String(entry.status)
			) ||
			(entry.last_error !== null && typeof entry.last_error !== 'string')
		)
			throw new Error('Invalid sequence trigger status or identity');
		return {
			id: sequenceId(entry.id),
			event_id: sequenceId(entry.event_id),
			sequence_id: id,
			revision_id: sequenceEmailId(entry.revision_id),
			kind: entry.kind as SequenceTriggerRecord['kind'],
			happened_at: timestamp(entry.happened_at),
			status: entry.status as SequenceTriggerRecord['status'],
			contact_id: entry.contact_id === null ? null : sequenceId(entry.contact_id),
			tracker_id: entry.tracker_id === null ? null : sequenceId(entry.tracker_id),
			attempts: count(entry.attempts),
			last_error: entry.last_error as string | null,
			processed_at: entry.processed_at === null ? null : timestamp(entry.processed_at)
		};
	});
	const page = count(meta.current_page, 1),
		size = count(meta.per_page, 1),
		total = count(meta.total),
		pages = count(meta.total_pages, 1);
	if (
		size > 100 ||
		data.length > size ||
		data.length > total ||
		pages !== Math.max(1, Math.ceil(total / size)) ||
		new Set(data.map((entry) => entry.id)).size !== data.length
	)
		throw new Error('Invalid sequence trigger pagination');
	return { data, page, pages, total };
}
export async function getSequenceTriggerHistory(id: string, page = 1) {
	id = sequenceId(id);
	count(page, 1);
	assertCrmClientRequest(`/api/admin/crm/sequences/${id}/triggers?page=${page}&per_page=20`);
	const response = await fetch(`/api/admin/crm/sequences/${id}/triggers?page=${page}&per_page=20`, {
		credentials: 'include',
		cache: 'no-store'
	});
	const value: unknown = await response.json().catch(() => null);
	notifyCrmDenial(response.status);
	if (!response.ok) {
		const row = value && typeof value === 'object' ? (value as Record<string, unknown>) : {};
		throw new Error(
			typeof row.message === 'string'
				? row.message
				: typeof row.error === 'string'
					? row.error
					: 'Saved trigger activity is unavailable'
		);
	}
	return sequenceTriggerHistory(value, id);
}

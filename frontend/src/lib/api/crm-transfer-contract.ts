import { assertCrmClientRequest } from '#lib/crm/request-gate.js';
import { notifyCrmDenial } from '#lib/crm/denial.js';
import type {
	ExportJob,
	ImportError,
	ImportJob,
	ImportSettings,
	ImportType,
	ImportStatus
} from '#lib/crm/types.js';
import type { JsonValue, PaginatedResponse } from './_types';
export const contactTransferFields = [
	'email',
	'first_name',
	'last_name',
	'phone',
	'company',
	'job_title',
	'status',
	'source',
	'tags',
	'custom_fields'
] as const;
export const contactExportFields = [
	'id',
	...contactTransferFields,
	'score',
	'last_contacted_at',
	'created_at',
	'updated_at'
] as const;
const kinds = ['contacts', 'tags', 'lists', 'sequences', 'campaigns', 'automations', 'templates'];
function fail(): never {
	throw new Error('The transfer service returned an invalid or unconfirmed response');
}
function object(value: unknown): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value)) return fail();
	return value as Record<string, unknown>;
}
function text(value: unknown): string {
	return typeof value === 'string' ? value : fail();
}
function count(value: unknown): number {
	return typeof value === 'number' && Number.isSafeInteger(value) && value >= 0 ? value : fail();
}
function date(value: unknown): string {
	const result = text(value);
	return Number.isFinite(Date.parse(result)) ? result : fail();
}
function optionalDate(value: unknown): string | null {
	return value === null ? null : date(value);
}
export function transferId(value: unknown): string {
	const id = text(value).toLowerCase();
	return /^[0-9a-f]{8}(?:-[0-9a-f]{4}){3}-[0-9a-f]{12}$/.test(id) ? id : fail();
}
function stringMap(value: unknown): Record<string, string> {
	return Object.fromEntries(
		Object.entries(object(value)).map(([key, value]) => [key, text(value)])
	);
}
function ids(value: unknown): string[] {
	if (
		!Array.isArray(value) ||
		value.length > 100 ||
		value.some((id) => typeof id !== 'string' || !/^[1-9]\d{0,18}$/.test(id))
	)
		return fail();
	return value as string[];
}
function common(value: unknown, expected?: string) {
	const row = object(value);
	const id = transferId(row.id);
	if (expected && id !== transferId(expected)) return fail();
	const kind = text(row.type);
	if (!kinds.includes(kind)) return fail();
	const status = text(row.status);
	if (!['pending', 'completed', 'failed', 'cancelled'].includes(status)) return fail();
	const fileType = text(row.file_type);
	if (!['csv', 'json'].includes(fileType)) return fail();
	const creator = row.created_by === null ? null : text(row.created_by);
	if (creator !== null && !/^[1-9]\d{0,18}$/.test(creator)) return fail();
	return {
		row,
		id,
		type: kind as ImportType,
		status: status as ImportStatus,
		file_type: fileType as 'csv' | 'json',
		file_name: text(row.file_name),
		created_by: creator,
		started_at: optionalDate(row.started_at),
		completed_at: optionalDate(row.completed_at),
		created_at: date(row.created_at),
		updated_at: date(row.updated_at)
	};
}
export function importJob(value: unknown, expected?: string): ImportJob {
	const { row, ...base } = common(value, expected);
	const total = count(row.total_rows),
		processed = count(row.processed_rows),
		success = count(row.success_count),
		created = count(row.created_count),
		updated = count(row.updated_count),
		skipped = count(row.skipped_count),
		errors = count(row.error_count);
	if (
		total < 1 ||
		total > 1000 ||
		processed > total ||
		success !== created + updated ||
		success + skipped > total ||
		errors > total ||
		!Array.isArray(row.errors) ||
		row.errors.length !== errors
	)
		return fail();
	const details: ImportError[] = row.errors.map((value) => {
		const error = object(value);
		const index = count(error.row);
		if (index < 1 || index > total) return fail();
		return {
			row: index,
			message: text(error.message),
			...(error.field === undefined ? {} : { field: text(error.field) })
		};
	});
	const settings = object(row.settings);
	let parsedSettings: ImportSettings | undefined;
	if (base.status === 'completed' || base.status === 'failed') {
		if (
			typeof settings.update_existing !== 'boolean' ||
			typeof settings.skip_duplicates !== 'boolean'
		)
			return fail();
		parsedSettings = {
			update_existing: settings.update_existing,
			skip_duplicates: settings.skip_duplicates,
			add_to_lists: ids(settings.add_to_lists),
			apply_tags: ids(settings.apply_tags),
			contact_status: settings.contact_status === null ? null : text(settings.contact_status)
		};
		if (processed !== total || !base.started_at || !base.completed_at) return fail();
		if (base.status === 'completed' && (errors !== 0 || success + skipped !== total)) return fail();
		if (base.status === 'failed' && (errors === 0 || success !== 0 || skipped !== 0)) return fail();
	} else if (processed !== 0 || success !== 0 || skipped !== 0 || errors !== 0) return fail();
	return {
		...base,
		total_rows: total,
		processed_rows: processed,
		success_count: success,
		created_count: created,
		updated_count: updated,
		skipped_count: skipped,
		error_count: errors,
		errors: details,
		field_mapping: stringMap(row.field_mapping),
		settings: parsedSettings
	};
}
export function exportJob(value: unknown, expected?: string): ExportJob {
	const { row, ...base } = common(value, expected);
	const total = count(row.total_records);
	if (
		total > 1000 ||
		base.status !== 'completed' ||
		!base.completed_at ||
		!base.started_at ||
		!Array.isArray(row.fields) ||
		row.fields.length === 0 ||
		row.fields.some(
			(field) =>
				typeof field !== 'string' ||
				!contactExportFields.includes(field as (typeof contactExportFields)[number])
		) ||
		new Set(row.fields).size !== row.fields.length
	)
		return fail();
	const path = `/api/admin/crm/exports/${base.id}/download`;
	if (row.file_url !== path) return fail();
	if (base.file_name !== `crm-contacts-${base.id}.${base.file_type}`) return fail();
	return {
		...base,
		fields: row.fields as string[],
		filters: object(row.filters),
		total_records: total,
		file_url: path
	};
}
export function transferPage<T>(value: unknown, read: (value: unknown) => T): PaginatedResponse<T> {
	const row = object(value),
		meta = object(row.meta);
	if (!Array.isArray(row.data)) return fail();
	const total = count(meta.total),
		page = count(meta.current_page),
		size = count(meta.per_page),
		pages = count(meta.total_pages);
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
		meta: { total, current_page: page, per_page: size, total_pages: pages }
	};
}
export function transferPreview(value: unknown): {
	headers: string[];
	rows: Record<string, JsonValue>[];
	total_rows: number;
} {
	const row = object(value),
		total = count(row.total_rows);
	if (
		!Array.isArray(row.headers) ||
		row.headers.length < 1 ||
		row.headers.length > 50 ||
		row.headers.some((value) => typeof value !== 'string') ||
		new Set(row.headers).size !== row.headers.length ||
		!Array.isArray(row.rows) ||
		row.rows.length > 20 ||
		row.rows.length > total ||
		total < 1 ||
		total > 1000
	)
		return fail();
	row.rows.forEach((value) => {
		if (Object.keys(object(value)).some((key) => !(row.headers as string[]).includes(key))) fail();
	});
	return {
		headers: row.headers as string[],
		rows: row.rows as Record<string, JsonValue>[],
		total_rows: total
	};
}
export async function transferRequest(
	endpoint: string,
	method: 'GET' | 'POST' = 'GET',
	body?: unknown
): Promise<Response> {
	const multipart = body instanceof FormData;
	if (method === 'GET' && body !== undefined)
		throw new Error('A GET transfer request cannot contain a body');
	const init: RequestInit = {
		method,
		credentials: 'same-origin',
		redirect: 'error',
		cache: 'no-store',
		headers: multipart ? {} : { 'Content-Type': 'application/json' },
		signal: AbortSignal.timeout(30000)
	};
	if (method !== 'GET' && body !== undefined) init.body = multipart ? body : JSON.stringify(body);
	assertCrmClientRequest(`/api/admin/crm/${endpoint}`, method);
	let response: Response;
	try {
		response = await fetch(`/api/admin/crm/${endpoint}`, init);
	} catch {
		throw new Error(
			'The transfer service could not confirm the request. Refresh its saved status before retrying.'
		);
	}
	notifyCrmDenial(response.status);
	if (!response.ok) {
		const source = await response.text();
		let message = source;
		try {
			const value: unknown = JSON.parse(source);
			const row = object(value);
			message =
				typeof row.error === 'string'
					? row.error
					: typeof row.message === 'string'
						? row.message
						: `Transfer request failed (${response.status})`;
		} catch {
			/* Plain errors remain visible. */
		}
		throw new Error(message.slice(0, 500) || `Transfer request failed (${response.status})`);
	}
	return response;
}
export async function transferJson(
	endpoint: string,
	method: 'GET' | 'POST' = 'GET',
	body?: unknown
): Promise<unknown> {
	const response = await transferRequest(endpoint, method, body);
	try {
		return await response.json();
	} catch {
		return fail();
	}
}
export function transferQuery(filters?: Record<string, string | number | undefined>): string {
	const params = new URLSearchParams();
	for (const [key, value] of Object.entries(filters ?? {}))
		if (value !== undefined && value !== '') params.set(key, String(value));
	return params.size ? `?${params}` : '';
}

export interface TransferOption {
	id: string;
	title: string;
}
export interface TransferOptionPage extends PaginatedResponse<TransferOption> {
	selected: TransferOption[];
	missing: string[];
}
export function transferOptions(value: unknown): TransferOptionPage {
	const row = object(value);
	const read = (value: unknown): TransferOption => {
		const row = object(value),
			id = text(row.id);
		if (!/^[1-9]\d{0,18}$/.test(id)) return fail();
		return { id, title: text(row.title) };
	};
	if (!Array.isArray(row.selected) || row.selected.length > 100) return fail();
	const selected = row.selected.map(read),
		missing = ids(row.missing),
		page = transferPage(value, read);
	if (
		new Set(selected.map((value) => value.id)).size !== selected.length ||
		new Set(missing).size !== missing.length ||
		missing.some((id) => selected.some((value) => value.id === id))
	)
		return fail();
	return { ...page, selected, missing };
}

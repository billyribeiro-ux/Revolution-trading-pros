import type { EmailTemplate } from './email.js';
import type { JsonValue } from './_types';

function object(value: unknown): value is Record<string, unknown> {
	return value !== null && typeof value === 'object' && !Array.isArray(value);
}
function json(value: unknown): value is JsonValue {
	if (value === null || typeof value === 'string' || typeof value === 'boolean') return true;
	if (typeof value === 'number') return Number.isFinite(value);
	if (Array.isArray(value)) return value.every(json);
	return object(value) && Object.values(value).every(json);
}
export function catalogTemplateId(value: unknown): string {
	if (
		(typeof value !== 'number' && typeof value !== 'string') ||
		!/^\d+$/.test(String(value)) ||
		!Number.isSafeInteger(Number(value)) ||
		Number(value) <= 0
	)
		throw new Error('Invalid email template identifier');
	return String(Number(value));
}
function text(row: Record<string, unknown>, key: string): string {
	if (typeof row[key] !== 'string') throw new Error('Invalid email template response');
	return row[key];
}
function optionalText(row: Record<string, unknown>, key: string): string | undefined {
	if (row[key] === undefined || row[key] === null) return undefined;
	return text(row, key);
}
function successful(value: unknown): value is Record<string, unknown> {
	return (
		object(value) &&
		value.success !== false &&
		(value.error === undefined || value.error === null || value.error === '')
	);
}
export function catalogTemplate(value: unknown): EmailTemplate {
	if (!object(value)) throw new Error('Invalid email template response');
	const tags = value.tags,
		variables = value.variables;
	if (
		typeof value.is_public !== 'boolean' ||
		typeof value.is_active !== 'boolean' ||
		typeof value.usage_count !== 'number' ||
		!Number.isSafeInteger(value.usage_count) ||
		value.usage_count < 0 ||
		!Array.isArray(tags) ||
		!tags.every((tag: unknown) => typeof tag === 'string') ||
		!Array.isArray(variables) ||
		!variables.every(json) ||
		(value.design_json !== undefined && !json(value.design_json))
	)
		throw new Error('Invalid email template catalog metadata');
	const html = typeof value.body_html === 'string' ? value.body_html : value.body;
	if (typeof html !== 'string') throw new Error('Invalid email template body');
	return {
		id: catalogTemplateId(value.id),
		name: text(value, 'name'),
		slug: text(value, 'slug'),
		subject: text(value, 'subject'),
		category: typeof value.category === 'string' ? value.category : text(value, 'email_type'),
		html_content: html,
		plain_text_content: optionalText(value, 'body_text'),
		description: optionalText(value, 'description'),
		thumbnail: optionalText(value, 'thumbnail'),
		design_json: value.design_json,
		is_public: value.is_public,
		is_active: value.is_active,
		usage_count: value.usage_count,
		tags,
		variables,
		created_at: text(value, 'created_at'),
		updated_at: text(value, 'updated_at')
	};
}
export function catalogTemplateEnvelope(value: unknown): { template: EmailTemplate } {
	if (!successful(value)) throw new Error('Email template operation was not confirmed');
	return { template: catalogTemplate(value.data ?? value.template) };
}
export function catalogTemplatePage(value: unknown): { templates: EmailTemplate[]; total: number } {
	if (!successful(value)) throw new Error('Email template listing was not confirmed');
	const rows = value.data ?? value.templates;
	const total = object(value.meta) ? value.meta.total : value.total;
	if (
		!Array.isArray(rows) ||
		typeof total !== 'number' ||
		!Number.isSafeInteger(total) ||
		total < 0
	)
		throw new Error('Invalid email template pagination');
	return { templates: rows.map(catalogTemplate), total };
}
export function catalogTemplateWrite(
	value: Partial<EmailTemplate>,
	create = false
): Record<string, unknown> {
	if (create && (!value.name?.trim() || !value.subject?.trim() || !value.html_content?.trim()))
		throw new Error('Name, subject and HTML content are required');
	const payload: Record<string, unknown> = {};
	for (const key of [
		'name',
		'slug',
		'subject',
		'description',
		'thumbnail',
		'design_json',
		'is_public',
		'is_active',
		'tags',
		'variables'
	] as const) {
		if (value[key] !== undefined) payload[key] = value[key];
	}
	if (value.category !== undefined) payload.email_type = value.category;
	if (value.html_content !== undefined) payload.body_html = value.html_content;
	if (value.plain_text_content !== undefined) payload.body_text = value.plain_text_content ?? '';
	return payload;
}
export function catalogTemplateDeleted(value: unknown): { success: boolean } {
	if (!successful(value) || (value.deleted !== true && value.success !== true))
		throw new Error('Email template deletion was not confirmed');
	return { success: true };
}

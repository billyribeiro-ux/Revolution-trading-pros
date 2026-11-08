import type { Form, FormField, FormSettings } from './forms.js';

type FormRecord = {
	id: number;
	name: string;
	slug: string;
	description?: string | null;
	fields: unknown;
	settings: unknown;
	is_published: boolean;
	submission_count?: number;
	created_at?: string;
	updated_at?: string;
};

function isRecord(value: unknown): value is Record<string, unknown> {
	return typeof value === 'object' && value !== null && !Array.isArray(value);
}

export function unwrapFormResponse(response: unknown): unknown {
	return isRecord(response) && 'data' in response ? response['data'] : response;
}

export function normalizeForm(response: unknown): Form {
	const value = unwrapFormResponse(response);
	if (
		!isRecord(value) ||
		typeof value['id'] !== 'number' ||
		typeof value['name'] !== 'string' ||
		typeof value['slug'] !== 'string' ||
		!Array.isArray(value['fields']) ||
		typeof value['is_published'] !== 'boolean'
	) {
		throw new Error('Invalid form response');
	}

	const row = value as FormRecord & { fields: unknown[] };
	const fields: FormField[] = row.fields.map((field: unknown, index: number) => {
		if (!isRecord(field) || typeof field['name'] !== 'string') {
			throw new Error('Invalid form field');
		}
		const fieldType = field['field_type'] ?? field['type'];
		if (typeof fieldType !== 'string') throw new Error('Invalid form field type');
		return {
			...(field as Partial<FormField>),
			id: typeof field['id'] === 'number' ? field['id'] : index + 1,
			name: field['name'],
			label: typeof field['label'] === 'string' ? field['label'] : field['name'],
			field_type: fieldType as FormField['field_type'],
			order: typeof field['order'] === 'number' ? field['order'] : index,
			required: field['required'] === true,
			width: typeof field['width'] === 'number' ? (field['width'] as FormField['width']) : 12
		};
	});

	return {
		id: row.id,
		title: row.name,
		slug: row.slug,
		description: row.description ?? undefined,
		fields,
		settings: isRecord(row.settings) ? (row.settings as FormSettings) : {},
		status: row.is_published ? 'published' : 'draft',
		submission_count: row.submission_count,
		created_at: row.created_at,
		updated_at: row.updated_at
	};
}

export function formWritePayload(form: Partial<Form>): Record<string, unknown> {
	return {
		...(form.title !== undefined ? { name: form.title } : {}),
		...(form.description !== undefined ? { description: form.description } : {}),
		...(form.fields !== undefined ? { fields: form.fields } : {}),
		...(form.settings !== undefined ? { settings: form.settings } : {}),
		...(form.status !== undefined ? { is_published: form.status === 'published' } : {})
	};
}

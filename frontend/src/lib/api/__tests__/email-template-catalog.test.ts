import { expect, it } from 'vite-plus/test';
import {
	catalogTemplateDeleted,
	catalogTemplateEnvelope,
	catalogTemplateId,
	catalogTemplatePage,
	catalogTemplateWrite
} from '../email-template-catalog.js';

const row = {
	id: 42,
	name: 'Receipt fixture',
	slug: 'receipt-fixture',
	subject: 'Hello {{name}}',
	body_html: '<p>Hello {{name}}</p>',
	body_text: null,
	email_type: 'marketing',
	description: 'Saved description',
	thumbnail: '/fixture.png',
	design_json: { blocks: [{ text: 'Fixture' }] },
	is_public: false,
	is_active: true,
	tags: ['fixture'],
	variables: [{ key: 'name' }],
	usage_count: 2,
	created_at: '2026-01-01T00:00:00',
	updated_at: '2026-01-02T00:00:00'
};

it('normalizes the actual authoring DTO and preserves saved catalog metadata', () => {
	const result = catalogTemplatePage({ data: [row], meta: { total: 21 } });
	expect(result.total).toBe(21);
	expect(result.templates[0]).toMatchObject({
		id: '42',
		category: 'marketing',
		html_content: row.body_html,
		design_json: row.design_json,
		tags: ['fixture'],
		usage_count: 2
	});
	expect(result.templates[0]?.plain_text_content).toBeUndefined();
	expect(catalogTemplateEnvelope({ data: row }).template.slug).toBe(row.slug);
});

it('rejects malformed successes instead of fabricating an empty catalog or record', () => {
	for (const value of [
		{},
		{ data: [] },
		{ data: [row], meta: { total: -1 } },
		{ success: false, templates: [], total: 0 }
	])
		expect(() => catalogTemplatePage(value)).toThrow();
	for (const value of [
		{},
		{ data: { ...row, usage_count: '2' } },
		{ data: { ...row, is_active: null } },
		{ error: 'Unavailable', data: row }
	])
		expect(() => catalogTemplateEnvelope(value)).toThrow();
});

it('maps editor names and clears text without sending receipt counts or database identity', () => {
	const saved = catalogTemplateEnvelope({ data: row }).template;
	expect(catalogTemplateWrite({ ...saved, plain_text_content: null })).toEqual({
		name: row.name,
		slug: row.slug,
		subject: row.subject,
		description: row.description,
		thumbnail: row.thumbnail,
		design_json: row.design_json,
		is_public: false,
		is_active: true,
		tags: row.tags,
		variables: row.variables,
		email_type: 'marketing',
		body_html: row.body_html,
		body_text: ''
	});
	expect(() => catalogTemplateWrite({ name: 'Missing content' }, true)).toThrow();
	expect(catalogTemplateWrite({ name: 'Display only' })).toEqual({ name: 'Display only' });
});

it('requires confirmed deletion and canonical positive identifiers', () => {
	expect(catalogTemplateDeleted({ deleted: true, id: 42 })).toEqual({ success: true });
	for (const response of [{}, { message: 'OK' }, { success: false, deleted: true }])
		expect(() => catalogTemplateDeleted(response)).toThrow();
	expect(catalogTemplateId('0042')).toBe('42');
	for (const value of ['../42', 0, -1, '1.5', Number.MAX_SAFE_INTEGER + 1])
		expect(() => catalogTemplateId(value)).toThrow();
});

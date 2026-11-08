import { describe, expect, it } from 'vite-plus/test';
import { formWritePayload, normalizeForm } from './forms-contract.js';

const row = {
	id: 42,
	name: 'Contact us',
	slug: 'contact-us',
	description: 'Ask a question',
	fields: [{ name: 'email', field_type: 'email', label: 'Email', required: true }],
	settings: { submit_text: 'Send' },
	is_published: true,
	submission_count: 3
};

describe('Rust forms contract', () => {
	it('maps the published API record into the public renderer without duplicate field keys', () => {
		const form = normalizeForm(row);
		expect(form).toMatchObject({
			id: 42,
			title: 'Contact us',
			slug: 'contact-us',
			status: 'published',
			fields: [{ id: 1, name: 'email', field_type: 'email', order: 0 }]
		});
	});

	it('unwraps the admin response and writes the Rust request fields', () => {
		const form = normalizeForm({ data: { ...row, is_published: false } });
		expect(form.status).toBe('draft');
		expect(formWritePayload(form)).toMatchObject({
			name: 'Contact us',
			is_published: false,
			fields: form.fields
		});
	});

	it('rejects malformed records instead of showing an empty or misleading form', () => {
		expect(() => normalizeForm({ data: { ...row, fields: null } })).toThrow(
			'Invalid form response'
		);
	});
});

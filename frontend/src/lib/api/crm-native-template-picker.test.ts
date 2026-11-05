// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
import {
	getCrmNativeTemplatePicker,
	parseCrmNativeTemplatePicker
} from './crm-native-template-picker';
const row = {
	id: '17',
	name: 'Saved native creative',
	subject: 'Hello {{ first_name }}',
	body_html: '<p>Actual body</p>',
	body_text: null
};
const page = {
	metrics_scope: 'contact_template_picker',
	data: [row],
	meta: { current_page: 2, per_page: 1, total: 2, last_page: 2 }
};
afterEach(() => {
	vi.unstubAllGlobals();
});
it('reads only the scoped CRM picker once and preserves actual creative and metadata', async () => {
	const fetch = vi.fn<typeof globalThis.fetch>().mockResolvedValue(Response.json(page));
	vi.stubGlobal('fetch', fetch);
	const result = await getCrmNativeTemplatePicker({
		search: 'Exact & native',
		page: 2,
		per_page: 1
	});
	expect(result.data).toEqual([row]);
	expect(fetch).toHaveBeenCalledExactlyOnceWith(
		'/api/admin/crm/contacts/native-templates?page=2&per_page=1&search=Exact+%26+native',
		expect.objectContaining({
			method: 'GET',
			cache: 'no-store',
			credentials: 'same-origin',
			redirect: 'error'
		})
	);
});
it('rejects a global catalog, mismatched pagination, duplicate IDs and fabricated numeric identities', () => {
	expect(() => parseCrmNativeTemplatePicker({ templates: [row], total: 1 })).toThrow(
		'invalid response'
	);
	expect(() => parseCrmNativeTemplatePicker(page, 1, 1)).toThrow();
	expect(() =>
		parseCrmNativeTemplatePicker({ ...page, data: [{ ...row, id: 17 }] }, 2, 1)
	).toThrow();
	expect(() =>
		parseCrmNativeTemplatePicker(
			{ ...page, data: [row, row], meta: { current_page: 1, per_page: 2, total: 2, last_page: 1 } },
			1,
			2
		)
	).toThrow();
});
it('keeps empty successful catalog distinct from failure and rejects an omitted text-body field', () => {
	expect(
		parseCrmNativeTemplatePicker({
			metrics_scope: 'contact_template_picker',
			data: [],
			meta: { current_page: 1, per_page: 25, total: 0, last_page: 1 }
		}).data
	).toEqual([]);
	const { body_text: _bodyText, ...missing } = row;
	expect(() => parseCrmNativeTemplatePicker({ ...page, data: [missing] }, 2, 1)).toThrow();
});
it('rejects invalid request bounds before fetch and never falls back to a native global endpoint after denial', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValue(Response.json({ message: 'Template access denied' }, { status: 403 }));
	vi.stubGlobal('fetch', fetch);
	await expect(getCrmNativeTemplatePicker({ page: 0 })).rejects.toThrow('bounded');
	expect(fetch).not.toHaveBeenCalled();
	await expect(getCrmNativeTemplatePicker()).rejects.toThrow('Template access denied');
	expect(fetch).toHaveBeenCalledTimes(1);
});

import { contactId, contactRequest } from './crm-contact-contract.js';
function fail(): never {
	throw new Error('The contact template picker returned an invalid response');
}
function object(value: unknown): Record<string, unknown> {
	return value && typeof value === 'object' && !Array.isArray(value)
		? (value as Record<string, unknown>)
		: fail();
}
function text(value: unknown): string {
	return typeof value === 'string' ? value : fail();
}
function count(value: unknown): number {
	return typeof value === 'number' && Number.isSafeInteger(value) && value >= 0 ? value : fail();
}
export function parseCrmNativeTemplatePicker(
	value: unknown,
	requestedPage = 1,
	requestedSize = 25
) {
	const row = object(value),
		meta = object(row.meta);
	if (row.metrics_scope !== 'contact_template_picker' || !Array.isArray(row.data)) return fail();
	const current_page = count(meta.current_page),
		per_page = count(meta.per_page),
		total = count(meta.total),
		last_page = count(meta.last_page);
	if (
		current_page !== requestedPage ||
		per_page !== requestedSize ||
		per_page < 1 ||
		last_page !== Math.max(1, Math.ceil(total / per_page))
	)
		return fail();
	const data = row.data.map((entry) => {
		const v = object(entry);
		if (typeof v.id !== 'string' || (v.body_text !== null && typeof v.body_text !== 'string'))
			return fail();
		return {
			id: contactId(v.id),
			name: text(v.name),
			subject: text(v.subject),
			body_html: text(v.body_html),
			body_text: v.body_text
		};
	});
	if (
		data.length > per_page ||
		new Set(data.map((v) => v.id)).size !== data.length ||
		data.length > total
	)
		return fail();
	return {
		metrics_scope: 'contact_template_picker' as const,
		data,
		meta: { current_page, per_page, total, last_page }
	};
}
/** Narrow CRM catalog only. Caller checks current capabilities; the server authorizes every request. */
export async function getCrmNativeTemplatePicker(
	filters: { search?: string; page?: number; per_page?: number } = {}
) {
	const page = filters.page ?? 1,
		size = filters.per_page ?? 25,
		search = filters.search ?? '';
	if (
		!Number.isSafeInteger(page) ||
		page < 1 ||
		page > 10000 ||
		!Number.isSafeInteger(size) ||
		size < 1 ||
		size > 100 ||
		new TextEncoder().encode(search).length > 500 ||
		search.includes('\0')
	)
		throw new Error('Provide bounded template search and pagination');
	const query = new URLSearchParams({ page: String(page), per_page: String(size) });
	if (search) query.set('search', search);
	return parseCrmNativeTemplatePicker(
		await contactRequest(`/native-templates?${query}`),
		page,
		size
	);
}

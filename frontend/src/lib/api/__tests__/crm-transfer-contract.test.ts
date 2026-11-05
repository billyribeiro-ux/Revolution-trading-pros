import { beforeEach } from 'vite-plus/test';
import { authorizedCrmRequestFixture } from '#lib/crm/__tests__/request-fixture.js';
let releaseCrmFixture: () => void = () => undefined;
beforeEach(() => {
	releaseCrmFixture = authorizedCrmRequestFixture();
});
afterEach(() => releaseCrmFixture());
import { afterEach, expect, it, vi } from 'vite-plus/test';
vi.mock('../client.svelte', () => ({ apiClient: { get: vi.fn(), post: vi.fn() } }));
import { CrmAPI } from '../crm.js';
import {
	importJob,
	exportJob,
	transferOptions,
	transferPage,
	transferPreview,
	transferRequest
} from '../crm-transfer-contract.js';
const id = '34ad48b0-615b-49e8-86ee-c0d9f77206ee',
	other = '18afd4a8-d2eb-46c7-852c-6b788b414dec',
	at = '2026-11-05T00:00:00Z';
function fixture() {
	return {
		id,
		type: 'contacts',
		file_name: 'contacts.csv',
		file_type: 'csv',
		status: 'pending',
		total_rows: 2,
		total_records: 2,
		processed_rows: 0,
		success_count: 0,
		created_count: 0,
		updated_count: 0,
		skipped_count: 0,
		error_count: 0,
		errors: [],
		field_mapping: {},
		settings: {},
		filters: {},
		fields: [],
		file_url: null,
		created_by: '42',
		started_at: null,
		completed_at: null,
		created_at: at,
		updated_at: at
	};
}
function completed() {
	return {
		...fixture(),
		status: 'completed',
		processed_rows: 2,
		success_count: 1,
		created_count: 1,
		skipped_count: 1,
		field_mapping: { Email: 'email' },
		settings: {
			update_existing: false,
			skip_duplicates: true,
			add_to_lists: [],
			apply_tags: [],
			contact_status: null
		},
		started_at: at,
		completed_at: at
	};
}
function exported() {
	return {
		...fixture(),
		file_name: `crm-contacts-${id}.csv`,
		status: 'completed',
		fields: ['email'],
		processed_rows: 2,
		file_url: `/api/admin/crm/exports/${id}/download`,
		started_at: at,
		completed_at: at
	};
}
afterEach(() => {
	vi.unstubAllGlobals();
	vi.clearAllMocks();
});
it('omits GET bodies and rejects accidental GET payloads before issuing a request', async () => {
	const transport = vi.fn<typeof fetch>().mockResolvedValue(Response.json({ data: [] }));
	vi.stubGlobal('fetch', transport);
	await expect(transferRequest('imports', 'GET', {})).rejects.toThrow(
		'GET transfer request cannot contain a body'
	);
	expect(transport).not.toHaveBeenCalled();
	await transferRequest('imports');
	expect(transport).toHaveBeenCalledOnce();
	expect(transport.mock.calls[0]?.[1]).not.toHaveProperty('body');
});
it('requires actual counts, lifecycle identity and a completed snapshot download', () => {
	expect(importJob(completed()).skipped_count).toBe(1);
	expect(() => importJob({ ...completed(), success_count: 2 })).toThrow(/invalid|unconfirmed/);
	expect(() => importJob({ ...completed(), error_count: 1 })).toThrow();
	expect(() => importJob(fixture(), other)).toThrow();
	expect(() => exportJob({ ...exported(), file_url: 'https://unrelated.test/download' })).toThrow();
	expect(() => exportJob({ ...exported(), file_name: 'pretend.csv' })).toThrow();
	expect(exportJob(exported()).total_records).toBe(2);
	const page = transferPage(
		{ data: [completed()], meta: { total: 2, per_page: 1, current_page: 2, total_pages: 2 } },
		importJob
	);
	expect(page.data[0]?.id).toBe(id);
});
it('checks real preview headers and paged reference selections instead of accepting malformed success', () => {
	expect(
		transferPreview({ headers: ['Email'], rows: [{ Email: 'actual@example.test' }], total_rows: 1 })
			.total_rows
	).toBe(1);
	expect(() =>
		transferPreview({ headers: ['Email'], rows: [{ Unmapped: 'value' }], total_rows: 1 })
	).toThrow();
	const options = {
		data: [{ id: '101', title: 'Beyond first100' }],
		meta: { total: 101, current_page: 2, per_page: 100, total_pages: 2 },
		selected: [{ id: '42', title: 'Selected elsewhere' }],
		missing: ['43']
	};
	expect(transferOptions(options).selected[0]?.title).toBe('Selected elsewhere');
	expect(() => transferOptions({ ...options, missing: ['42'] })).toThrow();
});
it('sends the original multipart body once without a fabricated content boundary', async () => {
	const transport = vi.fn<typeof fetch>().mockImplementation(async () => Response.json(fixture()));
	vi.stubGlobal('fetch', transport);
	const data = new FormData();
	data.set('type', 'contacts');
	data.set('file', new File(['email\na@example.test\n'], 'contacts.csv', { type: 'text/csv' }));
	expect((await new CrmAPI().createImportJob(data)).status).toBe('pending');
	expect(transport).toHaveBeenCalledExactlyOnceWith(
		'/api/admin/crm/imports',
		expect.objectContaining({
			method: 'POST',
			body: data,
			headers: {},
			credentials: 'same-origin',
			redirect: 'error'
		})
	);
});
it('does not retry a rejected or uncertain import and rejects unconfirmed execution/cancellation', async () => {
	const transport = vi
		.fn<typeof fetch>()
		.mockImplementation(async () =>
			Response.json({ error: 'Controlled unavailable import' }, { status: 503 })
		);
	vi.stubGlobal('fetch', transport);
	const api = new CrmAPI();
	await expect(api.startImportJob(id, {}, {})).rejects.toThrow('Controlled unavailable import');
	expect(transport).toHaveBeenCalledOnce();
	transport.mockImplementation(async () => Response.json(fixture()));
	await expect(api.startImportJob(id, {}, {})).rejects.toThrow(/not confirmed/);
	await expect(api.cancelImportJob(id)).rejects.toThrow(/not confirmed/);
	expect(transport).toHaveBeenCalledTimes(3);
});
it('checks export format and download content headers before creating a file', async () => {
	const transport = vi.fn<typeof fetch>().mockImplementation(async () => Response.json(exported()));
	vi.stubGlobal('fetch', transport);
	const api = new CrmAPI();
	await expect(api.createExportJob('contacts', {}, ['email'], 'json')).rejects.toThrow(
		/does not match/
	);
	transport.mockImplementation(
		async () =>
			new Response('email\na@example.test\n', {
				headers: {
					'Content-Type': 'text/csv; charset=utf-8',
					'Content-Disposition': `attachment; filename="crm-contacts-${id}.csv"`
				}
			})
	);
	expect((await api.downloadExport(id)).size).toBeGreaterThan(0);
	transport.mockImplementation(async () => Response.json({ success: true }));
	await expect(api.downloadExport(id)).rejects.toThrow(/expected export file/);
});

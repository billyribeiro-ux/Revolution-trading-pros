// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
import { downloadMemberExport } from '../member-export';
afterEach(() => vi.unstubAllGlobals());
it('rejects real backend errors before a file can be downloaded', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValue(Response.json({ message: 'Narrow the date range' }, { status: 422 }));
	vi.stubGlobal('fetch', fetch);
	await expect(downloadMemberExport('/fixture', 'pdf', null)).rejects.toThrow(
		'Narrow the date range'
	);
	expect(fetch).toHaveBeenCalledOnce();
	expect(fetch.mock.calls[0]?.[1]?.headers).toEqual({});
});
it('does not accept a successful JSON envelope as an Excel file', async () => {
	vi.stubGlobal('fetch', vi.fn().mockResolvedValue(Response.json({ success: true })));
	await expect(downloadMemberExport('/fixture', 'xlsx', 'fixture')).rejects.toThrow(
		'requested member export format'
	);
});
it('checks the actual binary signature, then returns the real bytes', async () => {
	const mime = 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet';
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValueOnce(new Response('fake XML', { headers: { 'Content-Type': mime } }))
		.mockResolvedValueOnce(
			new Response(new Uint8Array([0x50, 0x4b, 0x03, 0x04, 0x01]), {
				headers: { 'Content-Type': mime }
			})
		);
	vi.stubGlobal('fetch', fetch);
	await expect(downloadMemberExport('/fixture', 'xlsx', null)).rejects.toThrow('file is invalid');
	expect(
		new Uint8Array(await (await downloadMemberExport('/fixture', 'xlsx', null)).arrayBuffer())
	).toEqual(new Uint8Array([0x50, 0x4b, 0x03, 0x04, 0x01]));
});

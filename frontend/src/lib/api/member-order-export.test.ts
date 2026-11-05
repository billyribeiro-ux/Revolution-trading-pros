// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
import { downloadMemberOrders } from './member-order-export';
const heading =
	'id,member_id,order_number,recorded_status,currency,total_units,refund_units,net_recorded_units,created_at_recorded,completed_at_recorded,refunded_at_recorded,timestamp_basis\r\n';
async function csv(body: string, count = 1, change: Record<string, string> = {}) {
	const bytes = new TextEncoder().encode('\uFEFF' + body),
		sha = Array.from(new Uint8Array(await crypto.subtle.digest('SHA-256', bytes)), (v) =>
			v.toString(16).padStart(2, '0')
		).join('');
	return new Response(bytes, {
		headers: {
			'content-type': 'text/csv; charset=utf-8',
			'content-disposition': 'attachment; filename="member-9-orders.csv"',
			'x-export-member-id': '9',
			'x-export-row-count': String(count),
			'x-export-sha256': sha,
			'x-export-scope': 'all_recorded_member_orders',
			...change
		}
	});
}
const row =
	'7,9,"Order, with ""quotes""\nsecond line 🧭",paid,EUR,12.34,2.00,10.34,,12000-01-01T00:00:00,,recorded without timezone\r\n';
afterEach(() => {
	vi.unstubAllGlobals();
});
it('confirms actual bytes, scoped identity and logical multiline CSV records before returning a file', async () => {
	const fetcher = vi.fn<typeof fetch>().mockResolvedValue(await csv(heading + row));
	vi.stubGlobal('fetch', fetcher);
	const result = await downloadMemberOrders('9');
	expect(result.filename).toBe('member-9-orders.csv');
	expect(result.rows).toBe(1);
	expect(await result.blob.text()).toBe(heading + row);
	expect(fetcher).toHaveBeenCalledExactlyOnceWith(
		'/api/admin/member-management/9/orders/export?format=csv',
		expect.objectContaining({ cache: 'no-store', credentials: 'same-origin', redirect: 'error' })
	);
});
it('accepts a real headers-only export only when the server receipt confirms zero rows', async () => {
	vi.stubGlobal('fetch', vi.fn<typeof fetch>().mockResolvedValue(await csv(heading, 0)));
	expect((await downloadMemberOrders(9)).rows).toBe(0);
	vi.stubGlobal('fetch', vi.fn<typeof fetch>().mockResolvedValue(await csv(heading, 1)));
	await expect(downloadMemberOrders(9)).rejects.toThrow('row count');
});
it('rejects wrong member, attachment, format, scope, checksum and truncation even after HTTP200', async () => {
	const changes: Array<Record<string, string>> = [
		{ 'x-export-member-id': '10' },
		{ 'content-type': 'application/json' },
		{ 'content-disposition': 'attachment; filename="other.csv"' },
		{ 'x-export-scope': 'current_page' },
		{ 'x-export-sha256': '0'.repeat(64) },
		{ 'x-export-row-count': '2' }
	];
	for (const change of changes) {
		vi.stubGlobal(
			'fetch',
			vi.fn<typeof fetch>().mockResolvedValue(await csv(heading + row, 1, change))
		);
		await expect(downloadMemberOrders(9)).rejects.toThrow();
	}
	for (const invalid of [
		heading + row.slice(0, -2),
		heading + '"unclosed',
		heading + '1,2,3\r\n',
		'{}'
	]) {
		vi.stubGlobal('fetch', vi.fn<typeof fetch>().mockResolvedValue(await csv(invalid)));
		await expect(downloadMemberOrders(9)).rejects.toThrow();
	}
});
it('retains the actual storage error and makes one request without creating a file', async () => {
	const fetcher = vi
		.fn<typeof fetch>()
		.mockResolvedValue(Response.json({ message: 'Export audit unavailable' }, { status: 503 }));
	vi.stubGlobal('fetch', fetcher);
	await expect(downloadMemberOrders(9)).rejects.toThrow('Export audit unavailable');
	expect(fetcher).toHaveBeenCalledTimes(1);
});

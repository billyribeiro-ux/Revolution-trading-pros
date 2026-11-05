// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
vi.mock('../client.svelte', () => ({ apiClient: {} }));
vi.mock('#lib/stores/auth.svelte.js', () => ({ getAuthToken: () => null }));
vi.mock('../member-export', () => ({
	downloadMemberExport: vi.fn(async () => new Blob(['fixture']))
}));
import { membersApi } from '../members.js';
import { downloadMemberExport } from '../member-export';
afterEach(() => vi.clearAllMocks());
it('exports all established active filters while excluding list pagination for both entry points', async () => {
	const filters = {
		segment: 8,
		tag: 9,
		search: 'Exact & Unicode 李',
		status: 'active' as const,
		product_id: 10,
		plan_id: 11,
		date_from: '2020-02-29',
		date_to: '2020-03-01',
		spending_tier: 'high' as const,
		sort_by: 'created_at',
		sort_dir: 'asc' as const,
		page: 7,
		per_page: 1
	};
	await membersApi.exportMembers(filters);
	await membersApi.exportMembersAdvanced({ ...filters, format: 'xlsx' });
	const calls = vi.mocked(downloadMemberExport).mock.calls;
	expect(calls).toHaveLength(2);
	for (const [index, call] of calls.entries()) {
		const url = new URL(call[0], 'http://fixture.test');
		expect(url.pathname).toBe(
			index === 0 ? '/api/admin/members/export' : '/api/admin/member-management/export'
		);
		expect(url.searchParams.has('page')).toBe(false);
		expect(url.searchParams.has('per_page')).toBe(false);
		for (const [key, value] of Object.entries(filters))
			if (!['page', 'per_page'].includes(key))
				expect(url.searchParams.get(key)).toBe(String(value));
		expect(call[1]).toBe(index === 0 ? 'csv' : 'xlsx');
		expect(call[2]).toBeNull();
	}
});

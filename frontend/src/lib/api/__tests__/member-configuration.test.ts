// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
const api = vi.hoisted(() => ({ request: vi.fn() }));
vi.mock('#lib/utils/adminFetch.js', () => ({ adminFetch: api.request }));
import {
	applyMemberFilter,
	deleteMemberConfiguration,
	duplicateMemberFilter,
	memberConfigPage,
	memberFilterFields,
	memberTag,
	saveMemberFilter,
	savedMemberFilter
} from '../member-configuration.js';
afterEach(() => api.request.mockReset());
const filter = {
	id: 42,
	name: 'Recorded filter',
	filters: { tag: 12, status: 'active' },
	created_at: '2026-01-01T00:00:00',
	usage_count: 0
};
it('distinguishes actual zero counts from absent or malformed measurements', () => {
	expect(memberTag({ id: 12, name: 'Cohort', color: '#123456', member_count: 0 }).memberCount).toBe(
		0
	);
	expect(savedMemberFilter(filter).usageCount).toBe(0);
	expect(savedMemberFilter({ ...filter, usage_count: null }).usageCount).toBeNull();
	for (const value of [undefined, null, -1, '2']) {
		expect(() =>
			memberTag({ id: 12, name: 'Cohort', color: '#123456', member_count: value })
		).toThrow();
		if (value !== null)
			expect(() => savedMemberFilter({ ...filter, usage_count: value })).toThrow();
	}
});
it('rejects invalid editor JSON structures instead of sending fabricated settings', () => {
	expect(memberFilterFields({ tag: 12, search: 'Fixture' })).toEqual({
		tag: 12,
		search: 'Fixture'
	});
	for (const value of [[], null, { tag: [] }, { search: { nested: 'x' } }])
		expect(() => memberFilterFields(value)).toThrow();
});

it('preserves unusual filter keys for server validation instead of silently turning them into an empty audience filter', () => {
	const value: unknown = JSON.parse('{"__proto__":"literal"}');
	const fields = memberFilterFields(value);
	expect(Object.hasOwn(fields, '__proto__')).toBe(true);
	expect(Object.getPrototypeOf(fields)).toBe(Object.prototype);
	expect(JSON.stringify(fields)).toBe('{"__proto__":"literal"}');
});
it('requires pagination metadata and preserves search arguments', async () => {
	api.request.mockResolvedValueOnce({ filters: [filter], meta: { total: 21, current_page: 2 } });
	expect(await memberConfigPage('filters', 2, 'A & B')).toMatchObject({ total: 21, page: 2 });
	expect(api.request).toHaveBeenCalledWith(
		'/api/admin/members/filters?page=2&per_page=20&search=A+%26+B'
	);
	api.request.mockResolvedValueOnce({ filters: [] });
	await expect(memberConfigPage('filters', 1)).rejects.toThrow();
});
it('requires a recorded apply acknowledgment for the exact filter', async () => {
	api.request.mockResolvedValueOnce({
		applied: true,
		id: 42,
		filters: { tag: 12 },
		usage_count: 1
	});
	expect(await applyMemberFilter(42)).toEqual({ filters: { tag: 12 }, usageCount: 1 });
	api.request.mockResolvedValueOnce({ applied: true, id: 43, filters: {}, usage_count: 1 });
	await expect(applyMemberFilter(42)).rejects.toThrow('not confirmed');
	expect(api.request).toHaveBeenCalledTimes(2);
});
it('rejects identity mismatches when saving or duplicating a filter', async () => {
	api.request.mockResolvedValueOnce({ ...filter, id: 43 });
	await expect(saveMemberFilter({ name: 'Updated', filters: {} }, 42)).rejects.toThrow(
		'not confirmed'
	);
	api.request.mockResolvedValueOnce(filter);
	await expect(duplicateMemberFilter(42)).rejects.toThrow('not confirmed');
	api.request.mockResolvedValueOnce({ ...filter, id: 44 });
	expect((await duplicateMemberFilter(42)).id).toBe(44);
});
it('requires confirmed deletion before a row can disappear', async () => {
	api.request.mockResolvedValueOnce({ message: 'OK' });
	await expect(deleteMemberConfiguration('tags', 12)).rejects.toThrow();
	api.request.mockResolvedValueOnce({ deleted: true, id: 12 });
	await expect(deleteMemberConfiguration('tags', 12)).resolves.toBeUndefined();
});

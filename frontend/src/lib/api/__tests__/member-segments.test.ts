// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
const api = vi.hoisted(() => ({ request: vi.fn() }));
vi.mock('#lib/utils/adminFetch.js', () => ({ adminFetch: api.request }));
import {
	deleteMemberSegment,
	getSegmentAnalytics,
	getSegmentMembers,
	exportMemberSegment,
	listMemberSegments,
	memberSegment,
	saveMemberSegment
} from '../member-segments.js';
afterEach(() => api.request.mockReset());
const row = {
	id: 42,
	name: 'Actual cohort',
	description: null,
	type: 'smart',
	conditions: [{ field: 'total_spent', operator: 'gt', value: '400.00' }],
	memberCount: 1,
	lastUpdated: '2026-01-01T00:00:00Z',
	isSystem: false
};
it('keeps real zero counts distinct from malformed or absent counts', () => {
	expect(memberSegment({ ...row, memberCount: 0 })).toMatchObject({
		memberCount: 0,
		description: ''
	});
	for (const value of [undefined, null, -1, '0', NaN])
		expect(() => memberSegment({ ...row, memberCount: value })).toThrow();
	expect(() => memberSegment({ ...row, conditions: {} })).toThrow();
});
it('uses actual pagination and rejects an unconfirmed empty list', async () => {
	api.request.mockResolvedValueOnce({
		segments: [row],
		meta: { total: 21, current_page: 2, per_page: 20 }
	});
	expect(await listMemberSegments({ page: 2, search: 'A & B' })).toMatchObject({
		total: 21,
		page: 2,
		segments: [{ id: 42 }]
	});
	expect(api.request).toHaveBeenCalledWith('/api/admin/members/segments?page=2&search=A+%26+B');
	api.request.mockResolvedValueOnce({ segments: [] });
	await expect(listMemberSegments()).rejects.toThrow();
});
it('persists editable rules and requires a real record before acknowledging save', async () => {
	const input = { name: row.name, description: '', conditions: row.conditions };
	api.request.mockResolvedValueOnce(row);
	expect((await saveMemberSegment(input, 42)).id).toBe(42);
	expect(api.request).toHaveBeenCalledWith('/api/admin/members/segments/42', {
		method: 'PUT',
		body: JSON.stringify({ name: row.name, description: '', rules: row.conditions })
	});
	api.request.mockResolvedValueOnce({ message: 'OK' });
	await expect(saveMemberSegment(input)).rejects.toThrow();
	expect(api.request).toHaveBeenCalledTimes(2);
});
it('requires confirmation for the exact deleted record', async () => {
	api.request.mockResolvedValueOnce({ deleted: true, id: 43 });
	await expect(deleteMemberSegment(42)).rejects.toThrow('not confirmed');
	api.request.mockResolvedValueOnce({ deleted: true, id: 42 });
	await expect(deleteMemberSegment(42)).resolves.toBeUndefined();
});

it('preserves unavailable measurements and rejects incomplete analytics instead of generating values', async () => {
	api.request.mockResolvedValueOnce({
		total_members: 0,
		growth_30d: null,
		avg_spending: null,
		avg_lifetime_days: null,
		email_open_rate: null,
		conversion_rate: null,
		top_products: []
	});
	expect(await getSegmentAnalytics(42)).toMatchObject({
		total_members: 0,
		growth_30d: null,
		email_open_rate: null,
		avg_spending: null
	});
	api.request.mockResolvedValueOnce({ total_members: 0, top_products: [] });
	await expect(getSegmentAnalytics(42)).rejects.toThrow();
	api.request.mockResolvedValueOnce({ members: [] });
	await expect(getSegmentMembers(42)).rejects.toThrow();
});

it('requires an actual CSV response before exposing an export download', async () => {
	api.request.mockResolvedValueOnce(
		new Response('id,name\r\n42,Fixture\r\n', {
			headers: { 'Content-Type': 'text/csv; charset=utf-8' }
		})
	);
	expect(await (await exportMemberSegment(42)).text()).toContain('42,Fixture');
	expect(api.request).toHaveBeenCalledWith('/api/admin/members/segments/42/export', {
		rawResponse: true
	});
	api.request.mockResolvedValueOnce(Response.json({ message: 'Export started' }));
	await expect(exportMemberSegment(42)).rejects.toThrow('not confirmed');
});

it('preserves static membership when editing only its display information', async () => {
	api.request.mockResolvedValueOnce({ ...row, type: 'static', conditions: [] });
	await saveMemberSegment(
		{ name: 'Static cohort', description: 'Updated', type: 'static', conditions: [] },
		42
	);
	expect(api.request).toHaveBeenCalledWith('/api/admin/members/segments/42', {
		method: 'PUT',
		body: JSON.stringify({ name: 'Static cohort', description: 'Updated' })
	});
});

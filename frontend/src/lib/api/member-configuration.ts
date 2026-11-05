import { adminUtcTimestamp } from './admin-timestamp.js';
import { adminFetch } from '#lib/utils/adminFetch.js';
export interface MemberTag {
	id: number;
	name: string;
	color: string;
	memberCount: number;
}
export interface SavedMemberFilter {
	id: number;
	name: string;
	filters: Record<string, string | number>;
	createdAt: string;
	usageCount: number | null;
}
function object(value: unknown): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		throw new Error('Invalid member configuration response');
	const row = value as Record<string, unknown>;
	if (row.success === false || row.error)
		throw new Error('Member configuration operation was not confirmed');
	return row;
}
function integer(value: unknown, min = 0): number {
	if (typeof value !== 'number' || !Number.isSafeInteger(value) || value < min)
		throw new Error('Invalid member configuration count or identifier');
	return value;
}
function text(value: unknown): string {
	if (typeof value !== 'string') throw new Error('Invalid member configuration text');
	return value;
}
export function memberFilterFields(input: unknown): Record<string, string | number> {
	return Object.fromEntries(
		Object.entries(object(input)).map(([key, value]) => {
			if (typeof value !== 'string' && (typeof value !== 'number' || !Number.isFinite(value)))
				throw new Error('Invalid saved member filter');
			return [key, value] as const;
		})
	);
}

export function memberTag(value: unknown): MemberTag {
	const row = object(value),
		color = row.color === null ? '#64748b' : text(row.color);
	if (!/^#[0-9a-f]{6}$/i.test(color)) throw new Error('Invalid member tag color');
	return {
		id: integer(row.id, 1),
		name: text(row.name),
		color,
		memberCount: integer(row.member_count)
	};
}
export function savedMemberFilter(value: unknown): SavedMemberFilter {
	const row = object(value);
	return {
		id: integer(row.id, 1),
		name: text(row.name),
		filters: memberFilterFields(row.filters),
		createdAt: adminUtcTimestamp(text(row.created_at)),
		usageCount: row.usage_count === null ? null : integer(row.usage_count)
	};
}
export async function memberConfigPage(kind: 'tags' | 'filters', page: number, search = '') {
	const params = new URLSearchParams({ page: String(integer(page, 1)), per_page: '20' });
	if (search.trim()) params.set('search', search.trim());
	const result = object(await adminFetch<unknown>(`/api/admin/members/${kind}?${params}`)),
		meta = object(result.meta);
	const rows = result[kind];
	if (!Array.isArray(rows)) throw new Error('Invalid member configuration list');
	return { rows, total: integer(meta.total), page: integer(meta.current_page, 1) };
}
export async function saveMemberFilter(
	value: { name: string; filters: Record<string, string | number> },
	id?: number
) {
	const saved = savedMemberFilter(
		await adminFetch<unknown>(
			`/api/admin/members/filters${id === undefined ? '' : `/${integer(id, 1)}`}`,
			{ method: id === undefined ? 'POST' : 'PUT', body: JSON.stringify(value) }
		)
	);
	if (id !== undefined && saved.id !== id)
		throw new Error('Saved member filter identity was not confirmed');
	return saved;
}
export async function duplicateMemberFilter(id: number) {
	const copy = savedMemberFilter(
		await adminFetch<unknown>(`/api/admin/members/filters/${integer(id, 1)}/duplicate`, {
			method: 'POST'
		})
	);
	if (copy.id === id) throw new Error('Duplicate member filter identity was not confirmed');
	return copy;
}
export async function applyMemberFilter(id: number) {
	const result = object(
		await adminFetch<unknown>(`/api/admin/members/filters/${integer(id, 1)}/apply`, {
			method: 'POST'
		})
	);
	if (result.applied !== true || result.id !== id)
		throw new Error('Member filter apply was not confirmed');
	return { filters: memberFilterFields(result.filters), usageCount: integer(result.usage_count) };
}
export async function deleteMemberConfiguration(kind: 'tags' | 'filters', id: number) {
	const result = object(
		await adminFetch<unknown>(`/api/admin/members/${kind}/${integer(id, 1)}`, { method: 'DELETE' })
	);
	if (result.deleted !== true || result.id !== id)
		throw new Error('Member configuration deletion was not confirmed');
}

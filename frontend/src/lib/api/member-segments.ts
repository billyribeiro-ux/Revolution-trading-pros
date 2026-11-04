import { adminFetch } from '#lib/utils/adminFetch.js';

export interface MemberCondition {
	field: string;
	operator: string;
	value: string | number;
}
export interface MemberSegment {
	id: number;
	name: string;
	description: string;
	type: 'smart' | 'static';
	conditions: MemberCondition[];
	memberCount: number;
	lastUpdated: string;
	isSystem: boolean;
}
function record(value: unknown): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		throw new Error('Invalid member segment response');
	return value as Record<string, unknown>;
}
function text(row: Record<string, unknown>, key: string): string {
	const value = row[key];
	if (typeof value !== 'string') throw new Error('Invalid member segment response');
	return value;
}
function integer(value: unknown, min = 0): number {
	if (typeof value !== 'number' || !Number.isSafeInteger(value) || value < min)
		throw new Error('Invalid member segment count or identifier');
	return value;
}
export function memberSegment(value: unknown): MemberSegment {
	const row = record(value);
	if (
		!Array.isArray(row.conditions) ||
		!['smart', 'static'].includes(String(row.type)) ||
		typeof row.isSystem !== 'boolean'
	)
		throw new Error('Invalid member segment definition');
	const conditions = row.conditions.map((entry: unknown) => {
		const rule = record(entry);
		if (
			typeof rule.value !== 'string' &&
			(typeof rule.value !== 'number' || !Number.isFinite(rule.value))
		)
			throw new Error('Invalid member segment condition');
		return { field: text(rule, 'field'), operator: text(rule, 'operator'), value: rule.value };
	});
	return {
		id: integer(row.id, 1),
		name: text(row, 'name'),
		description: row.description === null ? '' : text(row, 'description'),
		type: row.type as 'smart' | 'static',
		conditions,
		memberCount: integer(row.memberCount),
		lastUpdated: text(row, 'lastUpdated'),
		isSystem: row.isSystem
	};
}
export async function listMemberSegments(
	params: { page?: number; per_page?: number; search?: string } = {}
) {
	const search = new URLSearchParams();
	for (const [key, value] of Object.entries(params))
		if (value !== undefined) search.set(key, String(value));
	const row = record(await adminFetch<unknown>(`/api/admin/members/segments?${search}`));
	const meta = record(row.meta);
	if (!Array.isArray(row.segments)) throw new Error('Invalid member segment list');
	return {
		segments: row.segments.map(memberSegment),
		total: integer(meta.total),
		page: integer(meta.current_page, 1),
		per_page: integer(meta.per_page, 1)
	};
}
export async function saveMemberSegment(
	value: { name: string; description: string; conditions: MemberCondition[] },
	id?: number
) {
	return memberSegment(
		await adminFetch<unknown>(
			`/api/admin/members/segments${id === undefined ? '' : `/${integer(id, 1)}`}`,
			{
				method: id === undefined ? 'POST' : 'PUT',
				body: JSON.stringify({
					name: value.name,
					description: value.description,
					rules: value.conditions
				})
			}
		)
	);
}
export async function deleteMemberSegment(id: number) {
	const result = record(
		await adminFetch<unknown>(`/api/admin/members/segments/${integer(id, 1)}`, { method: 'DELETE' })
	);
	if (result.deleted !== true || result.id !== id)
		throw new Error('Member segment deletion was not confirmed');
}

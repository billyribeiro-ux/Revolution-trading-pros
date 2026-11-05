import { adminUtcTimestamp } from './admin-timestamp.js';
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
		lastUpdated: adminUtcTimestamp(text(row, 'lastUpdated')),
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
	value: {
		name: string;
		description: string;
		conditions: MemberCondition[];
		type?: 'smart' | 'static';
	},
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
					...(value.type === 'static' && !value.conditions.length
						? {}
						: { rules: value.conditions })
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

export interface SegmentMemberPreview {
	id: number;
	name: string;
	email: string;
	avatar?: string;
	status: string;
	total_spent: number;
	joined_at: string;
}
export interface SegmentAnalytics {
	total_members: number;
	growth_30d: number | null;
	avg_spending: number | null;
	avg_lifetime_days: number | null;
	email_open_rate: number | null;
	conversion_rate: number | null;
	top_products: { name: string; count: number }[];
}
function measured(value: unknown, nullable = false): number | null {
	if (nullable && value === null) return null;
	if (typeof value !== 'number' || !Number.isFinite(value))
		throw new Error('Invalid member segment measurement');
	return value;
}
export async function getSegmentMembers(id: number): Promise<SegmentMemberPreview[]> {
	const data = record(
		await adminFetch<unknown>(`/api/admin/members?segment=${integer(id, 1)}&per_page=5`)
	);
	integer(record(data.pagination).total);
	if (!Array.isArray(data.members)) throw new Error('Invalid segment member list');
	return data.members.map((value: unknown) => {
		const row = record(value),
			spending = measured(row.total_spent);
		if (spending === null) throw new Error('Invalid member spending');
		return {
			id: integer(row.id, 1),
			name: text(row, 'name'),
			email: text(row, 'email'),
			status: text(row, 'status'),
			total_spent: spending,
			joined_at: text(row, 'joined_at'),
			...(typeof row.avatar === 'string' ? { avatar: row.avatar } : {})
		};
	});
}
export async function getSegmentAnalytics(id: number): Promise<SegmentAnalytics> {
	const row = record(
		await adminFetch<unknown>(`/api/admin/members/segments/${integer(id, 1)}/analytics`)
	);
	if (!Array.isArray(row.top_products)) throw new Error('Invalid segment product measurements');
	return {
		total_members: integer(row.total_members),
		growth_30d: measured(row.growth_30d, true),
		avg_spending: measured(row.avg_spending, true),
		avg_lifetime_days: measured(row.avg_lifetime_days, true),
		email_open_rate: measured(row.email_open_rate, true),
		conversion_rate: measured(row.conversion_rate, true),
		top_products: row.top_products.map((value: unknown) => {
			const product = record(value);
			return { name: text(product, 'name'), count: integer(product.count) };
		})
	};
}
export async function exportMemberSegment(id: number): Promise<Blob> {
	const response = await adminFetch<Response>(
		`/api/admin/members/segments/${integer(id, 1)}/export`,
		{ rawResponse: true }
	);
	if (!response.headers.get('Content-Type')?.startsWith('text/csv'))
		throw new Error('Segment CSV was not confirmed');
	return response.blob();
}

import { memberIdentity, memberProfileRequest } from './member-profile.js';
function invalid(): never {
	throw new Error('The member service returned an unconfirmed action receipt');
}
function object(v: unknown): Record<string, unknown> {
	return v !== null && typeof v === 'object' && !Array.isArray(v)
		? (v as Record<string, unknown>)
		: invalid();
}
function text(v: unknown): string {
	return typeof v === 'string' ? v : invalid();
}
function count(v: unknown): number {
	return typeof v === 'number' && Number.isSafeInteger(v) && v >= 0 ? v : invalid();
}
export function memberNumericIdentity(v: string | number): number {
	const id = memberIdentity(v),
		n = Number(id);
	if (!Number.isSafeInteger(n) || String(n) !== id)
		throw new Error('This older action cannot safely represent the recorded member identity');
	return n;
}
function tag(v: unknown) {
	const t = object(v);
	return { id: memberIdentity(t.id), name: text(t.name), slug: text(t.slug) };
}
export type ProfileTag = ReturnType<typeof tag>;
export function parseProfileTagCatalog(v: unknown, page: number, search: string) {
	const r = object(v),
		m = object(r.meta),
		total = count(m.total),
		per_page = count(m.per_page),
		current_page = count(m.current_page),
		total_pages = count(m.total_pages);
	if (
		current_page !== page ||
		per_page !== 25 ||
		total_pages !== Math.ceil(total / per_page) ||
		!Array.isArray(r.tags)
	)
		return invalid();
	const tags = r.tags.map(tag);
	if (
		tags.length !== Math.min(per_page, Math.max(0, total - (page - 1) * per_page)) ||
		new Set(tags.map((t) => t.id)).size !== tags.length
	)
		return invalid();
	return { tags, total, per_page, current_page, total_pages, search };
}
export function parseProfileTagAssignment(v: unknown, tagId: string) {
	const r = object(v),
		assigned = count(r.assigned),
		already = count(r.already_assigned);
	if (
		r.requested !== 1 ||
		assigned + already !== 1 ||
		!Array.isArray(r.tags) ||
		r.tags.length !== 1 ||
		tag(r.tags[0]).id !== memberIdentity(tagId)
	)
		return invalid();
	return { assigned, already_assigned: already };
}
export function parseProfileTagRemoval(v: unknown, member: string, tagId: string) {
	const r = object(v);
	if (
		memberIdentity(r.user_id) !== memberIdentity(member) ||
		memberIdentity(r.tag_id) !== memberIdentity(tagId) ||
		typeof r.removed !== 'boolean'
	)
		return invalid();
	return { removed: r.removed };
}
export function operationalExpiry(value: string): string {
	const raw = value.length === 16 ? `${value}:00` : value;
	if (!/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}$/.test(raw))
		throw new Error('Choose a complete expiry in operational UTC');
	const ms = Date.parse(`${raw}Z`);
	if (!Number.isFinite(ms) || new Date(ms).toISOString().slice(0, 19) !== raw)
		throw new Error('Choose a valid expiry in operational UTC');
	return raw;
}
export function extendedRecordedExpiry(recorded: string | null, days: number): string {
	if (!recorded || !Number.isSafeInteger(days) || days < 1 || days > 3650) return '';
	try {
		const raw = operationalExpiry(recorded.replace(/\.0+$/, '')),
			ms = Date.parse(`${raw}Z`) + days * 86400000;
		const next = new Date(ms).toISOString();
		return /^\d{4}-/.test(next) ? next.slice(0, 16) : '';
	} catch {
		return '';
	}
}
export function parseMembershipAction(
	v: unknown,
	member: string,
	expected: { id?: string; planId?: string; expiresAt?: string | null }
) {
	const r = object(object(v).membership),
		id = memberIdentity(r.id);
	if (
		memberIdentity(r.user_id) !== memberIdentity(member) ||
		(expected.id && id !== expected.id) ||
		(expected.planId && memberIdentity(r.plan_id) !== expected.planId) ||
		(expected.expiresAt !== undefined && r.expires_at !== expected.expiresAt)
	)
		return invalid();
	return {
		id,
		user_id: memberIdentity(r.user_id),
		plan_id: memberIdentity(r.plan_id),
		expires_at: r.expires_at === null ? null : text(r.expires_at),
		status: text(r.status)
	};
}
export const memberProfileActions = {
	tags(page = 1, search = '', signal?: AbortSignal) {
		if (!Number.isSafeInteger(page) || page < 1 || page > 10000 || search.length > 255)
			throw new Error('Invalid tag search');
		const q = new URLSearchParams({ page: String(page), per_page: '25', search });
		return memberProfileRequest(
			`/api/admin/members/tags?${q}`,
			(v) => parseProfileTagCatalog(v, page, search),
			{ signal }
		);
	},
	createTag(name: string) {
		const normalized = name.trim();
		if (!normalized || normalized.length > 255)
			throw new Error('Enter a tag name up to 255 characters');
		return memberProfileRequest(
			'/api/admin/members/tags',
			(v) => {
				const result = tag(v);
				return result.name === normalized ? result : invalid();
			},
			{ method: 'POST', body: { name: normalized } }
		);
	},
	assignTag(member: string, tagId: string) {
		return memberProfileRequest(
			'/api/admin/members/tags/assign',
			(v) => parseProfileTagAssignment(v, tagId),
			{
				method: 'POST',
				body: { user_id: memberNumericIdentity(member), tag_id: memberNumericIdentity(tagId) }
			}
		);
	},
	removeTag(member: string, tagId: string) {
		return memberProfileRequest(
			'/api/admin/members/tags/unassign',
			(v) => parseProfileTagRemoval(v, member, tagId),
			{
				method: 'DELETE',
				body: { user_id: memberNumericIdentity(member), tag_id: memberNumericIdentity(tagId) }
			}
		);
	},
	grant(member: string, planId: string, expiresAt: string | null) {
		return memberProfileRequest(
			'/api/admin/user-memberships',
			(v) => parseMembershipAction(v, member, { planId, expiresAt }),
			{
				method: 'POST',
				body: {
					user_id: memberNumericIdentity(member),
					plan_id: memberNumericIdentity(planId),
					expires_at: expiresAt,
					status: 'active'
				}
			}
		);
	},
	extend(member: string, id: string, expiresAt: string) {
		return memberProfileRequest(
			`/api/admin/user-memberships/${memberIdentity(id)}`,
			(v) => parseMembershipAction(v, member, { id, expiresAt }),
			{ method: 'PUT', body: { expires_at: expiresAt } }
		);
	},
	revoke(id: string) {
		return memberProfileRequest(
			`/api/admin/user-memberships/${memberIdentity(id)}`,
			(v) => {
				if (object(v).message !== 'Membership revoked successfully') return invalid();
				return true;
			},
			{ method: 'DELETE' }
		);
	},
	moderate(
		member: string,
		action: 'ban' | 'suspend' | 'unban',
		reason: string,
		days: number | null
	) {
		if (days !== null && (!Number.isSafeInteger(days) || days < 1 || days > 3650))
			throw new Error('Enter a duration from 1 to 3650 days');
		return memberProfileRequest(
			`/api/admin/member-management/${memberIdentity(member)}/${action}`,
			(v) => {
				if (!text(object(v).message).trim()) return invalid();
				return true;
			},
			{
				method: 'POST',
				body:
					action === 'unban'
						? {}
						: { reason: reason.trim() || undefined, duration_days: days ?? undefined }
			}
		);
	}
};

import { assertCrmClientRequest } from '#lib/crm/request-gate.js';
import { currentCrmPolicy } from '#lib/config/crm-policy.js';
import { crmActions, matchesCrmParameter, parseCrmJson, type CrmAction } from '#lib/crm/policy.js';
import { notifyCrmDenial } from '#lib/crm/denial.js';
export interface RolePermission {
	module: string;
	action: CrmAction;
	allowed: boolean;
}
export interface ManagerRoleRecord {
	id: string;
	name: string;
	slug: string;
	description: string | null;
	permissions: RolePermission[];
	users_count: number;
	is_default: boolean;
	version: number;
	created_at: string;
	updated_at: string;
}
export interface ManagerRecord {
	id: string;
	user_id: string;
	role_id: string;
	name: string | null;
	email: string;
	avatar: string | null;
	assigned_at: string;
	version: number;
	is_active: boolean;
	global_role: string | null;
	role: ManagerRoleRecord;
}
export interface ManagerUserOption {
	id: string;
	name: string | null;
	email: string;
	avatar: string | null;
	global_role: string | null;
	manager_id: string | null;
	role_id: string | null;
}
export interface ManagerPage<T> {
	data: T[];
	meta: { current_page: number; per_page: number; total: number; last_page: number };
}
export interface RolePatch {
	name?: string;
	slug?: string;
	description?: string | null;
	permissions?: RolePermission[];
	is_default?: boolean;
	expected_version?: number;
}
function bad(): never {
	throw new Error('The service returned malformed CRM security records');
}
function object(value: unknown, keys: string[]): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value)) return bad();
	const row = value as Record<string, unknown>;
	if (keys.length !== Object.keys(row).length || keys.some((key) => !Object.hasOwn(row, key)))
		return bad();
	return row;
}
function text(value: unknown): string {
	if (typeof value !== 'string' || !value.trim()) return bad();
	return value;
}
function nullable(value: unknown): string | null {
	return value === null ? null : text(value);
}
export function managerId(value: unknown): string {
	if (typeof value !== 'string' || !matchesCrmParameter('positive_integer', value)) return bad();
	return value;
}
function count(value: unknown, minimum = 0): number {
	if (typeof value !== 'number' || !Number.isSafeInteger(value) || value < minimum) return bad();
	return value;
}
function boolean(value: unknown): boolean {
	if (typeof value !== 'boolean') return bad();
	return value;
}
function date(value: unknown): string {
	const result = text(value);
	if (!Number.isFinite(Date.parse(result))) return bad();
	return result;
}
export function parseManagerRole(value: unknown): ManagerRoleRecord {
	const r = object(value, [
		'id',
		'name',
		'slug',
		'description',
		'permissions',
		'users_count',
		'is_default',
		'version',
		'created_at',
		'updated_at'
	]);
	if (!Array.isArray(r.permissions) || r.permissions.length > 100) return bad();
	const policy = currentCrmPolicy().policy,
		seen = new Set<string>();
	const permissions = r.permissions.map((value) => {
		const row = object(value, ['module', 'action', 'allowed']),
			module = text(row.module),
			action = crmActions.find((action) => action === row.action);
		if (
			!policy.modules.some((item) => item.id === module) ||
			!action ||
			seen.has(`${module}:${action}`)
		)
			return bad();
		seen.add(`${module}:${action}`);
		return { module, action, allowed: boolean(row.allowed) };
	});
	return {
		id: managerId(r.id),
		name: text(r.name),
		slug: text(r.slug),
		description: nullable(r.description),
		permissions,
		users_count: count(r.users_count),
		is_default: boolean(r.is_default),
		version: count(r.version, 1),
		created_at: date(r.created_at),
		updated_at: date(r.updated_at)
	};
}
export function parseManager(value: unknown): ManagerRecord {
	const r = object(value, [
			'id',
			'user_id',
			'role_id',
			'name',
			'email',
			'avatar',
			'assigned_at',
			'version',
			'is_active',
			'global_role',
			'role'
		]),
		role = parseManagerRole(r.role);
	if (role.id !== managerId(r.role_id)) return bad();
	return {
		id: managerId(r.id),
		user_id: managerId(r.user_id),
		role_id: role.id,
		name: nullable(r.name),
		email: text(r.email),
		avatar: nullable(r.avatar),
		assigned_at: date(r.assigned_at),
		version: count(r.version, 1),
		is_active: boolean(r.is_active),
		global_role: nullable(r.global_role),
		role
	};
}
function userOption(value: unknown): ManagerUserOption {
	const r = object(value, [
		'id',
		'name',
		'email',
		'avatar',
		'global_role',
		'manager_id',
		'role_id'
	]);
	if ((r.manager_id === null) !== (r.role_id === null)) return bad();
	return {
		id: managerId(r.id),
		name: nullable(r.name),
		email: text(r.email),
		avatar: nullable(r.avatar),
		global_role: nullable(r.global_role),
		manager_id: r.manager_id === null ? null : managerId(r.manager_id),
		role_id: r.role_id === null ? null : managerId(r.role_id)
	};
}
function page<T extends { id: string }>(
	value: unknown,
	parse: (value: unknown) => T
): ManagerPage<T> {
	const r = object(value, ['data', 'meta']),
		m = object(r.meta, ['current_page', 'per_page', 'total', 'last_page']);
	if (!Array.isArray(r.data)) return bad();
	const meta = {
		current_page: count(m.current_page, 1),
		per_page: count(m.per_page, 1),
		total: count(m.total),
		last_page: count(m.last_page, 1)
	};
	if (
		meta.per_page > 100 ||
		meta.last_page !== Math.max(1, Math.ceil(meta.total / meta.per_page)) ||
		r.data.length > meta.per_page
	)
		return bad();
	const data = r.data.map(parse);
	if (new Set(data.map((row) => row.id)).size !== data.length) return bad();
	return { data, meta };
}
async function request(path: string, method = 'GET', body?: unknown): Promise<unknown> {
	assertCrmClientRequest('/api/admin/crm' + path, method);
	const response = await fetch('/api/admin/crm' + path, {
		method,
		cache: 'no-store',
		credentials: 'same-origin',
		redirect: 'error',
		headers: { 'Content-Type': 'application/json', Accept: 'application/json' },
		...(body !== undefined ? { body: JSON.stringify(body) } : {}),
		signal: AbortSignal.timeout(30000)
	});
	notifyCrmDenial(response.status);
	const value = parseCrmJson(await response.text());
	if (!response.ok) {
		const r =
			value && typeof value === 'object' && !Array.isArray(value)
				? (value as Record<string, unknown>)
				: {};
		throw new Error(
			typeof r.error === 'string'
				? r.error
				: typeof r.message === 'string'
					? r.message
					: `CRM security request failed (${response.status})`
		);
	}
	return value;
}
function query(page: number, search: string): string {
	count(page, 1);
	if (page > 100000 || search.length > 500 || search.includes('\0'))
		throw new Error('Invalid CRM security search');
	return (
		'?' +
		new URLSearchParams({
			page: String(page),
			per_page: '25',
			...(search.trim() ? { search: search.trim() } : {})
		})
	);
}
export const managerAPI = {
	async roles(pageNumber = 1, search = '') {
		return page(await request('/manager-roles' + query(pageNumber, search)), parseManagerRole);
	},
	async managers(pageNumber = 1, search = '') {
		return page(await request('/managers' + query(pageNumber, search)), parseManager);
	},
	async users(pageNumber = 1, search = '') {
		return page(await request('/managers/users' + query(pageNumber, search)), userOption);
	},
	async role(id: string) {
		const result = parseManagerRole(await request('/manager-roles/' + managerId(id)));
		if (result.id !== id) return bad();
		return result;
	},
	async manager(id: string) {
		const result = parseManager(await request('/managers/' + managerId(id)));
		if (result.id !== id) return bad();
		return result;
	},
	async modules() {
		const value = await request('/manager-roles/modules'),
			compiled = currentCrmPolicy(),
			hash = await compiled.hash;
		if (!Array.isArray(value) || value.length !== compiled.policy.modules.length) return bad();
		const seen = new Set<string>();
		for (const input of value) {
			const r = object(input, [
					'id',
					'name',
					'actions',
					'manage_implies',
					'notes',
					'policy_version',
					'policy_hash'
				]),
				module = compiled.policy.modules.find((module) => module.id === r.id);
			if (
				!module ||
				seen.has(module.id) ||
				r.name !== module.name ||
				r.policy_hash !== hash ||
				r.policy_version !== compiled.policy.policy_version ||
				JSON.stringify(r.actions) !== JSON.stringify(compiled.policy.actions) ||
				JSON.stringify(r.manage_implies) !== JSON.stringify(compiled.policy.manage_implies) ||
				JSON.stringify(r.notes) !== JSON.stringify(module.notes)
			)
				return bad();
			seen.add(module.id);
		}
		return compiled.policy.modules;
	},
	async saveRole(id: string | undefined, fields: RolePatch) {
		if (id) count(fields.expected_version, 1);
		const result = parseManagerRole(
			await request(
				id ? '/manager-roles/' + managerId(id) : '/manager-roles',
				id ? 'PUT' : 'POST',
				fields
			)
		);
		if (id && result.id !== id) return bad();
		if (fields.expected_version !== undefined && result.version !== fields.expected_version + 1)
			return bad();
		for (const key of ['name', 'slug', 'description', 'is_default', 'permissions'] as const) {
			if (fields[key] !== undefined && JSON.stringify(result[key]) !== JSON.stringify(fields[key]))
				return bad();
		}
		return result;
	},
	async assign(user: string, role: string) {
		const result = parseManager(
			await request('/managers', 'POST', { user_id: managerId(user), role_id: managerId(role) })
		);
		if (result.user_id !== user || result.role_id !== role) return bad();
		return result;
	},
	async reassign(id: string, role: string, version: number) {
		count(version, 1);
		const result = parseManager(
			await request('/managers/' + managerId(id), 'PUT', {
				role_id: managerId(role),
				expected_version: version
			})
		);
		if (result.id !== id || result.role_id !== role || result.version !== version + 1) return bad();
		return result;
	},
	async remove(kind: 'managers' | 'manager-roles', id: string, version: number) {
		count(version, 1);
		const r = object(
			await request(`/${kind}/${managerId(id)}?expected_version=${version}`, 'DELETE'),
			kind === 'managers' ? ['deleted', 'id', 'user_id', 'version'] : ['deleted', 'id', 'version']
		);
		if (r.deleted !== true || r.id !== id || r.version !== version) return bad();
		if (kind === 'managers') managerId(r.user_id);
	}
};

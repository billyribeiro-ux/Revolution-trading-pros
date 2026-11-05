import {
	crmActions,
	matchesCrmParameter,
	parseCrmJson,
	type CrmGrant,
	type CrmPolicy,
	type CrmPrincipal
} from './policy.js';

export interface CrmCapabilities extends CrmPrincipal {
	readonly user_id: string;
	readonly manager_id: string | null;
	readonly role_id: string | null;
	readonly role_version: number | null;
	readonly assignment_version: number | null;
	readonly policy_version: 1;
	readonly policy_hash: string;
	readonly security_management_requires_global_admin: true;
	readonly default_role_is_automatic: false;
	readonly metrics_scope: 'current_crm_capabilities';
}
function fail(): never {
	throw new Error('Malformed or mismatched current CRM capabilities');
}
function positiveId(value: unknown): string {
	if (typeof value !== 'string' || !matchesCrmParameter('positive_integer', value)) return fail();
	return value;
}
function version(value: unknown): number {
	if (typeof value !== 'number' || !Number.isSafeInteger(value) || value < 1) return fail();
	return value;
}
/** Parse only the current verified backend response; never JWT, localStorage or client-submitted grants. */
export function parseCrmCapabilities(
	source: string,
	policy: CrmPolicy,
	hash: string,
	expectedUserId: string
): CrmCapabilities {
	const input = parseCrmJson(source);
	if (!input || typeof input !== 'object' || Array.isArray(input)) return fail();
	const row = input as Record<string, unknown>,
		keys = [
			'user_id',
			'global_admin',
			'manager_id',
			'role_id',
			'role_version',
			'assignment_version',
			'permissions',
			'policy_version',
			'policy_hash',
			'security_management_requires_global_admin',
			'default_role_is_automatic',
			'metrics_scope'
		];
	if (
		Object.keys(row).length !== keys.length ||
		keys.some((key) => !Object.hasOwn(row, key)) ||
		positiveId(row.user_id) !== expectedUserId ||
		row.policy_version !== policy.policy_version ||
		row.policy_hash !== hash ||
		!/^[a-f0-9]{64}$/.test(hash) ||
		typeof row.global_admin !== 'boolean' ||
		row.security_management_requires_global_admin !== true ||
		row.default_role_is_automatic !== false ||
		row.metrics_scope !== 'current_crm_capabilities' ||
		!Array.isArray(row.permissions) ||
		row.permissions.length > 100
	)
		return fail();
	const assigned = [row.manager_id, row.role_id, row.role_version, row.assignment_version];
	if (assigned.some((value) => value === null) && !assigned.every((value) => value === null))
		return fail();
	const manager_id = row.manager_id === null ? null : positiveId(row.manager_id),
		role_id = row.role_id === null ? null : positiveId(row.role_id),
		role_version = row.role_version === null ? null : version(row.role_version),
		assignment_version = row.assignment_version === null ? null : version(row.assignment_version);
	const permissions: CrmGrant[] = [],
		seen = new Set<string>();
	for (const input of row.permissions) {
		if (!input || typeof input !== 'object' || Array.isArray(input)) return fail();
		const grant = input as Record<string, unknown>,
			module = policy.modules.find((module) => module.id === grant.module)?.id,
			action = crmActions.find((action) => action === grant.action);
		if (
			Object.keys(grant).length !== 3 ||
			!Object.hasOwn(grant, 'module') ||
			!Object.hasOwn(grant, 'action') ||
			!Object.hasOwn(grant, 'allowed') ||
			!module ||
			!action ||
			grant.allowed !== true ||
			seen.has(`${module}:${action}`)
		)
			return fail();
		seen.add(`${module}:${action}`);
		permissions.push(Object.freeze({ module, action, allowed: true }));
	}
	if (!row.global_admin && manager_id === null && permissions.length !== 0) return fail();
	// The endpoint promises expanded effective grants. Partial or stale expansion fails closed.
	if (
		permissions.some(
			(grant) =>
				grant.action === 'manage' &&
				policy.manage_implies.some((action) => !seen.has(`${grant.module}:${action}`))
		)
	)
		return fail();
	if (row.global_admin && permissions.length !== policy.modules.length * crmActions.length)
		return fail();
	return Object.freeze({
		user_id: expectedUserId,
		global_admin: row.global_admin,
		manager_id,
		role_id,
		role_version,
		assignment_version,
		permissions: Object.freeze(permissions),
		policy_version: 1,
		policy_hash: hash,
		security_management_requires_global_admin: true,
		default_role_is_automatic: false,
		metrics_scope: 'current_crm_capabilities'
	});
}

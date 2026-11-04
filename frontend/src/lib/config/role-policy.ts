import policy from '../../../../api/role-policy.json';

export function isAdminRole(role: string | null | undefined): boolean {
	return policy.admin.includes((role ?? '').toLowerCase());
}

export function isSuperadminRole(role: string | null | undefined): boolean {
	return policy.superadmin.includes((role ?? '').toLowerCase());
}

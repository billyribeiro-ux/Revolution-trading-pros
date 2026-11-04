import { afterEach, beforeEach, expect, it, vi } from 'vite-plus/test';
vi.mock('$app/env', () => ({ browser: true }));
beforeEach(() => {
	vi.resetModules();
	vi.stubEnv('VITE_SUPERADMIN_EMAILS', 'configured-super@example.test');
	vi.stubEnv('VITE_DEVELOPER_EMAILS', 'configured-developer@example.test');
});
afterEach(() => vi.unstubAllEnvs());
it('email configuration and compatibility flags cannot elevate an ordinary account', async () => {
	const policy = await import('../roles.js');
	for (const email of ['configured-super@example.test', 'configured-developer@example.test']) {
		const user = {
			email,
			role: 'user',
			roles: ['user'],
			email_verified: true,
			is_admin: true,
			is_superadmin: true,
			is_developer: true
		};
		expect(policy.isSuperadmin(user)).toBe(false);
		expect(policy.isDeveloper(user)).toBe(false);
		expect(policy.isAdmin(user)).toBe(false);
		expect(policy.getHighestRole(user)).toBe('user');
		expect(policy.hasPermission(user, policy.PERMISSIONS.USERS_VIEW)).toBe(false);
	}
	expect(policy.isSuperadmin({ email: 'configured-super@example.test', role: 'admin' })).toBe(
		false
	);
	expect(policy.isDeveloper({ email: 'configured-developer@example.test', role: 'admin' })).toBe(
		false
	);
});
it('the authenticated API capability works with the actual privileged DB role', async () => {
	const policy = await import('../roles.js');
	const user = {
		email: 'server-configured@example.test',
		role: 'administrator',
		is_superadmin: true
	};
	expect(policy.isSuperadmin(user)).toBe(true);
	expect(policy.getHighestRole(user)).toBe('super-admin');
	expect(policy.hasPermission(user, policy.PERMISSIONS.USERS_VIEW)).toBe(true);
	expect(policy.isDeveloper({ role: 'admin', is_developer: true })).toBe(true);
	expect(policy.getHighestRole({ role: 'admin', is_developer: true })).toBe('admin');
});
it('single and array role aliases preserve navigation, permissions and highest-role labels', async () => {
	const policy = await import('../roles.js');
	for (const role of [
		'admin',
		'ADMIN',
		'administrator',
		'developer',
		'super_admin',
		'super-admin',
		'superadmin'
	]) {
		expect(policy.isAdmin({ role }), role).toBe(true);
		expect(policy.isAdmin({ roles: [role] }), role).toBe(true);
		expect(policy.getHighestRole({ role }), role).not.toBe('user');
	}
	expect(policy.hasPermission({ role: 'administrator' }, policy.PERMISSIONS.USERS_VIEW)).toBe(true);
	expect(policy.getUserPermissions({ role: 'administrator' })).toContain(
		policy.PERMISSIONS.USERS_VIEW
	);
	expect(policy.getHighestRole({ roles: ['member', 'developer', 'admin'] })).toBe('admin');
});

import { describe, expect, it } from 'vite-plus/test';
import { isAdminRole, isSuperadminRole } from './role-policy.js';

describe('shared API and frontend role policy', () => {
	it.each(['admin', 'administrator', 'developer', 'super-admin', 'super_admin', 'superadmin'])(
		'allows the established privileged role %s through the admin gate',
		(role) => {
			expect(isAdminRole(role)).toBe(true);
			expect(isAdminRole(role.toUpperCase())).toBe(true);
		}
	);
	it.each(['user', 'member', 'editor', 'marketing', 'moderator', '', null, undefined])(
		'rejects non-admin role %s',
		(role) => expect(isAdminRole(role)).toBe(false)
	);
	it('reserves superadmin operations for superadmin roles', () => {
		for (const role of ['super-admin', 'super_admin', 'superadmin']) {
			expect(isSuperadminRole(role)).toBe(true);
		}
		for (const role of ['admin', 'administrator', 'developer', 'user']) {
			expect(isSuperadminRole(role)).toBe(false);
		}
	});
});

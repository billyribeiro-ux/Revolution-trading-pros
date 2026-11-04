// @vitest-environment node
import { expect, it } from 'vite-plus/test';
import { requireSuperadmin } from '../auth.js';
function event(role: string | undefined, is_superadmin?: boolean) {
	const url = new URL('http://fixture.example.test/api/admin/connections');
	return {
		locals: { user: { id: 1, email: 'configured@example.test', role, is_superadmin } },
		cookies: { get: () => 'fixture-token' },
		request: new Request(url, { method: 'POST', body: JSON.stringify({ is_superadmin: true }) })
	} as unknown as Parameters<typeof requireSuperadmin>[0];
}
it('accepts verified backend elevation and base superadmin aliases in trusted locals', () => {
	expect(requireSuperadmin(event('administrator', true)).token).toBe('fixture-token');
	for (const role of ['super_admin', 'superadmin', 'super-admin', 'SUPER_ADMIN'])
		expect(requireSuperadmin(event(role)).user.role).toBe(role);
});
it('request body claims, an ordinary role or missing server capability cannot grant superadmin', () => {
	for (const e of [
		event('admin'),
		event('admin', false),
		event('user', true),
		event(undefined, true)
	]) {
		expect(() => requireSuperadmin(e)).toThrow(expect.objectContaining({ status: 403 }));
	}
});

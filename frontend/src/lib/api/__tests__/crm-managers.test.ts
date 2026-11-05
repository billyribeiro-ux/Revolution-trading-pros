// @vitest-environment node
import { afterEach, beforeEach, expect, it, vi } from 'vite-plus/test';
import { currentCrmPolicy } from '#lib/config/crm-policy.js';
import { managerAPI, parseManagerRole, parseManager } from '../crm-managers.js';
import { authorizedCrmRequestFixture } from '#lib/crm/__tests__/request-fixture.js';
const role = {
	id: '5',
	name: 'Contact reviewer',
	slug: 'contact-reviewer',
	description: null,
	permissions: [{ module: 'contacts', action: 'view', allowed: true }],
	users_count: 1,
	is_default: false,
	version: 3,
	created_at: '2026-11-05T12:00:00Z',
	updated_at: '2026-11-05T12:00:00Z'
};
const manager = {
	id: '8',
	user_id: '17',
	role_id: '5',
	name: null,
	email: 'fixture@example.test',
	avatar: null,
	assigned_at: '2026-11-05T12:00:00Z',
	version: 2,
	is_active: true,
	global_role: 'user',
	role
};
let release: () => void = () => undefined;
beforeEach(() => {
	release = authorizedCrmRequestFixture();
});
afterEach(() => {
	release();
	vi.unstubAllGlobals();
});
it('retains genuine nullable user facts and rejects mismatched nested role or duplicate grants', () => {
	expect(parseManager(manager)).toMatchObject({ name: null, global_role: 'user' });
	expect(() => parseManager({ ...manager, role_id: '6' })).toThrow();
	expect(() =>
		parseManagerRole({ ...role, permissions: [...role.permissions, ...role.permissions] })
	).toThrow();
	expect(() => parseManagerRole({ ...role, version: 0 })).toThrow();
});
it('sends sparse role patches with frozen CAS and rejects a nonmatching acknowledgement', async () => {
	const transport = vi
		.fn<typeof fetch>()
		.mockResolvedValueOnce(Response.json({ ...role, description: 'Reviewed', version: 4 }))
		.mockResolvedValueOnce(Response.json({ ...role, description: 'Different', version: 4 }));
	vi.stubGlobal('fetch', transport);
	await managerAPI.saveRole('5', { description: 'Reviewed', expected_version: 3 });
	expect(transport.mock.calls[0]?.[1]).toMatchObject({
		method: 'PUT',
		cache: 'no-store',
		credentials: 'same-origin',
		redirect: 'error'
	});
	expect(transport.mock.calls[0]?.[1]?.body).toBe(
		JSON.stringify({ description: 'Reviewed', expected_version: 3 })
	);
	await expect(
		managerAPI.saveRole('5', { description: 'Reviewed', expected_version: 3 })
	).rejects.toThrow('malformed');
	expect(transport).toHaveBeenCalledTimes(2);
});
it('surfaces stale conflicts without retries and validates assignment identity/version', async () => {
	const transport = vi
		.fn<typeof fetch>()
		.mockResolvedValueOnce(Response.json({ error: 'CRM role changed; reload' }, { status: 409 }))
		.mockResolvedValueOnce(Response.json({ ...manager, version: 3 }))
		.mockResolvedValueOnce(Response.json({ ...manager, user_id: '99' }));
	vi.stubGlobal('fetch', transport);
	await expect(
		managerAPI.saveRole('5', { description: null, expected_version: 3 })
	).rejects.toThrow('reload');
	expect(transport).toHaveBeenCalledOnce();
	expect(await managerAPI.reassign('8', '5', 2)).toMatchObject({ id: '8', version: 3 });
	await expect(managerAPI.assign('17', '5')).rejects.toThrow('malformed');
});
it('rejects corrupt page totals and validates deletion receipt rather than assuming success', async () => {
	const transport = vi
		.fn<typeof fetch>()
		.mockResolvedValueOnce(
			Response.json({
				data: [role],
				meta: { current_page: 1, per_page: 25, total: 26, last_page: 1 }
			})
		)
		.mockResolvedValueOnce(Response.json({ deleted: true, id: '8', user_id: '17', version: 1 }));
	vi.stubGlobal('fetch', transport);
	await expect(managerAPI.roles()).rejects.toThrow('malformed');
	await expect(managerAPI.remove('managers', '8', 2)).rejects.toThrow('malformed');
});

it('rejects missing or repeated module catalog entries even when its hash and length match', async () => {
	const compiled = currentCrmPolicy(),
		hash = await compiled.hash;
	const rows = compiled.policy.modules.map((module) => ({
		...module,
		actions: compiled.policy.actions,
		manage_implies: compiled.policy.manage_implies,
		policy_version: 1,
		policy_hash: hash
	}));
	const transport = vi
		.fn<typeof fetch>()
		.mockResolvedValueOnce(Response.json(rows))
		.mockResolvedValueOnce(Response.json(rows.map((row, index) => (index === 1 ? rows[0] : row))));
	vi.stubGlobal('fetch', transport);
	expect(await managerAPI.modules()).toEqual(compiled.policy.modules);
	await expect(managerAPI.modules()).rejects.toThrow('malformed');
});

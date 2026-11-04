import { afterEach, beforeEach, expect, it, vi } from 'vite-plus/test';
vi.mock('$app/env', () => ({ browser: true }));
const state = vi.hoisted(() => ({
	user: { id: 1, email: 'fixture@example.test', role: 'admin', is_superadmin: true }
}));
vi.mock('#lib/stores/auth.svelte.js', () => ({
	authStore: {
		getToken: () => 'fixture-token',
		get user() {
			return state.user;
		}
	}
}));
beforeEach(() => vi.resetModules());
afterEach(() => vi.unstubAllGlobals());
it('an empty real catalog stays empty instead of inventing memberships', async () => {
	const fetch = vi.fn<typeof globalThis.fetch>().mockResolvedValue(Response.json({ data: [] }));
	vi.stubGlobal('fetch', fetch);
	const { getUserMemberships } = await import('../user-memberships.js');
	const result = await getUserMemberships();
	expect(result.memberships).toEqual([]);
	expect(fetch).toHaveBeenCalledTimes(1);
});
it('unavailable catalog transport remains an error, never fixture entitlements', async () => {
	vi.stubGlobal(
		'fetch',
		vi
			.fn<typeof globalThis.fetch>()
			.mockRejectedValue(new TypeError('Controlled catalog unavailable'))
	);
	const { getUserMemberships } = await import('../user-memberships.js');
	await expect(getUserMemberships()).rejects.toThrow('Controlled catalog unavailable');
});
it('privileged catalog access has no invented billing dates or purchased price', async () => {
	vi.stubGlobal(
		'fetch',
		vi.fn<typeof globalThis.fetch>().mockResolvedValue(
			Response.json({
				data: [{ id: 101, name: 'Fixture room', slug: 'fixture-room', price: 999 }]
			})
		)
	);
	const { getUserMemberships } = await import('../user-memberships.js');
	const result = await getUserMemberships();
	expect(result.memberships).toHaveLength(1);
	expect(result.memberships[0]).toMatchObject({
		id: '101',
		name: 'Fixture room',
		accessSource: 'privileged',
		membershipType: 'complimentary',
		price: 0
	});
	expect(result.memberships[0]?.nextBillingDate).toBeUndefined();
	expect(result.memberships[0]?.startDate).toBeUndefined();
});

import { beforeEach, expect, it, vi } from 'vite-plus/test';
const api = vi.hoisted(() => ({
	stats: vi.fn(),
	catalog: vi.fn(),
	service: vi.fn(),
	members: vi.fn()
}));
vi.mock('#lib/api/members.js', () => ({
	membersApi: {
		getStats: api.stats,
		getServices: api.catalog,
		getMembersByService: api.service,
		getMembers: api.members
	}
}));
import { membersStore, serviceMembersStore } from '../members.svelte.js';
const pagination = { total: 1, per_page: 1, current_page: 1, last_page: 1 };
const plan = { id: 17, name: 'Off-page plan' };
const service = (id: number) => ({
	service: { id, name: `Plan ${id}` },
	stats: { total_members: 1 },
	members: [{ id, email: `${id}@example.test` }],
	pagination
});
beforeEach(() => {
	membersStore.reset();
	serviceMembersStore.reset();
	vi.resetAllMocks();
});
it('keeps statistics failures separate from successful member rows and recovers through a new read', async () => {
	api.members.mockResolvedValue({ members: [{ id: 9 }], pagination });
	await membersStore.loadMembers();
	api.stats.mockRejectedValueOnce(new Error('Statistics unavailable'));
	await membersStore.loadStats();
	expect(membersStore.state.stats).toBeNull();
	expect(membersStore.state.statsError).toBe('Statistics unavailable');
	expect(membersStore.state.statsLoading).toBe(false);
	expect(membersStore.members.map((m) => m.id)).toEqual([9]);
	expect(membersStore.error).toBeNull();
	api.stats.mockResolvedValueOnce({ overview: { total_members: 9 } });
	await membersStore.loadStats();
	expect(membersStore.stats?.overview.total_members).toBe(9);
	expect(membersStore.state.statsError).toBeNull();
	expect(api.stats).toHaveBeenCalledTimes(2);
});
it('discards an old statistics failure after a newer read succeeded', async () => {
	let rejectOld: ((reason: Error) => void) | undefined;
	api.stats
		.mockImplementationOnce(
			() =>
				new Promise((_resolve, reject) => {
					rejectOld = reject;
				})
		)
		.mockResolvedValueOnce({ overview: { total_members: 10 } });
	const old = membersStore.loadStats();
	await membersStore.loadStats();
	rejectOld?.(new Error('Old outage'));
	await old;
	expect(membersStore.stats?.overview.total_members).toBe(10);
	expect(membersStore.state.statsError).toBeNull();
});
it('retains plan-local filters through paging and clears them when the selected plan changes', async () => {
	api.service.mockImplementation(async (id: number) => service(id));
	await serviceMembersStore.loadServiceMembers(17, {
		search: 'Recorded',
		status: 'expired',
		per_page: 1,
		page: 1
	});
	await serviceMembersStore.loadServiceMembers(17, { page: 2 });
	expect(api.service).toHaveBeenLastCalledWith(17, {
		search: 'Recorded',
		status: 'expired',
		per_page: 1,
		page: 2
	});
	await serviceMembersStore.loadServiceMembers(18);
	expect(api.service).toHaveBeenLastCalledWith(18, { page: 1, per_page: 25 });
});
it('discards stale plan navigation and clears prior rows on a genuine read failure', async () => {
	let finishOld: ((value: unknown) => void) | undefined;
	api.service
		.mockImplementationOnce(
			() =>
				new Promise((resolve) => {
					finishOld = resolve;
				})
		)
		.mockResolvedValueOnce(service(18));
	const old = serviceMembersStore.loadServiceMembers(17);
	await serviceMembersStore.loadServiceMembers(18);
	finishOld?.(service(17));
	await old;
	expect(serviceMembersStore.service?.id).toBe(18);
	expect(serviceMembersStore.members.map((m) => m.id)).toEqual([18]);
	api.service.mockRejectedValueOnce(new Error('Service storage failed'));
	await serviceMembersStore.loadServiceMembers(18);
	expect(serviceMembersStore.state.error).toBe('Service storage failed');
	expect(serviceMembersStore.members).toEqual([]);
	expect(serviceMembersStore.stats).toBeNull();
	expect(serviceMembersStore.loading).toBe(false);
});
it('preserves the actual selected plan outside the current catalog page without duplicating it', async () => {
	api.catalog
		.mockResolvedValueOnce({
			services: [{ id: 18, name: 'Page member' }],
			selected: plan,
			pagination
		})
		.mockResolvedValueOnce({ services: [plan], selected: plan, pagination });
	await membersStore.loadServices({ selected_id: 17, page: 2 });
	expect(membersStore.state.services.map((p) => p.id)).toEqual([17, 18]);
	await membersStore.loadServices({ page: 1 });
	expect(membersStore.state.services.map((p) => p.id)).toEqual([17]);
	expect(api.catalog).toHaveBeenLastCalledWith({ selected_id: 17, page: 1 });
});

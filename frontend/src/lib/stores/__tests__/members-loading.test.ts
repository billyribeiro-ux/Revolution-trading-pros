import { expect, it, vi } from 'vite-plus/test';
const api = vi.hoisted(() => ({ request: vi.fn() }));
vi.mock('#lib/api/members.js', () => ({ membersApi: { getMembers: api.request } }));
import { membersStore } from '../members.svelte.js';
const result = (id: number) => ({
	members: [{ id, name: `Member ${id}`, email: `${id}@example.test` }],
	pagination: { total: 1, per_page: 25, current_page: 1, last_page: 1 }
});
it('keeps the latest filtered audience when older requests finish later', async () => {
	let first: ((value: unknown) => void) | undefined;
	let second: ((value: unknown) => void) | undefined;
	api.request.mockImplementationOnce(
		() =>
			new Promise((resolve) => {
				first = resolve;
			})
	);
	api.request.mockImplementationOnce(
		() =>
			new Promise((resolve) => {
				second = resolve;
			})
	);
	const old = membersStore.loadMembers({ segment: 1 });
	const latest = membersStore.loadMembers({ segment: 2 });
	second?.(result(2));
	await latest;
	first?.(result(1));
	await old;
	expect(membersStore.members.map((member) => member.id)).toEqual([2]);
	expect(membersStore.state.filters.segment).toBe(2);
});
it('exposes an unavailable audience and clears the prior list instead of retaining the wrong recipients', async () => {
	api.request.mockRejectedValueOnce(new Error('Audience storage unavailable'));
	await membersStore.loadMembers({ segment: 3 });
	expect(membersStore.error).toBe('Audience storage unavailable');
	expect(membersStore.members).toEqual([]);
	expect(membersStore.loading).toBe(false);
	api.request.mockResolvedValueOnce(result(3));
	await membersStore.loadMembers({ segment: 3 });
	expect(membersStore.error).toBeNull();
	expect(membersStore.members.map((member) => member.id)).toEqual([3]);
});

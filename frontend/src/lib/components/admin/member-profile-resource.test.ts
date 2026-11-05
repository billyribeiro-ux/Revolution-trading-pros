import { expect, it, vi } from 'vite-plus/test';
import { MemberProfileResource } from './member-profile-resource.svelte';
it('does not replace a newer member with an older reply even if its loader ignores abort', async () => {
	let complete: ((value: { member: string }) => void) | undefined;
	const loader = vi
		.fn<(member: string, query: object, signal: AbortSignal) => Promise<{ member: string }>>()
		.mockImplementationOnce(
			() =>
				new Promise((resolve) => {
					complete = resolve;
				})
		)
		.mockResolvedValueOnce({ member: '10' });
	const resource = new MemberProfileResource(loader);
	const old = resource.load(9);
	await resource.load(10);
	complete?.({ member: '9' });
	await old;
	expect(resource.result).toEqual({ member: '10' });
	expect(loader.mock.calls[0][2].aborted).toBe(true);
	expect(resource.loading).toBe(false);
});
it('keeps independent errors, retains exact filters on paging and clears filters on a different member', async () => {
	const loader = vi
		.fn<(member: string, query: object, signal: AbortSignal) => Promise<string>>()
		.mockResolvedValueOnce('saved')
		.mockRejectedValueOnce(new Error('Journal unavailable'))
		.mockResolvedValueOnce('recovered');
	const resource = new MemberProfileResource(loader);
	await resource.load(9, { page: 1, per_page: 10, action: 'member_note_created' });
	await resource.load(9, { page: 2 });
	expect(resource.result).toBeNull();
	expect(resource.error).toBe('Journal unavailable');
	expect(loader.mock.calls[1][1]).toEqual({ page: 2, per_page: 10, action: 'member_note_created' });
	await resource.load(10);
	expect(resource.result).toBe('recovered');
	expect(resource.error).toBe('');
	expect(loader.mock.calls[2][1]).toEqual({ page: 1, per_page: 25 });
});
it('reset aborts pending work and prevents a late reply reopening a closed drawer', async () => {
	let complete: ((value: string) => void) | undefined;
	const loader = () =>
		new Promise<string>((resolve) => {
			complete = resolve;
		});
	const resource = new MemberProfileResource(loader);
	const loading = resource.load(9);
	resource.reset();
	complete?.('late');
	await loading;
	expect(resource.result).toBeNull();
	expect(resource.loading).toBe(false);
});

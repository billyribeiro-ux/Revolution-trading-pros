import { afterEach, expect, it, vi } from 'vite-plus/test';

vi.mock('$app/env', () => ({ browser: false }));
vi.mock('#lib/stores/auth.svelte.js', () => ({ authStore: {} }));
vi.mock('#lib/utils/logger.js', () => ({
	logger: { debug: vi.fn(), warn: vi.fn(), error: vi.fn() }
}));
afterEach(() => vi.unstubAllGlobals());

it('shares a pending read, preserves its rejection and releases the entry for a later request', async () => {
	const replies: ((response: Response) => void)[] = [];
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockImplementationOnce(() => new Promise<Response>((resolve) => replies.push(resolve)))
		.mockResolvedValueOnce(
			new Response(JSON.stringify({ message: 'Fixture verified' }), { status: 200 })
		);
	vi.stubGlobal('fetch', fetch);
	const { verifyEmail } = await import('../auth');
	const results = Promise.allSettled([verifyEmail('fixture-token'), verifyEmail('fixture-token')]);
	await vi.waitFor(() => expect(fetch).toHaveBeenCalledTimes(1));
	replies.shift()?.(new Response(JSON.stringify({ message: 'Fixture rejected' }), { status: 400 }));
	const rejected = await results;
	expect(rejected.map((result) => result.status)).toEqual(['rejected', 'rejected']);
	for (const result of rejected)
		if (result.status === 'rejected') expect(result.reason).toBeInstanceOf(Error);
	expect(await verifyEmail('fixture-token')).toBe('Fixture verified');
	expect(fetch).toHaveBeenCalledTimes(2);
});

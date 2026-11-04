// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
import { POST } from './+server';

afterEach(() => vi.unstubAllGlobals());

it('rejects file and duplicate callback fields before contacting the provider', async () => {
	const fetch = vi.fn<typeof globalThis.fetch>();
	vi.stubGlobal('fetch', fetch);
	for (const duplicate of [false, true]) {
		const data = new FormData();
		data.append('code', 'fixture-code');
		if (duplicate) data.append('code', 'second-code');
		else data.append('id_token', new File(['fixture'], 'token.txt'));
		const request = new Request('https://fixture.example.test/api/auth/apple/callback', {
			method: 'POST',
			body: data
		});
		const response = await POST({ request } as unknown as Parameters<typeof POST>[0]);
		expect(response.status).toBe(400);
		expect((await response.json()).error).toBe('Invalid Apple sign-in callback form');
	}
	expect(fetch).not.toHaveBeenCalled();
});

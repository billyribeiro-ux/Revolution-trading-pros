// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
vi.mock('#lib/stores/auth.svelte.js', () => ({ authStore: { getToken: () => null } }));
vi.mock('#lib/utils/logger.js', () => ({ logger: { warn: vi.fn(), error: vi.fn() } }));
import { emailTemplatesApi } from '../admin.js';
afterEach(() => {
	vi.unstubAllGlobals();
	vi.clearAllMocks();
});
it('shows string, structured and plaintext backend validation messages', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValueOnce(
			Response.json({ error: 'Template has missing variables' }, { status: 400 })
		)
		.mockResolvedValueOnce(
			Response.json(
				{ error: { message: 'Structured validation failure', code: 'INVALID' } },
				{ status: 422 }
			)
		)
		.mockResolvedValueOnce(new Response('Plain validation failure', { status: 422 }));
	vi.stubGlobal('fetch', fetch);
	await expect(emailTemplatesApi.preview(1, {})).rejects.toMatchObject({
		message: 'Template has missing variables',
		status: 400
	});
	await expect(emailTemplatesApi.preview(2, {})).rejects.toMatchObject({
		message: 'Structured validation failure',
		status: 422
	});
	await expect(emailTemplatesApi.preview(3, {})).rejects.toMatchObject({
		message: 'Plain validation failure',
		status: 422
	});
	expect(fetch).toHaveBeenCalledTimes(3);
});
it('never automatically retries an uncertain email test submission', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValue(
			Response.json(
				{ error: 'Submission outcome is unknown', submission_status: 'unknown' },
				{ status: 502 }
			)
		);
	vi.stubGlobal('fetch', fetch);
	await expect(emailTemplatesApi.sendTest(42, 'fixture@example.test')).rejects.toMatchObject({
		message: 'Submission outcome is unknown',
		status: 502
	});
	expect(fetch).toHaveBeenCalledOnce();
});
it('keeps a raw successful record serializable when adding the response envelope', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValue(Response.json({ id: 123, name: 'Fixture', subject: 'Saved body' }));
	vi.stubGlobal('fetch', fetch);
	const response = await emailTemplatesApi.get(123);
	expect(response.data).toEqual({ id: 123, name: 'Fixture', subject: 'Saved body' });
	expect(() => JSON.stringify(response)).not.toThrow();
});

it('keeps healthy service requests available after repeated validation rejections', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockImplementation(async (_url, init) =>
			init?.method === 'POST'
				? Response.json({ error: 'Provide the missing variable' }, { status: 400 })
				: Response.json({ data: { id: 321 } })
		);
	vi.stubGlobal('fetch', fetch);
	for (let attempt = 0; attempt < 6; attempt++) {
		await expect(emailTemplatesApi.preview(321, {})).rejects.toMatchObject({
			message: 'Provide the missing variable',
			status: 400
		});
	}
	expect((await emailTemplatesApi.get(321)).data).toEqual({ id: 321 });
	expect(fetch).toHaveBeenCalledTimes(7);
});

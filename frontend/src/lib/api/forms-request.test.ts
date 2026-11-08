import { afterEach, beforeEach, describe, expect, it, vi } from 'vite-plus/test';
import formsService, { formsApi } from './forms';

function formRow(id: number, name: string) {
	return { id, name, slug: `form-${id}`, fields: [], settings: {}, is_published: false };
}

function jsonResponse(value: unknown): Response {
	return new Response(JSON.stringify(value), {
		status: 200,
		headers: { 'Content-Type': 'application/json' }
	});
}

describe('forms request delivery', () => {
	beforeEach(() => formsService.forms.set([]));
	afterEach(() => vi.unstubAllGlobals());

	it('sends distinct concurrent creates to the backend', async () => {
		const fetchMock = vi.fn(async (_url: string, options: RequestInit) => {
			if (typeof options.body !== 'string') throw new Error('Expected a JSON body');
			const body = JSON.parse(options.body) as { name: string };
			await Promise.resolve();
			return jsonResponse(formRow(body.name === 'Alpha' ? 101 : 102, body.name));
		});
		vi.stubGlobal('fetch', fetchMock);

		const [alpha, beta] = await Promise.all([
			formsApi.create({ title: 'Alpha' }),
			formsApi.create({ title: 'Beta' })
		]);

		expect(fetchMock).toHaveBeenCalledTimes(2);
		expect([alpha.id, beta.id]).toEqual([101, 102]);
	});

	it('does not retry a create after an ambiguous network failure', async () => {
		const fetchMock = vi.fn().mockRejectedValue(new TypeError('fetch failed'));
		vi.stubGlobal('fetch', fetchMock);

		await expect(formsApi.create({ title: 'One attempt' })).rejects.toThrow('fetch failed');
		expect(fetchMock).toHaveBeenCalledTimes(1);
	});

	it('uses the real admin route for the compatibility get method', async () => {
		const fetchMock = vi.fn().mockResolvedValue(jsonResponse(formRow(98765, 'Retrieved')));
		vi.stubGlobal('fetch', fetchMock);

		const form = await formsApi.get(98765);
		expect(form.title).toBe('Retrieved');
		expect(fetchMock).toHaveBeenCalledWith('/api/admin/forms/98765', expect.any(Object));
	});

	it('invalidates the cached form list after an update', async () => {
		let currentName = 'Before';
		let listCalls = 0;
		const fetchMock = vi.fn(async (url: string, options: RequestInit) => {
			if (url.includes('/admin/forms?')) {
				listCalls += 1;
				return jsonResponse({
					data: [formRow(81234, currentName)],
					meta: { total: 1, per_page: 20 }
				});
			}
			if (options.method === 'PUT') {
				currentName = 'After';
				return jsonResponse(formRow(81234, currentName));
			}
			throw new Error(`Unexpected request: ${url}`);
		});
		vi.stubGlobal('fetch', fetchMock);

		expect((await formsService.getForms(456)).forms[0].title).toBe('Before');
		await formsApi.update(81234, { title: 'After' });
		expect((await formsService.getForms(456)).forms[0].title).toBe('After');
		expect(listCalls).toBe(2);
	});
});

import { afterEach, beforeEach, describe, expect, it, vi } from 'vite-plus/test';

vi.mock('#lib/stores/auth.svelte.js', () => ({ getAuthToken: () => null }));

import { apiClient } from '../client.svelte.js';

const fetchMock = vi.fn<typeof fetch>();

beforeEach(() => {
	fetchMock.mockReset();
	vi.stubGlobal('fetch', fetchMock);
});

afterEach(() => {
	vi.unstubAllGlobals();
});

describe('API response contracts', () => {
	it('sends DELETE payloads used by contact-list membership removal', async () => {
		fetchMock.mockResolvedValueOnce(Response.json({ removed_count: 1 }));
		await apiClient.delete('/contract/members', { body: { contact_ids: ['42'] } });
		const init = fetchMock.mock.calls[0]?.[1];
		expect(init?.method).toBe('DELETE');
		expect(init?.body).toBe(JSON.stringify({ contact_ids: ['42'] }));
	});
	it('retains pagination metadata without changing default single-record unwrapping', async () => {
		const envelope = { data: [{ id: '1' }], meta: { total: 3, current_page: 1 } };
		fetchMock.mockResolvedValueOnce(Response.json(envelope));
		const path = `/contract/pagination-${crypto.randomUUID()}`;
		expect(await apiClient.get(path)).toEqual(envelope.data);
		fetchMock.mockResolvedValueOnce(Response.json(envelope));
		expect(await apiClient.get(path, { preserveEnvelope: true })).toEqual(envelope);
		expect(fetchMock).toHaveBeenCalledTimes(2);
		fetchMock.mockResolvedValueOnce(Response.json({ success: true, data: { id: 'single' } }));
		expect(await apiClient.get(`/contract/single-${crypto.randomUUID()}`)).toEqual({
			id: 'single'
		});
	});

	it('does not deduplicate concurrent writes with different payloads', async () => {
		const complete: Array<(response: Response) => void> = [];
		fetchMock.mockImplementation(
			() =>
				new Promise<Response>((resolve) => {
					complete.push(resolve);
				})
		);
		const path = `/contract/write-${crypto.randomUUID()}`;
		const first = apiClient.post(path, { name: 'First' });
		const second = apiClient.post(path, { name: 'Second' });
		await vi.waitFor(() => expect(fetchMock).toHaveBeenCalledTimes(2));
		complete[0]?.(Response.json({ id: 'first' }));
		complete[1]?.(Response.json({ id: 'second' }));
		expect(await Promise.all([first, second])).toEqual([{ id: 'first' }, { id: 'second' }]);
	});
});

describe('current CRM request boundaries', () => {
	it('checks current access before serving an earlier response and never caches CRM reads', async () => {
		const { authorizedCrmRequestFixture } = await import('#lib/crm/__tests__/request-fixture.js');
		const release = authorizedCrmRequestFixture();
		try {
			fetchMock
				.mockResolvedValueOnce(Response.json({ id: 'first' }))
				.mockResolvedValueOnce(Response.json({ id: 'second' }));
			expect(await apiClient.get('/admin/crm/contacts')).toEqual({ id: 'first' });
			expect(await apiClient.get('/admin/crm/contacts')).toEqual({ id: 'second' });
			expect(fetchMock).toHaveBeenCalledTimes(2);
		} finally {
			release();
		}
		await expect(apiClient.get('/admin/crm/contacts')).rejects.toThrow('context is unavailable');
		expect(fetchMock).toHaveBeenCalledTimes(2);
	});
	it('makes one attempt for an uncertain CRM write even when retry options ask for more', async () => {
		const { authorizedCrmRequestFixture } = await import('#lib/crm/__tests__/request-fixture.js');
		const release = authorizedCrmRequestFixture();
		try {
			fetchMock.mockRejectedValue(new Error('Controlled disconnect after write'));
			await expect(
				apiClient.post(
					'/admin/crm/contacts',
					{ email: 'fixture@example.test' },
					{ retry: { attempts: 3 } }
				)
			).rejects.toBeDefined();
			expect(fetchMock).toHaveBeenCalledOnce();
		} finally {
			release();
		}
	});
});

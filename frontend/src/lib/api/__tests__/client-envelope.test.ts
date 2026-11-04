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

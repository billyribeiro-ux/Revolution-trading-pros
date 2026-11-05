import { afterEach, expect, it, vi } from 'vite-plus/test';
vi.mock('#lib/stores/auth.svelte.js', () => ({ authStore: { getToken: () => null } }));
import { emailApi } from '../email.js';
const fixture = () => ({
	id: '42',
	authoring_version: 2,
	title: 'Saved',
	status: 'paused',
	trigger_type: 'manual',
	trigger_config: {},
	metrics_scope: 'sequence_outbox',
	definition_error: null,
	historical_steps: [],
	emails_count: 0,
	emails: [],
	subscribers_count: 0,
	created_at: '2026-11-05T00:00:00Z',
	updated_at: '2026-11-05T00:00:00Z'
});
afterEach(() => vi.unstubAllGlobals());
it('sends one version-checked secondary status change and requires the actual resulting status', async () => {
	const fetcher = vi
		.fn<typeof fetch>()
		.mockResolvedValue(Response.json({ ...fixture(), status: 'active' }));
	vi.stubGlobal('fetch', fetcher);
	expect((await emailApi.updateSequence('42', { status: 'active' }, 2)).sequence.status).toBe(
		'active'
	);
	expect(fetcher).toHaveBeenCalledExactlyOnceWith(
		'/api/admin/email/sequences/42',
		expect.objectContaining({
			method: 'PUT',
			body: JSON.stringify({ status: 'active', expected_version: 2 })
		})
	);
	fetcher.mockReset().mockResolvedValue(Response.json(fixture()));
	await expect(emailApi.updateSequence('42', { status: 'active' }, 2)).rejects.toThrow(
		'not confirmed'
	);
	expect(fetcher).toHaveBeenCalledTimes(1);
});
it('accepts only a distinct empty draft duplicate and does not retry a service or network failure', async () => {
	const fetcher = vi
		.fn<typeof fetch>()
		.mockResolvedValue(Response.json({ ...fixture(), id: '43', status: 'draft' }));
	vi.stubGlobal('fetch', fetcher);
	expect((await emailApi.duplicateSequence('42')).sequence.id).toBe('43');
	fetcher.mockReset().mockResolvedValue(Response.json({ ...fixture(), status: 'draft' }));
	await expect(emailApi.duplicateSequence('42')).rejects.toThrow('not confirmed');
	fetcher
		.mockReset()
		.mockResolvedValue(Response.json({ error: 'Fixture unavailable' }, { status: 503 }));
	await expect(emailApi.duplicateSequence('42')).rejects.toThrow('Fixture unavailable');
	expect(fetcher).toHaveBeenCalledTimes(1);
	fetcher.mockReset().mockRejectedValue(new Error('Fixture uncertainty'));
	await expect(emailApi.duplicateSequence('42')).rejects.toThrow('uncertainty');
	expect(fetcher).toHaveBeenCalledTimes(1);
});

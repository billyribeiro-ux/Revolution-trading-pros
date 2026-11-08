import { afterEach, describe, expect, it, vi } from 'vite-plus/test';
vi.mock('#lib/stores/auth.svelte.js', () => ({ authStore: { getToken: () => null } }));
import { mediaApi, type RawMedia } from '../media';

const media: RawMedia = {
	id: 42,
	filename: 'stored-fixture.png',
	original_filename: 'fixture.png',
	mime_type: 'image/png',
	size: 123,
	path: 'uploads/stored-fixture.png',
	url: 'https://assets.example.test/uploads/stored-fixture.png',
	delivery_url: '/api/media/image/01234567-89ab-cdef-0123-456789abcdef',
	title: null,
	alt_text: null,
	caption: null,
	description: null,
	collection: null,
	width: 100,
	height: 50,
	is_optimized: false,
	tags: ['fixture'],
	metadata: {},
	created_by: 7,
	has_webp: false,
	processing_status: null,
	scan_status: 'clean',
	created_at: '2026-11-03T12:00:00Z',
	updated_at: '2026-11-03T12:00:00Z'
};
afterEach(() => vi.unstubAllGlobals());
describe('public media client contracts', () => {
	it('preserves server pagination and public URLs while providing same-origin previews', async () => {
		const fetcher = vi.fn().mockResolvedValue(
			Response.json({
				success: true,
				data: [media],
				meta: { current_page: 2, per_page: 5, total: 12, last_page: 3 }
			})
		);
		vi.stubGlobal('fetch', fetcher);
		const result = await mediaApi.getFiles({
			page: 2,
			per_page: 5,
			file_type: 'document',
			optimized: false
		});
		expect(result.pagination).toEqual({ page: 2, per_page: 5, total: 12, total_pages: 3 });
		expect(result.files[0]).toMatchObject({
			id: '42',
			filename: 'fixture.png',
			file_size: 123,
			url: media.url,
			preview_url: '/api/admin/media/content/42',
			cdn_url: media.delivery_url,
			tags: ['fixture'],
			created_by: 7,
			usage_count: null
		});
		const query = new URL(fetcher.mock.calls[0]?.[0] as string, 'http://fixture.test').searchParams;
		expect(query.get('type')).toBe('document');
		expect(query.get('is_optimized')).toBe('false');
		expect(query.has('optimized')).toBe(false);
		expect(query.has('file_type')).toBe(false);
	});
	it('keeps the editor upload file.url contract when the backend returns a record array', async () => {
		vi.stubGlobal(
			'fetch',
			vi.fn().mockResolvedValue(Response.json({ success: true, data: [media] }))
		);
		const result = await mediaApi.uploadFile(
			new File(['fixture'], 'fixture.png', { type: 'image/png' })
		);
		expect(result.success).toBe(true);
		expect(result.file.id).toBe('42');
		expect(result.file.url).toBe(media.url);
	});
	it('retains unknown legacy sizes and authors and does not expose an infected preview', async () => {
		vi.stubGlobal(
			'fetch',
			vi.fn().mockResolvedValue(
				Response.json({
					success: true,
					data: { ...media, size: null, created_by: null, url: null, scan_status: 'infected' }
				})
			)
		);
		const result = await mediaApi.getFile('42');
		expect(result.file.file_size).toBeNull();
		expect(result.file.created_by).toBeNull();
		expect(result.file.preview_url).toBeUndefined();
		expect(result.file.url).toBe('');
	});
	it('returns a queued job instead of casting it to a completed media file', async () => {
		vi.stubGlobal(
			'fetch',
			vi.fn().mockResolvedValue(
				Response.json(
					{
						success: true,
						data: { id: 9, media_id: 42, status: 'pending', attempts: 0 },
						message: 'Queued'
					},
					{ status: 202 }
				)
			)
		);
		const result = await mediaApi.optimize('42');
		expect(result.data).toMatchObject({ id: 9, media_id: 42, status: 'pending' });
		expect(result.data).not.toHaveProperty('filename');
	});
});

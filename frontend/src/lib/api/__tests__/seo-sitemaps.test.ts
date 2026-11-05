// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
import { authors } from '#lib/authors/authors.js';
import {
	sitemapAdmin,
	saveSitemapSettings,
	saveVideoSource,
	type VideoSourceWrite
} from '../seo-sitemaps.js';
function submittedBody(value: unknown): unknown {
	if (typeof value !== 'string') throw new Error('Expected an actual JSON request body');
	return JSON.parse(value);
}
const author = authors[0];
if (!author) throw new Error('Actual author registry is empty');
const source = () => ({
	id: '2',
	title: 'Actual fixture source',
	slug: 'actual-fixture-source',
	path: '/learning-center/actual-fixture-source',
	published_at: '2026-11-05T12:00:00Z',
	trader_id: '3',
	author_name: author.name,
	author_slug: null,
	description: 'The actual source video description is shared with its public page and sitemap.',
	video_url: 'https://media.example.test/actual.mp4',
	video_platform: 'self-hosted',
	thumbnail_url: 'https://media.example.test/actual.jpg',
	duration: 600,
	mapping_id: null,
	source_hash: '0'.repeat(64)
});
const settings = {
	enabled: true,
	include_thumbnails: true,
	include_uploader: true,
	detect_from_content: true,
	platforms: ['self-hosted', 'bunny', 'youtube', 'vimeo', 'dailymotion', 'ted', 'wistia']
};
const preview = () => ({
	kind: 'video',
	metrics_scope: 'saved_sitemap_sources',
	settings_version: 1,
	settings,
	last_generation: null,
	preview: {
		data: [
			{
				record: source(),
				eligible: false,
				exclusion_reason: 'Public watch page has not been approved'
			}
		],
		meta: { total: 1, current_page: 1, per_page: 20, last_page: 1 }
	}
});
afterEach(() => vi.unstubAllGlobals());
it('preserves excluded incomplete legacy video metadata for explicit repair without inventing eligibility', () => {
	const value = preview();
	const entry = value.preview.data[0];
	if (!entry) throw new Error('Missing actual fixture source');
	Object.assign(entry.record, {
		slug: 'Legacy Invalid Slug',
		duration: -1,
		published_at: null,
		author_name: null,
		video_url: null,
		video_platform: null
	});
	const parsed = sitemapAdmin(value, 'video');
	expect(parsed.preview.data[0]?.eligible).toBe(false);
	expect(parsed.preview.data[0]?.record.published_at).toBeNull();
	expect(parsed.preview.data[0]?.record.author_name).toBeNull();
	expect(parsed.preview.data[0]?.record).toMatchObject({
		duration: -1,
		video_url: null,
		video_platform: null
	});
	Object.assign(entry, { eligible: true, exclusion_reason: null });
	expect(() => sitemapAdmin(value, 'video')).toThrow();
});
it('rejects missing settings, invalid scopes, inconsistent eligibility and shortened successful pages', () => {
	for (const patch of [
		{ settings_version: 0 },
		{ metrics_scope: 'fake_counters' },
		{ settings: {} },
		{ preview: { ...preview().preview, data: [] } }
	])
		expect(() => sitemapAdmin({ ...preview(), ...patch }, 'video')).toThrow();
	const value = preview();
	const entry = value.preview.data[0];
	if (!entry) throw new Error('Missing actual fixture source');
	entry.eligible = true;
	expect(() => sitemapAdmin(value, 'video')).toThrow();
});
it('submits the saved settings version once and rejects a wrong acknowledgment or a concurrent edit', async () => {
	const upstream = vi.fn<typeof fetch>().mockResolvedValue(
		Response.json({
			kind: 'video',
			saved: true,
			settings_version: 3,
			settings: { ...settings, enabled: false }
		})
	);
	vi.stubGlobal('fetch', upstream);
	await expect(saveSitemapSettings('video', settings, 2)).rejects.toThrow('submitted intent');
	expect(submittedBody(upstream.mock.calls[0]?.[1]?.body)).toEqual({
		expected_version: 2,
		config: { ...settings, platforms: [...settings.platforms].sort() }
	});
	expect(upstream).toHaveBeenCalledOnce();
	upstream
		.mockClear()
		.mockResolvedValue(
			Response.json({ error: 'Sitemap settings changed; reload before saving' }, { status: 409 })
		);
	await expect(saveSitemapSettings('video', settings, 2)).rejects.toThrow('reload');
	expect(upstream).toHaveBeenCalledOnce();
});
it('verifies actual video CRUD readback including original date, trader, duration and platform before reporting a source save', async () => {
	const row = source();
	const fields: VideoSourceWrite = {
		title: row.title,
		description: row.description,
		video_url: row.video_url,
		video_platform: row.video_platform,
		thumbnail_url: row.thumbnail_url,
		duration: row.duration,
		published_at: row.published_at,
		trader_id: row.trader_id
	};
	const upstream = vi
		.fn<typeof fetch>()
		.mockResolvedValueOnce(Response.json({ success: true, data: { id: 2 } }))
		.mockResolvedValueOnce(Response.json({ record: row }));
	vi.stubGlobal('fetch', upstream);
	expect((await saveVideoSource(fields)).id).toBe('2');
	expect(upstream).toHaveBeenCalledTimes(2);
	expect(submittedBody(upstream.mock.calls[0]?.[1]?.body)).toMatchObject({
		trader_id: 3,
		duration: 600,
		published_at: row.published_at,
		content_type: 'learning_center',
		is_published: true,
		room_ids: []
	});
	upstream
		.mockReset()
		.mockResolvedValueOnce(Response.json({ success: true, data: { id: 2 } }))
		.mockResolvedValueOnce(Response.json({ record: { ...row, video_platform: 'different' } }));
	await expect(saveVideoSource(fields, '2')).rejects.toThrow('did not match');
	expect(upstream).toHaveBeenCalledTimes(2);
});

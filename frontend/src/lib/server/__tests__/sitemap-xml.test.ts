import { expect, it } from 'vite-plus/test';
import { renderSourceSitemap } from '../sitemap-xml.js';
import { authors } from '#lib/authors/authors.js';
const author = authors[0];
if (!author) throw new Error('Actual author registry is empty');
const at = '2026-11-05T12:00:00Z',
	now = Date.parse(at);
const news = () => ({
	kind: 'news',
	metrics_scope: 'published_sitemap_sources',
	settings_version: 1,
	settings: {
		enabled: true,
		publication_name: 'Fixture & Publication',
		language: 'en',
		max_age_hours: 48,
		exclude_categories: []
	},
	entries: {
		data: [
			{
				eligible: true,
				exclusion_reason: null,
				record: {
					id: '1',
					title: 'Fixture <article> & title',
					path: '/blog/fixture-article',
					slug: 'fixture-article',
					published_at: at,
					author_name: author.name,
					author_slug: author.slug,
					canonical: null
				}
			}
		],
		meta: { current_page: 1, per_page: 1000, total: 1, last_page: 1 }
	}
});
it('emits original article dates, exact escaped titles and the configured publication without invented fields', () => {
	const xml = renderSourceSitemap(news(), 'news', false, now);
	expect(xml).toContain('<news:title>Fixture &lt;article&gt; &amp; title</news:title>');
	expect(xml).toContain(`<news:publication_date>${at}</news:publication_date>`);
	expect(xml).toContain('<news:name>Fixture &amp; Publication</news:name>');
	expect(xml).not.toMatch(/<priority>|<changefreq>|rating|review/);
});
it('splits bounded source pages into a sitemap index and rejects excluded, stale, future or cross-canonical articles', () => {
	const source = news();
	source.entries.meta = { current_page: 1, per_page: 1, total: 2, last_page: 2 };
	expect(renderSourceSitemap(source, 'news', false, now)).toContain('/news-sitemap.xml?page=2');
	expect(renderSourceSitemap(source, 'news', true, now)).toContain('<urlset');
	for (const patch of [
		{ published_at: null },
		{ published_at: '2026-11-02T12:00:00Z' },
		{ published_at: '2026-11-06T12:00:00Z' },
		{ author_slug: 'invented-author' },
		{ canonical: '/different-url' }
	]) {
		const source = news();
		Object.assign(source.entries.data[0]?.record ?? {}, patch);
		expect(() => renderSourceSitemap(source, 'news', false, now)).toThrow();
	}
});
it('allows an intentional empty disabled feed while refusing a fabricated successful feed with missing source facts', () => {
	const source = news();
	source.settings.enabled = false;
	source.entries.data = [];
	source.entries.meta.total = 0;
	expect(renderSourceSitemap(source, 'news', false, now)).toContain('<urlset');
	expect(() => renderSourceSitemap({ kind: 'news', entries: [] }, 'news', false, now)).toThrow();
});
it('emits only complete approved video metadata and never turns missing duration or authorship into a measurement', () => {
	const video = {
		id: '2',
		title: 'Fixture video',
		path: '/learning-center/fixture-video',
		slug: 'fixture-video',
		published_at: at,
		author_name: author.name,
		author_slug: author.slug,
		description:
			'Actual saved fixture description used by both the watch page and its video sitemap.',
		video_url: 'https://media.example.test/fixture.mp4',
		video_platform: 'self-hosted',
		thumbnail_url: 'https://media.example.test/thumbnail.jpg?a=1&b=2',
		duration: 600,
		mapping_id: '5',
		source_hash: '0'.repeat(64),
		playback_url: 'https://media.example.test/fixture.mp4',
		playback_kind: 'video'
	};
	const source = {
		kind: 'video',
		metrics_scope: 'published_sitemap_sources',
		settings_version: 1,
		settings: {
			enabled: true,
			include_thumbnails: true,
			include_uploader: true,
			detect_from_content: true
		},
		entries: {
			data: [{ eligible: true, exclusion_reason: null, record: video }],
			meta: { total: 1, current_page: 1, per_page: 1000, last_page: 1 }
		}
	};
	const xml = renderSourceSitemap(source, 'video', false, now);
	expect(xml).toContain('<video:duration>600</video:duration>');
	expect(xml).toContain(
		'<video:content_loc>https://media.example.test/fixture.mp4</video:content_loc>'
	);
	expect(xml).toContain('thumbnail.jpg?a=1&amp;b=2');
	for (const patch of [
		{ duration: null },
		{ duration: 0 },
		{ author_slug: null },
		{ thumbnail_url: null },
		{ path: '/dashboard/private-video' }
	])
		expect(() =>
			renderSourceSitemap(
				{
					...source,
					entries: {
						...source.entries,
						data: [{ ...source.entries.data[0], record: { ...video, ...patch } }]
					}
				},
				'video',
				false,
				now
			)
		).toThrow();
});

import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';
import { authors } from '#lib/authors/authors.js';
import {
	object,
	sitemapPage,
	sitemapSettings,
	sitemapSource,
	publicVideo,
	type SitemapKind,
	type SourceRecord
} from '#lib/api/seo-sitemaps.js';

/** Required extension fields only; dates and authors always come from saved facts. */
export function xmlText(value: string): string {
	for (const character of value) {
		const code = character.codePointAt(0);
		if (
			code === undefined ||
			(code < 32 && ![9, 10, 13].includes(code)) ||
			code === 65534 ||
			code === 65535 ||
			(code >= 55296 && code <= 57343)
		)
			throw new Error('A source contains characters that XML cannot represent');
	}

	return value
		.replaceAll('&', '&amp;')
		.replaceAll('<', '&lt;')
		.replaceAll('>', '&gt;')
		.replaceAll('"', '&quot;')
		.replaceAll("'", '&apos;');
}
function canonical(record: SourceRecord): string {
	if (
		record.canonical &&
		record.canonical !== record.path &&
		record.canonical !== `${PRODUCTION_ORIGIN}${record.path}`
	)
		throw new Error('Source canonical does not match its sitemap URL');
	return `${PRODUCTION_ORIGIN}${record.path}`;
}
export function renderSourceSitemap(
	value: unknown,
	kind: SitemapKind,
	explicitPage: boolean,
	now = Date.now()
): string {
	const row = object(value);
	if (
		typeof row.settings_version !== 'number' ||
		!Number.isSafeInteger(row.settings_version) ||
		row.settings_version < 1
	)
		throw new Error('Saved sitemap settings version is unavailable');
	if (row.kind !== kind || row.metrics_scope !== 'published_sitemap_sources')
		throw new Error('Published sitemap source scope did not match');
	const settings = sitemapSettings(row.settings, kind);
	const page = sitemapPage(row.entries, (value) => {
		const entry = object(value);
		if (entry.eligible !== true || entry.exclusion_reason !== null)
			throw new Error('Public sitemap contains an excluded source');
		return entry.record;
	});
	if (!settings.enabled && (page.meta.total || page.data.length))
		throw new Error('Disabled sitemap returned published entries');
	if (page.meta.current_page > page.meta.last_page)
		throw new RangeError('Sitemap page does not exist');
	if (!explicitPage && page.meta.last_page > 1) {
		if (page.meta.last_page > 50000)
			throw new Error('Sitemap index exceeds its supported file count');
		const files = Array.from(
			{ length: page.meta.last_page },
			(_, index) =>
				`<sitemap><loc>${xmlText(`${PRODUCTION_ORIGIN}/${kind}-sitemap.xml?page=${index + 1}`)}</loc></sitemap>`
		).join('\n');
		return `<?xml version="1.0" encoding="UTF-8"?>\n<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n${files}\n</sitemapindex>\n`;
	}
	const entries = page.data
		.map((value) => {
			if (kind === 'news') {
				if (!('publication_name' in settings))
					throw new Error('News publication settings are unavailable');
				const source = sitemapSource(value, 'news');
				if (
					!source.published_at ||
					!authors.some(
						(author) => author.slug === source.author_slug && author.name === source.author_name
					)
				)
					throw new Error('Original publication date or actual author is unavailable');
				const published = Date.parse(source.published_at);
				if (published > now || published < now - settings.max_age_hours * 3600000)
					throw new Error('Article is outside the current publication window');
				return `<url><loc>${xmlText(canonical(source))}</loc><news:news><news:publication><news:name>${xmlText(settings.publication_name)}</news:name><news:language>${xmlText(settings.language)}</news:language></news:publication><news:publication_date>${xmlText(source.published_at)}</news:publication_date><news:title>${xmlText(source.title)}</news:title></news:news></url>`;
			}
			const source = publicVideo(value);
			if (Date.parse(source.published_at) > now)
				throw new Error('Video publication date is in the future');
			return `<url><loc>${xmlText(canonical(source))}</loc><video:video><video:thumbnail_loc>${xmlText(source.thumbnail_url)}</video:thumbnail_loc><video:title>${xmlText(source.title)}</video:title><video:description>${xmlText(source.description)}</video:description><video:${source.playback_kind === 'video' ? 'content_loc' : 'player_loc'}>${xmlText(source.playback_url)}</video:${source.playback_kind === 'video' ? 'content_loc' : 'player_loc'}><video:duration>${source.duration}</video:duration><video:publication_date>${xmlText(source.published_at)}</video:publication_date>${'include_uploader' in settings && settings.include_uploader ? `<video:uploader info="${xmlText(`${PRODUCTION_ORIGIN}/authors/${source.author_slug}`)}">${xmlText(source.author_name)}</video:uploader>` : ''}</video:video></url>`;
		})
		.join('\n');
	const result = `<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:${kind === 'news' ? 'news="http://www.google.com/schemas/sitemap-news/0.9"' : 'video="http://www.google.com/schemas/sitemap-video/1.1"'}>\n${entries}\n</urlset>\n`;
	if (new TextEncoder().encode(result).byteLength > 50000000)
		throw new Error('Sitemap exceeds its uncompressed size limit');
	return result;
}

import { authors } from '#lib/authors/authors.js';

export type SitemapKind = 'news' | 'video';
export interface NewsSitemapSettings {
	enabled: boolean;
	publication_name: string;
	language: string;
	max_age_hours: number;
	exclude_categories: string[];
}
export interface VideoSitemapSettings {
	enabled: boolean;
	include_thumbnails: boolean;
	include_uploader: boolean;
	detect_from_content: boolean;
}
export type SitemapSettings = NewsSitemapSettings | VideoSitemapSettings;
export interface SourceRecord {
	id: string;
	title: string;
	path: string;
	slug: string;
	published_at: string | null;
	author_name: string | null;
	author_slug: string | null;
	canonical?: string | null;
}
export interface PublicVideo extends SourceRecord {
	description: string;
	video_url: string;
	video_platform: string;
	thumbnail_url: string;
	duration: number;
	playback_url: string;
	playback_kind: 'video' | 'iframe';
	author_slug: string;
	author_name: string;
	published_at: string;
}
export interface VideoCandidate extends SourceRecord {
	description: string | null;
	video_url: string;
	video_platform: string;
	thumbnail_url: string | null;
	duration: number | null;
	mapping_id: string | null;
	source_hash: string;
}
export interface SitemapCandidate {
	record: SourceRecord | VideoCandidate;
	eligible: boolean;
	exclusion_reason: string | null;
}
export interface SitemapPage<T> {
	data: T[];
	meta: { total: number; current_page: number; per_page: number; last_page: number };
}
export interface SitemapGeneration {
	id: string;
	created_at: string;
	entry_count: number;
	settings_version: number;
	source_hash: string;
}
export interface SitemapAdmin {
	kind: SitemapKind;
	settings: SitemapSettings;
	settings_version: number;
	preview: SitemapPage<SitemapCandidate>;
	last_generation: SitemapGeneration | null;
}
export function object(value: unknown): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		throw new Error('Invalid sitemap response');
	const row = value as Record<string, unknown>;
	if (row.success === false || row.error)
		throw new Error(
			typeof row.error === 'string' ? row.error : 'Sitemap operation was not confirmed'
		);
	return row;
}
function text(value: unknown): string {
	if (typeof value !== 'string') throw new Error('Invalid saved sitemap text');
	return value;
}
function integer(value: unknown, min = 0): number {
	if (typeof value !== 'number' || !Number.isSafeInteger(value) || value < min)
		throw new Error('Invalid saved sitemap count');
	return value;
}
function id(value: unknown): string {
	const result = text(value);
	if (!/^[1-9]\d{0,18}$/.test(result)) throw new Error('Invalid sitemap source identity');
	return result;
}
function timestamp(value: unknown): string {
	const result = text(value);
	if (!Number.isFinite(Date.parse(result))) throw new Error('Invalid saved publication date');
	return result;
}
function nullable(value: unknown): string | null {
	return value === null ? null : text(value);
}
function bool(value: unknown): boolean {
	if (typeof value !== 'boolean') throw new Error('Invalid saved sitemap option');
	return value;
}
function hash(value: unknown): string {
	const result = text(value);
	if (!/^[a-f0-9]{64}$/.test(result)) throw new Error('Invalid sitemap source digest');
	return result;
}
function uuid(value: unknown): string {
	const result = text(value);
	if (!/^[a-f0-9]{8}(?:-[a-f0-9]{4}){3}-[a-f0-9]{12}$/i.test(result))
		throw new Error('Invalid sitemap generation identity');
	return result;
}
export function sitemapSettings(value: unknown, kind: SitemapKind): SitemapSettings {
	const row = object(value);
	if (kind === 'news') {
		if (
			!Array.isArray(row.exclude_categories) ||
			row.exclude_categories.some((value) => typeof value !== 'string') ||
			!/^(?:[a-z]{2,3}|zh-(?:cn|tw))$/.test(text(row.language))
		)
			throw new Error('Invalid saved news sitemap settings');
		const maxAge = integer(row.max_age_hours, 1);
		if (maxAge > 48 || !text(row.publication_name).trim())
			throw new Error('Invalid saved news publication settings');
		return {
			enabled: bool(row.enabled),
			publication_name: text(row.publication_name),
			language: text(row.language),
			max_age_hours: maxAge,
			exclude_categories: row.exclude_categories as string[]
		};
	}
	if (row.include_thumbnails !== true) throw new Error('Video sitemaps require actual thumbnails');
	return {
		enabled: bool(row.enabled),
		include_thumbnails: true,
		include_uploader: bool(row.include_uploader),
		detect_from_content: bool(row.detect_from_content)
	};
}
export function sitemapSource(
	value: unknown,
	kind: SitemapKind,
	allowUnreviewed = false
): SourceRecord | VideoCandidate {
	const row = object(value),
		slug = text(row.slug),
		path = text(row.path);
	if (
		!allowUnreviewed &&
		(!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(slug) ||
			path !== `/${kind === 'news' ? 'blog' : 'learning-center'}/${slug}`)
	)
		throw new Error('Invalid canonical sitemap source path');
	const common: SourceRecord = {
		id: id(row.id),
		title: text(row.title),
		path,
		slug,
		published_at: row.published_at === null ? null : timestamp(row.published_at),
		author_name: nullable(row.author_name),
		author_slug: nullable(row.author_slug)
	};
	if (kind === 'news') return { ...common, canonical: nullable(row.canonical) };
	return {
		...common,
		description: nullable(row.description),
		video_url: text(row.video_url),
		video_platform: text(row.video_platform),
		thumbnail_url: nullable(row.thumbnail_url),
		duration: row.duration === null ? null : integer(row.duration),
		mapping_id: row.mapping_id === null ? null : id(row.mapping_id),
		source_hash: hash(row.source_hash)
	};
}
export function sitemapPage<T>(value: unknown, read: (value: unknown) => T): SitemapPage<T> {
	const row = object(value),
		meta = object(row.meta);
	if (!Array.isArray(row.data)) throw new Error('Invalid sitemap source list');
	const total = integer(meta.total),
		page = integer(meta.current_page, 1),
		size = integer(meta.per_page, 1),
		last = integer(meta.last_page, 1);
	if (
		size > 1000 ||
		row.data.length > size ||
		total < row.data.length ||
		last !== Math.max(1, Math.ceil(total / size)) ||
		row.data.length !== (page > last ? 0 : Math.max(0, Math.min(size, total - (page - 1) * size)))
	)
		throw new Error('Invalid sitemap pagination');
	return {
		data: row.data.map((value) => read(value)),
		meta: { total, current_page: page, per_page: size, last_page: last }
	};
}
export function sitemapGeneration(value: unknown): SitemapGeneration {
	const row = object(value);
	return {
		id: uuid(row.id),
		created_at: timestamp(row.created_at),
		entry_count: integer(row.entry_count),
		settings_version: integer(row.settings_version, 1),
		source_hash: hash(row.source_hash)
	};
}
export function sitemapAdmin(value: unknown, kind: SitemapKind): SitemapAdmin {
	const row = object(value);
	if (row.kind !== kind || row.metrics_scope !== 'saved_sitemap_sources')
		throw new Error('Sitemap source scope did not match');
	return {
		kind,
		settings: sitemapSettings(row.settings, kind),
		settings_version: integer(row.settings_version, 1),
		last_generation: row.last_generation === null ? null : sitemapGeneration(row.last_generation),
		preview: sitemapPage(row.preview, (value) => {
			const entry = object(value);
			const eligible = bool(entry.eligible),
				reason = nullable(entry.exclusion_reason);
			if (eligible !== (reason === null))
				throw new Error('Sitemap eligibility does not match its explanation');
			return {
				record: sitemapSource(entry.record, kind, !eligible),
				eligible,
				exclusion_reason: reason
			};
		})
	};
}
export function publicVideo(value: unknown): PublicVideo {
	const row = object(value),
		source = sitemapSource(row, 'video');
	const author = authors.find(
		(author) => author.slug === source.author_slug && author.name === source.author_name
	);
	if (
		!author ||
		!source.published_at ||
		!('description' in source) ||
		!source.description ||
		source.description.length < 50 ||
		source.description.length > 2048 ||
		!source.thumbnail_url ||
		source.duration === null ||
		source.duration < 1 ||
		source.duration > 28800 ||
		!['video', 'iframe'].includes(String(row.playback_kind))
	)
		throw new Error('Public video metadata is incomplete or lacks its actual author');
	for (const value of [source.thumbnail_url, text(row.playback_url)]) {
		const url = new URL(value);
		if (!['https:', 'http:'].includes(url.protocol) || url.username || url.password || url.hash)
			throw new Error('Invalid public video media URL');
	}
	return {
		...source,
		description: source.description,
		thumbnail_url: source.thumbnail_url,
		duration: source.duration,
		published_at: source.published_at,
		author_slug: author.slug,
		author_name: author.name,
		playback_url: text(row.playback_url),
		playback_kind: row.playback_kind as PublicVideo['playback_kind']
	};
}
async function request(path: string, method = 'GET', body?: unknown): Promise<unknown> {
	const response = await fetch(path, {
		method,
		credentials: 'include',
		cache: 'no-store',
		headers: { 'Content-Type': 'application/json' },
		...(body !== undefined ? { body: JSON.stringify(body) } : {})
	});
	const value: unknown = await response.json().catch(() => null);
	if (!response.ok) {
		const row = value && typeof value === 'object' ? (value as Record<string, unknown>) : {};
		throw new Error(
			typeof row.message === 'string'
				? row.message
				: typeof row.error === 'string'
					? row.error
					: 'Sitemap service is unavailable'
		);
	}
	return value;
}
export async function getSitemapAdmin(kind: SitemapKind, page = 1): Promise<SitemapAdmin> {
	return sitemapAdmin(
		await request(`/api/admin/seo/sitemaps/${kind}?page=${integer(page, 1)}&per_page=20`),
		kind
	);
}
export async function saveSitemapSettings(
	kind: SitemapKind,
	config: SitemapSettings,
	expectedVersion: number
) {
	const row = object(
		await request(`/api/admin/seo/sitemaps/${kind}`, 'PUT', {
			config: sitemapSettings(config, kind),
			expected_version: integer(expectedVersion, 1)
		})
	);
	if (row.saved !== true || row.kind !== kind || row.settings_version !== expectedVersion + 1)
		throw new Error('Sitemap settings save was not confirmed');
	return {
		settings: sitemapSettings(row.settings, kind),
		version: integer(row.settings_version, 1)
	};
}
export async function generateSitemap(kind: SitemapKind): Promise<SitemapGeneration> {
	const row = object(await request(`/api/admin/seo/sitemaps/${kind}/generate`, 'POST', {}));
	if (row.generated !== true || row.kind !== kind)
		throw new Error('Sitemap generation was not confirmed');
	return sitemapGeneration(row.generation);
}
export async function approveVideo(video: VideoCandidate, authorSlug: string) {
	if (!authors.some((author) => author.slug === authorSlug))
		throw new Error('Choose an existing public author profile');
	const row = object(
		await request('/api/admin/seo/video-publications', 'POST', {
			video_id: id(video.id),
			author_slug: authorSlug,
			expected_source_hash: hash(video.source_hash)
		})
	);
	if (row.approved !== true || row.video_id !== video.id || row.path !== video.path)
		throw new Error('Public video approval was not confirmed');
	return row;
}
export async function removeVideoPublication(videoId: string) {
	const row = object(await request(`/api/admin/seo/video-publications/${id(videoId)}`, 'DELETE'));
	if (row.removed !== true || row.video_id !== videoId)
		throw new Error('Video publication removal was not confirmed');
}

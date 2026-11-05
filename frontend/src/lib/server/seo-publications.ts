import { error, type RequestEvent } from '@sveltejs/kit';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';
import {
	object,
	publicVideo,
	sitemapPage,
	sitemapSource,
	type SitemapKind
} from '#lib/api/seo-sitemaps.js';
import { renderSourceSitemap } from './sitemap-xml.js';
import { requireAdmin } from './auth.js';
const root = API_BASE_URL || BACKEND_URL || 'http://localhost:8080';
/** Public sources are always fetched without an authenticated user's credentials. */
export async function readPublication(path: string): Promise<unknown> {
	let response: Response;
	try {
		response = await fetch(`${root}/api/seo/${path}`, {
			headers: { Accept: 'application/json' },
			credentials: 'omit',
			redirect: 'error',
			cache: 'no-store',
			signal: AbortSignal.timeout(20000)
		});
	} catch {
		error(503, 'Published content is temporarily unavailable');
	}
	if (!response.ok)
		error(
			response.status === 404 ? 404 : 503,
			response.status === 404
				? 'Public publication not found'
				: 'Published content is temporarily unavailable'
		);
	try {
		return await response.json();
	} catch {
		error(503, 'Published content response is invalid');
	}
}
export async function readPublicVideo(slug: string) {
	if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(slug)) error(404, 'Public video not found');
	const video = publicVideo(await readPublication(`public-videos/${slug}`));
	if (video.slug !== slug || Date.parse(video.published_at) > Date.now())
		error(503, 'Public video metadata did not match');
	return video;
}
export async function readPublicVideos(page = 1, size = 20) {
	const row = object(await readPublication(`public-videos?page=${page}&per_page=${size}`));
	if (row.metrics_scope !== 'approved_public_videos')
		error(503, 'Public video catalog did not match');
	return sitemapPage(row.entries, (value) => {
		const entry = object(value);
		if (entry.eligible !== true || entry.exclusion_reason !== null)
			error(503, 'Public video catalog contains an excluded source');
		return publicVideo(entry.record);
	});
}
export async function sourceSitemap(url: URL, kind: SitemapKind): Promise<Response> {
	const raw = url.searchParams.get('page');
	if (raw !== null && !/^[1-9]\d{0,6}$/.test(raw)) error(400, 'Choose a positive sitemap page');
	const page = raw ? Number(raw) : 1;
	const value = await readPublication(`sitemaps/${kind}?page=${page}&per_page=1000`);
	let xml: string;
	try {
		xml = renderSourceSitemap(value, kind, raw !== null);
	} catch (cause) {
		if (cause instanceof RangeError) error(404, cause.message);
		error(503, 'Sitemap sources are invalid or temporarily unavailable');
	}
	return new Response(xml, {
		headers: {
			'Content-Type': 'application/xml; charset=utf-8',
			'Cache-Control': 'public, max-age=0, s-maxage=60',
			'X-Robots-Tag': 'noindex'
		}
	});
}
/** One attempt preserves write intent: an ambiguous response is never replayed automatically. */
export async function proxyPublicationAdmin(
	event: RequestEvent,
	namespace: 'seo' | 'videos'
): Promise<Response> {
	const { token } = requireAdmin(event),
		rest = event.params.rest ?? '',
		method = event.request.method;
	const videoSource =
		rest === 'video-sources' || /^video-sources\/(?:options|[1-9]\d{0,18})$/.test(rest);
	const allowed =
		namespace === 'seo' &&
		(videoSource
			? rest === 'video-sources'
				? method === 'POST'
				: rest === 'video-sources/options'
					? method === 'GET'
					: method === 'PUT'
			: /^(?:sitemaps\/(?:news|video)(?:\/generate)?|video-publications(?:\/[1-9]\d{0,18})?)$/.test(
					rest
				) &&
				(rest.endsWith('/generate')
					? method === 'POST'
					: rest.startsWith('sitemaps/')
						? ['GET', 'PUT'].includes(method)
						: rest === 'video-publications'
							? method === 'POST'
							: ['GET', 'DELETE'].includes(method)));
	const targetPath = videoSource ? `videos${rest.slice('video-sources'.length)}` : `seo/${rest}`;
	const headers = new Headers({
		'Cache-Control': 'private, no-store',
		'X-Content-Type-Options': 'nosniff'
	});
	if (!allowed)
		return Response.json({ error: 'Publication endpoint not found' }, { status: 404, headers });
	let body: string | undefined;
	if (!['GET', 'HEAD'].includes(method)) {
		body = await event.request.text();
		if (new TextEncoder().encode(body).byteLength > 100000)
			return Response.json(
				{ error: 'Publication request exceeds 100KB' },
				{ status: 413, headers }
			);
	}
	try {
		const response = await fetch(
			`${root}/api/admin/${targetPath}${event.url.search}`
				.replace(/\/(?=\?)/, '')
				.replace(/\/$/, ''),
			{
				method,
				headers: {
					Authorization: `Bearer ${token}`,
					'Content-Type': 'application/json',
					Accept: 'application/json'
				},
				body,
				credentials: 'omit',
				redirect: 'error',
				signal: AbortSignal.timeout(30000)
			}
		);
		headers.set('Content-Type', response.headers.get('Content-Type') ?? 'application/json');
		return new Response(response.body, { status: response.status, headers });
	} catch {
		return Response.json(
			{ error: 'Publication service is unavailable; reload saved state before retrying a write' },
			{ status: 503, headers }
		);
	}
}

export async function readIndexableArticlePaths(): Promise<string[]> {
	const paths: string[] = [];
	for (let page = 1; ; page += 1) {
		const row = object(await readPublication(`public-articles?page=${page}&per_page=1000`));
		if (row.metrics_scope !== 'indexable_public_articles')
			error(503, 'Public article catalog did not match');
		const result = sitemapPage(row.entries, (value) => {
			const entry = object(value);
			if (entry.eligible !== true || entry.exclusion_reason !== null)
				error(503, 'Public article catalog contains an excluded source');
			return sitemapSource(entry.record, 'news');
		});
		paths.push(...result.data.map((source) => source.path));
		if (page >= result.meta.last_page) return paths;
		if (page >= 50000) error(503, 'Public article catalog exceeds its supported size');
	}
}
export async function readPublicVideoPaths(): Promise<string[]> {
	const paths: string[] = [];
	for (let page = 1; ; page += 1) {
		const result = await readPublicVideos(page, 1000);
		paths.push(...result.data.map((source) => source.path));
		if (page >= result.meta.last_page) return paths;
		if (page >= 50000) error(503, 'Public video catalog exceeds its supported size');
	}
}

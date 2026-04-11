/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Revolution Trading Pros - Sitemap Dynamic Route Resolver
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Shared helpers that resolve canonical slug lists for every
 *              publicly indexable dynamic route (blog, classes, indicators,
 *              learning center, ...) at sitemap render time. The main
 *              `sitemap.xml`, `news-sitemap.xml`, and `video-sitemap.xml`
 *              endpoints call into this module so there is a SINGLE source of
 *              truth for "which dynamic URLs does Google see?".
 *
 *              Every fetcher is defensive:
 *                - Honours upstream errors (returns []) so a flaky API never
 *                  breaks the sitemap response.
 *                - Filters out falsy slugs so `paramValues` never contains
 *                  `undefined` / empty strings (which `super-sitemap` would
 *                  emit as `/blog/` → duplicate URLs / soft-404s).
 *                - De-duplicates the returned array.
 *                - Accepts a SvelteKit `fetch` so it works inside `+server.ts`
 *                  load functions and inherits request cookies/adapter
 *                  behaviour on Cloudflare Pages.
 *
 * @standards Google Search Essentials (Nov 2025) + Apple ICT 11 engineering
 * @version   1.0.0
 */

// ─── API base ──────────────────────────────────────────────────────────────────
// Kept local so this module has no runtime dependency on the $app/env store —
// sitemap endpoints must render both during SSR and during build-time prerender
// probes, and `$env/dynamic/private` is not available in that context.
const API_URL =
	(typeof import.meta !== 'undefined' && (import.meta as any).env?.VITE_API_URL) ||
	'https://revolution-trading-pros-api.fly.dev';

// Fetch type that works with both SvelteKit's event.fetch and globalThis.fetch.
type FetchLike = typeof globalThis.fetch;

// ─── Types ─────────────────────────────────────────────────────────────────────

export interface SlugWithLastmod {
	slug: string;
	lastmod?: string;
}

export interface NewsArticleEntry {
	slug: string;
	title: string;
	publicationDate: string;
	imageUrl?: string;
	author?: string;
	keywords?: string[];
	stockTickers?: string[];
}

export interface VideoResourceEntry {
	pageUrl: string;
	title: string;
	description: string;
	thumbnailUrl: string;
	playerUrl?: string;
	duration?: number;
	publicationDate?: string;
	tags?: string[];
	platform?: 'youtube' | 'vimeo' | 'dailymotion' | 'ted' | 'wistia' | 'self-hosted';
	videoId?: string;
}

// ─── Internal utilities ────────────────────────────────────────────────────────

function dedupe<T extends string>(values: T[]): T[] {
	return Array.from(new Set(values.filter(Boolean)));
}

async function safeJson<T>(
	fetchFn: FetchLike,
	url: string,
	timeoutMs = 8000
): Promise<T | null> {
	const controller = new AbortController();
	const timer = setTimeout(() => controller.abort(), timeoutMs);
	try {
		const res = await fetchFn(url, {
			headers: { Accept: 'application/json' },
			signal: controller.signal
		});
		if (!res.ok) return null;
		return (await res.json()) as T;
	} catch {
		return null;
	} finally {
		clearTimeout(timer);
	}
}

// ─── Blog posts ────────────────────────────────────────────────────────────────
// GET /api/posts → PaginatedPosts { data: Post[], ... }

interface PostShape {
	slug?: string;
	published_at?: string;
	updated_at?: string;
	indexable?: boolean;
	title?: string;
	featured_image?: string | null;
	meta_description?: string | null;
	excerpt?: string | null;
	author?: { name?: string } | null;
}

interface PaginatedPostsShape {
	data?: PostShape[];
}

async function fetchAllPosts(fetchFn: FetchLike): Promise<PostShape[]> {
	// Rust API default per_page is small; request the max to minimise requests.
	const json = await safeJson<PaginatedPostsShape>(
		fetchFn,
		`${API_URL}/api/posts?per_page=1000`
	);
	const posts = json?.data ?? [];
	// Drop any post flagged as not indexable.
	return posts.filter((p) => p?.slug && p.indexable !== false);
}

export async function fetchBlogSlugs(fetchFn: FetchLike): Promise<string[]> {
	const posts = await fetchAllPosts(fetchFn);
	return dedupe(posts.map((p) => p.slug!));
}

export async function fetchBlogSlugsWithLastmod(
	fetchFn: FetchLike
): Promise<SlugWithLastmod[]> {
	const posts = await fetchAllPosts(fetchFn);
	const seen = new Set<string>();
	const out: SlugWithLastmod[] = [];
	for (const p of posts) {
		if (!p.slug || seen.has(p.slug)) continue;
		seen.add(p.slug);
		out.push({
			slug: p.slug,
			lastmod: p.updated_at ?? p.published_at ?? undefined
		});
	}
	return out;
}

export async function fetchRecentNewsArticles(
	fetchFn: FetchLike,
	windowHours = 48
): Promise<NewsArticleEntry[]> {
	const posts = await fetchAllPosts(fetchFn);
	const cutoff = Date.now() - windowHours * 60 * 60 * 1000;
	const entries: NewsArticleEntry[] = [];
	for (const p of posts) {
		if (!p.slug || !p.title) continue;
		const publicationDate = p.published_at ?? p.updated_at;
		if (!publicationDate) continue;
		const published = Date.parse(publicationDate);
		if (Number.isNaN(published) || published < cutoff) continue;
		entries.push({
			slug: p.slug,
			title: p.title,
			publicationDate: new Date(published).toISOString(),
			imageUrl: p.featured_image ?? undefined,
			author: p.author?.name ?? undefined
		});
	}
	return entries;
}

// ─── Classes (served via Courses API) ──────────────────────────────────────────
// GET /api/courses → { courses: CourseListItem[], total, page, per_page }

interface CourseListItemShape {
	slug?: string;
	title?: string;
	updated_at?: string;
	created_at?: string;
	is_published?: boolean;
	status?: string;
}

interface CoursesListShape {
	courses?: CourseListItemShape[];
	data?: CourseListItemShape[];
}

async function fetchAllCourses(fetchFn: FetchLike): Promise<CourseListItemShape[]> {
	const json = await safeJson<CoursesListShape>(
		fetchFn,
		`${API_URL}/api/courses?per_page=1000`
	);
	const list = json?.courses ?? json?.data ?? [];
	return list.filter(
		(c) => c?.slug && c.is_published !== false && c.status !== 'draft'
	);
}

export async function fetchClassSlugs(fetchFn: FetchLike): Promise<string[]> {
	const list = await fetchAllCourses(fetchFn);
	return dedupe(list.map((c) => c.slug!));
}

export async function fetchClassSlugsWithLastmod(
	fetchFn: FetchLike
): Promise<SlugWithLastmod[]> {
	const list = await fetchAllCourses(fetchFn);
	const seen = new Set<string>();
	const out: SlugWithLastmod[] = [];
	for (const c of list) {
		if (!c.slug || seen.has(c.slug)) continue;
		seen.add(c.slug);
		out.push({ slug: c.slug, lastmod: c.updated_at ?? c.created_at });
	}
	return out;
}

// ─── Indicators ────────────────────────────────────────────────────────────────
// GET /api/indicators → { data: [...] } or { indicators: [...] } depending on
// backend — handle both to be future-proof.

interface IndicatorShape {
	slug?: string;
	is_published?: boolean;
	status?: string;
	updated_at?: string;
	created_at?: string;
}

interface IndicatorListShape {
	data?: IndicatorShape[];
	indicators?: IndicatorShape[];
}

async function fetchAllIndicators(fetchFn: FetchLike): Promise<IndicatorShape[]> {
	const json = await safeJson<IndicatorListShape>(
		fetchFn,
		`${API_URL}/api/indicators?per_page=1000`
	);
	const list = json?.data ?? json?.indicators ?? [];
	return list.filter(
		(i) => i?.slug && i.is_published !== false && i.status !== 'draft'
	);
}

export async function fetchIndicatorSlugs(fetchFn: FetchLike): Promise<string[]> {
	const list = await fetchAllIndicators(fetchFn);
	return dedupe(list.map((i) => i.slug!));
}

// ─── Learning Center resources ────────────────────────────────────────────────
// GET /api/learning-center — optional endpoint. If the Rust API doesn't yet
// expose it, the fetcher returns [] and the route is simply omitted from the
// sitemap (existing behaviour) until the backend ships it.

interface LearningResourceShape {
	slug?: string;
	updated_at?: string;
}

interface LearningListShape {
	data?: LearningResourceShape[];
	resources?: LearningResourceShape[];
}

export async function fetchLearningCenterSlugs(
	fetchFn: FetchLike
): Promise<string[]> {
	const json = await safeJson<LearningListShape>(
		fetchFn,
		`${API_URL}/api/learning-center?per_page=1000`
	);
	const list = json?.data ?? json?.resources ?? [];
	return dedupe(
		list.map((r) => r?.slug).filter((s): s is string => typeof s === 'string')
	);
}

// ─── Posts legacy ──────────────────────────────────────────────────────────────
// `/posts/[slug]` currently has no public +page.svelte (only /edit). Exported
// as a no-op so the sitemap caller stays symmetric with the route table.
export async function fetchPostSlugs(_fetchFn: FetchLike): Promise<string[]> {
	return [];
}

// ─── Videos (for video-sitemap.xml) ───────────────────────────────────────────
// GET /api/videos → { data: Video[] } | { videos: Video[] }

interface VideoShape {
	id: number;
	title: string;
	description?: string;
	url: string;
	platform?: string;
	video_id?: string;
	thumbnail_url?: string;
	duration?: number;
	is_active?: boolean;
	created_at?: string;
	updated_at?: string;
}

interface VideoListShape {
	data?: VideoShape[];
	videos?: VideoShape[];
}

export async function fetchVideoResources(
	fetchFn: FetchLike
): Promise<VideoResourceEntry[]> {
	const json = await safeJson<VideoListShape>(
		fetchFn,
		`${API_URL}/api/videos?per_page=500`
	);
	const list = json?.data ?? json?.videos ?? [];

	return list
		.filter((v) => v && v.is_active !== false && v.url && v.title)
		.map<VideoResourceEntry>((v) => ({
			// Videos live on the public video detail route (fallback: /videos/{id}).
			// We keep `/videos/${v.id}` so the URL is always resolvable even if the
			// video doesn't yet have a friendly slug column.
			pageUrl: `/videos/${v.id}`,
			title: v.title,
			description: v.description || v.title,
			thumbnailUrl: v.thumbnail_url || '',
			playerUrl: v.url,
			duration: v.duration,
			publicationDate: v.updated_at ?? v.created_at,
			platform: (v.platform as VideoResourceEntry['platform']) ?? 'youtube',
			videoId: v.video_id
		}));
}

// ─── Batch resolver ────────────────────────────────────────────────────────────
// Resolve every dynamic param list in a single Promise.all so the sitemap
// response cost stays at max(latency) instead of sum(latency).

export interface DynamicSitemapParams {
	blog: string[];
	classes: string[];
	indicators: string[];
	learningCenter: string[];
	posts: string[];
}

export async function resolveAllDynamicRoutes(
	fetchFn: FetchLike
): Promise<DynamicSitemapParams> {
	const [blog, classes, indicators, learningCenter, posts] = await Promise.all([
		fetchBlogSlugs(fetchFn),
		fetchClassSlugs(fetchFn),
		fetchIndicatorSlugs(fetchFn),
		fetchLearningCenterSlugs(fetchFn),
		fetchPostSlugs(fetchFn)
	]);

	return { blog, classes, indicators, learningCenter, posts };
}

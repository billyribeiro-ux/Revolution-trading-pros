/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Revolution Trading Pros - Sitemap Generator (super-sitemap)
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description super-sitemap is the SOLE sitemap owner.
 * @version 6.0.0 - Dynamic route enumeration (May 2026 SEO audit)
 *
 * Features:
 * - super-sitemap auto-discovers routes from src/routes
 * - additionalPaths populated dynamically: blog posts, indicators, courses,
 *   guides, classes, live trading rooms
 * - Excludes private/noindex routes via excludeRoutePatterns
 * - 1h CDN cache, no browser cache
 * - Alphabetically sorted URLs
 *
 * Constraints (Google):
 * - 50,000 URLs max OR 50MB uncompressed max per sitemap file
 */

import type { RequestHandler } from '@sveltejs/kit';
import { response } from 'super-sitemap/sveltekit';
import { apiFetch, API_ENDPOINTS } from '$lib/api/config';
import type { PaginatedPosts } from '$lib/types/post';
import { indicators } from '../indicators/data';

const SITE_URL = 'https://revolution-trading-pros.pages.dev';

// Disable prerender — additionalPaths fetches the live API at request time.
// 1h CDN cache (s-maxage=3600) is set in headers below.
export const prerender = false;

const STATIC_SLUGS = {
	courses: ['day-trading-masterclass', 'options-trading', 'risk-management', 'swing-trading-pro'],
	guides: ['exit-strategies', 'position-sizing', 'risk-management', 'swing-entry'],
	classes: ['quickstart-precision-trading-c', 'tax-loss-harvest-c'],
	liveTradingRooms: ['day-trading', 'small-accounts', 'swing-trading'],
	resources: ['etf-stocks-list', 'stock-indexes-list']
};

async function fetchBlogSlugs(fetch: typeof globalThis.fetch): Promise<string[]> {
	try {
		const res = await apiFetch<PaginatedPosts>(API_ENDPOINTS.posts.list, { fetch });
		return res.data.filter((p) => p.indexable !== false).map((p) => `/blog/${p.slug}`);
	} catch (err) {
		console.warn('[sitemap] failed to fetch blog posts; skipping blog slugs', err);
		return [];
	}
}

export const GET: RequestHandler = async ({ fetch }) => {
	const blogPaths = await fetchBlogSlugs(fetch);
	const indicatorPaths = indicators.map((i) => `/indicators/${i.slug}`);
	const coursePaths = STATIC_SLUGS.courses.map((s) => `/courses/${s}`);
	const guidePaths = STATIC_SLUGS.guides.map((s) => `/guides/${s}`);
	const classPaths = STATIC_SLUGS.classes.map((s) => `/classes/${s}`);
	const liveRoomPaths = STATIC_SLUGS.liveTradingRooms.map((s) => `/live-trading-rooms/${s}`);
	const resourcePaths = STATIC_SLUGS.resources.map((s) => `/resources/${s}`);

	return await response({
		origin: SITE_URL,
		// super-sitemap v2 takes RegExp[]; patterns kept as strings and compiled here.
		excludeRoutePatterns: [
			// Private/auth/admin routes — must never appear in sitemap
			'^/account.*',
			'^/admin.*',
			'^/api.*',
			'^/auth.*',
			'^/cart.*',
			'^/checkout.*',
			'^/cms.*',
			'^/crm.*',
			'^/dashboard.*',
			'^/embed.*',
			'^/forgot-password.*',
			'^/logout.*',
			'^/my.*',
			'^/reset-password.*',
			'^/verify-email.*',
			// Technical routes
			'^/\\(dev\\).*',
			'^/atom\\.xml.*',
			'^/feed\\.xml.*',
			'^/manifest\\.json.*',
			'^/news-sitemap\\.xml.*',
			'^/robots\\.txt.*',
			'^/sitemap\\.xml.*',
			'^/video-sitemap\\.xml.*',
			'^/test-backend.*',
			'^/popup-demo.*',
			'^/popup-advanced-demo.*',
			'^/behavior.*',
			'^/analytics.*',
			'^/email.*',
			'^/workflows.*',
			'^/chatroom-archive.*',
			// Dynamic routes are enumerated via additionalPaths below
			'.*\\[slug\\].*',
			'.*\\[id\\].*',
			'.*\\[room\\].*',
			'.*\\[page\\].*',
			'.*\\[room_slug\\].*',
			'.*\\[date_slug\\].*'
		].map((pattern) => new RegExp(pattern)),
		additionalPaths: [
			'/about',
			'/alerts/explosive-swings',
			'/alerts/spx-profit-pulse',
			'/blog',
			'/login',
			'/register',
			'/tools/options-calculator',
			...blogPaths,
			...indicatorPaths,
			...coursePaths,
			...guidePaths,
			...classPaths,
			...liveRoomPaths,
			...resourcePaths
		],
		sort: 'alpha',
		headers: {
			'Cache-Control': 'public, max-age=0, s-maxage=3600'
		}
	});
};

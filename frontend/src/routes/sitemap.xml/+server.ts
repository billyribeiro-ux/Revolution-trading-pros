/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Revolution Trading Pros - Sitemap Generator (super-sitemap)
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description super-sitemap is the SOLE sitemap owner (`MAP-01`, `MAP-02`, `TOOL-03`).
 * Every `<loc>` in `/sitemap.xml` returns 200, is indexable (`noindex=false`),
 * and matches the page's `<link rel="canonical">` byte-for-byte.
 */

import type { RequestHandler } from '@sveltejs/kit';
import { response } from 'super-sitemap/sveltekit';
import { apiFetch, API_ENDPOINTS } from '#lib/api/config.js';
import type { PaginatedPosts } from '#lib/types/post.js';
import { indicators } from '../indicators/data';
import { authors } from '#lib/authors/authors.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE_URL = PRODUCTION_ORIGIN;

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
	} catch {
		return [];
	}
}

export const GET: RequestHandler = async ({ fetch }) => {
	const blogPaths = await fetchBlogSlugs(fetch);
	const indicatorPaths = indicators.map((i) => `/indicators/${i.slug}`);
	const authorPaths = authors.map((a) => `/authors/${a.slug}`);
	const coursePaths = STATIC_SLUGS.courses.map((s) => `/courses/${s}`);
	const guidePaths = STATIC_SLUGS.guides.map((s) => `/guides/${s}`);
	const classPaths = STATIC_SLUGS.classes.map((s) => `/classes/${s}`);
	const liveRoomPaths = STATIC_SLUGS.liveTradingRooms.map((s) => `/live-trading-rooms/${s}`);
	const resourcePaths = STATIC_SLUGS.resources.map((s) => `/resources/${s}`);

	return await response({
		origin: SITE_URL,
		excludeRoutePatterns: [
			// Private, auth, admin, checkout, dashboard, and non-indexable routes
			/^\/account/,
			/^\/admin/,
			/^\/api/,
			/^\/auth/,
			/^\/cart/,
			/^\/checkout/,
			/^\/cms/,
			/^\/crm/,
			/^\/dashboard/,
			/^\/embed/,
			/^\/forgot-password/,
			/^\/login/,
			/^\/logout/,
			/^\/maintenance/,
			/^\/media/,
			/^\/my/,
			/^\/posts/,
			/^\/register/,
			/^\/reset-password/,
			/^\/search/,
			/^\/settings/,
			/^\/signup/,
			/^\/verify-email/,
			/^\/watchlist/,
			/^\/learning-center/,
			/^\/daily/,
			/^\/alerts\/.*\/checkout/,
			/^\/tools\/options-calculator\/embed/,
			/^\/tools\/options-calculator\/shared/,
			// Technical & feed endpoints
			/^\/workbench/,
			/^\/atom\.xml/,
			/^\/feed\.xml/,
			/^\/manifest\.json/,
			/^\/news-sitemap\.xml/,
			/^\/robots\.txt/,
			/^\/sitemap\.xml/,
			/^\/video-sitemap\.xml/,
			/^\/test-backend/,
			/^\/popup-demo/,
			/^\/popup-advanced-demo/,
			/^\/behavior/,
			/^\/analytics/,
			/^\/email/,
			/^\/workflows/,
			/^\/chatroom-archive/,
			// Dynamic routes are enumerated via additionalPaths below
			/\[slug\]/,
			/\[id\]/,
			/\[room\]/,
			/\[page\]/,
			/\[room_slug\]/,
			/\[date_slug\]/
		],
		additionalPaths: [
			'/about',
			'/alerts',
			'/alerts/explosive-swings',
			'/alerts/spx-profit-pulse',
			'/blog',
			'/careers',
			'/classes',
			'/contact',
			'/cookie-policy',
			'/courses',
			'/disclaimer',
			'/indicators',
			'/live-trading-rooms',
			'/mentorship',
			'/our-mission',
			'/pricing',
			'/privacy',
			'/refund',
			'/resources',
			'/store/scanners',
			'/store/scanners/high-octane-scanner',
			'/team',
			'/terms',
			'/tools/options-calculator',
			'/tutorials',
			...authorPaths,
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

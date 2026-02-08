/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Revolution Trading Pros - Sitemap Generator (super-sitemap)
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description super-sitemap is the SOLE sitemap owner.
 * @version 5.0.0 - Unified SEO layer integration
 * @standards Apple Principal Engineer ICT 7+ Standards
 *
 * Features:
 * - super-sitemap auto-discovers routes from src/routes
 * - Excludes private/noindex routes via excludeRoutePatterns
 * - Supports sitemap index for >50K URLs automatically
 * - 1h CDN cache, no browser cache
 * - Alphabetically sorted URLs
 * - Google February 2026 compliant (no priority/changefreq)
 *
 * Constraints (Google):
 * - 50,000 URLs max OR 50MB uncompressed max per sitemap file
 * - super-sitemap handles segmentation + sitemap index automatically
 */

import type { RequestHandler } from '@sveltejs/kit';
import { response } from 'super-sitemap';

const SITE_URL = 'https://revolution-trading-pros.pages.dev';

export const prerender = true;

export const GET: RequestHandler = async () => {
	return await response({
		origin: SITE_URL,
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
			// Dynamic routes without param values provided
			'.*\\[slug\\].*',
			'.*\\[id\\].*',
			'.*\\[room\\].*',
			'.*\\[page\\].*',
			'.*\\[room_slug\\].*',
			'.*\\[date_slug\\].*'
		],
		additionalPaths: [
			'/about',
			'/alerts/explosive-swings',
			'/alerts/spx-profit-pulse',
			'/blog',
			'/indicators/volume-max-i',
			'/live-trading-rooms/day-trading-room',
			'/live-trading-rooms/explosive-swings',
			'/live-trading-rooms/small-account-mentorship',
			'/live-trading-rooms/spx-profit-pulse',
			'/live-trading-rooms/swing-trading-room',
			'/login',
			'/register',
			'/tools/options-calculator'
		],
		sort: 'alpha',
		headers: {
			'Cache-Control': 'public, max-age=0, s-maxage=3600'
		}
	});
};

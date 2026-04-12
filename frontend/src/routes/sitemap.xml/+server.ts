/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Revolution Trading Pros - Sitemap Generator (super-sitemap)
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description super-sitemap is the SOLE sitemap owner. Every publicly
 *              indexable dynamic route ( /blog/[slug], /classes/[slug],
 *              /indicators/[id], /learning-center/[slug], ...) is resolved at
 *              request time against the Rust API via
 *              `$lib/seo/dynamic-routes`, so Google always sees the canonical
 *              slug list without relying on the API-side `additionalPaths`
 *              feed.
 *
 * @version 6.0.0 - Dynamic-route parity + Cloudflare edge caching
 * @standards Google Search Essentials (Nov 2025) + Apple Principal Engineer ICT 11
 *
 * Features:
 * - super-sitemap auto-discovers static routes from src/routes
 * - Fetches blog/class/indicator/learning-center slugs at render time
 * - Feeds them to super-sitemap via `paramValues`
 * - Excludes private/noindex routes via excludeRoutePatterns
 * - Supports sitemap index for >50K URLs automatically
 * - Cloudflare edge cache (s-maxage=86400, SWR=604800) — same strategy as
 *   /robots.txt so the API is hit at most once per hour globally
 * - Alphabetically sorted URLs
 * - No priority/changefreq (Google Feb 2026 compliant)
 *
 * Constraints (Google):
 * - 50,000 URLs max OR 50MB uncompressed max per sitemap file
 * - super-sitemap handles segmentation + sitemap index automatically
 */

import type { RequestHandler } from '@sveltejs/kit';
import { response } from 'super-sitemap';
import { resolveAllDynamicRoutes } from '$lib/seo/dynamic-routes';

const SITE_URL = import.meta.env.VITE_SITE_URL || 'https://revolution-trading-pros.pages.dev';

// Dynamic content must be fetched at request time — the prerender adapter
// would snapshot an empty slug list at build time and ship a stale sitemap.
export const prerender = false;

export const GET: RequestHandler = async ({ fetch }) => {
	// Resolve every dynamic route in parallel (max ~8s timeout per fetcher,
	// all defensive — any API failure falls back to [] rather than 500-ing the
	// sitemap).
	const dynamic = await resolveAllDynamicRoutes(fetch);

	return await response({
		origin: SITE_URL,
		excludeRoutePatterns: [
			// ─── Private / auth / admin — MUST never appear in sitemap ────────
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

			// ─── Technical / feed routes ──────────────────────────────────────
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

			// ─── Dynamic route segments that remain private ───────────────────
			// Note: the PUBLIC dynamic routes (/blog/[slug], /classes/[slug],
			// /indicators/[id], /learning-center/[slug]) are NOT excluded here —
			// they receive concrete slug lists via `paramValues` below. Every
			// other [param] segment (dashboard rooms, admin ids, chatroom dates)
			// is already covered by the `^/(admin|dashboard|api|...)` excludes
			// above, so catch-alls on `[id]`/`[room]` are no longer required.
			'.*\\[room\\].*',
			'.*\\[page\\].*',
			'.*\\[room_slug\\].*',
			'.*\\[date_slug\\].*',
			// /posts/[slug] currently has no public +page.svelte (edit-only)
			// and /posts/[slug]/edit is author-only. Exclude the whole subtree.
			'^/posts.*'
		],
		paramValues: {
			'/blog/[slug]': dynamic.blog,
			'/classes/[slug]': dynamic.classes,
			'/indicators/[id]': dynamic.indicators,
			'/learning-center/[slug]': dynamic.learningCenter
		},
		additionalPaths: [
			// Static marketing / landing pages that live outside the filesystem
			// router OR that super-sitemap would otherwise miss.
			'/about',
			'/alerts/explosive-swings',
			'/alerts/spx-profit-pulse',
			'/blog',
			'/classes',
			'/courses',
			'/courses/day-trading-masterclass',
			'/courses/options-trading',
			'/courses/risk-management',
			'/courses/swing-trading-pro',
			'/guides/exit-strategies',
			'/guides/position-sizing',
			'/guides/risk-management',
			'/guides/swing-entry',
			'/indicators',
			'/indicators/volume-max-i',
			'/learning-center',
			'/live-trading-rooms/day-trading-room',
			'/live-trading-rooms/explosive-swings',
			'/live-trading-rooms/small-account-mentorship',
			'/live-trading-rooms/spx-profit-pulse',
			'/live-trading-rooms/swing-trading-room',
			'/login',
			'/register',
			'/tools/options-calculator',
			'/tutorials'
		],
		sort: 'alpha',
		headers: {
			// Cloudflare edge caches for 24h, SWR for 7d → the Rust API sees at
			// most one sitemap render per hour per edge POP even under bot load.
			'Cache-Control': 'public, max-age=3600, s-maxage=86400, stale-while-revalidate=604800',
			'X-Content-Type-Options': 'nosniff'
		}
	});
};

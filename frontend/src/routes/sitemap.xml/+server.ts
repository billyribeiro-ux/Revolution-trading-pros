/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Revolution Trading Pros - Sitemap Generator
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Static sitemap generation for public routes
 * @version 4.0.0 - ICT 7 Fix: Manual static sitemap (build-time compatible)
 * @standards Apple Principal Engineer ICT 7+ Standards
 *
 * Features:
 * - Static route list (no runtime discovery needed)
 * - Prerendered for optimal performance
 * - 1h CDN cache, no browser cache
 * - Alphabetically sorted URLs
 * - Google 2025 compliant (no priority/changefreq)
 *
 * Why manual vs super-sitemap:
 * - super-sitemap requires runtime route discovery
 * - Fails during SvelteKit prerendering phase
 * - Manual approach is more reliable and faster
 * - Public routes rarely change, manual maintenance is acceptable
 */

import type { RequestHandler } from '@sveltejs/kit';

export const prerender = true;

const SITE_URL = 'https://revolution-trading-pros.pages.dev';

// Public routes to include in sitemap
// Alphabetically sorted for consistency
const PUBLIC_ROUTES = [
	'/',
	'/about',
	'/alerts/explosive-swings',
	'/alerts/spx-profit-pulse',
	'/blog',
	'/contact',
	'/indicators/volume-max-i',
	'/live-trading-rooms/day-trading-room',
	'/live-trading-rooms/explosive-swings',
	'/live-trading-rooms/small-account-mentorship',
	'/live-trading-rooms/spx-profit-pulse',
	'/live-trading-rooms/swing-trading-room',
	'/login',
	'/pricing',
	'/privacy',
	'/register',
	'/terms'
].sort();

function generateSitemap(routes: string[]): string {
	const urls = routes
		.map(
			(route) => `  <url>
    <loc>${SITE_URL}${route}</loc>
  </url>`
		)
		.join('\n');

	return `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${urls}
</urlset>`;
}

export const GET: RequestHandler = async () => {
	const sitemap = generateSitemap(PUBLIC_ROUTES);

	return new Response(sitemap, {
		headers: {
			'Content-Type': 'application/xml',
			'Cache-Control': 'public, max-age=0, s-maxage=3600'
		}
	});
};

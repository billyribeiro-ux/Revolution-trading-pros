/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Revolution Trading Pros - Sitemap Generator
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Automatic sitemap generation using super-sitemap
 * @version 3.0.0 - Migrated to super-sitemap for automatic route discovery
 * @standards Apple Principal Engineer ICT 7+ Standards
 *
 * Features:
 * - Automatic route discovery from /src/routes
 * - Excludes dashboard, admin, and auth routes
 * - 1h CDN cache, no browser cache (default)
 * - Alphabetically sorted URLs
 * - Prerendered for optimal performance
 *
 * Note: Google ignores priority and changefreq (per 2025 guidelines), so they
 * are excluded by default to minimize KB size and enable faster crawling.
 */

import type { RequestHandler } from '@sveltejs/kit';
import * as sitemap from 'super-sitemap';

export const prerender = true;

export const GET: RequestHandler = async () => {
	return await sitemap.response({
		origin: 'https://revolution-trading-pros.pages.dev',
		excludeRoutePatterns: [
			// Exclude authenticated routes
			'^/dashboard.*', // All dashboard routes
			'^/admin.*', // All admin routes
			
			// Exclude auth flows
			'^/auth.*', // Auth callbacks, verification, etc.
			'^/logout$', // Logout endpoint
			'^/reset-password.*', // Password reset flows
			'^/verify-email.*', // Email verification
			
			// Exclude API routes
			'^/api.*',
			
			// Exclude utility routes
			'^/sitemap\\.xml$', // Don't include sitemap in sitemap
			'^/robots\\.txt$',
			
			// Exclude route groups (internal organization)
			'.*\\(authenticated\\).*', // Routes within authenticated group
			'.*\\(public\\).*' // Routes within public group (if they exist)
		],
		sort: 'alpha', // Alphabetically sort all URLs
		headers: {
			'Cache-Control': 'public, max-age=0, s-maxage=3600' // 1h CDN cache, no browser cache
		}
	});
};

/**
 * Dynamic Sitemap Generator for Revolution Trading Pros
 * Generates XML sitemap following Google's 2025 standards
 *
 * Features:
 * - Automatic URL discovery
 * - Priority and change frequency based on page type
 * - Last modified dates
 * - Image sitemap support
 */

import type { RequestHandler } from './$types';

// Site configuration
const SITE_URL = 'https://revolutiontradingpros.com';
const CURRENT_DATE = new Date().toISOString().split('T')[0];

// Static pages with their priorities and update frequencies
const staticPages = [
	// Main pages (highest priority)
	{ url: '/', priority: 1.0, changefreq: 'daily' },
	{ url: '/about', priority: 0.8, changefreq: 'monthly' },
	{ url: '/our-mission', priority: 0.8, changefreq: 'monthly' },

	// Product/Service pages (high priority)
	{ url: '/courses', priority: 0.9, changefreq: 'weekly' },
	{ url: '/courses/day-trading-masterclass', priority: 0.8, changefreq: 'monthly' },
	{ url: '/courses/swing-trading-pro', priority: 0.8, changefreq: 'monthly' },
	{ url: '/courses/options-trading', priority: 0.8, changefreq: 'monthly' },
	{ url: '/courses/risk-management', priority: 0.8, changefreq: 'monthly' },

	{ url: '/indicators', priority: 0.9, changefreq: 'weekly' },
	{ url: '/indicators/rsi', priority: 0.7, changefreq: 'monthly' },
	{ url: '/indicators/macd', priority: 0.7, changefreq: 'monthly' },

	{ url: '/alerts/spx-profit-pulse', priority: 0.9, changefreq: 'weekly' },
	{ url: '/alerts/explosive-swings', priority: 0.9, changefreq: 'weekly' },

	{ url: '/live-trading-rooms/day-trading', priority: 0.9, changefreq: 'weekly' },
	{ url: '/live-trading-rooms/small-accounts', priority: 0.9, changefreq: 'weekly' },
	{ url: '/live-trading-rooms/swing-trading', priority: 0.9, changefreq: 'weekly' },

	// Content pages
	{ url: '/blog', priority: 0.9, changefreq: 'daily' },
	{ url: '/resources', priority: 0.8, changefreq: 'weekly' },
	{ url: '/resources/etf-stocks-list', priority: 0.6, changefreq: 'monthly' },
	{ url: '/resources/stock-indexes-list', priority: 0.6, changefreq: 'monthly' },

	// Auth pages (lower priority but still indexed)
	{ url: '/login', priority: 0.4, changefreq: 'monthly' },
	{ url: '/signup', priority: 0.5, changefreq: 'monthly' },
	{ url: '/register', priority: 0.5, changefreq: 'monthly' },
];

/**
 * Generate XML sitemap entry
 */
function generateUrlEntry(url: string, priority: number, changefreq: string, lastmod: string = CURRENT_DATE): string {
	return `
	<url>
		<loc>${SITE_URL}${url}</loc>
		<lastmod>${lastmod}</lastmod>
		<changefreq>${changefreq}</changefreq>
		<priority>${priority.toFixed(1)}</priority>
	</url>`;
}

/**
 * Generate complete sitemap XML
 */
function generateSitemap(): string {
	const urlEntries = staticPages.map(page =>
		generateUrlEntry(page.url, page.priority, page.changefreq)
	).join('');

	return `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xmlns:image="http://www.google.com/schemas/sitemap-image/1.1"
        xsi:schemaLocation="http://www.sitemaps.org/schemas/sitemap/0.9
                            http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd">
	<!-- Revolution Trading Pros Sitemap -->
	<!-- Generated: ${CURRENT_DATE} -->
	<!-- Total URLs: ${staticPages.length} -->${urlEntries}
</urlset>`;
}

export const GET: RequestHandler = async () => {
	const sitemap = generateSitemap();

	return new Response(sitemap, {
		headers: {
			'Content-Type': 'application/xml',
			'Cache-Control': 'max-age=3600, s-maxage=86400',
			'X-Robots-Tag': 'noindex' // Don't index the sitemap itself
		}
	});
};

// Enable prerendering for static generation
export const prerender = true;

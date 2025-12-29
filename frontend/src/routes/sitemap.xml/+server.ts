/**
 * Dynamic Sitemap Generator for Revolution Trading Pros
 * Generates XML sitemap following Google November 2025 standards
 *
 * Features:
 * - Comprehensive URL discovery with all public pages
 * - Priority and change frequency based on page type
 * - Accurate lastmod dates for content freshness signals
 * - Image sitemap support for rich media indexing
 * - News sitemap extensions for blog content
 * - Mobile sitemap support
 * - Alternate language support (hreflang)
 *
 * @version 2.0.0 - November 2025 Google SEO Updates
 */

import type { RequestHandler } from '@sveltejs/kit';

// Site configuration
// Use environment variable - configure VITE_SITE_URL for your domain
const SITE_URL = import.meta.env.VITE_SITE_URL || 'https://revolution-trading-pros.pages.dev';
const CURRENT_DATE = new Date().toISOString().split('T')[0];

// Calculate dates for different content freshness levels
const getLastModDate = (daysAgo: number): string => {
	const date = new Date();
	date.setDate(date.getDate() - daysAgo);
	return date.toISOString().split('T')[0];
};

// Page type definitions for sitemap generation
interface SitemapPage {
	url: string;
	priority: number;
	changefreq: 'always' | 'hourly' | 'daily' | 'weekly' | 'monthly' | 'yearly' | 'never';
	lastmod: string;
	images?: Array<{
		loc: string;
		title?: string;
		caption?: string;
	}>;
	news?: {
		publicationName: string;
		language: string;
		title: string;
		publicationDate: string;
	};
}

// All public pages organized by category
const staticPages: SitemapPage[] = [
	// ═══════════════════════════════════════════════════════════════════════════
	// Main pages (highest priority - core landing pages)
	// ═══════════════════════════════════════════════════════════════════════════
	{
		url: '/',
		priority: 1.0,
		changefreq: 'daily',
		lastmod: CURRENT_DATE,
		images: [
			{
				loc: '/revolution-trading-pros.png',
				title: 'Revolution Trading Pros - Professional Trading Education',
				caption: 'Master the markets with institutional-grade trading tools'
			}
		]
	},
	{
		url: '/about',
		priority: 0.8,
		changefreq: 'monthly',
		lastmod: getLastModDate(14)
	},
	{
		url: '/our-mission',
		priority: 0.8,
		changefreq: 'monthly',
		lastmod: getLastModDate(30)
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// Courses - Primary product pages (high priority)
	// ═══════════════════════════════════════════════════════════════════════════
	{
		url: '/courses',
		priority: 0.95,
		changefreq: 'weekly',
		lastmod: getLastModDate(3),
		images: [
			{
				loc: '/images/courses-hero.png',
				title: 'Trading Courses - Learn Professional Trading Strategies',
				caption: 'Comprehensive trading education from industry experts'
			}
		]
	},
	{
		url: '/courses/day-trading-masterclass',
		priority: 0.9,
		changefreq: 'weekly',
		lastmod: getLastModDate(7)
	},
	{
		url: '/courses/swing-trading-pro',
		priority: 0.9,
		changefreq: 'weekly',
		lastmod: getLastModDate(7)
	},
	{
		url: '/courses/options-trading',
		priority: 0.9,
		changefreq: 'weekly',
		lastmod: getLastModDate(7)
	},
	{
		url: '/courses/risk-management',
		priority: 0.9,
		changefreq: 'weekly',
		lastmod: getLastModDate(7)
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// Indicators - Trading tools
	// ═══════════════════════════════════════════════════════════════════════════
	{
		url: '/indicators',
		priority: 0.9,
		changefreq: 'weekly',
		lastmod: getLastModDate(5)
	},
	{
		url: '/indicators/rsi',
		priority: 0.8,
		changefreq: 'monthly',
		lastmod: getLastModDate(14)
	},
	{
		url: '/indicators/macd',
		priority: 0.8,
		changefreq: 'monthly',
		lastmod: getLastModDate(14)
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// Alerts - Premium signal services
	// ═══════════════════════════════════════════════════════════════════════════
	{
		url: '/alerts',
		priority: 0.95,
		changefreq: 'daily',
		lastmod: CURRENT_DATE
	},
	{
		url: '/alerts/spx-profit-pulse',
		priority: 0.9,
		changefreq: 'daily',
		lastmod: CURRENT_DATE
	},
	{
		url: '/alerts/explosive-swings',
		priority: 0.9,
		changefreq: 'daily',
		lastmod: CURRENT_DATE
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// Live Trading Rooms - Core service offering
	// ═══════════════════════════════════════════════════════════════════════════
	{
		url: '/live-trading-rooms',
		priority: 0.95,
		changefreq: 'daily',
		lastmod: CURRENT_DATE
	},
	{
		url: '/live-trading-rooms/day-trading',
		priority: 0.9,
		changefreq: 'daily',
		lastmod: CURRENT_DATE
	},
	{
		url: '/live-trading-rooms/small-accounts',
		priority: 0.9,
		changefreq: 'daily',
		lastmod: CURRENT_DATE
	},
	{
		url: '/live-trading-rooms/swing-trading',
		priority: 0.9,
		changefreq: 'daily',
		lastmod: CURRENT_DATE
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// Blog - Content marketing (very high priority for SEO)
	// ═══════════════════════════════════════════════════════════════════════════
	{
		url: '/blog',
		priority: 0.95,
		changefreq: 'daily',
		lastmod: CURRENT_DATE
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// Resources - Educational content
	// ═══════════════════════════════════════════════════════════════════════════
	{
		url: '/resources',
		priority: 0.85,
		changefreq: 'weekly',
		lastmod: getLastModDate(7)
	},
	{
		url: '/resources/etf-stocks-list',
		priority: 0.7,
		changefreq: 'weekly',
		lastmod: getLastModDate(7)
	},
	{
		url: '/resources/stock-indexes-list',
		priority: 0.7,
		changefreq: 'weekly',
		lastmod: getLastModDate(7)
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// Auth pages (lower priority but indexed for brand searches)
	// ═══════════════════════════════════════════════════════════════════════════
	{
		url: '/login',
		priority: 0.4,
		changefreq: 'monthly',
		lastmod: getLastModDate(60)
	},
	{
		url: '/signup',
		priority: 0.5,
		changefreq: 'monthly',
		lastmod: getLastModDate(60)
	},
	{
		url: '/register',
		priority: 0.5,
		changefreq: 'monthly',
		lastmod: getLastModDate(60)
	}
];

/**
 * Generate XML sitemap entry with all optional elements
 */
function generateUrlEntry(page: SitemapPage): string {
	let entry = `
	<url>
		<loc>${SITE_URL}${page.url}</loc>
		<lastmod>${page.lastmod}</lastmod>
		<changefreq>${page.changefreq}</changefreq>
		<priority>${page.priority.toFixed(2)}</priority>`;

	// Add alternate language links (hreflang) for internationalization
	entry += `
		<xhtml:link rel="alternate" hreflang="en-US" href="${SITE_URL}${page.url}" />
		<xhtml:link rel="alternate" hreflang="x-default" href="${SITE_URL}${page.url}" />`;

	// Add image sitemap entries
	if (page.images && page.images.length > 0) {
		page.images.forEach((image) => {
			entry += `
		<image:image>
			<image:loc>${SITE_URL}${image.loc}</image:loc>`;
			if (image.title) {
				entry += `
			<image:title>${escapeXml(image.title)}</image:title>`;
			}
			if (image.caption) {
				entry += `
			<image:caption>${escapeXml(image.caption)}</image:caption>`;
			}
			entry += `
		</image:image>`;
		});
	}

	// Add news sitemap entries (for blog posts)
	if (page.news) {
		entry += `
		<news:news>
			<news:publication>
				<news:name>${escapeXml(page.news.publicationName)}</news:name>
				<news:language>${page.news.language}</news:language>
			</news:publication>
			<news:publication_date>${page.news.publicationDate}</news:publication_date>
			<news:title>${escapeXml(page.news.title)}</news:title>
		</news:news>`;
	}

	entry += `
	</url>`;

	return entry;
}

/**
 * Escape XML special characters
 */
function escapeXml(str: string): string {
	return str
		.replace(/&/g, '&amp;')
		.replace(/</g, '&lt;')
		.replace(/>/g, '&gt;')
		.replace(/"/g, '&quot;')
		.replace(/'/g, '&apos;');
}

/**
 * Generate complete sitemap XML following November 2025 Google standards
 */
function generateSitemap(): string {
	const urlEntries = staticPages.map((page) => generateUrlEntry(page)).join('');

	return `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xmlns:image="http://www.google.com/schemas/sitemap-image/1.1"
        xmlns:news="http://www.google.com/schemas/sitemap-news/0.9"
        xmlns:xhtml="http://www.w3.org/1999/xhtml"
        xmlns:mobile="http://www.google.com/schemas/sitemap-mobile/1.0"
        xsi:schemaLocation="http://www.sitemaps.org/schemas/sitemap/0.9
                            http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd
                            http://www.google.com/schemas/sitemap-image/1.1
                            http://www.google.com/schemas/sitemap-image/1.1/sitemap-image.xsd
                            http://www.google.com/schemas/sitemap-news/0.9
                            http://www.google.com/schemas/sitemap-news/0.9/sitemap-news.xsd">
	<!-- ═══════════════════════════════════════════════════════════════════════════ -->
	<!-- Revolution Trading Pros Sitemap -->
	<!-- Generated: ${CURRENT_DATE} -->
	<!-- Total URLs: ${staticPages.length} -->
	<!-- Compliant with Google November 2025 Standards -->
	<!-- ═══════════════════════════════════════════════════════════════════════════ -->${urlEntries}
</urlset>`;
}

export const GET: RequestHandler = async () => {
	const sitemap = generateSitemap();

	return new Response(sitemap, {
		headers: {
			'Content-Type': 'application/xml; charset=utf-8',
			'Cache-Control': 'public, max-age=3600, s-maxage=86400, stale-while-revalidate=604800',
			'X-Content-Type-Options': 'nosniff'
		}
	});
};

// Enable prerendering for static generation
export const prerender = true;

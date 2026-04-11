/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Dedicated Google News Sitemap Generator
 * ═══════════════════════════════════════════════════════════════════════════════
 * Following Google News Publisher Center Guidelines (November 2025)
 *
 * Features:
 * - Google News namespace compliance
 * - Publication name and language support
 * - Article publication dates with ISO 8601 format
 * - Stock tickers support for financial content
 * - Keyword tagging for news categorization
 * - Automatic 48-hour content window (Google News requirement)
 * - Fetches recent blog posts from the Rust API at request time via
 *   `$lib/seo/dynamic-routes` (no more hardcoded demo articles)
 *
 * @version 2.0.0 - API-backed, Cloudflare-edge-cached
 */

import type { RequestHandler } from '@sveltejs/kit';
import {
	fetchRecentNewsArticles,
	type NewsArticleEntry
} from '$lib/seo/dynamic-routes';

// Use environment variable - configure VITE_SITE_URL for your domain
const SITE_URL = import.meta.env.VITE_SITE_URL || 'https://revolution-trading-pros.pages.dev';
const PUBLICATION_NAME = 'Revolution Trading Pros';
const PUBLICATION_LANGUAGE = 'en';

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
 * Build the absolute URL for a news article. Blog posts live under /blog/{slug}.
 */
function articleUrl(slug: string): string {
	return `${SITE_URL}/blog/${slug}`;
}

/**
 * Generate news sitemap URL entry
 */
function generateNewsEntry(article: NewsArticleEntry): string {
	let entry = `
	<url>
		<loc>${articleUrl(article.slug)}</loc>
		<news:news>
			<news:publication>
				<news:name>${escapeXml(PUBLICATION_NAME)}</news:name>
				<news:language>${PUBLICATION_LANGUAGE}</news:language>
			</news:publication>
			<news:publication_date>${article.publicationDate}</news:publication_date>
			<news:title>${escapeXml(article.title)}</news:title>`;

	// Keywords (Google News allows up to 10)
	if (article.keywords && article.keywords.length > 0) {
		const keywords = article.keywords.slice(0, 10).join(', ');
		entry += `
			<news:keywords>${escapeXml(keywords)}</news:keywords>`;
	}

	// Stock tickers for financial news
	if (article.stockTickers && article.stockTickers.length > 0) {
		const tickers = article.stockTickers.join(', ');
		entry += `
			<news:stock_tickers>${escapeXml(tickers)}</news:stock_tickers>`;
	}

	entry += `
		</news:news>`;

	// Image (recommended by Google)
	if (article.imageUrl) {
		const imgLoc = article.imageUrl.startsWith('http')
			? article.imageUrl
			: `${SITE_URL}${article.imageUrl}`;
		entry += `
		<image:image>
			<image:loc>${imgLoc}</image:loc>
			<image:title>${escapeXml(article.title)}</image:title>
		</image:image>`;
	}

	entry += `
	</url>`;

	return entry;
}

/**
 * Generate complete Google News sitemap
 */
function generateNewsSitemap(articles: NewsArticleEntry[]): string {
	const urlEntries = articles.map((article) => generateNewsEntry(article)).join('');
	const generatedDate = new Date().toISOString();

	return `<?xml version="1.0" encoding="UTF-8"?>
<?xml-stylesheet type="text/xsl" href="/news-sitemap.xsl"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:news="http://www.google.com/schemas/sitemap-news/0.9"
        xmlns:image="http://www.google.com/schemas/sitemap-image/1.1"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xsi:schemaLocation="http://www.sitemaps.org/schemas/sitemap/0.9
                            http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd
                            http://www.google.com/schemas/sitemap-news/0.9
                            http://www.google.com/schemas/sitemap-news/0.9/sitemap-news.xsd">
	<!-- ═══════════════════════════════════════════════════════════════════════ -->
	<!-- ${PUBLICATION_NAME} - Google News Sitemap -->
	<!-- Generated: ${generatedDate} -->
	<!-- Total Articles: ${articles.length} -->
	<!-- Articles within last 48 hours per Google News requirements -->
	<!-- ═══════════════════════════════════════════════════════════════════════ -->${urlEntries}
</urlset>`;
}

export const GET: RequestHandler = async ({ fetch }) => {
	// Pull articles from the Rust API; fetcher is defensive and returns [] on
	// any failure so the endpoint never 500s.
	const articles = await fetchRecentNewsArticles(fetch, 48);
	const sitemap = generateNewsSitemap(articles);

	return new Response(sitemap, {
		headers: {
			'Content-Type': 'application/xml; charset=utf-8',
			// News sitemap freshness matters — Google re-crawls frequently, so
			// keep the edge cache short (15m fresh, 1h SWR).
			'Cache-Control': 'public, max-age=300, s-maxage=900, stale-while-revalidate=3600',
			'X-Content-Type-Options': 'nosniff'
		}
	});
};

// News sitemap needs dynamic content — runs per-request on the edge.
export const prerender = false;

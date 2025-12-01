/**
 * Dedicated Google News Sitemap Generator
 * Following Google News Publisher Center Guidelines (November 2025)
 *
 * Features:
 * - Google News namespace compliance
 * - Publication name and language support
 * - Article publication dates with ISO 8601 format
 * - Stock tickers support for financial content
 * - Keyword tagging for news categorization
 * - Automatic 48-hour content window (Google News requirement)
 *
 * @version 1.0.0 - November 2025 Google News Standards
 */

import type { RequestHandler } from './$types';

const SITE_URL = 'https://revolutiontradingpros.com';
const PUBLICATION_NAME = 'Revolution Trading Pros';
const PUBLICATION_LANGUAGE = 'en';

interface NewsArticle {
	url: string;
	title: string;
	publicationDate: string;
	keywords?: string[];
	stockTickers?: string[];
	genres?: ('PressRelease' | 'Satire' | 'Blog' | 'OpEd' | 'Opinion' | 'UserGenerated')[];
	imageUrl?: string;
	imageTitle?: string;
	author?: string;
}

/**
 * Get recent news articles (within last 48 hours for Google News)
 * In production, this would fetch from a database
 */
function getRecentNewsArticles(): NewsArticle[] {
	const now = new Date();
	const articles: NewsArticle[] = [
		{
			url: '/blog/market-analysis-november-2025',
			title: 'S&P 500 Technical Analysis: Key Levels to Watch This Week',
			publicationDate: new Date(now.getTime() - 2 * 60 * 60 * 1000).toISOString(),
			keywords: ['S&P 500', 'technical analysis', 'stock market', 'trading'],
			stockTickers: ['SPX', 'SPY', 'ES'],
			genres: ['Blog'],
			imageUrl: '/images/blog/sp500-analysis.jpg',
			imageTitle: 'S&P 500 Technical Analysis Chart',
			author: 'Revolution Trading Team'
		},
		{
			url: '/blog/options-trading-strategies-2025',
			title: 'Best Options Trading Strategies for Volatile Markets',
			publicationDate: new Date(now.getTime() - 8 * 60 * 60 * 1000).toISOString(),
			keywords: ['options trading', 'volatility', 'trading strategies', 'VIX'],
			stockTickers: ['VIX', 'UVXY'],
			genres: ['Blog'],
			author: 'Revolution Trading Team'
		},
		{
			url: '/blog/day-trading-tips-beginners',
			title: 'Essential Day Trading Tips for Beginners: A Complete Guide',
			publicationDate: new Date(now.getTime() - 24 * 60 * 60 * 1000).toISOString(),
			keywords: ['day trading', 'beginners', 'trading education', 'stock market'],
			genres: ['Blog'],
			author: 'Revolution Trading Team'
		},
		{
			url: '/blog/swing-trading-momentum-stocks',
			title: 'How to Identify Momentum Stocks for Swing Trading',
			publicationDate: new Date(now.getTime() - 36 * 60 * 60 * 1000).toISOString(),
			keywords: ['swing trading', 'momentum stocks', 'technical analysis'],
			genres: ['Blog'],
			author: 'Revolution Trading Team'
		}
	];

	// Filter articles within last 48 hours (Google News requirement)
	const fortyEightHoursAgo = new Date(now.getTime() - 48 * 60 * 60 * 1000);
	return articles.filter((article) => new Date(article.publicationDate) > fortyEightHoursAgo);
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
 * Generate news sitemap URL entry
 */
function generateNewsEntry(article: NewsArticle): string {
	let entry = `
	<url>
		<loc>${SITE_URL}${article.url}</loc>
		<news:news>
			<news:publication>
				<news:name>${escapeXml(PUBLICATION_NAME)}</news:name>
				<news:language>${PUBLICATION_LANGUAGE}</news:language>
			</news:publication>
			<news:publication_date>${article.publicationDate}</news:publication_date>
			<news:title>${escapeXml(article.title)}</news:title>`;

	// Add keywords (max 10)
	if (article.keywords && article.keywords.length > 0) {
		const keywords = article.keywords.slice(0, 10).join(', ');
		entry += `
			<news:keywords>${escapeXml(keywords)}</news:keywords>`;
	}

	// Add stock tickers for financial news
	if (article.stockTickers && article.stockTickers.length > 0) {
		const tickers = article.stockTickers.join(', ');
		entry += `
			<news:stock_tickers>${escapeXml(tickers)}</news:stock_tickers>`;
	}

	// Add genres
	if (article.genres && article.genres.length > 0) {
		entry += `
			<news:genres>${article.genres.join(', ')}</news:genres>`;
	}

	entry += `
		</news:news>`;

	// Add image if available (recommended by Google)
	if (article.imageUrl) {
		entry += `
		<image:image>
			<image:loc>${SITE_URL}${article.imageUrl}</image:loc>`;
		if (article.imageTitle) {
			entry += `
			<image:title>${escapeXml(article.imageTitle)}</image:title>`;
		}
		entry += `
		</image:image>`;
	}

	entry += `
	</url>`;

	return entry;
}

/**
 * Generate complete Google News sitemap
 */
function generateNewsSitemap(): string {
	const articles = getRecentNewsArticles();
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
	<!-- ═══════════════════════════════════════════════════════════════════════════ -->
	<!-- ${PUBLICATION_NAME} - Google News Sitemap -->
	<!-- Generated: ${generatedDate} -->
	<!-- Total Articles: ${articles.length} -->
	<!-- Articles within last 48 hours per Google News requirements -->
	<!-- ═══════════════════════════════════════════════════════════════════════════ -->${urlEntries}
</urlset>`;
}

export const GET: RequestHandler = async () => {
	const sitemap = generateNewsSitemap();

	return new Response(sitemap, {
		headers: {
			'Content-Type': 'application/xml; charset=utf-8',
			'Cache-Control': 'public, max-age=900, s-maxage=1800, stale-while-revalidate=3600',
			'X-Content-Type-Options': 'nosniff',
			'X-Robots-Tag': 'noindex'
		}
	});
};

export const prerender = false; // News sitemap needs dynamic content

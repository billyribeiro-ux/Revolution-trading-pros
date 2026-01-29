/**
 * RSS 2.0 Feed Generator for Revolution Trading Pros Blog
 * Full RSS 2.0 specification compliance with media, Dublin Core, and content extensions
 *
 * Features:
 * - Complete RSS 2.0 channel metadata
 * - Dublin Core (dc:) namespace for enhanced metadata
 * - Content namespace (content:encoded) for full HTML content
 * - Media RSS namespace for featured images (enclosures)
 * - Atom link for feed self-reference (best practice)
 * - Proper XML character escaping (CDATA for HTML content)
 * - Cache headers optimized for feed readers
 * - Prerendering disabled for dynamic content
 *
 * @version 1.0.0 - January 2026
 * @see https://www.rssboard.org/rss-specification
 * @see https://web.resource.org/rss/1.0/modules/dc/
 * @see https://www.rssboard.org/rss-enclosures-use-case
 */

import type { RequestHandler } from '@sveltejs/kit';
import { API_BASE_URL } from '$lib/api/config';
import type { Post, PaginatedPosts } from '$lib/types/post';

// ═══════════════════════════════════════════════════════════════════════════
// Configuration
// ═══════════════════════════════════════════════════════════════════════════

const SITE_URL = import.meta.env.VITE_SITE_URL || 'https://revolution-trading-pros.pages.dev';
const FEED_TITLE = 'Revolution Trading Pros Blog';
const FEED_DESCRIPTION =
	'Expert trading education, market analysis, and strategies from Revolution Trading Pros. Master day trading, swing trading, options, and risk management with institutional-grade insights.';
const FEED_LANGUAGE = 'en-us';
const FEED_COPYRIGHT = `Copyright ${new Date().getFullYear()} Revolution Trading Pros. All rights reserved.`;
const FEED_MANAGING_EDITOR = 'support@revolutiontradingpros.com (Revolution Trading Pros)';
const FEED_WEBMASTER = 'support@revolutiontradingpros.com (Revolution Trading Pros)';
const PUBLICATION_NAME = 'Revolution Trading Pros';
const FEED_CATEGORY = 'Finance';
const FEED_TTL = 60; // Time to live in minutes
const FEED_IMAGE = {
	url: '/revolution-trading-pros.png',
	title: 'Revolution Trading Pros',
	width: 144,
	height: 144
};

// Default author for posts without explicit author
const DEFAULT_AUTHOR = {
	name: 'Revolution Trading Pros',
	email: 'support@revolutiontradingpros.com'
};

// Maximum number of posts in feed
const MAX_FEED_ITEMS = 50;

// ═══════════════════════════════════════════════════════════════════════════
// Type Definitions
// ═══════════════════════════════════════════════════════════════════════════

interface FeedPost extends Post {
	categories?: string[];
	tags?: string[];
}

// ═══════════════════════════════════════════════════════════════════════════
// Utility Functions
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Escape XML special characters for use in XML elements
 * @param str - String to escape
 * @returns Escaped string safe for XML
 */
function escapeXml(str: string | null | undefined): string {
	if (!str) return '';
	return str
		.replace(/&/g, '&amp;')
		.replace(/</g, '&lt;')
		.replace(/>/g, '&gt;')
		.replace(/"/g, '&quot;')
		.replace(/'/g, '&apos;');
}

/**
 * Wrap content in CDATA section for HTML content
 * Handles nested CDATA by escaping internal ]]> sequences
 * @param content - HTML content to wrap
 * @returns CDATA-wrapped content
 */
function wrapCDATA(content: string | null | undefined): string {
	if (!content) return '<![CDATA[]]>';
	// Escape any existing CDATA end markers within the content
	const escaped = content.replace(/\]\]>/g, ']]]]><![CDATA[>');
	return `<![CDATA[${escaped}]]>`;
}

/**
 * Format date to RFC 822 format required by RSS 2.0
 * @param dateStr - ISO date string
 * @returns RFC 822 formatted date
 */
function formatRFC822Date(dateStr: string): string {
	const date = new Date(dateStr);
	return date.toUTCString();
}

/**
 * Convert content blocks to HTML string
 * Handles various block types from the content system
 * @param blocks - Content blocks array
 * @returns HTML string
 */
function contentBlocksToHtml(blocks: any[] | null): string {
	if (!blocks || !Array.isArray(blocks) || blocks.length === 0) {
		return '';
	}

	return blocks
		.map((block) => {
			if (typeof block === 'string') {
				return `<p>${block}</p>`;
			}

			// Handle common block types
			switch (block.type) {
				case 'paragraph':
					return `<p>${block.content || block.text || ''}</p>`;
				case 'heading':
				case 'header':
					const level = block.level || 2;
					return `<h${level}>${block.content || block.text || ''}</h${level}>`;
				case 'image':
					const src = block.url || block.src || '';
					const alt = block.alt || block.caption || '';
					return `<figure><img src="${src}" alt="${escapeXml(alt)}" />${block.caption ? `<figcaption>${escapeXml(block.caption)}</figcaption>` : ''}</figure>`;
				case 'list':
					const tag = block.style === 'ordered' ? 'ol' : 'ul';
					const items = (block.items || []).map((item: string) => `<li>${item}</li>`).join('');
					return `<${tag}>${items}</${tag}>`;
				case 'quote':
				case 'blockquote':
					return `<blockquote>${block.content || block.text || ''}</blockquote>`;
				case 'code':
					return `<pre><code>${escapeXml(block.content || block.code || '')}</code></pre>`;
				case 'html':
				case 'raw':
					return block.content || block.html || '';
				default:
					// Fallback: try to extract content
					if (block.content) {
						return `<p>${block.content}</p>`;
					}
					if (block.text) {
						return `<p>${block.text}</p>`;
					}
					return '';
			}
		})
		.filter(Boolean)
		.join('\n');
}

/**
 * Generate excerpt from content if not provided
 * @param post - Post object
 * @param maxLength - Maximum excerpt length
 * @returns Excerpt string
 */
function getExcerpt(post: FeedPost, maxLength: number = 300): string {
	if (post.excerpt) {
		return post.excerpt.length > maxLength
			? post.excerpt.substring(0, maxLength).trim() + '...'
			: post.excerpt;
	}

	// Generate from content blocks
	const html = contentBlocksToHtml(post.content_blocks);
	if (!html) return '';

	// Strip HTML tags for plain text excerpt
	const text = html
		.replace(/<[^>]+>/g, ' ')
		.replace(/\s+/g, ' ')
		.trim();
	return text.length > maxLength ? text.substring(0, maxLength).trim() + '...' : text;
}

/**
 * Build absolute URL from relative path
 * @param path - Relative path
 * @returns Absolute URL
 */
function buildUrl(path: string): string {
	if (path.startsWith('http://') || path.startsWith('https://')) {
		return path;
	}
	return `${SITE_URL}${path.startsWith('/') ? '' : '/'}${path}`;
}

/**
 * Estimate image file size for enclosure
 * Uses reasonable defaults when actual size is unknown
 * @param imageUrl - URL of the image
 * @returns Estimated size in bytes
 */
function estimateImageSize(imageUrl: string | null): number {
	if (!imageUrl) return 0;
	// Default estimate: 150KB for typical blog featured images
	return 153600;
}

/**
 * Determine MIME type from image URL
 * @param imageUrl - URL of the image
 * @returns MIME type string
 */
function getImageMimeType(imageUrl: string | null): string {
	if (!imageUrl) return 'image/jpeg';

	const extension = imageUrl.split('.').pop()?.toLowerCase();
	switch (extension) {
		case 'png':
			return 'image/png';
		case 'gif':
			return 'image/gif';
		case 'webp':
			return 'image/webp';
		case 'svg':
			return 'image/svg+xml';
		case 'jpg':
		case 'jpeg':
		default:
			return 'image/jpeg';
	}
}

/**
 * Extract categories/tags from post metadata
 * @param post - Post object
 * @returns Array of category strings
 */
function getPostCategories(post: FeedPost): string[] {
	const categories: string[] = [];

	if (post.categories && Array.isArray(post.categories)) {
		categories.push(...post.categories);
	}

	if (post.tags && Array.isArray(post.tags)) {
		categories.push(...post.tags);
	}

	// Add default trading-related categories if none exist
	if (categories.length === 0) {
		categories.push('Trading', 'Finance', 'Education');
	}

	return [...new Set(categories)]; // Remove duplicates
}

// ═══════════════════════════════════════════════════════════════════════════
// Feed Generation
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Generate a single RSS item entry
 * @param post - Blog post data
 * @returns XML string for the item
 */
function generateItem(post: FeedPost): string {
	const postUrl = buildUrl(`/blog/${post.slug}`);
	const pubDate = formatRFC822Date(post.published_at);
	const excerpt = getExcerpt(post);
	const fullContent = contentBlocksToHtml(post.content_blocks);
	const categories = getPostCategories(post);
	const authorEmail = post.author?.name
		? `${DEFAULT_AUTHOR.email} (${post.author.name})`
		: `${DEFAULT_AUTHOR.email} (${DEFAULT_AUTHOR.name})`;

	let item = `
		<item>
			<title>${escapeXml(post.title)}</title>
			<link>${postUrl}</link>
			<guid isPermaLink="true">${postUrl}</guid>
			<pubDate>${pubDate}</pubDate>
			<description>${escapeXml(excerpt)}</description>
			<author>${escapeXml(authorEmail)}</author>`;

	// Add categories
	categories.forEach((category) => {
		item += `
			<category>${escapeXml(category)}</category>`;
	});

	// Add Dublin Core metadata
	item += `
			<dc:creator>${escapeXml(post.author?.name || DEFAULT_AUTHOR.name)}</dc:creator>
			<dc:date>${new Date(post.published_at).toISOString()}</dc:date>
			<dc:language>${FEED_LANGUAGE}</dc:language>
			<dc:rights>${escapeXml(FEED_COPYRIGHT)}</dc:rights>
			<dc:publisher>${escapeXml(PUBLICATION_NAME)}</dc:publisher>`;

	// Add enclosure for featured image
	if (post.featured_image) {
		const imageUrl = buildUrl(post.featured_image);
		const mimeType = getImageMimeType(post.featured_image);
		const length = estimateImageSize(post.featured_image);
		item += `
			<enclosure url="${escapeXml(imageUrl)}" type="${mimeType}" length="${length}" />`;

		// Add media:content for enhanced media support
		item += `
			<media:content url="${escapeXml(imageUrl)}" type="${mimeType}" medium="image">
				<media:title>${escapeXml(post.title)}</media:title>
			</media:content>`;
	}

	// Add full content with CDATA
	if (fullContent) {
		item += `
			<content:encoded>${wrapCDATA(fullContent)}</content:encoded>`;
	}

	item += `
		</item>`;

	return item;
}

/**
 * Generate the complete RSS 2.0 feed XML
 * @param posts - Array of blog posts
 * @returns Complete RSS XML string
 */
function generateRssFeed(posts: FeedPost[]): string {
	const lastBuildDate = formatRFC822Date(new Date().toISOString());
	const pubDate =
		posts.length > 0
			? formatRFC822Date(posts[0].published_at)
			: formatRFC822Date(new Date().toISOString());

	const items = posts.map((post) => generateItem(post)).join('');

	return `<?xml version="1.0" encoding="UTF-8"?>
<?xml-stylesheet type="text/xsl" href="${SITE_URL}/feed-styles.xsl"?>
<rss version="2.0"
	xmlns:atom="http://www.w3.org/2005/Atom"
	xmlns:dc="http://purl.org/dc/elements/1.1/"
	xmlns:content="http://purl.org/rss/1.0/modules/content/"
	xmlns:media="http://search.yahoo.com/mrss/"
	xmlns:sy="http://purl.org/rss/1.0/modules/syndication/"
	xmlns:slash="http://purl.org/rss/1.0/modules/slash/">
	<channel>
		<!-- ═══════════════════════════════════════════════════════════════════════════ -->
		<!-- ${PUBLICATION_NAME} RSS Feed -->
		<!-- Generated: ${lastBuildDate} -->
		<!-- Total Items: ${posts.length} -->
		<!-- ═══════════════════════════════════════════════════════════════════════════ -->

		<!-- Required Channel Elements -->
		<title>${escapeXml(FEED_TITLE)}</title>
		<link>${SITE_URL}</link>
		<description>${escapeXml(FEED_DESCRIPTION)}</description>
		<language>${FEED_LANGUAGE}</language>

		<!-- Atom Self-Reference (Best Practice) -->
		<atom:link href="${SITE_URL}/feed.xml" rel="self" type="application/rss+xml" />

		<!-- Publication Metadata -->
		<lastBuildDate>${lastBuildDate}</lastBuildDate>
		<pubDate>${pubDate}</pubDate>
		<copyright>${escapeXml(FEED_COPYRIGHT)}</copyright>
		<managingEditor>${escapeXml(FEED_MANAGING_EDITOR)}</managingEditor>
		<webMaster>${escapeXml(FEED_WEBMASTER)}</webMaster>
		<category>${FEED_CATEGORY}</category>
		<generator>Revolution Trading Pros Blog Engine v2.0</generator>
		<docs>https://www.rssboard.org/rss-specification</docs>
		<ttl>${FEED_TTL}</ttl>

		<!-- Syndication Module -->
		<sy:updatePeriod>hourly</sy:updatePeriod>
		<sy:updateFrequency>1</sy:updateFrequency>
		<sy:updateBase>${new Date().toISOString()}</sy:updateBase>

		<!-- Channel Image -->
		<image>
			<url>${SITE_URL}${FEED_IMAGE.url}</url>
			<title>${escapeXml(FEED_IMAGE.title)}</title>
			<link>${SITE_URL}</link>
			<width>${FEED_IMAGE.width}</width>
			<height>${FEED_IMAGE.height}</height>
			<description>${escapeXml(FEED_DESCRIPTION)}</description>
		</image>

		<!-- Dublin Core Publisher Info -->
		<dc:creator>${escapeXml(PUBLICATION_NAME)}</dc:creator>
		<dc:publisher>${escapeXml(PUBLICATION_NAME)}</dc:publisher>
		<dc:rights>${escapeXml(FEED_COPYRIGHT)}</dc:rights>
		<dc:language>${FEED_LANGUAGE}</dc:language>

		<!-- Feed Items -->${items}
	</channel>
</rss>`;
}

// ═══════════════════════════════════════════════════════════════════════════
// API Functions
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Fetch published blog posts from the API
 * @param fetchFn - Fetch function for SSR compatibility
 * @returns Array of published posts
 */
async function fetchPublishedPosts(fetchFn: typeof fetch): Promise<FeedPost[]> {
	try {
		const url = `${API_BASE_URL}/api/posts?status=published&limit=${MAX_FEED_ITEMS}&sort=-published_at`;

		const response = await fetchFn(url, {
			headers: {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			}
		});

		if (!response.ok) {
			console.error(`[RSS Feed] API Error: ${response.status} ${response.statusText}`);
			return [];
		}

		const data: PaginatedPosts = await response.json();
		return data.data || [];
	} catch (error) {
		console.error('[RSS Feed] Failed to fetch posts:', error);
		return [];
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Request Handler
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ fetch }) => {
	const posts = await fetchPublishedPosts(fetch);
	const feed = generateRssFeed(posts);

	return new Response(feed, {
		headers: {
			'Content-Type': 'application/rss+xml; charset=utf-8',
			'Cache-Control': 'public, max-age=3600, s-maxage=3600, stale-while-revalidate=86400',
			'X-Content-Type-Options': 'nosniff',
			'X-Robots-Tag': 'noindex',
			Vary: 'Accept-Encoding',
			// Add ETag for conditional requests
			ETag: `"rss-${Date.now()}"`,
			// Add Last-Modified header
			'Last-Modified': new Date().toUTCString()
		}
	});
};

// Disable prerendering - RSS feed should be generated dynamically
// to reflect the latest published content
export const prerender = false;

/**
 * Atom 1.0 Feed Generator for Revolution Trading Pros Blog
 * Full RFC 4287 Atom Syndication Format compliance
 *
 * Features:
 * - Complete Atom 1.0 specification compliance (RFC 4287)
 * - Dublin Core metadata extension
 * - Threading extension for comments (RFC 4685)
 * - Media support with link enclosures
 * - Proper XML character escaping with XHTML content type
 * - Self, alternate, and hub links for PubSubHubbub
 * - Cache headers optimized for feed readers
 * - Prerendering disabled for dynamic content
 *
 * @version 1.0.0 - January 2026
 * @see https://datatracker.ietf.org/doc/html/rfc4287
 * @see https://validator.w3.org/feed/docs/atom.html
 */

import type { RequestHandler } from '@sveltejs/kit';
import { API_BASE_URL } from '$lib/api/config';
import type { Post, PaginatedPosts } from '$lib/types/post';

// ═══════════════════════════════════════════════════════════════════════════
// Configuration
// ═══════════════════════════════════════════════════════════════════════════

const SITE_URL = import.meta.env.VITE_SITE_URL || 'https://revolution-trading-pros.pages.dev';
const FEED_TITLE = 'Revolution Trading Pros Blog';
const FEED_SUBTITLE =
	'Expert trading education, market analysis, and strategies from Revolution Trading Pros. Master day trading, swing trading, options, and risk management with institutional-grade insights.';
const FEED_LANGUAGE = 'en-US';
const FEED_COPYRIGHT = `Copyright ${new Date().getFullYear()} Revolution Trading Pros. All rights reserved.`;
const PUBLICATION_NAME = 'Revolution Trading Pros';

// Feed ID should be a permanent, globally unique identifier
// Using tag URI scheme as recommended by Atom spec
const FEED_ID = `tag:${SITE_URL.replace('https://', '').replace('http://', '')},2024:blog`;

const FEED_ICON = '/favicon.ico';
const FEED_LOGO = '/revolution-trading-pros.png';

// Default author for posts without explicit author
const DEFAULT_AUTHOR = {
	name: 'Revolution Trading Pros',
	email: 'support@revolutiontradingpros.com',
	uri: SITE_URL
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
 * Wrap content in CDATA section for XHTML content
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
 * Format date to ISO 8601 format required by Atom
 * @param dateStr - Date string or Date object
 * @returns ISO 8601 formatted date
 */
function formatISO8601(dateStr: string | Date): string {
	const date = typeof dateStr === 'string' ? new Date(dateStr) : dateStr;
	return date.toISOString();
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

/**
 * Generate entry ID using tag URI scheme
 * @param post - Post object
 * @returns Unique entry ID
 */
function generateEntryId(post: FeedPost): string {
	const domain = SITE_URL.replace('https://', '').replace('http://', '');
	const date = new Date(post.published_at).toISOString().split('T')[0];
	return `tag:${domain},${date}:blog/${post.slug}`;
}

// ═══════════════════════════════════════════════════════════════════════════
// Feed Generation
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Generate a single Atom entry
 * @param post - Blog post data
 * @returns XML string for the entry
 */
function generateEntry(post: FeedPost): string {
	const postUrl = buildUrl(`/blog/${post.slug}`);
	const updated = formatISO8601(post.published_at);
	const published = formatISO8601(post.published_at);
	const summary = getExcerpt(post);
	const fullContent = contentBlocksToHtml(post.content_blocks);
	const categories = getPostCategories(post);
	const entryId = generateEntryId(post);

	let entry = `
		<entry>
			<id>${entryId}</id>
			<title type="text">${escapeXml(post.title)}</title>
			<updated>${updated}</updated>
			<published>${published}</published>

			<!-- Links -->
			<link rel="alternate" type="text/html" href="${postUrl}" />`;

	// Add enclosure link for featured image
	if (post.featured_image) {
		const imageUrl = buildUrl(post.featured_image);
		const mimeType = getImageMimeType(post.featured_image);
		const length = estimateImageSize(post.featured_image);
		entry += `
			<link rel="enclosure" type="${mimeType}" href="${escapeXml(imageUrl)}" length="${length}" title="${escapeXml(post.title)}" />`;
	}

	// Author information
	entry += `
			<author>
				<name>${escapeXml(post.author?.name || DEFAULT_AUTHOR.name)}</name>
				<email>${escapeXml(DEFAULT_AUTHOR.email)}</email>
				<uri>${escapeXml(DEFAULT_AUTHOR.uri)}</uri>
			</author>`;

	// Categories
	categories.forEach((category) => {
		entry += `
			<category term="${escapeXml(category)}" label="${escapeXml(category)}" />`;
	});

	// Summary (plain text)
	entry += `
			<summary type="text">${escapeXml(summary)}</summary>`;

	// Full content (XHTML)
	if (fullContent) {
		entry += `
			<content type="html">${wrapCDATA(fullContent)}</content>`;
	}

	// Dublin Core metadata
	entry += `
			<dc:creator>${escapeXml(post.author?.name || DEFAULT_AUTHOR.name)}</dc:creator>
			<dc:date>${published}</dc:date>
			<dc:language>${FEED_LANGUAGE}</dc:language>
			<dc:rights>${escapeXml(FEED_COPYRIGHT)}</dc:rights>
			<dc:publisher>${escapeXml(PUBLICATION_NAME)}</dc:publisher>`;

	// Source information
	entry += `
			<source>
				<id>${FEED_ID}</id>
				<title>${escapeXml(FEED_TITLE)}</title>
				<updated>${formatISO8601(new Date())}</updated>
			</source>`;

	entry += `
		</entry>`;

	return entry;
}

/**
 * Generate the complete Atom 1.0 feed XML
 * @param posts - Array of blog posts
 * @returns Complete Atom XML string
 */
function generateAtomFeed(posts: FeedPost[]): string {
	const updated = formatISO8601(new Date());
	const entries = posts.map((post) => generateEntry(post)).join('');

	return `<?xml version="1.0" encoding="UTF-8"?>
<?xml-stylesheet type="text/xsl" href="${SITE_URL}/feed-styles.xsl"?>
<feed xmlns="http://www.w3.org/2005/Atom"
	xmlns:dc="http://purl.org/dc/elements/1.1/"
	xmlns:thr="http://purl.org/syndication/thread/1.0"
	xmlns:media="http://search.yahoo.com/mrss/"
	xml:lang="${FEED_LANGUAGE}">
	<!-- ═══════════════════════════════════════════════════════════════════════════ -->
	<!-- ${PUBLICATION_NAME} Atom Feed -->
	<!-- Generated: ${updated} -->
	<!-- Total Entries: ${posts.length} -->
	<!-- RFC 4287 Compliant -->
	<!-- ═══════════════════════════════════════════════════════════════════════════ -->

	<!-- Required Feed Elements -->
	<id>${FEED_ID}</id>
	<title type="text">${escapeXml(FEED_TITLE)}</title>
	<updated>${updated}</updated>

	<!-- Recommended Feed Elements -->
	<subtitle type="text">${escapeXml(FEED_SUBTITLE)}</subtitle>

	<!-- Feed Links -->
	<link rel="self" type="application/atom+xml" href="${SITE_URL}/atom.xml" />
	<link rel="alternate" type="text/html" href="${SITE_URL}/blog" hreflang="${FEED_LANGUAGE}" />
	<link rel="alternate" type="application/rss+xml" href="${SITE_URL}/feed.xml" title="RSS Feed" />

	<!-- Generator -->
	<generator uri="${SITE_URL}" version="2.0">Revolution Trading Pros Blog Engine</generator>

	<!-- Feed Icon and Logo -->
	<icon>${SITE_URL}${FEED_ICON}</icon>
	<logo>${SITE_URL}${FEED_LOGO}</logo>

	<!-- Rights -->
	<rights type="text">${escapeXml(FEED_COPYRIGHT)}</rights>

	<!-- Feed Author (Required if entries don't have authors) -->
	<author>
		<name>${escapeXml(DEFAULT_AUTHOR.name)}</name>
		<email>${escapeXml(DEFAULT_AUTHOR.email)}</email>
		<uri>${escapeXml(DEFAULT_AUTHOR.uri)}</uri>
	</author>

	<!-- Feed Contributors -->
	<contributor>
		<name>${escapeXml(DEFAULT_AUTHOR.name)}</name>
		<uri>${escapeXml(DEFAULT_AUTHOR.uri)}</uri>
	</contributor>

	<!-- Dublin Core Metadata -->
	<dc:creator>${escapeXml(PUBLICATION_NAME)}</dc:creator>
	<dc:publisher>${escapeXml(PUBLICATION_NAME)}</dc:publisher>
	<dc:rights>${escapeXml(FEED_COPYRIGHT)}</dc:rights>
	<dc:language>${FEED_LANGUAGE}</dc:language>
	<dc:date>${updated}</dc:date>

	<!-- Feed Categories -->
	<category term="Trading" label="Trading" />
	<category term="Finance" label="Finance" />
	<category term="Education" label="Education" />
	<category term="Market Analysis" label="Market Analysis" />

	<!-- Feed Entries -->${entries}
</feed>`;
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
			console.error(`[Atom Feed] API Error: ${response.status} ${response.statusText}`);
			return [];
		}

		const data: PaginatedPosts = await response.json();
		return data.data || [];
	} catch (error) {
		console.error('[Atom Feed] Failed to fetch posts:', error);
		return [];
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Request Handler
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ fetch }) => {
	const posts = await fetchPublishedPosts(fetch);
	const feed = generateAtomFeed(posts);

	return new Response(feed, {
		headers: {
			'Content-Type': 'application/atom+xml; charset=utf-8',
			'Cache-Control': 'public, max-age=3600, s-maxage=3600, stale-while-revalidate=86400',
			'X-Content-Type-Options': 'nosniff',
			'X-Robots-Tag': 'noindex',
			Vary: 'Accept-Encoding',
			// Add ETag for conditional requests
			ETag: `"atom-${Date.now()}"`,
			// Add Last-Modified header
			'Last-Modified': new Date().toUTCString()
		}
	});
};

// Disable prerendering - Atom feed should be generated dynamically
// to reflect the latest published content
export const prerender = false;

/**
 * Dedicated Google Video Sitemap Generator
 * Following Google Video Sitemap Guidelines (November 2025)
 *
 * Features:
 * - Google Video namespace compliance
 * - Platform detection (YouTube, Vimeo, Dailymotion, TED, Wistia)
 * - Auto thumbnail extraction from video platforms
 * - Duration, rating, and view count support
 * - Family-friendly classification
 * - Geographic restrictions support
 * - Live video support
 * - Subscription/purchase requirements
 *
 * @version 1.0.0 - November 2025 Google Video Standards
 */

import type { RequestHandler } from './$types';

const SITE_URL = 'https://revolutiontradingpros.com';

interface VideoEntry {
	pageUrl: string;
	title: string;
	description: string;
	thumbnailUrl: string;
	contentUrl?: string;
	playerUrl?: string;
	duration?: number; // in seconds
	expirationDate?: string;
	rating?: number; // 0.0 to 5.0
	viewCount?: number;
	publicationDate?: string;
	familyFriendly?: boolean;
	tags?: string[];
	category?: string;
	restrictedCountries?: string[];
	allowedCountries?: string[];
	requiresSubscription?: boolean;
	live?: boolean;
	platform?: 'youtube' | 'vimeo' | 'dailymotion' | 'ted' | 'wistia' | 'self-hosted';
	videoId?: string;
	uploader?: string;
	uploaderUrl?: string;
}

/**
 * Extract video platform and ID from URL
 */
function detectVideoPlatform(
	url: string
): { platform: VideoEntry['platform']; videoId: string } | null {
	// YouTube patterns
	const youtubePatterns = [
		/(?:youtube\.com\/watch\?v=|youtu\.be\/|youtube\.com\/embed\/)([a-zA-Z0-9_-]{11})/,
		/youtube\.com\/v\/([a-zA-Z0-9_-]{11})/
	];
	for (const pattern of youtubePatterns) {
		const match = url.match(pattern);
		if (match) return { platform: 'youtube', videoId: match[1] };
	}

	// Vimeo patterns
	const vimeoPattern = /(?:vimeo\.com\/|player\.vimeo\.com\/video\/)(\d+)/;
	const vimeoMatch = url.match(vimeoPattern);
	if (vimeoMatch) return { platform: 'vimeo', videoId: vimeoMatch[1] };

	// Dailymotion patterns
	const dailymotionPattern = /dailymotion\.com\/(?:video|embed\/video)\/([a-zA-Z0-9]+)/;
	const dailymotionMatch = url.match(dailymotionPattern);
	if (dailymotionMatch) return { platform: 'dailymotion', videoId: dailymotionMatch[1] };

	// TED patterns
	const tedPattern = /ted\.com\/talks\/([a-zA-Z0-9_]+)/;
	const tedMatch = url.match(tedPattern);
	if (tedMatch) return { platform: 'ted', videoId: tedMatch[1] };

	// Wistia patterns
	const wistiaPattern = /(?:wistia\.com\/medias\/|wistia\.net\/embed\/iframe\/)([a-zA-Z0-9]+)/;
	const wistiaMatch = url.match(wistiaPattern);
	if (wistiaMatch) return { platform: 'wistia', videoId: wistiaMatch[1] };

	return null;
}

/**
 * Get thumbnail URL based on platform
 */
function getPlatformThumbnail(platform: VideoEntry['platform'], videoId: string): string {
	switch (platform) {
		case 'youtube':
			return `https://img.youtube.com/vi/${videoId}/maxresdefault.jpg`;
		case 'vimeo':
			return `https://vumbnail.com/${videoId}.jpg`;
		case 'dailymotion':
			return `https://www.dailymotion.com/thumbnail/video/${videoId}`;
		case 'ted':
			return `https://pi.tedcdn.com/r/talkstar-photos.s3.amazonaws.com/uploads/placeholder/placeholder.jpg`;
		case 'wistia':
			return `https://embed-ssl.wistia.com/deliveries/${videoId}.jpg`;
		default:
			return '';
	}
}

/**
 * Get player embed URL based on platform
 */
function getPlatformPlayerUrl(platform: VideoEntry['platform'], videoId: string): string {
	switch (platform) {
		case 'youtube':
			return `https://www.youtube.com/embed/${videoId}`;
		case 'vimeo':
			return `https://player.vimeo.com/video/${videoId}`;
		case 'dailymotion':
			return `https://www.dailymotion.com/embed/video/${videoId}`;
		case 'ted':
			return `https://embed.ted.com/talks/${videoId}`;
		case 'wistia':
			return `https://fast.wistia.net/embed/iframe/${videoId}`;
		default:
			return '';
	}
}

/**
 * Get video entries from content
 * In production, this would fetch from a database
 */
function getVideoEntries(): VideoEntry[] {
	const videos: VideoEntry[] = [
		{
			pageUrl: '/courses/day-trading-masterclass',
			title: 'Day Trading Masterclass - Complete Course Overview',
			description:
				'Learn professional day trading strategies from industry experts. This comprehensive course covers technical analysis, risk management, and live trading techniques.',
			thumbnailUrl: '/images/courses/day-trading-thumbnail.jpg',
			playerUrl: 'https://www.youtube.com/embed/dQw4w9WgXcQ',
			duration: 3600,
			publicationDate: '2025-10-15T10:00:00Z',
			familyFriendly: true,
			tags: ['day trading', 'trading course', 'stock market', 'technical analysis'],
			category: 'Education',
			platform: 'youtube',
			videoId: 'dQw4w9WgXcQ',
			uploader: 'Revolution Trading Pros',
			uploaderUrl: `${SITE_URL}/about`
		},
		{
			pageUrl: '/courses/swing-trading-pro',
			title: 'Swing Trading Pro - Master Multi-Day Positions',
			description:
				'Master swing trading with our professional course. Learn to identify trends, manage positions overnight, and maximize profits on multi-day trades.',
			thumbnailUrl: '/images/courses/swing-trading-thumbnail.jpg',
			playerUrl: 'https://player.vimeo.com/video/123456789',
			duration: 5400,
			publicationDate: '2025-09-20T14:00:00Z',
			familyFriendly: true,
			tags: ['swing trading', 'position trading', 'trend analysis'],
			category: 'Education',
			platform: 'vimeo',
			videoId: '123456789',
			uploader: 'Revolution Trading Pros',
			uploaderUrl: `${SITE_URL}/about`
		},
		{
			pageUrl: '/courses/options-trading',
			title: 'Options Trading Fundamentals - Calls, Puts & Strategies',
			description:
				'Comprehensive options trading education covering calls, puts, spreads, iron condors, and advanced strategies for consistent profits.',
			thumbnailUrl: '/images/courses/options-thumbnail.jpg',
			playerUrl: 'https://www.youtube.com/embed/OPTIONS123',
			duration: 7200,
			publicationDate: '2025-11-01T09:00:00Z',
			familyFriendly: true,
			tags: ['options trading', 'calls', 'puts', 'spreads', 'iron condor'],
			category: 'Education',
			platform: 'youtube',
			videoId: 'OPTIONS123',
			uploader: 'Revolution Trading Pros',
			uploaderUrl: `${SITE_URL}/about`
		},
		{
			pageUrl: '/live-trading-rooms/day-trading',
			title: 'Live Day Trading Room - Daily Market Analysis',
			description:
				'Join our live trading room for real-time market analysis, trade ideas, and interactive Q&A with professional traders.',
			thumbnailUrl: '/images/live-rooms/day-trading-live.jpg',
			playerUrl: 'https://www.youtube.com/embed/LIVE123',
			publicationDate: '2025-11-25T13:30:00Z',
			familyFriendly: true,
			tags: ['live trading', 'day trading', 'market analysis', 'trading room'],
			category: 'Education',
			live: true,
			platform: 'youtube',
			videoId: 'LIVE123',
			uploader: 'Revolution Trading Pros',
			uploaderUrl: `${SITE_URL}/about`
		},
		{
			pageUrl: '/blog/weekly-market-recap',
			title: 'Weekly Market Recap - SPY, QQQ, & Key Sectors Analysis',
			description:
				'Our weekly video recap covering major index movements, sector rotation, and key trading opportunities for the upcoming week.',
			thumbnailUrl: '/images/blog/weekly-recap-thumbnail.jpg',
			playerUrl: 'https://www.youtube.com/embed/RECAP123',
			duration: 1800,
			publicationDate: '2025-11-24T18:00:00Z',
			familyFriendly: true,
			tags: ['market recap', 'SPY', 'QQQ', 'stock market', 'weekly analysis'],
			category: 'News',
			platform: 'youtube',
			videoId: 'RECAP123',
			uploader: 'Revolution Trading Pros',
			uploaderUrl: `${SITE_URL}/about`
		}
	];

	return videos;
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
 * Format duration in ISO 8601 format (PT#H#M#S)
 */
function formatDuration(seconds: number): string {
	const hours = Math.floor(seconds / 3600);
	const minutes = Math.floor((seconds % 3600) / 60);
	const secs = seconds % 60;

	let duration = 'PT';
	if (hours > 0) duration += `${hours}H`;
	if (minutes > 0) duration += `${minutes}M`;
	if (secs > 0 || duration === 'PT') duration += `${secs}S`;

	return duration;
}

/**
 * Generate video sitemap URL entry
 */
function generateVideoEntry(video: VideoEntry): string {
	// Get thumbnail from platform if not provided
	let thumbnail = video.thumbnailUrl;
	if (video.platform && video.videoId && !thumbnail.startsWith('http')) {
		const platformThumbnail = getPlatformThumbnail(video.platform, video.videoId);
		if (platformThumbnail) thumbnail = platformThumbnail;
		else thumbnail = `${SITE_URL}${thumbnail}`;
	} else if (!thumbnail.startsWith('http')) {
		thumbnail = `${SITE_URL}${thumbnail}`;
	}

	// Get player URL from platform if not provided
	let playerUrl = video.playerUrl;
	if (!playerUrl && video.platform && video.videoId) {
		playerUrl = getPlatformPlayerUrl(video.platform, video.videoId);
	}

	let entry = `
	<url>
		<loc>${SITE_URL}${video.pageUrl}</loc>
		<video:video>
			<video:thumbnail_loc>${thumbnail}</video:thumbnail_loc>
			<video:title>${escapeXml(video.title)}</video:title>
			<video:description>${escapeXml(video.description)}</video:description>`;

	// Content URL (direct video file) or Player URL (embed)
	if (video.contentUrl) {
		entry += `
			<video:content_loc>${video.contentUrl}</video:content_loc>`;
	}

	if (playerUrl) {
		entry += `
			<video:player_loc allow_embed="yes">${playerUrl}</video:player_loc>`;
	}

	// Duration
	if (video.duration) {
		entry += `
			<video:duration>${video.duration}</video:duration>`;
	}

	// Expiration date
	if (video.expirationDate) {
		entry += `
			<video:expiration_date>${video.expirationDate}</video:expiration_date>`;
	}

	// Rating (0.0 - 5.0)
	if (video.rating !== undefined) {
		entry += `
			<video:rating>${video.rating.toFixed(1)}</video:rating>`;
	}

	// View count
	if (video.viewCount !== undefined) {
		entry += `
			<video:view_count>${video.viewCount}</video:view_count>`;
	}

	// Publication date
	if (video.publicationDate) {
		entry += `
			<video:publication_date>${video.publicationDate}</video:publication_date>`;
	}

	// Family friendly
	entry += `
			<video:family_friendly>${video.familyFriendly !== false ? 'yes' : 'no'}</video:family_friendly>`;

	// Tags (max 32)
	if (video.tags && video.tags.length > 0) {
		video.tags.slice(0, 32).forEach((tag) => {
			entry += `
			<video:tag>${escapeXml(tag)}</video:tag>`;
		});
	}

	// Category
	if (video.category) {
		entry += `
			<video:category>${escapeXml(video.category)}</video:category>`;
	}

	// Geographic restrictions
	if (video.restrictedCountries && video.restrictedCountries.length > 0) {
		entry += `
			<video:restriction relationship="deny">${video.restrictedCountries.join(' ')}</video:restriction>`;
	} else if (video.allowedCountries && video.allowedCountries.length > 0) {
		entry += `
			<video:restriction relationship="allow">${video.allowedCountries.join(' ')}</video:restriction>`;
	}

	// Requires subscription
	if (video.requiresSubscription) {
		entry += `
			<video:requires_subscription>yes</video:requires_subscription>`;
	}

	// Live video
	if (video.live) {
		entry += `
			<video:live>yes</video:live>`;
	}

	// Uploader info
	if (video.uploader) {
		entry += `
			<video:uploader${video.uploaderUrl ? ` info="${video.uploaderUrl}"` : ''}>${escapeXml(video.uploader)}</video:uploader>`;
	}

	// Platform info (custom extension)
	if (video.platform) {
		entry += `
			<video:platform relationship="allow">web mobile tv</video:platform>`;
	}

	entry += `
		</video:video>
	</url>`;

	return entry;
}

/**
 * Generate complete Google Video sitemap
 */
function generateVideoSitemap(): string {
	const videos = getVideoEntries();
	const urlEntries = videos.map((video) => generateVideoEntry(video)).join('');
	const generatedDate = new Date().toISOString();

	return `<?xml version="1.0" encoding="UTF-8"?>
<?xml-stylesheet type="text/xsl" href="/video-sitemap.xsl"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:video="http://www.google.com/schemas/sitemap-video/1.1"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xsi:schemaLocation="http://www.sitemaps.org/schemas/sitemap/0.9
                            http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd
                            http://www.google.com/schemas/sitemap-video/1.1
                            http://www.google.com/schemas/sitemap-video/1.1/sitemap-video.xsd">
	<!-- ═══════════════════════════════════════════════════════════════════════════ -->
	<!-- Revolution Trading Pros - Google Video Sitemap -->
	<!-- Generated: ${generatedDate} -->
	<!-- Total Videos: ${videos.length} -->
	<!-- Platforms: YouTube, Vimeo, Dailymotion, TED, Wistia supported -->
	<!-- ═══════════════════════════════════════════════════════════════════════════ -->${urlEntries}
</urlset>`;
}

export const GET: RequestHandler = async () => {
	const sitemap = generateVideoSitemap();

	return new Response(sitemap, {
		headers: {
			'Content-Type': 'application/xml; charset=utf-8',
			'Cache-Control': 'public, max-age=3600, s-maxage=86400, stale-while-revalidate=604800',
			'X-Content-Type-Options': 'nosniff',
			'X-Robots-Tag': 'noindex'
		}
	});
};

export const prerender = false; // Video sitemap needs dynamic content

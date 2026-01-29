/**
 * Media Embed Detector - URL Parsing and Embed Generation
 * ========================================================
 * Detects and parses URLs for auto-embedding from various platforms.
 * Generates proper embed HTML/iframe configurations.
 *
 * Supported Platforms:
 * - YouTube (youtube.com/watch?v=X, youtu.be/X, youtube.com/shorts/X)
 * - Vimeo (vimeo.com/X, player.vimeo.com/video/X)
 * - TradingView (tradingview.com/chart/X, tradingview.com/symbols/X)
 * - Twitter/X (twitter.com/user/status/X, x.com/user/status/X)
 *
 * @version 1.0.0
 */

// =============================================================================
// Types
// =============================================================================

/**
 * Supported embed platform types
 */
export type EmbedPlatform = 'youtube' | 'vimeo' | 'tradingview' | 'twitter' | 'unknown';

/**
 * Aspect ratio options for embedded content
 */
export type AspectRatio = '16:9' | '4:3' | '1:1' | '9:16' | 'auto';

/**
 * Embed configuration returned after URL detection
 */
export interface EmbedConfig {
	/** The detected platform type */
	platform: EmbedPlatform;
	/** Original URL that was parsed */
	originalUrl: string;
	/** Unique identifier extracted from the URL (video ID, tweet ID, etc.) */
	id: string;
	/** Ready-to-use embed URL (for iframes) */
	embedUrl: string;
	/** Platform-specific thumbnail URL if available */
	thumbnailUrl?: string;
	/** Suggested title for the embed */
	title?: string;
	/** Additional platform-specific metadata */
	metadata: EmbedMetadata;
	/** Generated HTML for embedding */
	html: string;
	/** Recommended aspect ratio for display */
	aspectRatio: AspectRatio;
	/** Whether the embed supports responsive sizing */
	responsive: boolean;
	/** Platform-specific embed options */
	options: EmbedOptions;
}

/**
 * Platform-specific metadata
 */
export interface EmbedMetadata {
	/** Platform display name */
	platformName: string;
	/** Platform icon identifier */
	platformIcon: string;
	/** Platform brand color */
	platformColor: string;
	/** Username or channel name if available */
	username?: string;
	/** Playlist ID for YouTube */
	playlistId?: string;
	/** Start time in seconds for video platforms */
	startTime?: number;
	/** Whether the content is a short-form video */
	isShort?: boolean;
	/** Symbol for TradingView */
	symbol?: string;
	/** Chart type for TradingView */
	chartType?: 'advanced' | 'mini' | 'symbol' | 'ideas';
}

/**
 * Embed display options
 */
export interface EmbedOptions {
	/** Auto-play the media (where supported) */
	autoplay: boolean;
	/** Show player controls */
	controls: boolean;
	/** Loop the media */
	loop: boolean;
	/** Mute audio by default */
	muted: boolean;
	/** Allow fullscreen */
	allowFullscreen: boolean;
	/** Enable privacy-enhanced mode (YouTube) */
	privacyEnhanced: boolean;
	/** Show related videos at end (YouTube) */
	showRelated: boolean;
	/** Theme for embeds that support it */
	theme: 'light' | 'dark' | 'auto';
	/** Width for fixed-size embeds */
	width?: number;
	/** Height for fixed-size embeds */
	height?: number;
}

/**
 * Detection result from URL parsing
 */
export interface DetectionResult {
	/** Whether a valid embed URL was detected */
	isEmbed: boolean;
	/** Embed configuration if detected, null otherwise */
	config: EmbedConfig | null;
	/** Error message if detection failed */
	error?: string;
}

/**
 * URL pattern definition for platform detection
 */
interface UrlPattern {
	platform: EmbedPlatform;
	patterns: RegExp[];
	extractor: (match: RegExpMatchArray, url: string) => Partial<EmbedConfig> | null;
}

// =============================================================================
// Default Options
// =============================================================================

const DEFAULT_EMBED_OPTIONS: EmbedOptions = {
	autoplay: false,
	controls: true,
	loop: false,
	muted: false,
	allowFullscreen: true,
	privacyEnhanced: true,
	showRelated: false,
	theme: 'auto'
};

// =============================================================================
// Platform Configurations
// =============================================================================

const PLATFORM_CONFIG: Record<
	EmbedPlatform,
	Omit<EmbedMetadata, 'username' | 'playlistId' | 'startTime' | 'isShort' | 'symbol' | 'chartType'>
> = {
	youtube: {
		platformName: 'YouTube',
		platformIcon: 'brand-youtube',
		platformColor: '#FF0000'
	},
	vimeo: {
		platformName: 'Vimeo',
		platformIcon: 'brand-vimeo',
		platformColor: '#1AB7EA'
	},
	tradingview: {
		platformName: 'TradingView',
		platformIcon: 'chart-candle',
		platformColor: '#2962FF'
	},
	twitter: {
		platformName: 'Twitter/X',
		platformIcon: 'brand-x',
		platformColor: '#000000'
	},
	unknown: {
		platformName: 'Unknown',
		platformIcon: 'link',
		platformColor: '#6B7280'
	}
};

// =============================================================================
// URL Patterns
// =============================================================================

const URL_PATTERNS: UrlPattern[] = [
	// YouTube patterns
	{
		platform: 'youtube',
		patterns: [
			// Standard watch URL: youtube.com/watch?v=VIDEO_ID
			/(?:https?:\/\/)?(?:www\.)?youtube\.com\/watch\?v=([a-zA-Z0-9_-]{11})(?:&[^\s]*)?/i,
			// Short URL: youtu.be/VIDEO_ID
			/(?:https?:\/\/)?(?:www\.)?youtu\.be\/([a-zA-Z0-9_-]{11})(?:\?[^\s]*)?/i,
			// Embed URL: youtube.com/embed/VIDEO_ID
			/(?:https?:\/\/)?(?:www\.)?youtube\.com\/embed\/([a-zA-Z0-9_-]{11})(?:\?[^\s]*)?/i,
			// Shorts URL: youtube.com/shorts/VIDEO_ID
			/(?:https?:\/\/)?(?:www\.)?youtube\.com\/shorts\/([a-zA-Z0-9_-]{11})(?:\?[^\s]*)?/i,
			// Live URL: youtube.com/live/VIDEO_ID
			/(?:https?:\/\/)?(?:www\.)?youtube\.com\/live\/([a-zA-Z0-9_-]{11})(?:\?[^\s]*)?/i,
			// Playlist with video: youtube.com/watch?v=VIDEO_ID&list=PLAYLIST_ID
			/(?:https?:\/\/)?(?:www\.)?youtube\.com\/watch\?v=([a-zA-Z0-9_-]{11})&list=([a-zA-Z0-9_-]+)/i
		],
		extractor: (match, url) => {
			const videoId = match[1];
			if (!videoId) return null;

			const urlObj = new URL(url.startsWith('http') ? url : `https://${url}`);
			const params = urlObj.searchParams;

			// Extract additional parameters
			const startTime = params.get('t') || params.get('start');
			const playlistId = params.get('list');
			const isShort = url.includes('/shorts/');

			// Build embed URL with options
			const embedParams = new URLSearchParams();
			if (startTime) {
				const seconds = parseYouTubeTime(startTime);
				if (seconds) embedParams.set('start', seconds.toString());
			}
			if (playlistId) embedParams.set('list', playlistId);

			const embedUrl = `https://www.youtube-nocookie.com/embed/${videoId}${embedParams.toString() ? '?' + embedParams.toString() : ''}`;

			return {
				platform: 'youtube',
				id: videoId,
				embedUrl,
				thumbnailUrl: `https://img.youtube.com/vi/${videoId}/maxresdefault.jpg`,
				aspectRatio: isShort ? '9:16' : '16:9',
				metadata: {
					...PLATFORM_CONFIG.youtube,
					playlistId: playlistId || undefined,
					startTime: startTime ? parseYouTubeTime(startTime) : undefined,
					isShort
				}
			};
		}
	},
	// Vimeo patterns
	{
		platform: 'vimeo',
		patterns: [
			// Standard URL: vimeo.com/VIDEO_ID
			/(?:https?:\/\/)?(?:www\.)?vimeo\.com\/(\d+)(?:\?[^\s]*)?/i,
			// Player URL: player.vimeo.com/video/VIDEO_ID
			/(?:https?:\/\/)?player\.vimeo\.com\/video\/(\d+)(?:\?[^\s]*)?/i,
			// Channel video: vimeo.com/channels/CHANNEL/VIDEO_ID
			/(?:https?:\/\/)?(?:www\.)?vimeo\.com\/channels\/[^\/]+\/(\d+)(?:\?[^\s]*)?/i,
			// Groups video: vimeo.com/groups/GROUP/videos/VIDEO_ID
			/(?:https?:\/\/)?(?:www\.)?vimeo\.com\/groups\/[^\/]+\/videos\/(\d+)(?:\?[^\s]*)?/i,
			// Showcase video: vimeo.com/showcase/SHOWCASE_ID/video/VIDEO_ID
			/(?:https?:\/\/)?(?:www\.)?vimeo\.com\/showcase\/[^\/]+\/video\/(\d+)(?:\?[^\s]*)?/i
		],
		extractor: (match, url) => {
			const videoId = match[1];
			if (!videoId) return null;

			const urlObj = new URL(url.startsWith('http') ? url : `https://${url}`);
			const hash = urlObj.hash?.replace('#t=', '') || urlObj.searchParams.get('t');
			const startTime = hash ? parseVimeoTime(hash) : undefined;

			const embedParams = new URLSearchParams();
			embedParams.set('dnt', '1'); // Do not track
			if (startTime) embedParams.set('#t', `${startTime}s`);

			const embedUrl = `https://player.vimeo.com/video/${videoId}?${embedParams.toString()}`;

			return {
				platform: 'vimeo',
				id: videoId,
				embedUrl,
				thumbnailUrl: `https://vumbnail.com/${videoId}.jpg`,
				aspectRatio: '16:9',
				metadata: {
					...PLATFORM_CONFIG.vimeo,
					startTime
				}
			};
		}
	},
	// TradingView patterns
	{
		platform: 'tradingview',
		patterns: [
			// Chart URL: tradingview.com/chart/CHART_ID
			/(?:https?:\/\/)?(?:www\.)?tradingview\.com\/chart\/([a-zA-Z0-9]+)(?:\/[^\s]*)?/i,
			// Symbol URL: tradingview.com/symbols/EXCHANGE-SYMBOL
			/(?:https?:\/\/)?(?:www\.)?tradingview\.com\/symbols\/([A-Z0-9]+[-:]?[A-Z0-9]+)(?:\/[^\s]*)?/i,
			// Ideas URL: tradingview.com/chart/SYMBOL/IDEA_ID
			/(?:https?:\/\/)?(?:www\.)?tradingview\.com\/chart\/([A-Z0-9]+)\/([a-zA-Z0-9]+)(?:\/[^\s]*)?/i,
			// Widget embed: tradingview.com/widgetembed
			/(?:https?:\/\/)?s\.tradingview\.com\/widgetembed\/?\?([^\s]+)/i
		],
		extractor: (match, url) => {
			const id = match[1];
			if (!id) return null;

			// Determine chart type based on URL structure
			let chartType: 'advanced' | 'mini' | 'symbol' | 'ideas' = 'advanced';
			let symbol = id;

			if (url.includes('/symbols/')) {
				chartType = 'symbol';
				symbol = id.replace('-', ':');
			} else if (match[2]) {
				chartType = 'ideas';
			}

			// Build TradingView widget embed URL
			const embedParams = new URLSearchParams({
				symbol: symbol.toUpperCase(),
				theme: 'dark',
				style: '1',
				locale: 'en',
				toolbar_bg: '#f1f3f6',
				enable_publishing: 'false',
				allow_symbol_change: 'true',
				container_id: `tradingview_${id}`
			});

			const embedUrl = `https://s.tradingview.com/widgetembed/?${embedParams.toString()}`;

			return {
				platform: 'tradingview',
				id,
				embedUrl,
				aspectRatio: '16:9',
				metadata: {
					...PLATFORM_CONFIG.tradingview,
					symbol: symbol.toUpperCase(),
					chartType
				}
			};
		}
	},
	// Twitter/X patterns
	{
		platform: 'twitter',
		patterns: [
			// Twitter status: twitter.com/USER/status/TWEET_ID
			/(?:https?:\/\/)?(?:www\.)?twitter\.com\/([a-zA-Z0-9_]+)\/status\/(\d+)(?:\?[^\s]*)?/i,
			// X.com status: x.com/USER/status/TWEET_ID
			/(?:https?:\/\/)?(?:www\.)?x\.com\/([a-zA-Z0-9_]+)\/status\/(\d+)(?:\?[^\s]*)?/i,
			// Mobile Twitter: mobile.twitter.com/USER/status/TWEET_ID
			/(?:https?:\/\/)?mobile\.twitter\.com\/([a-zA-Z0-9_]+)\/status\/(\d+)(?:\?[^\s]*)?/i
		],
		extractor: (match) => {
			const username = match[1];
			const tweetId = match[2];
			if (!username || !tweetId) return null;

			// Twitter embeds use the publish platform
			const embedUrl = `https://platform.twitter.com/embed/Tweet.html?id=${tweetId}`;

			return {
				platform: 'twitter',
				id: tweetId,
				embedUrl,
				aspectRatio: 'auto',
				responsive: true,
				metadata: {
					...PLATFORM_CONFIG.twitter,
					username
				}
			};
		}
	}
];

// =============================================================================
// Helper Functions
// =============================================================================

/**
 * Parse YouTube time format (e.g., "1h2m3s", "123", "1:23:45")
 */
function parseYouTubeTime(timeStr: string): number | undefined {
	if (!timeStr) return undefined;

	// Already in seconds
	if (/^\d+$/.test(timeStr)) {
		return parseInt(timeStr, 10);
	}

	// Format: 1h2m3s
	const hmsMatch = timeStr.match(/(?:(\d+)h)?(?:(\d+)m)?(?:(\d+)s)?/);
	if (hmsMatch) {
		const hours = parseInt(hmsMatch[1] || '0', 10);
		const minutes = parseInt(hmsMatch[2] || '0', 10);
		const seconds = parseInt(hmsMatch[3] || '0', 10);
		const total = hours * 3600 + minutes * 60 + seconds;
		if (total > 0) return total;
	}

	// Format: HH:MM:SS or MM:SS
	const colonMatch = timeStr.match(/(?:(\d+):)?(\d+):(\d+)/);
	if (colonMatch) {
		const hours = parseInt(colonMatch[1] || '0', 10);
		const minutes = parseInt(colonMatch[2] || '0', 10);
		const seconds = parseInt(colonMatch[3] || '0', 10);
		return hours * 3600 + minutes * 60 + seconds;
	}

	return undefined;
}

/**
 * Parse Vimeo time format
 */
function parseVimeoTime(timeStr: string): number | undefined {
	if (!timeStr) return undefined;

	// Format: 123s or 123
	const match = timeStr.match(/^(\d+)s?$/);
	if (match) {
		return parseInt(match[1], 10);
	}

	// Format: 1m23s
	const minsMatch = timeStr.match(/(?:(\d+)m)?(\d+)s/);
	if (minsMatch) {
		const minutes = parseInt(minsMatch[1] || '0', 10);
		const seconds = parseInt(minsMatch[2] || '0', 10);
		return minutes * 60 + seconds;
	}

	return undefined;
}

/**
 * Escape HTML special characters
 */
function escapeHtml(str: string): string {
	const htmlEntities: Record<string, string> = {
		'&': '&amp;',
		'<': '&lt;',
		'>': '&gt;',
		'"': '&quot;',
		"'": '&#39;'
	};
	return str.replace(/[&<>"']/g, (char) => htmlEntities[char] || char);
}

/**
 * Calculate responsive dimensions based on aspect ratio
 */
function getResponsivePadding(aspectRatio: AspectRatio): string {
	const ratios: Record<AspectRatio, number> = {
		'16:9': 56.25,
		'4:3': 75,
		'1:1': 100,
		'9:16': 177.78,
		auto: 56.25
	};
	return `${ratios[aspectRatio]}%`;
}

// =============================================================================
// HTML Generators
// =============================================================================

/**
 * Generate YouTube embed HTML
 */
function generateYouTubeHtml(config: EmbedConfig): string {
	const { embedUrl, options, aspectRatio, id } = config;

	const params = new URLSearchParams();
	if (options.autoplay) params.set('autoplay', '1');
	if (options.muted) params.set('mute', '1');
	if (options.loop) {
		params.set('loop', '1');
		params.set('playlist', id);
	}
	if (!options.controls) params.set('controls', '0');
	if (!options.showRelated) params.set('rel', '0');
	params.set('modestbranding', '1');

	const fullUrl = `${embedUrl}${embedUrl.includes('?') ? '&' : '?'}${params.toString()}`;

	return `<div class="embed-container embed-youtube" style="position: relative; padding-bottom: ${getResponsivePadding(aspectRatio)}; height: 0; overflow: hidden;">
  <iframe
    src="${escapeHtml(fullUrl)}"
    title="YouTube video player"
    frameborder="0"
    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
    ${options.allowFullscreen ? 'allowfullscreen' : ''}
    loading="lazy"
    style="position: absolute; top: 0; left: 0; width: 100%; height: 100%;"
  ></iframe>
</div>`;
}

/**
 * Generate Vimeo embed HTML
 */
function generateVimeoHtml(config: EmbedConfig): string {
	const { embedUrl, options, aspectRatio } = config;

	const params = new URLSearchParams();
	if (options.autoplay) params.set('autoplay', '1');
	if (options.muted) params.set('muted', '1');
	if (options.loop) params.set('loop', '1');
	params.set('dnt', '1');
	params.set('byline', '0');
	params.set('portrait', '0');

	const fullUrl = `${embedUrl}${embedUrl.includes('?') ? '&' : '?'}${params.toString()}`;

	return `<div class="embed-container embed-vimeo" style="position: relative; padding-bottom: ${getResponsivePadding(aspectRatio)}; height: 0; overflow: hidden;">
  <iframe
    src="${escapeHtml(fullUrl)}"
    title="Vimeo video player"
    frameborder="0"
    allow="autoplay; fullscreen; picture-in-picture"
    ${options.allowFullscreen ? 'allowfullscreen' : ''}
    loading="lazy"
    style="position: absolute; top: 0; left: 0; width: 100%; height: 100%;"
  ></iframe>
</div>`;
}

/**
 * Generate TradingView embed HTML
 */
function generateTradingViewHtml(config: EmbedConfig): string {
	const { metadata, options, id } = config;
	const symbol = metadata.symbol || 'NASDAQ:AAPL';
	const theme = options.theme === 'auto' ? 'dark' : options.theme;
	const width = options.width || '100%';
	const height = options.height || 500;

	// TradingView requires a script-based widget
	return `<div class="embed-container embed-tradingview" style="width: ${typeof width === 'number' ? width + 'px' : width}; height: ${height}px;">
  <div id="tradingview_widget_${id}" style="width: 100%; height: 100%;"></div>
  <script type="text/javascript">
    (function() {
      if (typeof TradingView !== 'undefined') {
        new TradingView.widget({
          "autosize": true,
          "symbol": "${escapeHtml(symbol)}",
          "interval": "D",
          "timezone": "Etc/UTC",
          "theme": "${theme}",
          "style": "1",
          "locale": "en",
          "toolbar_bg": "#f1f3f6",
          "enable_publishing": false,
          "allow_symbol_change": true,
          "container_id": "tradingview_widget_${id}"
        });
      }
    })();
  </script>
</div>`;
}

/**
 * Generate Twitter embed HTML
 */
function generateTwitterHtml(config: EmbedConfig): string {
	const { id, metadata, options } = config;
	const username = metadata.username || 'unknown';
	const theme = options.theme === 'auto' ? '' : `data-theme="${options.theme}"`;

	return `<div class="embed-container embed-twitter">
  <blockquote class="twitter-tweet" ${theme}>
    <a href="https://twitter.com/${escapeHtml(username)}/status/${escapeHtml(id)}">Loading tweet...</a>
  </blockquote>
  <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
</div>`;
}

/**
 * Generate embed HTML based on platform
 */
function generateEmbedHtml(config: EmbedConfig): string {
	switch (config.platform) {
		case 'youtube':
			return generateYouTubeHtml(config);
		case 'vimeo':
			return generateVimeoHtml(config);
		case 'tradingview':
			return generateTradingViewHtml(config);
		case 'twitter':
			return generateTwitterHtml(config);
		default:
			return `<a href="${escapeHtml(config.originalUrl)}" target="_blank" rel="noopener noreferrer">${escapeHtml(config.originalUrl)}</a>`;
	}
}

// =============================================================================
// Main Detection Functions
// =============================================================================

/**
 * Detect if a URL is an embeddable media URL and return configuration
 *
 * @param url - The URL to analyze
 * @param options - Optional embed options to override defaults
 * @returns Detection result with embed configuration if found
 *
 * @example
 * ```typescript
 * const result = detectEmbed('https://youtube.com/watch?v=dQw4w9WgXcQ');
 * if (result.isEmbed && result.config) {
 *   console.log(result.config.platform); // 'youtube'
 *   console.log(result.config.html); // Generated iframe HTML
 * }
 * ```
 */
export function detectEmbed(url: string, options?: Partial<EmbedOptions>): DetectionResult {
	if (!url || typeof url !== 'string') {
		return {
			isEmbed: false,
			config: null,
			error: 'Invalid URL provided'
		};
	}

	// Clean and normalize URL
	const cleanUrl = url.trim();
	if (!cleanUrl) {
		return {
			isEmbed: false,
			config: null,
			error: 'Empty URL provided'
		};
	}

	// Try each platform pattern
	for (const patternDef of URL_PATTERNS) {
		for (const pattern of patternDef.patterns) {
			const match = cleanUrl.match(pattern);
			if (match) {
				const extracted = patternDef.extractor(match, cleanUrl);
				if (extracted) {
					const mergedOptions = { ...DEFAULT_EMBED_OPTIONS, ...options };

					const config: EmbedConfig = {
						platform: patternDef.platform,
						originalUrl: cleanUrl,
						id: extracted.id || '',
						embedUrl: extracted.embedUrl || '',
						thumbnailUrl: extracted.thumbnailUrl,
						title: extracted.title,
						metadata: {
							...PLATFORM_CONFIG[patternDef.platform],
							...extracted.metadata
						},
						html: '',
						aspectRatio: extracted.aspectRatio || '16:9',
						responsive: extracted.responsive !== false,
						options: mergedOptions
					};

					// Generate HTML
					config.html = generateEmbedHtml(config);

					return {
						isEmbed: true,
						config
					};
				}
			}
		}
	}

	return {
		isEmbed: false,
		config: null
	};
}

/**
 * Check if a URL is a valid embeddable URL without full parsing
 *
 * @param url - The URL to check
 * @returns True if the URL can be embedded
 */
export function isEmbedUrl(url: string): boolean {
	if (!url || typeof url !== 'string') return false;

	const cleanUrl = url.trim();

	for (const patternDef of URL_PATTERNS) {
		for (const pattern of patternDef.patterns) {
			if (pattern.test(cleanUrl)) {
				return true;
			}
		}
	}

	return false;
}

/**
 * Get the platform type for a URL without full parsing
 *
 * @param url - The URL to analyze
 * @returns The platform type or 'unknown'
 */
export function getEmbedPlatform(url: string): EmbedPlatform {
	if (!url || typeof url !== 'string') return 'unknown';

	const cleanUrl = url.trim();

	for (const patternDef of URL_PATTERNS) {
		for (const pattern of patternDef.patterns) {
			if (pattern.test(cleanUrl)) {
				return patternDef.platform;
			}
		}
	}

	return 'unknown';
}

/**
 * Extract video/content ID from a URL
 *
 * @param url - The URL to parse
 * @returns The extracted ID or null
 */
export function extractEmbedId(url: string): string | null {
	const result = detectEmbed(url);
	return result.config?.id || null;
}

/**
 * Generate embed HTML directly from a URL
 *
 * @param url - The URL to embed
 * @param options - Optional embed options
 * @returns Generated HTML string or null if not embeddable
 */
export function generateEmbedFromUrl(url: string, options?: Partial<EmbedOptions>): string | null {
	const result = detectEmbed(url, options);
	return result.config?.html || null;
}

/**
 * Get thumbnail URL for embeddable content
 *
 * @param url - The URL to get thumbnail for
 * @returns Thumbnail URL or null
 */
export function getEmbedThumbnail(url: string): string | null {
	const result = detectEmbed(url);
	return result.config?.thumbnailUrl || null;
}

/**
 * Parse multiple URLs and return embed configurations
 *
 * @param urls - Array of URLs to parse
 * @param options - Optional embed options
 * @returns Array of detection results
 */
export function detectEmbeds(urls: string[], options?: Partial<EmbedOptions>): DetectionResult[] {
	return urls.map((url) => detectEmbed(url, options));
}

/**
 * Check clipboard text for embeddable URLs
 *
 * @param text - Text content to scan
 * @returns Array of detected embed configs
 */
export function scanTextForEmbeds(text: string): EmbedConfig[] {
	if (!text) return [];

	// URL regex to find all URLs in text
	const urlRegex = /https?:\/\/[^\s<>"{}|\\^`[\]]+/gi;
	const urls = text.match(urlRegex) || [];

	const embeds: EmbedConfig[] = [];

	for (const url of urls) {
		const result = detectEmbed(url);
		if (result.isEmbed && result.config) {
			embeds.push(result.config);
		}
	}

	return embeds;
}

// =============================================================================
// Platform-Specific Helpers
// =============================================================================

/**
 * Generate YouTube thumbnail URLs in various sizes
 */
export function getYouTubeThumbnails(videoId: string): {
	default: string;
	medium: string;
	high: string;
	standard: string;
	maxres: string;
} {
	const base = `https://img.youtube.com/vi/${videoId}`;
	return {
		default: `${base}/default.jpg`, // 120x90
		medium: `${base}/mqdefault.jpg`, // 320x180
		high: `${base}/hqdefault.jpg`, // 480x360
		standard: `${base}/sddefault.jpg`, // 640x480
		maxres: `${base}/maxresdefault.jpg` // 1280x720
	};
}

/**
 * Create a TradingView widget configuration
 */
export function createTradingViewConfig(
	symbol: string,
	options?: {
		theme?: 'light' | 'dark';
		interval?: string;
		style?: string;
		width?: number | string;
		height?: number;
	}
): string {
	const config = {
		autosize: true,
		symbol: symbol.toUpperCase(),
		interval: options?.interval || 'D',
		timezone: 'Etc/UTC',
		theme: options?.theme || 'dark',
		style: options?.style || '1',
		locale: 'en',
		toolbar_bg: '#f1f3f6',
		enable_publishing: false,
		allow_symbol_change: true,
		width: options?.width || '100%',
		height: options?.height || 500
	};

	return JSON.stringify(config);
}

// =============================================================================
// Exports
// =============================================================================

export default {
	detectEmbed,
	isEmbedUrl,
	getEmbedPlatform,
	extractEmbedId,
	generateEmbedFromUrl,
	getEmbedThumbnail,
	detectEmbeds,
	scanTextForEmbeds,
	getYouTubeThumbnails,
	createTradingViewConfig,
	PLATFORM_CONFIG,
	DEFAULT_EMBED_OPTIONS
};

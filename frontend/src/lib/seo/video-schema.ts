/**
 * Video Schema Generator - Enterprise-Grade Video Structured Data
 * Following Google Video Schema Best Practices (November 2025)
 *
 * Features:
 * - Auto-detection from YouTube, Vimeo, Dailymotion, TED, Wistia
 * - VideoObject schema generation
 * - BroadcastEvent for live streams
 * - VideoGallery for multiple videos
 * - Clip schema for key moments
 * - SeekToAction for direct navigation
 * - HowToStep integration for tutorial videos
 * - Course/Episode integration
 * - Thumbnail optimization
 *
 * @version 1.0.0 - November 2025
 */

// ═══════════════════════════════════════════════════════════════════════════
// Type Definitions
// ═══════════════════════════════════════════════════════════════════════════

export type VideoPlatform = 'youtube' | 'vimeo' | 'dailymotion' | 'ted' | 'wistia' | 'self-hosted';

export interface VideoClip {
	name: string;
	startOffset: number; // in seconds
	endOffset: number; // in seconds
	url?: string;
}

export interface VideoInteractionStatistic {
	type: 'WatchAction' | 'LikeAction' | 'DislikeAction' | 'CommentAction' | 'ShareAction';
	count: number;
}

export interface VideoInput {
	url: string;
	embedUrl?: string;
	title: string;
	description: string;
	thumbnailUrl: string | string[];
	uploadDate: string; // ISO 8601
	duration?: string; // ISO 8601 duration (PT1H30M)
	durationSeconds?: number;
	contentUrl?: string;
	expires?: string; // ISO 8601
	hasPart?: VideoClip[];
	interactionStatistic?: VideoInteractionStatistic[];
	regionsAllowed?: string[];
	publication?: {
		isLiveBroadcast: boolean;
		startDate?: string;
		endDate?: string;
	};
	transcript?: string;
	author?: {
		name: string;
		url?: string;
	};
	publisher?: {
		name: string;
		logo: string;
		url: string;
	};
	// Course integration
	courseInfo?: {
		name: string;
		description: string;
		provider: string;
		episodeNumber?: number;
		seasonNumber?: number;
	};
	// Tutorial integration
	howToSteps?: {
		name: string;
		text: string;
		startOffset: number;
		endOffset: number;
	}[];
}

export interface VideoSchemaOptions {
	includeSeekToAction: boolean;
	includeBreadcrumbs: boolean;
	includePublisher: boolean;
	defaultPublisher?: {
		name: string;
		logo: string;
		url: string;
	};
}

export interface DetectedVideo {
	platform: VideoPlatform;
	videoId: string;
	embedUrl: string;
	thumbnailUrl: string;
	oEmbedUrl?: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// Platform Detection
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Detect video platform and extract video ID from URL
 */
export function detectVideoPlatform(url: string): DetectedVideo | null {
	// YouTube patterns
	const youtubePatterns = [
		/(?:youtube\.com\/watch\?v=|youtu\.be\/|youtube\.com\/embed\/)([a-zA-Z0-9_-]{11})/,
		/youtube\.com\/v\/([a-zA-Z0-9_-]{11})/,
		/youtube\.com\/shorts\/([a-zA-Z0-9_-]{11})/
	];

	for (const pattern of youtubePatterns) {
		const match = url.match(pattern);
		if (match) {
			return {
				platform: 'youtube',
				videoId: match[1],
				embedUrl: `https://www.youtube.com/embed/${match[1]}`,
				thumbnailUrl: `https://img.youtube.com/vi/${match[1]}/maxresdefault.jpg`,
				oEmbedUrl: `https://www.youtube.com/oembed?url=https://www.youtube.com/watch?v=${match[1]}&format=json`
			};
		}
	}

	// Vimeo patterns
	const vimeoPattern = /(?:vimeo\.com\/|player\.vimeo\.com\/video\/)(\d+)/;
	const vimeoMatch = url.match(vimeoPattern);
	if (vimeoMatch) {
		return {
			platform: 'vimeo',
			videoId: vimeoMatch[1],
			embedUrl: `https://player.vimeo.com/video/${vimeoMatch[1]}`,
			thumbnailUrl: `https://vumbnail.com/${vimeoMatch[1]}.jpg`,
			oEmbedUrl: `https://vimeo.com/api/oembed.json?url=https://vimeo.com/${vimeoMatch[1]}`
		};
	}

	// Dailymotion patterns
	const dailymotionPattern = /dailymotion\.com\/(?:video|embed\/video)\/([a-zA-Z0-9]+)/;
	const dailymotionMatch = url.match(dailymotionPattern);
	if (dailymotionMatch) {
		return {
			platform: 'dailymotion',
			videoId: dailymotionMatch[1],
			embedUrl: `https://www.dailymotion.com/embed/video/${dailymotionMatch[1]}`,
			thumbnailUrl: `https://www.dailymotion.com/thumbnail/video/${dailymotionMatch[1]}`,
			oEmbedUrl: `https://www.dailymotion.com/services/oembed?url=https://www.dailymotion.com/video/${dailymotionMatch[1]}`
		};
	}

	// TED patterns
	const tedPattern = /ted\.com\/talks\/([a-zA-Z0-9_]+)/;
	const tedMatch = url.match(tedPattern);
	if (tedMatch) {
		return {
			platform: 'ted',
			videoId: tedMatch[1],
			embedUrl: `https://embed.ted.com/talks/${tedMatch[1]}`,
			thumbnailUrl: `https://pi.tedcdn.com/r/talkstar-assets.s3.amazonaws.com/production/playlists/${tedMatch[1]}.jpg`
		};
	}

	// Wistia patterns
	const wistiaPattern = /(?:wistia\.com\/medias\/|wistia\.net\/embed\/iframe\/)([a-zA-Z0-9]+)/;
	const wistiaMatch = url.match(wistiaPattern);
	if (wistiaMatch) {
		return {
			platform: 'wistia',
			videoId: wistiaMatch[1],
			embedUrl: `https://fast.wistia.net/embed/iframe/${wistiaMatch[1]}`,
			thumbnailUrl: `https://embed-ssl.wistia.com/deliveries/${wistiaMatch[1]}.jpg`,
			oEmbedUrl: `https://fast.wistia.com/oembed?url=https://home.wistia.com/medias/${wistiaMatch[1]}`
		};
	}

	return null;
}

/**
 * Parse content to find all video URLs
 */
export function findVideosInContent(content: string): DetectedVideo[] {
	const videos: DetectedVideo[] = [];

	// Common video URL patterns
	const urlPattern =
		/(https?:\/\/[^\s<>"]+(?:youtube|youtu\.be|vimeo|dailymotion|ted\.com|wistia)[^\s<>"]*)/gi;
	const matches: string[] = content.match(urlPattern) || [];

	// Also check for iframe embeds
	const iframePattern = /<iframe[^>]+src=["']([^"']+)["'][^>]*>/gi;
	let iframeMatch: RegExpExecArray | null;
	while ((iframeMatch = iframePattern.exec(content)) !== null) {
		matches.push(iframeMatch[1]);
	}

	for (const url of matches) {
		const detected = detectVideoPlatform(url);
		if (detected && !videos.some((v) => v.videoId === detected.videoId)) {
			videos.push(detected);
		}
	}

	return videos;
}

// ═══════════════════════════════════════════════════════════════════════════
// Duration Utilities
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Convert seconds to ISO 8601 duration format
 */
export function secondsToIsoDuration(seconds: number): string {
	const hours = Math.floor(seconds / 3600);
	const minutes = Math.floor((seconds % 3600) / 60);
	const secs = Math.floor(seconds % 60);

	let duration = 'PT';
	if (hours > 0) duration += `${hours}H`;
	if (minutes > 0) duration += `${minutes}M`;
	if (secs > 0 || duration === 'PT') duration += `${secs}S`;

	return duration;
}

/**
 * Parse ISO 8601 duration to seconds
 */
export function isoDurationToSeconds(duration: string): number {
	const match = duration.match(/PT(?:(\d+)H)?(?:(\d+)M)?(?:(\d+)S)?/);
	if (!match) return 0;

	const hours = parseInt(match[1] || '0', 10);
	const minutes = parseInt(match[2] || '0', 10);
	const seconds = parseInt(match[3] || '0', 10);

	return hours * 3600 + minutes * 60 + seconds;
}

// ═══════════════════════════════════════════════════════════════════════════
// Schema Generation
// ═══════════════════════════════════════════════════════════════════════════

const DEFAULT_OPTIONS: VideoSchemaOptions = {
	includeSeekToAction: true,
	includeBreadcrumbs: false,
	includePublisher: true,
	defaultPublisher: {
		name: 'Revolution Trading Pros',
		logo: 'https://revolutiontradingpros.com/logo.png',
		url: 'https://revolutiontradingpros.com'
	}
};

/**
 * Generate VideoObject JSON-LD schema
 */
export function generateVideoSchema(
	video: VideoInput,
	options: Partial<VideoSchemaOptions> = {}
): Record<string, any> {
	const opts = { ...DEFAULT_OPTIONS, ...options };

	// Calculate duration
	let duration = video.duration;
	if (!duration && video.durationSeconds) {
		duration = secondsToIsoDuration(video.durationSeconds);
	}

	// Ensure thumbnail is an array
	const thumbnails = Array.isArray(video.thumbnailUrl)
		? video.thumbnailUrl
		: [video.thumbnailUrl];

	const schema: Record<string, any> = {
		'@context': 'https://schema.org',
		'@type': 'VideoObject',
		name: video.title,
		description: video.description,
		thumbnailUrl: thumbnails,
		uploadDate: video.uploadDate
	};

	// Add URLs
	if (video.contentUrl) {
		schema.contentUrl = video.contentUrl;
	}

	if (video.embedUrl) {
		schema.embedUrl = video.embedUrl;
	}

	// Add duration
	if (duration) {
		schema.duration = duration;
	}

	// Add expiration
	if (video.expires) {
		schema.expires = video.expires;
	}

	// Add interaction statistics
	if (video.interactionStatistic && video.interactionStatistic.length > 0) {
		schema.interactionStatistic = video.interactionStatistic.map((stat) => ({
			'@type': 'InteractionCounter',
			interactionType: { '@type': stat.type },
			userInteractionCount: stat.count
		}));
	}

	// Add regions allowed
	if (video.regionsAllowed && video.regionsAllowed.length > 0) {
		schema.regionsAllowed = video.regionsAllowed.join(' ');
	}

	// Add transcript
	if (video.transcript) {
		schema.transcript = video.transcript;
	}

	// Add author
	if (video.author) {
		schema.author = {
			'@type': 'Person',
			name: video.author.name
		};
		if (video.author.url) {
			schema.author.url = video.author.url;
		}
	}

	// Add publisher
	if (opts.includePublisher) {
		const pub = video.publisher || opts.defaultPublisher;
		if (pub) {
			schema.publisher = {
				'@type': 'Organization',
				name: pub.name,
				logo: {
					'@type': 'ImageObject',
					url: pub.logo
				},
				url: pub.url
			};
		}
	}

	// Add live broadcast event
	if (video.publication?.isLiveBroadcast) {
		schema.publication = {
			'@type': 'BroadcastEvent',
			isLiveBroadcast: true
		};
		if (video.publication.startDate) {
			schema.publication.startDate = video.publication.startDate;
		}
		if (video.publication.endDate) {
			schema.publication.endDate = video.publication.endDate;
		}
	}

	// Add clips (key moments)
	if (video.hasPart && video.hasPart.length > 0) {
		schema.hasPart = video.hasPart.map((clip) => ({
			'@type': 'Clip',
			name: clip.name,
			startOffset: clip.startOffset,
			endOffset: clip.endOffset,
			url: clip.url || `${video.url}?t=${clip.startOffset}`
		}));
	}

	// Add SeekToAction for video scrubbing
	if (opts.includeSeekToAction && video.embedUrl) {
		schema.potentialAction = {
			'@type': 'SeekToAction',
			target: `${video.embedUrl}?t={seek_to_second_number}`,
			'startOffset-input': 'required name=seek_to_second_number'
		};
	}

	return schema;
}

/**
 * Generate VideoObject schema from detected video
 */
export function generateSchemaFromDetected(
	detected: DetectedVideo,
	metadata: {
		title: string;
		description: string;
		uploadDate: string;
		durationSeconds?: number;
	},
	options?: Partial<VideoSchemaOptions>
): Record<string, any> {
	return generateVideoSchema(
		{
			url: detected.embedUrl,
			embedUrl: detected.embedUrl,
			title: metadata.title,
			description: metadata.description,
			thumbnailUrl: detected.thumbnailUrl,
			uploadDate: metadata.uploadDate,
			durationSeconds: metadata.durationSeconds
		},
		options
	);
}

/**
 * Generate Course/Episode schema for video courses
 */
export function generateCourseVideoSchema(video: VideoInput): Record<string, any> {
	if (!video.courseInfo) {
		return generateVideoSchema(video);
	}

	const videoSchema = generateVideoSchema(video);

	return {
		'@context': 'https://schema.org',
		'@type': 'Course',
		name: video.courseInfo.name,
		description: video.courseInfo.description,
		provider: {
			'@type': 'Organization',
			name: video.courseInfo.provider
		},
		hasPart: {
			'@type': 'Episode',
			name: video.title,
			description: video.description,
			video: videoSchema,
			...(video.courseInfo.episodeNumber && { episodeNumber: video.courseInfo.episodeNumber }),
			...(video.courseInfo.seasonNumber && {
				partOfSeason: {
					'@type': 'CreativeWorkSeason',
					seasonNumber: video.courseInfo.seasonNumber
				}
			})
		}
	};
}

/**
 * Generate HowTo schema for tutorial videos
 */
export function generateHowToVideoSchema(video: VideoInput): Record<string, any> {
	if (!video.howToSteps || video.howToSteps.length === 0) {
		return generateVideoSchema(video);
	}

	const videoSchema = generateVideoSchema(video);

	return {
		'@context': 'https://schema.org',
		'@type': 'HowTo',
		name: video.title,
		description: video.description,
		video: videoSchema,
		step: video.howToSteps.map((step, index) => ({
			'@type': 'HowToStep',
			position: index + 1,
			name: step.name,
			text: step.text,
			video: {
				'@type': 'VideoObject',
				name: step.name,
				startOffset: step.startOffset,
				endOffset: step.endOffset,
				url: `${video.url}?t=${step.startOffset}`
			}
		}))
	};
}

/**
 * Generate ItemList schema for video galleries
 */
export function generateVideoGallerySchema(
	videos: VideoInput[],
	listName: string
): Record<string, any> {
	return {
		'@context': 'https://schema.org',
		'@type': 'ItemList',
		name: listName,
		numberOfItems: videos.length,
		itemListElement: videos.map((video, index) => ({
			'@type': 'ListItem',
			position: index + 1,
			item: generateVideoSchema(video)
		}))
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// HTML Generation
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Generate JSON-LD script tag
 */
export function generateJsonLdScript(schema: Record<string, any>): string {
	return `<script type="application/ld+json">\n${JSON.stringify(schema, null, 2)}\n</script>`;
}

/**
 * Generate multiple JSON-LD scripts
 */
export function generateMultipleJsonLdScripts(schemas: Record<string, any>[]): string {
	return schemas.map((schema) => generateJsonLdScript(schema)).join('\n');
}

// ═══════════════════════════════════════════════════════════════════════════
// Validation
// ═══════════════════════════════════════════════════════════════════════════

export interface ValidationResult {
	valid: boolean;
	errors: string[];
	warnings: string[];
}

/**
 * Validate video schema for Google requirements
 */
export function validateVideoSchema(schema: Record<string, any>): ValidationResult {
	const errors: string[] = [];
	const warnings: string[] = [];

	// Required fields
	if (!schema.name) errors.push('Missing required field: name');
	if (!schema.description) errors.push('Missing required field: description');
	if (!schema.thumbnailUrl) errors.push('Missing required field: thumbnailUrl');
	if (!schema.uploadDate) errors.push('Missing required field: uploadDate');
	if (!schema.contentUrl && !schema.embedUrl) {
		errors.push('Missing required field: contentUrl or embedUrl');
	}

	// Recommended fields
	if (!schema.duration) warnings.push('Missing recommended field: duration');
	if (!schema.interactionStatistic) {
		warnings.push('Consider adding interactionStatistic for better visibility');
	}

	// Description length
	if (schema.description && schema.description.length < 50) {
		warnings.push('Description is short. Consider adding more detail (50+ characters recommended)');
	}

	// Thumbnail validation
	if (schema.thumbnailUrl) {
		const thumbnails = Array.isArray(schema.thumbnailUrl)
			? schema.thumbnailUrl
			: [schema.thumbnailUrl];

		if (thumbnails.length < 3) {
			warnings.push('Consider providing multiple thumbnail sizes for better display');
		}
	}

	return {
		valid: errors.length === 0,
		errors,
		warnings
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// Export Default
// ═══════════════════════════════════════════════════════════════════════════

export default {
	detectVideoPlatform,
	findVideosInContent,
	secondsToIsoDuration,
	isoDurationToSeconds,
	generateVideoSchema,
	generateSchemaFromDetected,
	generateCourseVideoSchema,
	generateHowToVideoSchema,
	generateVideoGallerySchema,
	generateJsonLdScript,
	generateMultipleJsonLdScripts,
	validateVideoSchema
};

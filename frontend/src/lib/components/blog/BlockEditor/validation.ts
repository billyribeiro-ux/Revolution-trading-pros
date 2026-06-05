/**
 * Revolution Trading Pros - Block Editor Validation System
 * ═══════════════════════════════════════════════════════════════════════════
 * Comprehensive JSON Schema validation using Valibot for 30+ block types
 * Includes XSS protection, URL security, and content sanitization
 *
 * @version 4.0.0 Enterprise
 * @author Revolution Trading Pros
 */

import * as v from 'valibot';
import type { Block, BlockType, BlockContent, BlockItem } from './types';

// =============================================================================
// Constants - Max Lengths and Security Constraints
// =============================================================================

export const MAX_LENGTHS = {
	// Text fields
	TEXT_SHORT: 256,
	TEXT_MEDIUM: 1024,
	TEXT_LONG: 5000,
	TEXT_CONTENT: 50000,
	HTML_CONTENT: 100000,
	CODE_CONTENT: 500000,

	// URLs
	URL: 2048,

	// Identifiers
	ID: 128,
	ANCHOR: 64,
	CLASS_NAME: 256,
	CUSTOM_CSS: 10000,

	// Lists
	LIST_ITEMS: 100,
	CHILDREN_BLOCKS: 50,
	GALLERY_ITEMS: 100,

	// Special
	TICKER_SYMBOL: 10,
	LANGUAGE_CODE: 20,
	CSS_VALUE: 128,
	AI_PROMPT: 2000,
	AI_OUTPUT: 100000
} as const;

// =============================================================================
// Security Patterns
// =============================================================================

// Dangerous URL protocols
const DANGEROUS_PROTOCOLS = ['javascript:', 'data:', 'vbscript:', 'file:'];

// XSS patterns to detect
const XSS_PATTERNS = [
	/<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>/gi,
	/javascript:/gi,
	/on\w+\s*=/gi,
	/expression\s*\(/gi,
	/url\s*\(\s*['"]?\s*javascript:/gi,
	/<iframe\b/gi,
	/<object\b/gi,
	/<embed\b/gi,
	/<applet\b/gi,
	/<meta\b[^>]*http-equiv/gi,
	/<link\b[^>]*rel\s*=\s*['"]?import/gi,
	/<base\b/gi,
	/data:\s*text\/html/gi,
	/<!--[\s\S]*?-->/g
];

// Allowed embed types
const ALLOWED_EMBED_TYPES = [
	'youtube',
	'vimeo',
	'twitter',
	'instagram',
	'tiktok',
	'soundcloud',
	'spotify',
	'custom'
] as const;

// =============================================================================
// Security Utility Functions
// =============================================================================

/**
 * Checks if a URL is safe (no dangerous protocols)
 */
export function isSecureUrl(url: string): boolean {
	if (!url || typeof url !== 'string') return false;
	const normalizedUrl = url.toLowerCase().trim();
	return !DANGEROUS_PROTOCOLS.some((protocol) => normalizedUrl.startsWith(protocol));
}

/**
 * Detects potential XSS patterns in HTML content
 */
export function containsXSS(html: string): boolean {
	if (!html || typeof html !== 'string') return false;
	return XSS_PATTERNS.some((pattern) => pattern.test(html));
}

/**
 * Sanitizes HTML by removing dangerous patterns
 * Note: For production, use DOMPurify for comprehensive sanitization
 */
export function sanitizeHtml(html: string): string {
	if (!html || typeof html !== 'string') return '';

	let sanitized = html;

	// Remove script tags
	sanitized = sanitized.replace(/<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>/gi, '');

	// Remove event handlers
	sanitized = sanitized.replace(/\s+on\w+\s*=\s*["'][^"']*["']/gi, '');
	sanitized = sanitized.replace(/\s+on\w+\s*=\s*[^\s>]+/gi, '');

	// Remove javascript: URLs
	sanitized = sanitized.replace(/javascript:[^\s"']*/gi, '');

	// Remove dangerous tags
	sanitized = sanitized.replace(/<(iframe|object|embed|applet|base|meta)[^>]*>/gi, '');

	// Remove HTML comments (can hide malicious content)
	sanitized = sanitized.replace(/<!--[\s\S]*?-->/g, '');

	return sanitized;
}

/**
 * Sanitizes a URL by removing dangerous protocols
 */
export function sanitizeUrl(url: string): string {
	if (!url || typeof url !== 'string') return '';
	const normalizedUrl = url.toLowerCase().trim();

	for (const protocol of DANGEROUS_PROTOCOLS) {
		if (normalizedUrl.startsWith(protocol)) {
			return '';
		}
	}

	return url.trim();
}

// =============================================================================
// Custom Valibot Refinements
// =============================================================================

const secureUrlSchema = v.pipe(
	v.string(),
	v.maxLength(MAX_LENGTHS.URL),
	v.check((url) => !url || isSecureUrl(url), 'URL contains potentially dangerous protocol')
);

const safeHtmlSchema = v.pipe(
	v.string(),
	v.maxLength(MAX_LENGTHS.HTML_CONTENT),
	v.check(
		(html) => !html || !containsXSS(html),
		'HTML content contains potentially dangerous XSS patterns'
	)
);

const cssValueSchema = v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.CSS_VALUE)));

// =============================================================================
// Base Schema Components
// =============================================================================

// Block ID Schema
const blockIdSchema = v.pipe(v.string(), v.minLength(1), v.maxLength(MAX_LENGTHS.ID));

// Animation Settings Schema
const animationSettingsSchema = v.optional(
	v.object({
		type: v.picklist(['none', 'fade', 'slide', 'zoom', 'bounce', 'flip', 'rotate']),
		direction: v.optional(v.picklist(['up', 'down', 'left', 'right'])),
		duration: v.optional(v.pipe(v.number(), v.minValue(0), v.maxValue(10000))),
		delay: v.optional(v.pipe(v.number(), v.minValue(0), v.maxValue(10000))),
		easing: v.optional(v.pipe(v.string(), v.maxLength(64))),
		triggerOnScroll: v.optional(v.boolean()),
		repeat: v.optional(v.boolean())
	})
);

// Block Comment Schema - using a simpler approach for recursion
const baseBlockCommentSchema = v.object({
	id: blockIdSchema,
	userId: v.pipe(v.string(), v.maxLength(MAX_LENGTHS.ID)),
	userName: v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_SHORT)),
	userAvatar: v.optional(secureUrlSchema),
	content: v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_LONG)),
	createdAt: v.pipe(v.string(), v.maxLength(64)),
	resolved: v.optional(v.boolean())
});

// For nested replies, we use a simpler flat structure to avoid deep recursion
const blockCommentSchema = v.object({
	...baseBlockCommentSchema.entries,
	replies: v.optional(v.pipe(v.array(baseBlockCommentSchema), v.maxLength(50)))
});

// Block Metadata Schema
const blockMetadataSchema = v.object({
	createdAt: v.union([v.pipe(v.string(), v.isoTimestamp()), v.pipe(v.string(), v.maxLength(64))]),
	updatedAt: v.union([v.pipe(v.string(), v.isoTimestamp()), v.pipe(v.string(), v.maxLength(64))]),
	createdBy: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.ID))),
	version: v.pipe(v.number(), v.integer(), v.minValue(0)),
	locked: v.optional(v.boolean()),
	lockedBy: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.ID))),
	comments: v.optional(v.pipe(v.array(blockCommentSchema), v.maxLength(100))),
	aiGenerated: v.optional(v.boolean())
});

// Block Item Schema (for lists, accordions, tabs, etc.)
const blockItemSchema: v.GenericSchema<BlockItem> = v.lazy(() =>
	v.object({
		id: blockIdSchema,
		content: v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_LONG)),
		checked: v.optional(v.boolean()),
		collapsed: v.optional(v.boolean()),
		children: v.optional(v.pipe(v.array(blockItemSchema), v.maxLength(MAX_LENGTHS.LIST_ITEMS)))
	})
);

// Focal Point Schema
const focalPointSchema = v.optional(
	v.object({
		x: v.pipe(v.number(), v.minValue(0), v.maxValue(1)),
		y: v.pipe(v.number(), v.minValue(0), v.maxValue(1))
	})
);

// =============================================================================
// Block Settings Schema
// =============================================================================

const blockSettingsSchema = v.object({
	// Typography
	textAlign: v.optional(v.picklist(['left', 'center', 'right', 'justify'])),
	fontSize: cssValueSchema,
	fontWeight: cssValueSchema,
	fontFamily: v.optional(v.pipe(v.string(), v.maxLength(256))),
	lineHeight: cssValueSchema,
	letterSpacing: cssValueSchema,
	textColor: cssValueSchema,
	textTransform: v.optional(v.picklist(['none', 'uppercase', 'lowercase', 'capitalize'])),

	// Heading specific
	level: v.optional(
		v.union([v.literal(1), v.literal(2), v.literal(3), v.literal(4), v.literal(5), v.literal(6)])
	),
	anchor: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.ANCHOR))),

	// Spacing - shorthand
	padding: cssValueSchema,
	margin: cssValueSchema,

	// Spacing - individual
	marginTop: cssValueSchema,
	marginBottom: cssValueSchema,
	marginLeft: cssValueSchema,
	marginRight: cssValueSchema,
	paddingTop: cssValueSchema,
	paddingBottom: cssValueSchema,
	paddingLeft: cssValueSchema,
	paddingRight: cssValueSchema,

	// Background
	backgroundColor: cssValueSchema,
	backgroundImage: v.optional(secureUrlSchema),
	backgroundGradient: v.optional(v.pipe(v.string(), v.maxLength(512))),
	backgroundOverlay: cssValueSchema,

	// Border
	borderWidth: cssValueSchema,
	borderColor: cssValueSchema,
	borderStyle: v.optional(v.picklist(['solid', 'dashed', 'dotted', 'none'])),
	borderRadius: cssValueSchema,

	// Shadow
	boxShadow: v.optional(v.pipe(v.string(), v.maxLength(256))),

	// Animation
	animation: animationSettingsSchema,
	animationDuration: cssValueSchema,
	animationDelay: cssValueSchema,

	// Transform
	rotate: cssValueSchema,
	scale: cssValueSchema,
	translateX: cssValueSchema,
	translateY: cssValueSchema,
	skewX: cssValueSchema,
	skewY: cssValueSchema,

	// Visual Effects
	opacity: v.optional(v.pipe(v.number(), v.minValue(0), v.maxValue(1))),
	blendMode: v.optional(
		v.picklist([
			'normal',
			'multiply',
			'screen',
			'overlay',
			'darken',
			'lighten',
			'color-dodge',
			'color-burn',
			'hard-light',
			'soft-light',
			'difference',
			'exclusion',
			'hue',
			'saturation',
			'color',
			'luminosity'
		])
	),

	// Filters
	filterBlur: cssValueSchema,
	filterBrightness: cssValueSchema,
	filterContrast: cssValueSchema,
	filterGrayscale: cssValueSchema,
	filterSaturate: cssValueSchema,

	// Layout
	width: cssValueSchema,
	maxWidth: cssValueSchema,
	height: cssValueSchema,
	display: v.optional(v.picklist(['block', 'flex', 'grid', 'inline', 'none'])),
	flexDirection: v.optional(v.picklist(['row', 'column'])),
	justifyContent: cssValueSchema,
	alignItems: cssValueSchema,
	gap: cssValueSchema,
	columnCount: v.optional(v.pipe(v.number(), v.integer(), v.minValue(1), v.maxValue(12))),
	columnLayout: v.optional(v.pipe(v.string(), v.maxLength(64))),

	// Image specific
	imageSize: v.optional(v.picklist(['thumbnail', 'medium', 'large', 'full', 'custom'])),
	imageWidth: v.optional(v.pipe(v.number(), v.integer(), v.minValue(1), v.maxValue(10000))),
	imageHeight: v.optional(v.pipe(v.number(), v.integer(), v.minValue(1), v.maxValue(10000))),
	objectFit: v.optional(v.picklist(['cover', 'contain', 'fill', 'none'])),
	focalPoint: focalPointSchema,
	imageFilter: v.optional(v.pipe(v.string(), v.maxLength(256))),

	// Link
	linkUrl: v.optional(secureUrlSchema),
	linkTarget: v.optional(v.picklist(['_blank', '_self'])),
	linkRel: v.optional(v.pipe(v.string(), v.maxLength(64))),

	// Button specific
	buttonStyle: v.optional(v.picklist(['primary', 'secondary', 'outline', 'ghost', 'link'])),
	buttonSize: v.optional(v.picklist(['small', 'medium', 'large'])),
	buttonIcon: v.optional(v.pipe(v.string(), v.maxLength(64))),
	buttonIconPosition: v.optional(v.picklist(['left', 'right'])),

	// Responsive visibility
	hideOnDesktop: v.optional(v.boolean()),
	hideOnTablet: v.optional(v.boolean()),
	hideOnMobile: v.optional(v.boolean()),

	// Responsive typography
	tabletFontSize: cssValueSchema,
	mobileFontSize: cssValueSchema,

	// Responsive spacing
	tabletPadding: cssValueSchema,
	mobilePadding: cssValueSchema,

	// Advanced
	customCSS: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.CUSTOM_CSS))),
	customId: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.ID))),
	customClass: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.CLASS_NAME))),
	ariaLabel: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_SHORT))),
	position: v.optional(v.picklist(['static', 'relative', 'absolute', 'fixed', 'sticky'])),
	zIndex: v.optional(v.pipe(v.number(), v.integer(), v.minValue(-9999), v.maxValue(9999))),
	positionTop: cssValueSchema,
	positionRight: cssValueSchema,
	positionBottom: cssValueSchema,
	positionLeft: cssValueSchema
});

// =============================================================================
// Block Type Enumeration
// =============================================================================

export const ALL_BLOCK_TYPES = [
	// Text Blocks
	'paragraph',
	'heading',
	'quote',
	'pullquote',
	'code',
	'preformatted',
	'list',
	'checklist',
	// Media Blocks
	'image',
	'gallery',
	'video',
	'audio',
	'file',
	'embed',
	'gif',
	// Layout Blocks
	'columns',
	'group',
	'separator',
	'divider',
	'spacer',
	'row',
	// Interactive Blocks
	'button',
	'buttons',
	'accordion',
	'tabs',
	'toggle',
	'toc',
	// Trading-Specific Blocks
	'ticker',
	'chart',
	'priceAlert',
	'tradingIdea',
	'riskDisclaimer',
	// Advanced Blocks
	'callout',
	'card',
	'testimonial',
	'cta',
	'countdown',
	'socialShare',
	'author',
	'relatedPosts',
	'newsletter',
	// AI-Powered Blocks
	'aiGenerated',
	'aiSummary',
	'aiTranslation',
	// Custom/Dynamic
	'shortcode',
	'html',
	'reusable'
] as const;

export const blockTypeSchema = v.picklist(ALL_BLOCK_TYPES);

// =============================================================================
// Block Content Schemas - Type-Specific
// =============================================================================

// Forward declaration for recursive block schema
type BlockSchema = v.GenericSchema<Block>;

// Create a lazy block schema for nested blocks
const createBlockSchema = (): BlockSchema =>
	v.lazy(() =>
		v.object({
			id: blockIdSchema,
			type: blockTypeSchema,
			content: blockContentSchema,
			settings: blockSettingsSchema,
			metadata: blockMetadataSchema
		})
	) as BlockSchema;

// Paragraph Block Content
const paragraphContentSchema = v.object({
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_CONTENT))),
	html: v.optional(safeHtmlSchema)
});

// Heading Block Content
const headingContentSchema = v.object({
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_MEDIUM))),
	html: v.optional(safeHtmlSchema)
});

// Quote Block Content
const quoteContentSchema = v.object({
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_LONG))),
	html: v.optional(safeHtmlSchema)
});

// Pullquote Block Content
const pullquoteContentSchema = v.object({
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_LONG))),
	html: v.optional(safeHtmlSchema)
});

// Code Block Content
const codeContentSchema = v.object({
	code: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.CODE_CONTENT))),
	language: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.LANGUAGE_CODE)))
});

// Preformatted Block Content
const preformattedContentSchema = v.object({
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.CODE_CONTENT)))
});

// List Block Content
const listContentSchema = v.object({
	listType: v.optional(v.picklist(['bullet', 'number', 'check'])),
	listItems: v.optional(
		v.pipe(
			v.array(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_LONG))),
			v.maxLength(MAX_LENGTHS.LIST_ITEMS)
		)
	),
	items: v.optional(v.pipe(v.array(blockItemSchema), v.maxLength(MAX_LENGTHS.LIST_ITEMS)))
});

// Checklist Block Content
const checklistContentSchema = v.object({
	items: v.optional(v.pipe(v.array(blockItemSchema), v.maxLength(MAX_LENGTHS.LIST_ITEMS)))
});

// Image Block Content
const imageContentSchema = v.object({
	mediaId: v.optional(v.pipe(v.number(), v.integer(), v.gtValue(0))),
	mediaUrl: v.optional(secureUrlSchema),
	mediaAlt: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_SHORT))),
	mediaCaption: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_MEDIUM)))
});

// Gallery Block Content
const galleryContentSchema = v.object({
	items: v.optional(
		v.pipe(
			v.array(
				v.object({
					id: blockIdSchema,
					content: v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_MEDIUM)),
					mediaId: v.optional(v.pipe(v.number(), v.integer(), v.gtValue(0))),
					mediaUrl: v.optional(secureUrlSchema),
					mediaAlt: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_SHORT)))
				})
			),
			v.maxLength(MAX_LENGTHS.GALLERY_ITEMS)
		)
	)
});

// Video Block Content
const videoContentSchema = v.object({
	mediaId: v.optional(v.pipe(v.number(), v.integer(), v.gtValue(0))),
	mediaUrl: v.optional(secureUrlSchema),
	mediaCaption: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_MEDIUM))),
	embedUrl: v.optional(secureUrlSchema),
	embedType: v.optional(v.picklist(ALLOWED_EMBED_TYPES))
});

// Audio Block Content
const audioContentSchema = v.object({
	mediaId: v.optional(v.pipe(v.number(), v.integer(), v.gtValue(0))),
	mediaUrl: v.optional(secureUrlSchema),
	mediaCaption: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_MEDIUM)))
});

// File Block Content
const fileContentSchema = v.object({
	mediaId: v.optional(v.pipe(v.number(), v.integer(), v.gtValue(0))),
	mediaUrl: v.optional(secureUrlSchema),
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_SHORT))) // filename display
});

// Embed Block Content
const embedContentSchema = v.object({
	embedUrl: v.optional(secureUrlSchema),
	embedType: v.optional(v.picklist(ALLOWED_EMBED_TYPES)),
	html: v.optional(safeHtmlSchema)
});

// GIF Block Content
const gifContentSchema = v.object({
	mediaUrl: v.optional(secureUrlSchema),
	mediaAlt: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_SHORT)))
});

// Columns Block Content
const columnsContentSchema = v.object({
	columnCount: v.optional(v.pipe(v.number(), v.integer(), v.minValue(1), v.maxValue(12))),
	columnLayout: v.optional(v.pipe(v.string(), v.maxLength(64))),
	children: v.optional(
		v.pipe(v.array(createBlockSchema()), v.maxLength(MAX_LENGTHS.CHILDREN_BLOCKS))
	)
});

// Group Block Content
const groupContentSchema = v.object({
	children: v.optional(
		v.pipe(v.array(createBlockSchema()), v.maxLength(MAX_LENGTHS.CHILDREN_BLOCKS))
	)
});

// Separator Block Content
const separatorContentSchema = v.object({});

// Spacer Block Content
const spacerContentSchema = v.object({});

// Row Block Content
const rowContentSchema = v.object({
	children: v.optional(
		v.pipe(v.array(createBlockSchema()), v.maxLength(MAX_LENGTHS.CHILDREN_BLOCKS))
	)
});

// Button Block Content
const buttonContentSchema = v.object({
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_SHORT))),
	html: v.optional(safeHtmlSchema)
});

// Buttons Block Content
const buttonsContentSchema = v.object({
	children: v.optional(v.pipe(v.array(createBlockSchema()), v.maxLength(20)))
});

// Accordion Block Content
const accordionContentSchema = v.object({
	items: v.optional(v.pipe(v.array(blockItemSchema), v.maxLength(50))),
	children: v.optional(
		v.pipe(v.array(createBlockSchema()), v.maxLength(MAX_LENGTHS.CHILDREN_BLOCKS))
	)
});

// Tabs Block Content
const tabsContentSchema = v.object({
	items: v.optional(v.pipe(v.array(blockItemSchema), v.maxLength(20))),
	children: v.optional(
		v.pipe(v.array(createBlockSchema()), v.maxLength(MAX_LENGTHS.CHILDREN_BLOCKS))
	)
});

// Toggle Block Content
const toggleContentSchema = v.object({
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_SHORT))),
	children: v.optional(
		v.pipe(v.array(createBlockSchema()), v.maxLength(MAX_LENGTHS.CHILDREN_BLOCKS))
	)
});

// Table of Contents Block Content
const tocContentSchema = v.object({});

// Ticker Block Content
const tickerContentSchema = v.object({
	ticker: v.optional(
		v.pipe(
			v.string(),
			v.maxLength(MAX_LENGTHS.TICKER_SYMBOL),
			v.regex(/^[A-Z0-9.^=-]+$/i, 'Invalid ticker symbol format')
		)
	)
});

// Chart Block Content
const chartContentSchema = v.object({
	ticker: v.optional(
		v.pipe(
			v.string(),
			v.maxLength(MAX_LENGTHS.TICKER_SYMBOL),
			v.regex(/^[A-Z0-9.^=-]+$/i, 'Invalid ticker symbol format')
		)
	),
	chartType: v.optional(v.picklist(['line', 'candle', 'bar']))
});

// Price Alert Block Content
const priceAlertContentSchema = v.object({
	ticker: v.optional(
		v.pipe(
			v.string(),
			v.maxLength(MAX_LENGTHS.TICKER_SYMBOL),
			v.regex(/^[A-Z0-9.^=-]+$/i, 'Invalid ticker symbol format')
		)
	),
	priceTarget: v.optional(v.pipe(v.number(), v.gtValue(0))),
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_MEDIUM)))
});

// Trading Idea Block Content
const tradingIdeaContentSchema = v.object({
	ticker: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TICKER_SYMBOL))),
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_LONG))),
	html: v.optional(safeHtmlSchema),
	priceTarget: v.optional(v.number()),
	chartType: v.optional(v.picklist(['line', 'candle', 'bar']))
});

// Risk Disclaimer Block Content
const riskDisclaimerContentSchema = v.object({
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_LONG))),
	html: v.optional(safeHtmlSchema)
});

// Callout Block Content
const calloutContentSchema = v.object({
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_LONG))),
	html: v.optional(safeHtmlSchema)
});

// Card Block Content
const cardContentSchema = v.object({
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_LONG))),
	html: v.optional(safeHtmlSchema),
	mediaId: v.optional(v.pipe(v.number(), v.integer(), v.gtValue(0))),
	mediaUrl: v.optional(secureUrlSchema),
	mediaAlt: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_SHORT))),
	children: v.optional(
		v.pipe(v.array(createBlockSchema()), v.maxLength(MAX_LENGTHS.CHILDREN_BLOCKS))
	)
});

// Testimonial Block Content
const testimonialContentSchema = v.object({
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_LONG))),
	html: v.optional(safeHtmlSchema),
	mediaUrl: v.optional(secureUrlSchema) // avatar
});

// CTA Block Content
const ctaContentSchema = v.object({
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_LONG))),
	html: v.optional(safeHtmlSchema),
	children: v.optional(
		v.pipe(v.array(createBlockSchema()), v.maxLength(MAX_LENGTHS.CHILDREN_BLOCKS))
	)
});

// Countdown Block Content
const countdownContentSchema = v.object({
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_SHORT))) // target date
});

// Social Share Block Content
const socialShareContentSchema = v.object({});

// Author Block Content
const authorContentSchema = v.object({
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_LONG))), // bio
	html: v.optional(safeHtmlSchema),
	mediaUrl: v.optional(secureUrlSchema) // avatar
});

// Related Posts Block Content
const relatedPostsContentSchema = v.object({
	items: v.optional(
		v.pipe(
			v.array(
				v.object({
					id: blockIdSchema,
					content: v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_MEDIUM))
				})
			),
			v.maxLength(20)
		)
	)
});

// Newsletter Block Content
const newsletterContentSchema = v.object({
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_MEDIUM))),
	html: v.optional(safeHtmlSchema)
});

// AI Generated Block Content
const aiGeneratedContentSchema = v.object({
	aiPrompt: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.AI_PROMPT))),
	aiModel: v.optional(v.pipe(v.string(), v.maxLength(64))),
	aiOutput: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.AI_OUTPUT))),
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_CONTENT))),
	html: v.optional(safeHtmlSchema)
});

// AI Summary Block Content
const aiSummaryContentSchema = v.object({
	aiPrompt: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.AI_PROMPT))),
	aiModel: v.optional(v.pipe(v.string(), v.maxLength(64))),
	aiOutput: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_LONG))),
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_LONG)))
});

// AI Translation Block Content
const aiTranslationContentSchema = v.object({
	aiPrompt: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.AI_PROMPT))),
	aiModel: v.optional(v.pipe(v.string(), v.maxLength(64))),
	aiOutput: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.AI_OUTPUT))),
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_CONTENT))),
	language: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.LANGUAGE_CODE)))
});

// Shortcode Block Content
const shortcodeContentSchema = v.object({
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_MEDIUM))),
	code: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_MEDIUM)))
});

// HTML Block Content - with strict XSS validation
const htmlBlockContentSchema = v.object({
	html: v.optional(safeHtmlSchema)
});

// Reusable Block Content
const reusableContentSchema = v.object({
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.ID))), // reference ID
	children: v.optional(
		v.pipe(v.array(createBlockSchema()), v.maxLength(MAX_LENGTHS.CHILDREN_BLOCKS))
	)
});

// =============================================================================
// Combined Block Content Schema
// =============================================================================

const blockContentSchema = v.object({
	// Text content
	text: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_CONTENT))),
	html: v.optional(safeHtmlSchema),

	// Media content
	mediaId: v.optional(v.pipe(v.number(), v.integer(), v.gtValue(0))),
	mediaUrl: v.optional(secureUrlSchema),
	mediaAlt: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_SHORT))),
	mediaCaption: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_MEDIUM))),

	// Nested content
	children: v.optional(
		v.pipe(v.array(createBlockSchema()), v.maxLength(MAX_LENGTHS.CHILDREN_BLOCKS))
	),
	items: v.optional(v.pipe(v.array(blockItemSchema), v.maxLength(MAX_LENGTHS.LIST_ITEMS))),

	// Special content
	embedUrl: v.optional(secureUrlSchema),
	embedType: v.optional(v.picklist(ALLOWED_EMBED_TYPES)),

	// Code content
	code: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.CODE_CONTENT))),
	language: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.LANGUAGE_CODE))),

	// List content
	listType: v.optional(v.picklist(['bullet', 'number', 'check'])),
	listItems: v.optional(
		v.pipe(
			v.array(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TEXT_LONG))),
			v.maxLength(MAX_LENGTHS.LIST_ITEMS)
		)
	),

	// Columns/Layout
	columnCount: v.optional(v.pipe(v.number(), v.integer(), v.minValue(1), v.maxValue(12))),
	columnLayout: v.optional(v.pipe(v.string(), v.maxLength(64))),

	// Trading-specific
	ticker: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.TICKER_SYMBOL))),
	chartType: v.optional(v.picklist(['line', 'candle', 'bar'])),
	priceTarget: v.optional(v.number()),

	// AI content
	aiPrompt: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.AI_PROMPT))),
	aiModel: v.optional(v.pipe(v.string(), v.maxLength(64))),
	aiOutput: v.optional(v.pipe(v.string(), v.maxLength(MAX_LENGTHS.AI_OUTPUT)))
});

// =============================================================================
// Complete Block Schema
// =============================================================================

export const blockSchema = v.object({
	id: blockIdSchema,
	type: blockTypeSchema,
	content: blockContentSchema,
	settings: blockSettingsSchema,
	metadata: blockMetadataSchema
});

// =============================================================================
// Type-Specific Block Schemas with Content Validation
// =============================================================================

const createTypeSpecificBlockSchema = <T extends v.GenericSchema>(
	type: BlockType,
	contentSchema: T
) => {
	return v.object({
		id: blockIdSchema,
		type: v.literal(type),
		content: contentSchema,
		settings: blockSettingsSchema,
		metadata: blockMetadataSchema
	});
};

// Export all type-specific schemas
export const blockSchemas = {
	// Text Blocks
	paragraph: createTypeSpecificBlockSchema('paragraph', paragraphContentSchema),
	heading: createTypeSpecificBlockSchema('heading', headingContentSchema),
	quote: createTypeSpecificBlockSchema('quote', quoteContentSchema),
	pullquote: createTypeSpecificBlockSchema('pullquote', pullquoteContentSchema),
	code: createTypeSpecificBlockSchema('code', codeContentSchema),
	preformatted: createTypeSpecificBlockSchema('preformatted', preformattedContentSchema),
	list: createTypeSpecificBlockSchema('list', listContentSchema),
	checklist: createTypeSpecificBlockSchema('checklist', checklistContentSchema),

	// Media Blocks
	image: createTypeSpecificBlockSchema('image', imageContentSchema),
	gallery: createTypeSpecificBlockSchema('gallery', galleryContentSchema),
	video: createTypeSpecificBlockSchema('video', videoContentSchema),
	audio: createTypeSpecificBlockSchema('audio', audioContentSchema),
	file: createTypeSpecificBlockSchema('file', fileContentSchema),
	embed: createTypeSpecificBlockSchema('embed', embedContentSchema),
	gif: createTypeSpecificBlockSchema('gif', gifContentSchema),

	// Layout Blocks
	columns: createTypeSpecificBlockSchema('columns', columnsContentSchema),
	group: createTypeSpecificBlockSchema('group', groupContentSchema),
	separator: createTypeSpecificBlockSchema('separator', separatorContentSchema),
	divider: createTypeSpecificBlockSchema('divider', separatorContentSchema),
	spacer: createTypeSpecificBlockSchema('spacer', spacerContentSchema),
	row: createTypeSpecificBlockSchema('row', rowContentSchema),

	// Interactive Blocks
	button: createTypeSpecificBlockSchema('button', buttonContentSchema),
	buttons: createTypeSpecificBlockSchema('buttons', buttonsContentSchema),
	accordion: createTypeSpecificBlockSchema('accordion', accordionContentSchema),
	tabs: createTypeSpecificBlockSchema('tabs', tabsContentSchema),
	toggle: createTypeSpecificBlockSchema('toggle', toggleContentSchema),
	toc: createTypeSpecificBlockSchema('toc', tocContentSchema),

	// Trading Blocks
	ticker: createTypeSpecificBlockSchema('ticker', tickerContentSchema),
	chart: createTypeSpecificBlockSchema('chart', chartContentSchema),
	priceAlert: createTypeSpecificBlockSchema('priceAlert', priceAlertContentSchema),
	tradingIdea: createTypeSpecificBlockSchema('tradingIdea', tradingIdeaContentSchema),
	riskDisclaimer: createTypeSpecificBlockSchema('riskDisclaimer', riskDisclaimerContentSchema),

	// Advanced Blocks
	callout: createTypeSpecificBlockSchema('callout', calloutContentSchema),
	card: createTypeSpecificBlockSchema('card', cardContentSchema),
	testimonial: createTypeSpecificBlockSchema('testimonial', testimonialContentSchema),
	cta: createTypeSpecificBlockSchema('cta', ctaContentSchema),
	countdown: createTypeSpecificBlockSchema('countdown', countdownContentSchema),
	socialShare: createTypeSpecificBlockSchema('socialShare', socialShareContentSchema),
	author: createTypeSpecificBlockSchema('author', authorContentSchema),
	relatedPosts: createTypeSpecificBlockSchema('relatedPosts', relatedPostsContentSchema),
	newsletter: createTypeSpecificBlockSchema('newsletter', newsletterContentSchema),

	// AI Blocks
	aiGenerated: createTypeSpecificBlockSchema('aiGenerated', aiGeneratedContentSchema),
	aiSummary: createTypeSpecificBlockSchema('aiSummary', aiSummaryContentSchema),
	aiTranslation: createTypeSpecificBlockSchema('aiTranslation', aiTranslationContentSchema),

	// Custom Blocks
	shortcode: createTypeSpecificBlockSchema('shortcode', shortcodeContentSchema),
	html: createTypeSpecificBlockSchema('html', htmlBlockContentSchema),
	reusable: createTypeSpecificBlockSchema('reusable', reusableContentSchema)
} as const;

// =============================================================================
// Validation Result Types
// =============================================================================

export interface ValidationError {
	path: PropertyKey[];
	message: string;
	code: string;
}

export interface ValidationResult {
	success: boolean;
	data?: Block;
	errors?: ValidationError[];
}

export interface ValidationResults {
	success: boolean;
	data?: Block[];
	results: ValidationResult[];
	totalErrors: number;
}

// Map a valibot issue to the local ValidationError shape (parity with the
// previous zod-based output: path as plain keys, message, and a `code`).
function toValidationError(issue: v.BaseIssue<unknown>): ValidationError {
	return {
		path: (issue.path ?? []).map((p) => (p as { key: PropertyKey }).key),
		message: issue.message,
		code: issue.type
	};
}

// =============================================================================
// Validation Functions
// =============================================================================

/**
 * Validates a single block against its type-specific schema
 */
export function validateBlock(block: unknown): ValidationResult {
	// First, validate basic block structure
	const basicResult = v.safeParse(blockSchema, block);

	if (!basicResult.success) {
		return {
			success: false,
			errors: basicResult.issues.map(toValidationError)
		};
	}

	// Then validate type-specific content
	const blockData = basicResult.output;
	const typeSchema = blockSchemas[blockData.type as keyof typeof blockSchemas];

	if (typeSchema) {
		const typeResult = v.safeParse(typeSchema, blockData);
		if (!typeResult.success) {
			return {
				success: false,
				errors: typeResult.issues.map(toValidationError)
			};
		}
	}

	return {
		success: true,
		data: blockData as Block
	};
}

/**
 * Validates multiple blocks
 */
export function validateBlocks(blocks: unknown[]): ValidationResults {
	if (!Array.isArray(blocks)) {
		return {
			success: false,
			results: [
				{
					success: false,
					errors: [
						{
							path: [],
							message: 'Expected an array of blocks',
							code: 'invalid_type'
						}
					]
				}
			],
			totalErrors: 1
		};
	}

	const results = blocks.map((block) => validateBlock(block));
	const totalErrors = results.reduce((sum, r) => sum + (r.errors?.length || 0), 0);

	return {
		success: results.every((r) => r.success),
		data: results.filter((r) => r.success).map((r) => r.data as Block),
		results,
		totalErrors
	};
}

/**
 * Type guard to check if a string is a valid block type
 */
export function isValidBlockType(type: string): type is BlockType {
	return (ALL_BLOCK_TYPES as readonly string[]).includes(type);
}

/**
 * Gets the schema for a specific block type
 */
export function getBlockSchema(type: BlockType) {
	return blockSchemas[type as keyof typeof blockSchemas];
}

// =============================================================================
// Sanitization Functions
// =============================================================================

/**
 * Sanitizes a block by removing dangerous content
 */
export function sanitizeBlock(block: Block): Block {
	const sanitized = { ...block };
	const content = { ...block.content };
	const settings = { ...block.settings };

	// Sanitize HTML content
	if (content.html) {
		content.html = sanitizeHtml(content.html);
	}

	// Sanitize URLs
	if (content.mediaUrl) {
		content.mediaUrl = sanitizeUrl(content.mediaUrl);
	}
	if (content.embedUrl) {
		content.embedUrl = sanitizeUrl(content.embedUrl);
	}

	// Sanitize settings URLs
	if (settings.linkUrl) {
		settings.linkUrl = sanitizeUrl(settings.linkUrl);
	}
	if (settings.backgroundImage) {
		settings.backgroundImage = sanitizeUrl(settings.backgroundImage);
	}

	// Sanitize nested children
	if (content.children) {
		content.children = content.children.map((child) => sanitizeBlock(child));
	}

	// Sanitize items with content
	if (content.items) {
		content.items = content.items.map((item) => ({
			...item,
			content: sanitizeHtml(item.content),
			children: item.children
				? item.children.map((child) => ({
						...child,
						content: sanitizeHtml(child.content)
					}))
				: undefined
		}));
	}

	// Truncate fields that exceed max lengths
	if (content.text && content.text.length > MAX_LENGTHS.TEXT_CONTENT) {
		content.text = content.text.slice(0, MAX_LENGTHS.TEXT_CONTENT);
	}
	if (content.code && content.code.length > MAX_LENGTHS.CODE_CONTENT) {
		content.code = content.code.slice(0, MAX_LENGTHS.CODE_CONTENT);
	}
	if (content.aiPrompt && content.aiPrompt.length > MAX_LENGTHS.AI_PROMPT) {
		content.aiPrompt = content.aiPrompt.slice(0, MAX_LENGTHS.AI_PROMPT);
	}
	if (content.aiOutput && content.aiOutput.length > MAX_LENGTHS.AI_OUTPUT) {
		content.aiOutput = content.aiOutput.slice(0, MAX_LENGTHS.AI_OUTPUT);
	}

	// Sanitize custom CSS (remove dangerous patterns)
	if (settings.customCSS) {
		settings.customCSS = sanitizeCSS(settings.customCSS);
	}

	sanitized.content = content;
	sanitized.settings = settings;

	return sanitized;
}

/**
 * Sanitizes CSS to remove dangerous patterns
 */
export function sanitizeCSS(css: string): string {
	if (!css) return '';

	let sanitized = css;

	// Remove expression() which can execute JavaScript
	sanitized = sanitized.replace(/expression\s*\([^)]*\)/gi, '');

	// Remove url() with javascript:
	sanitized = sanitized.replace(/url\s*\(\s*['"]?\s*javascript:[^)]*\)/gi, '');

	// Remove behavior: which can load HTCs
	sanitized = sanitized.replace(/behavior\s*:\s*[^;]*/gi, '');

	// Remove -moz-binding which can load XBL
	sanitized = sanitized.replace(/-moz-binding\s*:\s*[^;]*/gi, '');

	// Truncate to max length
	if (sanitized.length > MAX_LENGTHS.CUSTOM_CSS) {
		sanitized = sanitized.slice(0, MAX_LENGTHS.CUSTOM_CSS);
	}

	return sanitized;
}

/**
 * Sanitizes multiple blocks
 */
export function sanitizeBlocks(blocks: Block[]): Block[] {
	return blocks.map((block) => sanitizeBlock(block));
}

// =============================================================================
// Validation with Sanitization
// =============================================================================

/**
 * Validates and sanitizes a block in one operation
 */
export function validateAndSanitizeBlock(block: unknown): ValidationResult {
	const result = validateBlock(block);

	if (result.success && result.data) {
		return {
			...result,
			data: sanitizeBlock(result.data)
		};
	}

	return result;
}

/**
 * Validates and sanitizes multiple blocks
 */
export function validateAndSanitizeBlocks(blocks: unknown[]): ValidationResults {
	const results = validateBlocks(blocks);

	if (results.data) {
		results.data = sanitizeBlocks(results.data);
	}

	return results;
}

// =============================================================================
// Schema Export for External Use
// =============================================================================

export const schemas = {
	block: blockSchema,
	blockContent: blockContentSchema,
	blockSettings: blockSettingsSchema,
	blockMetadata: blockMetadataSchema,
	blockItem: blockItemSchema,
	blockType: blockTypeSchema,
	animationSettings: animationSettingsSchema,
	blockComment: blockCommentSchema,
	...blockSchemas
};

// =============================================================================
// TypeScript Type Inference from Schemas
// =============================================================================

export type ValidatedBlock = v.InferOutput<typeof blockSchema>;
export type ValidatedBlockContent = v.InferOutput<typeof blockContentSchema>;
export type ValidatedBlockSettings = v.InferOutput<typeof blockSettingsSchema>;
export type ValidatedBlockMetadata = v.InferOutput<typeof blockMetadataSchema>;
export type ValidatedBlockItem = v.InferOutput<typeof blockItemSchema>;
export type ValidatedBlockType = v.InferOutput<typeof blockTypeSchema>;
export type ValidatedAnimationSettings = v.InferOutput<typeof animationSettingsSchema>;

// Type-specific block types
export type ValidatedParagraphBlock = v.InferOutput<typeof blockSchemas.paragraph>;
export type ValidatedHeadingBlock = v.InferOutput<typeof blockSchemas.heading>;
export type ValidatedQuoteBlock = v.InferOutput<typeof blockSchemas.quote>;
export type ValidatedPullquoteBlock = v.InferOutput<typeof blockSchemas.pullquote>;
export type ValidatedCodeBlock = v.InferOutput<typeof blockSchemas.code>;
export type ValidatedPreformattedBlock = v.InferOutput<typeof blockSchemas.preformatted>;
export type ValidatedListBlock = v.InferOutput<typeof blockSchemas.list>;
export type ValidatedChecklistBlock = v.InferOutput<typeof blockSchemas.checklist>;
export type ValidatedImageBlock = v.InferOutput<typeof blockSchemas.image>;
export type ValidatedGalleryBlock = v.InferOutput<typeof blockSchemas.gallery>;
export type ValidatedVideoBlock = v.InferOutput<typeof blockSchemas.video>;
export type ValidatedAudioBlock = v.InferOutput<typeof blockSchemas.audio>;
export type ValidatedFileBlock = v.InferOutput<typeof blockSchemas.file>;
export type ValidatedEmbedBlock = v.InferOutput<typeof blockSchemas.embed>;
export type ValidatedGifBlock = v.InferOutput<typeof blockSchemas.gif>;
export type ValidatedColumnsBlock = v.InferOutput<typeof blockSchemas.columns>;
export type ValidatedGroupBlock = v.InferOutput<typeof blockSchemas.group>;
export type ValidatedSeparatorBlock = v.InferOutput<typeof blockSchemas.separator>;
export type ValidatedSpacerBlock = v.InferOutput<typeof blockSchemas.spacer>;
export type ValidatedRowBlock = v.InferOutput<typeof blockSchemas.row>;
export type ValidatedButtonBlock = v.InferOutput<typeof blockSchemas.button>;
export type ValidatedButtonsBlock = v.InferOutput<typeof blockSchemas.buttons>;
export type ValidatedAccordionBlock = v.InferOutput<typeof blockSchemas.accordion>;
export type ValidatedTabsBlock = v.InferOutput<typeof blockSchemas.tabs>;
export type ValidatedToggleBlock = v.InferOutput<typeof blockSchemas.toggle>;
export type ValidatedTocBlock = v.InferOutput<typeof blockSchemas.toc>;
export type ValidatedTickerBlock = v.InferOutput<typeof blockSchemas.ticker>;
export type ValidatedChartBlock = v.InferOutput<typeof blockSchemas.chart>;
export type ValidatedPriceAlertBlock = v.InferOutput<typeof blockSchemas.priceAlert>;
export type ValidatedTradingIdeaBlock = v.InferOutput<typeof blockSchemas.tradingIdea>;
export type ValidatedRiskDisclaimerBlock = v.InferOutput<typeof blockSchemas.riskDisclaimer>;
export type ValidatedCalloutBlock = v.InferOutput<typeof blockSchemas.callout>;
export type ValidatedCardBlock = v.InferOutput<typeof blockSchemas.card>;
export type ValidatedTestimonialBlock = v.InferOutput<typeof blockSchemas.testimonial>;
export type ValidatedCtaBlock = v.InferOutput<typeof blockSchemas.cta>;
export type ValidatedCountdownBlock = v.InferOutput<typeof blockSchemas.countdown>;
export type ValidatedSocialShareBlock = v.InferOutput<typeof blockSchemas.socialShare>;
export type ValidatedAuthorBlock = v.InferOutput<typeof blockSchemas.author>;
export type ValidatedRelatedPostsBlock = v.InferOutput<typeof blockSchemas.relatedPosts>;
export type ValidatedNewsletterBlock = v.InferOutput<typeof blockSchemas.newsletter>;
export type ValidatedAiGeneratedBlock = v.InferOutput<typeof blockSchemas.aiGenerated>;
export type ValidatedAiSummaryBlock = v.InferOutput<typeof blockSchemas.aiSummary>;
export type ValidatedAiTranslationBlock = v.InferOutput<typeof blockSchemas.aiTranslation>;
export type ValidatedShortcodeBlock = v.InferOutput<typeof blockSchemas.shortcode>;
export type ValidatedHtmlBlock = v.InferOutput<typeof blockSchemas.html>;
export type ValidatedReusableBlock = v.InferOutput<typeof blockSchemas.reusable>;

// =============================================================================
// Utility Types
// =============================================================================

export type BlockSchemaMap = typeof blockSchemas;
export type BlockSchemaKey = keyof BlockSchemaMap;

/**
 * Helper to get the content type for a specific block type
 */
export type BlockContentFor<T extends BlockType> = T extends keyof BlockSchemaMap
	? v.InferOutput<BlockSchemaMap[T]>['content']
	: BlockContent;

// =============================================================================
// Default Exports
// =============================================================================

export default {
	// Validation functions
	validateBlock,
	validateBlocks,
	isValidBlockType,
	getBlockSchema,

	// Sanitization functions
	sanitizeBlock,
	sanitizeBlocks,
	sanitizeHtml,
	sanitizeUrl,
	sanitizeCSS,

	// Combined functions
	validateAndSanitizeBlock,
	validateAndSanitizeBlocks,

	// Security utilities
	isSecureUrl,
	containsXSS,

	// Schemas
	schemas,
	blockSchemas,

	// Constants
	MAX_LENGTHS,
	ALL_BLOCK_TYPES
};

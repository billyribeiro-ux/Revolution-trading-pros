/**
 * Revolution Trading Pros - Block Editor Validation System
 * ═══════════════════════════════════════════════════════════════════════════
 * Comprehensive JSON Schema validation using Zod for 30+ block types
 * Includes XSS protection, URL security, and content sanitization
 *
 * @version 4.0.0 Enterprise
 * @author Revolution Trading Pros
 */

import { z } from 'zod';
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
// Custom Zod Refinements
// =============================================================================

const secureUrlSchema = z
	.string()
	.max(MAX_LENGTHS.URL)
	.refine((url) => !url || isSecureUrl(url), {
		message: 'URL contains potentially dangerous protocol'
	});

const safeHtmlSchema = z
	.string()
	.max(MAX_LENGTHS.HTML_CONTENT)
	.refine((html) => !html || !containsXSS(html), {
		message: 'HTML content contains potentially dangerous XSS patterns'
	});

const cssValueSchema = z.string().max(MAX_LENGTHS.CSS_VALUE).optional();

// =============================================================================
// Base Schema Components
// =============================================================================

// Block ID Schema
const blockIdSchema = z.string().min(1).max(MAX_LENGTHS.ID);

// Animation Settings Schema
const animationSettingsSchema = z
	.object({
		type: z.enum(['none', 'fade', 'slide', 'zoom', 'bounce', 'flip', 'rotate']),
		direction: z.enum(['up', 'down', 'left', 'right']).optional(),
		duration: z.number().min(0).max(10000).optional(),
		delay: z.number().min(0).max(10000).optional(),
		easing: z.string().max(64).optional(),
		triggerOnScroll: z.boolean().optional(),
		repeat: z.boolean().optional()
	})
	.optional();

// Block Comment Schema - using a simpler approach for recursion
const baseBlockCommentSchema = z.object({
	id: blockIdSchema,
	userId: z.string().max(MAX_LENGTHS.ID),
	userName: z.string().max(MAX_LENGTHS.TEXT_SHORT),
	userAvatar: secureUrlSchema.optional(),
	content: z.string().max(MAX_LENGTHS.TEXT_LONG),
	createdAt: z.string().max(64),
	resolved: z.boolean().optional()
});

// For nested replies, we use a simpler flat structure to avoid deep recursion
const blockCommentSchema = baseBlockCommentSchema.extend({
	replies: z.array(baseBlockCommentSchema).max(50).optional()
});

// Block Metadata Schema
const blockMetadataSchema = z.object({
	createdAt: z.string().datetime({ offset: true }).or(z.string().max(64)),
	updatedAt: z.string().datetime({ offset: true }).or(z.string().max(64)),
	createdBy: z.string().max(MAX_LENGTHS.ID).optional(),
	version: z.number().int().min(0),
	locked: z.boolean().optional(),
	lockedBy: z.string().max(MAX_LENGTHS.ID).optional(),
	comments: z.array(blockCommentSchema).max(100).optional(),
	aiGenerated: z.boolean().optional()
});

// Block Item Schema (for lists, accordions, tabs, etc.)
const blockItemSchema: z.ZodType<BlockItem> = z.lazy(() =>
	z.object({
		id: blockIdSchema,
		content: z.string().max(MAX_LENGTHS.TEXT_LONG),
		checked: z.boolean().optional(),
		collapsed: z.boolean().optional(),
		children: z.array(blockItemSchema).max(MAX_LENGTHS.LIST_ITEMS).optional()
	})
);

// Focal Point Schema
const focalPointSchema = z
	.object({
		x: z.number().min(0).max(1),
		y: z.number().min(0).max(1)
	})
	.optional();

// =============================================================================
// Block Settings Schema
// =============================================================================

const blockSettingsSchema = z.object({
	// Typography
	textAlign: z.enum(['left', 'center', 'right', 'justify']).optional(),
	fontSize: cssValueSchema,
	fontWeight: cssValueSchema,
	fontFamily: z.string().max(256).optional(),
	lineHeight: cssValueSchema,
	letterSpacing: cssValueSchema,
	textColor: cssValueSchema,
	textTransform: z.enum(['none', 'uppercase', 'lowercase', 'capitalize']).optional(),

	// Heading specific
	level: z
		.union([z.literal(1), z.literal(2), z.literal(3), z.literal(4), z.literal(5), z.literal(6)])
		.optional(),
	anchor: z.string().max(MAX_LENGTHS.ANCHOR).optional(),

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
	backgroundImage: secureUrlSchema.optional(),
	backgroundGradient: z.string().max(512).optional(),
	backgroundOverlay: cssValueSchema,

	// Border
	borderWidth: cssValueSchema,
	borderColor: cssValueSchema,
	borderStyle: z.enum(['solid', 'dashed', 'dotted', 'none']).optional(),
	borderRadius: cssValueSchema,

	// Shadow
	boxShadow: z.string().max(256).optional(),

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
	opacity: z.number().min(0).max(1).optional(),
	blendMode: z
		.enum([
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
		.optional(),

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
	display: z.enum(['block', 'flex', 'grid', 'inline', 'none']).optional(),
	flexDirection: z.enum(['row', 'column']).optional(),
	justifyContent: cssValueSchema,
	alignItems: cssValueSchema,
	gap: cssValueSchema,
	columnCount: z.number().int().min(1).max(12).optional(),
	columnLayout: z.string().max(64).optional(),

	// Image specific
	imageSize: z.enum(['thumbnail', 'medium', 'large', 'full', 'custom']).optional(),
	imageWidth: z.number().int().min(1).max(10000).optional(),
	imageHeight: z.number().int().min(1).max(10000).optional(),
	objectFit: z.enum(['cover', 'contain', 'fill', 'none']).optional(),
	focalPoint: focalPointSchema,
	imageFilter: z.string().max(256).optional(),

	// Link
	linkUrl: secureUrlSchema.optional(),
	linkTarget: z.enum(['_blank', '_self']).optional(),
	linkRel: z.string().max(64).optional(),

	// Button specific
	buttonStyle: z.enum(['primary', 'secondary', 'outline', 'ghost', 'link']).optional(),
	buttonSize: z.enum(['small', 'medium', 'large']).optional(),
	buttonIcon: z.string().max(64).optional(),
	buttonIconPosition: z.enum(['left', 'right']).optional(),

	// Responsive visibility
	hideOnDesktop: z.boolean().optional(),
	hideOnTablet: z.boolean().optional(),
	hideOnMobile: z.boolean().optional(),

	// Responsive typography
	tabletFontSize: cssValueSchema,
	mobileFontSize: cssValueSchema,

	// Responsive spacing
	tabletPadding: cssValueSchema,
	mobilePadding: cssValueSchema,

	// Advanced
	customCSS: z.string().max(MAX_LENGTHS.CUSTOM_CSS).optional(),
	customId: z.string().max(MAX_LENGTHS.ID).optional(),
	customClass: z.string().max(MAX_LENGTHS.CLASS_NAME).optional(),
	ariaLabel: z.string().max(MAX_LENGTHS.TEXT_SHORT).optional(),
	position: z.enum(['static', 'relative', 'absolute', 'fixed', 'sticky']).optional(),
	zIndex: z.number().int().min(-9999).max(9999).optional(),
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

export const blockTypeSchema = z.enum(ALL_BLOCK_TYPES);

// =============================================================================
// Block Content Schemas - Type-Specific
// =============================================================================

// Forward declaration for recursive block schema
type BlockSchema = z.ZodType<Block>;

// Create a lazy block schema for nested blocks
const createBlockSchema = (): BlockSchema =>
	z.lazy(() =>
		z.object({
			id: blockIdSchema,
			type: blockTypeSchema,
			content: blockContentSchema,
			settings: blockSettingsSchema,
			metadata: blockMetadataSchema
		})
	) as BlockSchema;

// Paragraph Block Content
const paragraphContentSchema = z.object({
	text: z.string().max(MAX_LENGTHS.TEXT_CONTENT).optional(),
	html: safeHtmlSchema.optional()
});

// Heading Block Content
const headingContentSchema = z.object({
	text: z.string().max(MAX_LENGTHS.TEXT_MEDIUM).optional(),
	html: safeHtmlSchema.optional()
});

// Quote Block Content
const quoteContentSchema = z.object({
	text: z.string().max(MAX_LENGTHS.TEXT_LONG).optional(),
	html: safeHtmlSchema.optional()
});

// Pullquote Block Content
const pullquoteContentSchema = z.object({
	text: z.string().max(MAX_LENGTHS.TEXT_LONG).optional(),
	html: safeHtmlSchema.optional()
});

// Code Block Content
const codeContentSchema = z.object({
	code: z.string().max(MAX_LENGTHS.CODE_CONTENT).optional(),
	language: z.string().max(MAX_LENGTHS.LANGUAGE_CODE).optional()
});

// Preformatted Block Content
const preformattedContentSchema = z.object({
	text: z.string().max(MAX_LENGTHS.CODE_CONTENT).optional()
});

// List Block Content
const listContentSchema = z.object({
	listType: z.enum(['bullet', 'number', 'check']).optional(),
	listItems: z.array(z.string().max(MAX_LENGTHS.TEXT_LONG)).max(MAX_LENGTHS.LIST_ITEMS).optional(),
	items: z.array(blockItemSchema).max(MAX_LENGTHS.LIST_ITEMS).optional()
});

// Checklist Block Content
const checklistContentSchema = z.object({
	items: z.array(blockItemSchema).max(MAX_LENGTHS.LIST_ITEMS).optional()
});

// Image Block Content
const imageContentSchema = z.object({
	mediaId: z.number().int().positive().optional(),
	mediaUrl: secureUrlSchema.optional(),
	mediaAlt: z.string().max(MAX_LENGTHS.TEXT_SHORT).optional(),
	mediaCaption: z.string().max(MAX_LENGTHS.TEXT_MEDIUM).optional()
});

// Gallery Block Content
const galleryContentSchema = z.object({
	items: z
		.array(
			z.object({
				id: blockIdSchema,
				content: z.string().max(MAX_LENGTHS.TEXT_MEDIUM),
				mediaId: z.number().int().positive().optional(),
				mediaUrl: secureUrlSchema.optional(),
				mediaAlt: z.string().max(MAX_LENGTHS.TEXT_SHORT).optional()
			})
		)
		.max(MAX_LENGTHS.GALLERY_ITEMS)
		.optional()
});

// Video Block Content
const videoContentSchema = z.object({
	mediaId: z.number().int().positive().optional(),
	mediaUrl: secureUrlSchema.optional(),
	mediaCaption: z.string().max(MAX_LENGTHS.TEXT_MEDIUM).optional(),
	embedUrl: secureUrlSchema.optional(),
	embedType: z.enum(ALLOWED_EMBED_TYPES).optional()
});

// Audio Block Content
const audioContentSchema = z.object({
	mediaId: z.number().int().positive().optional(),
	mediaUrl: secureUrlSchema.optional(),
	mediaCaption: z.string().max(MAX_LENGTHS.TEXT_MEDIUM).optional()
});

// File Block Content
const fileContentSchema = z.object({
	mediaId: z.number().int().positive().optional(),
	mediaUrl: secureUrlSchema.optional(),
	text: z.string().max(MAX_LENGTHS.TEXT_SHORT).optional() // filename display
});

// Embed Block Content
const embedContentSchema = z.object({
	embedUrl: secureUrlSchema.optional(),
	embedType: z.enum(ALLOWED_EMBED_TYPES).optional(),
	html: safeHtmlSchema.optional()
});

// GIF Block Content
const gifContentSchema = z.object({
	mediaUrl: secureUrlSchema.optional(),
	mediaAlt: z.string().max(MAX_LENGTHS.TEXT_SHORT).optional()
});

// Columns Block Content
const columnsContentSchema = z.object({
	columnCount: z.number().int().min(1).max(12).optional(),
	columnLayout: z.string().max(64).optional(),
	children: z.array(createBlockSchema()).max(MAX_LENGTHS.CHILDREN_BLOCKS).optional()
});

// Group Block Content
const groupContentSchema = z.object({
	children: z.array(createBlockSchema()).max(MAX_LENGTHS.CHILDREN_BLOCKS).optional()
});

// Separator Block Content
const separatorContentSchema = z.object({});

// Spacer Block Content
const spacerContentSchema = z.object({});

// Row Block Content
const rowContentSchema = z.object({
	children: z.array(createBlockSchema()).max(MAX_LENGTHS.CHILDREN_BLOCKS).optional()
});

// Button Block Content
const buttonContentSchema = z.object({
	text: z.string().max(MAX_LENGTHS.TEXT_SHORT).optional(),
	html: safeHtmlSchema.optional()
});

// Buttons Block Content
const buttonsContentSchema = z.object({
	children: z.array(createBlockSchema()).max(20).optional()
});

// Accordion Block Content
const accordionContentSchema = z.object({
	items: z.array(blockItemSchema).max(50).optional(),
	children: z.array(createBlockSchema()).max(MAX_LENGTHS.CHILDREN_BLOCKS).optional()
});

// Tabs Block Content
const tabsContentSchema = z.object({
	items: z.array(blockItemSchema).max(20).optional(),
	children: z.array(createBlockSchema()).max(MAX_LENGTHS.CHILDREN_BLOCKS).optional()
});

// Toggle Block Content
const toggleContentSchema = z.object({
	text: z.string().max(MAX_LENGTHS.TEXT_SHORT).optional(),
	children: z.array(createBlockSchema()).max(MAX_LENGTHS.CHILDREN_BLOCKS).optional()
});

// Table of Contents Block Content
const tocContentSchema = z.object({});

// Ticker Block Content
const tickerContentSchema = z.object({
	ticker: z
		.string()
		.max(MAX_LENGTHS.TICKER_SYMBOL)
		.regex(/^[A-Z0-9.^=-]+$/i, 'Invalid ticker symbol format')
		.optional()
});

// Chart Block Content
const chartContentSchema = z.object({
	ticker: z
		.string()
		.max(MAX_LENGTHS.TICKER_SYMBOL)
		.regex(/^[A-Z0-9.^=-]+$/i, 'Invalid ticker symbol format')
		.optional(),
	chartType: z.enum(['line', 'candle', 'bar']).optional()
});

// Price Alert Block Content
const priceAlertContentSchema = z.object({
	ticker: z
		.string()
		.max(MAX_LENGTHS.TICKER_SYMBOL)
		.regex(/^[A-Z0-9.^=-]+$/i, 'Invalid ticker symbol format')
		.optional(),
	priceTarget: z.number().positive().optional(),
	text: z.string().max(MAX_LENGTHS.TEXT_MEDIUM).optional()
});

// Trading Idea Block Content
const tradingIdeaContentSchema = z.object({
	ticker: z.string().max(MAX_LENGTHS.TICKER_SYMBOL).optional(),
	text: z.string().max(MAX_LENGTHS.TEXT_LONG).optional(),
	html: safeHtmlSchema.optional(),
	priceTarget: z.number().optional(),
	chartType: z.enum(['line', 'candle', 'bar']).optional()
});

// Risk Disclaimer Block Content
const riskDisclaimerContentSchema = z.object({
	text: z.string().max(MAX_LENGTHS.TEXT_LONG).optional(),
	html: safeHtmlSchema.optional()
});

// Callout Block Content
const calloutContentSchema = z.object({
	text: z.string().max(MAX_LENGTHS.TEXT_LONG).optional(),
	html: safeHtmlSchema.optional()
});

// Card Block Content
const cardContentSchema = z.object({
	text: z.string().max(MAX_LENGTHS.TEXT_LONG).optional(),
	html: safeHtmlSchema.optional(),
	mediaId: z.number().int().positive().optional(),
	mediaUrl: secureUrlSchema.optional(),
	mediaAlt: z.string().max(MAX_LENGTHS.TEXT_SHORT).optional(),
	children: z.array(createBlockSchema()).max(MAX_LENGTHS.CHILDREN_BLOCKS).optional()
});

// Testimonial Block Content
const testimonialContentSchema = z.object({
	text: z.string().max(MAX_LENGTHS.TEXT_LONG).optional(),
	html: safeHtmlSchema.optional(),
	mediaUrl: secureUrlSchema.optional() // avatar
});

// CTA Block Content
const ctaContentSchema = z.object({
	text: z.string().max(MAX_LENGTHS.TEXT_LONG).optional(),
	html: safeHtmlSchema.optional(),
	children: z.array(createBlockSchema()).max(MAX_LENGTHS.CHILDREN_BLOCKS).optional()
});

// Countdown Block Content
const countdownContentSchema = z.object({
	text: z.string().max(MAX_LENGTHS.TEXT_SHORT).optional() // target date
});

// Social Share Block Content
const socialShareContentSchema = z.object({});

// Author Block Content
const authorContentSchema = z.object({
	text: z.string().max(MAX_LENGTHS.TEXT_LONG).optional(), // bio
	html: safeHtmlSchema.optional(),
	mediaUrl: secureUrlSchema.optional() // avatar
});

// Related Posts Block Content
const relatedPostsContentSchema = z.object({
	items: z
		.array(
			z.object({
				id: blockIdSchema,
				content: z.string().max(MAX_LENGTHS.TEXT_MEDIUM)
			})
		)
		.max(20)
		.optional()
});

// Newsletter Block Content
const newsletterContentSchema = z.object({
	text: z.string().max(MAX_LENGTHS.TEXT_MEDIUM).optional(),
	html: safeHtmlSchema.optional()
});

// AI Generated Block Content
const aiGeneratedContentSchema = z.object({
	aiPrompt: z.string().max(MAX_LENGTHS.AI_PROMPT).optional(),
	aiModel: z.string().max(64).optional(),
	aiOutput: z.string().max(MAX_LENGTHS.AI_OUTPUT).optional(),
	text: z.string().max(MAX_LENGTHS.TEXT_CONTENT).optional(),
	html: safeHtmlSchema.optional()
});

// AI Summary Block Content
const aiSummaryContentSchema = z.object({
	aiPrompt: z.string().max(MAX_LENGTHS.AI_PROMPT).optional(),
	aiModel: z.string().max(64).optional(),
	aiOutput: z.string().max(MAX_LENGTHS.TEXT_LONG).optional(),
	text: z.string().max(MAX_LENGTHS.TEXT_LONG).optional()
});

// AI Translation Block Content
const aiTranslationContentSchema = z.object({
	aiPrompt: z.string().max(MAX_LENGTHS.AI_PROMPT).optional(),
	aiModel: z.string().max(64).optional(),
	aiOutput: z.string().max(MAX_LENGTHS.AI_OUTPUT).optional(),
	text: z.string().max(MAX_LENGTHS.TEXT_CONTENT).optional(),
	language: z.string().max(MAX_LENGTHS.LANGUAGE_CODE).optional()
});

// Shortcode Block Content
const shortcodeContentSchema = z.object({
	text: z.string().max(MAX_LENGTHS.TEXT_MEDIUM).optional(),
	code: z.string().max(MAX_LENGTHS.TEXT_MEDIUM).optional()
});

// HTML Block Content - with strict XSS validation
const htmlBlockContentSchema = z.object({
	html: safeHtmlSchema.optional()
});

// Reusable Block Content
const reusableContentSchema = z.object({
	text: z.string().max(MAX_LENGTHS.ID).optional(), // reference ID
	children: z.array(createBlockSchema()).max(MAX_LENGTHS.CHILDREN_BLOCKS).optional()
});

// =============================================================================
// Combined Block Content Schema
// =============================================================================

const blockContentSchema = z.object({
	// Text content
	text: z.string().max(MAX_LENGTHS.TEXT_CONTENT).optional(),
	html: safeHtmlSchema.optional(),

	// Media content
	mediaId: z.number().int().positive().optional(),
	mediaUrl: secureUrlSchema.optional(),
	mediaAlt: z.string().max(MAX_LENGTHS.TEXT_SHORT).optional(),
	mediaCaption: z.string().max(MAX_LENGTHS.TEXT_MEDIUM).optional(),

	// Nested content
	children: z.array(createBlockSchema()).max(MAX_LENGTHS.CHILDREN_BLOCKS).optional(),
	items: z.array(blockItemSchema).max(MAX_LENGTHS.LIST_ITEMS).optional(),

	// Special content
	embedUrl: secureUrlSchema.optional(),
	embedType: z.enum(ALLOWED_EMBED_TYPES).optional(),

	// Code content
	code: z.string().max(MAX_LENGTHS.CODE_CONTENT).optional(),
	language: z.string().max(MAX_LENGTHS.LANGUAGE_CODE).optional(),

	// List content
	listType: z.enum(['bullet', 'number', 'check']).optional(),
	listItems: z.array(z.string().max(MAX_LENGTHS.TEXT_LONG)).max(MAX_LENGTHS.LIST_ITEMS).optional(),

	// Columns/Layout
	columnCount: z.number().int().min(1).max(12).optional(),
	columnLayout: z.string().max(64).optional(),

	// Trading-specific
	ticker: z.string().max(MAX_LENGTHS.TICKER_SYMBOL).optional(),
	chartType: z.enum(['line', 'candle', 'bar']).optional(),
	priceTarget: z.number().optional(),

	// AI content
	aiPrompt: z.string().max(MAX_LENGTHS.AI_PROMPT).optional(),
	aiModel: z.string().max(64).optional(),
	aiOutput: z.string().max(MAX_LENGTHS.AI_OUTPUT).optional()
});

// =============================================================================
// Complete Block Schema
// =============================================================================

export const blockSchema = z.object({
	id: blockIdSchema,
	type: blockTypeSchema,
	content: blockContentSchema,
	settings: blockSettingsSchema,
	metadata: blockMetadataSchema
});

// =============================================================================
// Type-Specific Block Schemas with Content Validation
// =============================================================================

const createTypeSpecificBlockSchema = <T extends z.ZodType>(type: BlockType, contentSchema: T) => {
	return z.object({
		id: blockIdSchema,
		type: z.literal(type),
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

// =============================================================================
// Validation Functions
// =============================================================================

/**
 * Validates a single block against its type-specific schema
 */
export function validateBlock(block: unknown): ValidationResult {
	// First, validate basic block structure
	const basicResult = blockSchema.safeParse(block);

	if (!basicResult.success) {
		return {
			success: false,
			errors: basicResult.error.issues.map((e) => ({
				path: e.path,
				message: e.message,
				code: e.code
			}))
		};
	}

	// Then validate type-specific content
	const blockData = basicResult.data;
	const typeSchema = blockSchemas[blockData.type as keyof typeof blockSchemas];

	if (typeSchema) {
		const typeResult = typeSchema.safeParse(blockData);
		if (!typeResult.success) {
			return {
				success: false,
				errors: typeResult.error.issues.map((e) => ({
					path: e.path,
					message: e.message,
					code: e.code
				}))
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

export type ValidatedBlock = z.infer<typeof blockSchema>;
export type ValidatedBlockContent = z.infer<typeof blockContentSchema>;
export type ValidatedBlockSettings = z.infer<typeof blockSettingsSchema>;
export type ValidatedBlockMetadata = z.infer<typeof blockMetadataSchema>;
export type ValidatedBlockItem = z.infer<typeof blockItemSchema>;
export type ValidatedBlockType = z.infer<typeof blockTypeSchema>;
export type ValidatedAnimationSettings = z.infer<typeof animationSettingsSchema>;

// Type-specific block types
export type ValidatedParagraphBlock = z.infer<typeof blockSchemas.paragraph>;
export type ValidatedHeadingBlock = z.infer<typeof blockSchemas.heading>;
export type ValidatedQuoteBlock = z.infer<typeof blockSchemas.quote>;
export type ValidatedPullquoteBlock = z.infer<typeof blockSchemas.pullquote>;
export type ValidatedCodeBlock = z.infer<typeof blockSchemas.code>;
export type ValidatedPreformattedBlock = z.infer<typeof blockSchemas.preformatted>;
export type ValidatedListBlock = z.infer<typeof blockSchemas.list>;
export type ValidatedChecklistBlock = z.infer<typeof blockSchemas.checklist>;
export type ValidatedImageBlock = z.infer<typeof blockSchemas.image>;
export type ValidatedGalleryBlock = z.infer<typeof blockSchemas.gallery>;
export type ValidatedVideoBlock = z.infer<typeof blockSchemas.video>;
export type ValidatedAudioBlock = z.infer<typeof blockSchemas.audio>;
export type ValidatedFileBlock = z.infer<typeof blockSchemas.file>;
export type ValidatedEmbedBlock = z.infer<typeof blockSchemas.embed>;
export type ValidatedGifBlock = z.infer<typeof blockSchemas.gif>;
export type ValidatedColumnsBlock = z.infer<typeof blockSchemas.columns>;
export type ValidatedGroupBlock = z.infer<typeof blockSchemas.group>;
export type ValidatedSeparatorBlock = z.infer<typeof blockSchemas.separator>;
export type ValidatedSpacerBlock = z.infer<typeof blockSchemas.spacer>;
export type ValidatedRowBlock = z.infer<typeof blockSchemas.row>;
export type ValidatedButtonBlock = z.infer<typeof blockSchemas.button>;
export type ValidatedButtonsBlock = z.infer<typeof blockSchemas.buttons>;
export type ValidatedAccordionBlock = z.infer<typeof blockSchemas.accordion>;
export type ValidatedTabsBlock = z.infer<typeof blockSchemas.tabs>;
export type ValidatedToggleBlock = z.infer<typeof blockSchemas.toggle>;
export type ValidatedTocBlock = z.infer<typeof blockSchemas.toc>;
export type ValidatedTickerBlock = z.infer<typeof blockSchemas.ticker>;
export type ValidatedChartBlock = z.infer<typeof blockSchemas.chart>;
export type ValidatedPriceAlertBlock = z.infer<typeof blockSchemas.priceAlert>;
export type ValidatedTradingIdeaBlock = z.infer<typeof blockSchemas.tradingIdea>;
export type ValidatedRiskDisclaimerBlock = z.infer<typeof blockSchemas.riskDisclaimer>;
export type ValidatedCalloutBlock = z.infer<typeof blockSchemas.callout>;
export type ValidatedCardBlock = z.infer<typeof blockSchemas.card>;
export type ValidatedTestimonialBlock = z.infer<typeof blockSchemas.testimonial>;
export type ValidatedCtaBlock = z.infer<typeof blockSchemas.cta>;
export type ValidatedCountdownBlock = z.infer<typeof blockSchemas.countdown>;
export type ValidatedSocialShareBlock = z.infer<typeof blockSchemas.socialShare>;
export type ValidatedAuthorBlock = z.infer<typeof blockSchemas.author>;
export type ValidatedRelatedPostsBlock = z.infer<typeof blockSchemas.relatedPosts>;
export type ValidatedNewsletterBlock = z.infer<typeof blockSchemas.newsletter>;
export type ValidatedAiGeneratedBlock = z.infer<typeof blockSchemas.aiGenerated>;
export type ValidatedAiSummaryBlock = z.infer<typeof blockSchemas.aiSummary>;
export type ValidatedAiTranslationBlock = z.infer<typeof blockSchemas.aiTranslation>;
export type ValidatedShortcodeBlock = z.infer<typeof blockSchemas.shortcode>;
export type ValidatedHtmlBlock = z.infer<typeof blockSchemas.html>;
export type ValidatedReusableBlock = z.infer<typeof blockSchemas.reusable>;

// =============================================================================
// Utility Types
// =============================================================================

export type BlockSchemaMap = typeof blockSchemas;
export type BlockSchemaKey = keyof BlockSchemaMap;

/**
 * Helper to get the content type for a specific block type
 */
export type BlockContentFor<T extends BlockType> = T extends keyof BlockSchemaMap
	? z.infer<BlockSchemaMap[T]>['content']
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

/**
 * Image SEO Module - Enterprise-Grade Image Optimization
 * Following Google Image SEO Best Practices (November 2025)
 *
 * Features:
 * - Auto alt text generation from filename
 * - Auto title attribute generation
 * - Case transformation (titlecase, sentencecase, lowercase, uppercase)
 * - Strip numbers and special characters
 * - Custom prefix/suffix support
 * - Caption generation
 * - Avatar image handling
 * - Placeholder replacement
 * - Batch processing support
 * - SEO-friendly filename suggestions
 *
 * @version 1.0.0 - November 2025
 */

// ═══════════════════════════════════════════════════════════════════════════
// Type Definitions
// ═══════════════════════════════════════════════════════════════════════════

export type CaseTransform = 'titlecase' | 'sentencecase' | 'lowercase' | 'uppercase' | 'none';

export interface ImageSeoSettings {
	// Alt text settings
	altTextEnabled: boolean;
	altTextFormat: string; // Template: %filename%, %title%, %site%, %sep%
	altTextCase: CaseTransform;
	altTextPrefix: string;
	altTextSuffix: string;
	altTextStripNumbers: boolean;
	altTextStripSpecialChars: boolean;

	// Title attribute settings
	titleEnabled: boolean;
	titleFormat: string;
	titleCase: CaseTransform;
	titlePrefix: string;
	titleSuffix: string;

	// Caption settings
	captionEnabled: boolean;
	captionFormat: string;
	captionCase: CaseTransform;

	// General settings
	separator: string;
	siteName: string;
	overwriteExisting: boolean;

	// Avatar settings
	avatarAltFormat: string;

	// Placeholder settings
	placeholderText: string;
}

export interface ImageMetadata {
	src: string;
	alt?: string;
	title?: string;
	caption?: string;
	width?: number;
	height?: number;
	filename?: string;
	extension?: string;
}

export interface ProcessedImage extends ImageMetadata {
	generatedAlt: string;
	generatedTitle: string;
	generatedCaption?: string;
	suggestedFilename?: string;
	seoScore: number;
	issues: string[];
	recommendations: string[];
}

// ═══════════════════════════════════════════════════════════════════════════
// Default Settings
// ═══════════════════════════════════════════════════════════════════════════

export const defaultImageSeoSettings: ImageSeoSettings = {
	altTextEnabled: true,
	altTextFormat: '%filename%',
	altTextCase: 'titlecase',
	altTextPrefix: '',
	altTextSuffix: '',
	altTextStripNumbers: true,
	altTextStripSpecialChars: true,

	titleEnabled: true,
	titleFormat: '%filename% %sep% %site%',
	titleCase: 'titlecase',
	titlePrefix: '',
	titleSuffix: '',

	captionEnabled: false,
	captionFormat: '%filename%',
	captionCase: 'sentencecase',

	separator: '-',
	siteName: 'Revolution Trading Pros',
	overwriteExisting: false,

	avatarAltFormat: 'Avatar of %name%',
	placeholderText: 'Image'
};

// ═══════════════════════════════════════════════════════════════════════════
// Core Functions
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Extract filename from image URL or path
 */
export function extractFilename(src: string): { filename: string; extension: string } {
	const url = src.split('?')[0]; // Remove query params
	if (!url) return { filename: '', extension: '' };

	const parts = url.split('/');
	const fullFilename = parts[parts.length - 1];
	if (!fullFilename) return { filename: '', extension: '' };

	const lastDot = fullFilename.lastIndexOf('.');

	if (lastDot === -1) {
		return { filename: fullFilename, extension: '' };
	}

	return {
		filename: fullFilename.substring(0, lastDot),
		extension: fullFilename.substring(lastDot + 1).toLowerCase()
	};
}

/**
 * Clean filename for SEO-friendly text
 */
export function cleanFilename(
	filename: string,
	stripNumbers: boolean = true,
	stripSpecialChars: boolean = true
): string {
	let cleaned = filename;

	// Replace common separators with spaces
	cleaned = cleaned.replace(/[-_]+/g, ' ');

	// Strip numbers if enabled
	if (stripNumbers) {
		cleaned = cleaned.replace(/\d+/g, '');
	}

	// Strip special characters if enabled
	if (stripSpecialChars) {
		cleaned = cleaned.replace(/[^a-zA-Z0-9\s]/g, '');
	}

	// Normalize whitespace
	cleaned = cleaned.replace(/\s+/g, ' ').trim();

	return cleaned;
}

/**
 * Apply case transformation
 */
export function applyCase(text: string, caseType: CaseTransform): string {
	switch (caseType) {
		case 'titlecase':
			return toTitleCase(text);
		case 'sentencecase':
			return toSentenceCase(text);
		case 'lowercase':
			return text.toLowerCase();
		case 'uppercase':
			return text.toUpperCase();
		case 'none':
		default:
			return text;
	}
}

/**
 * Convert text to Title Case
 */
export function toTitleCase(text: string): string {
	const smallWords = new Set([
		'a',
		'an',
		'and',
		'as',
		'at',
		'but',
		'by',
		'for',
		'if',
		'in',
		'nor',
		'of',
		'on',
		'or',
		'so',
		'the',
		'to',
		'up',
		'yet',
		'via'
	]);

	return text
		.toLowerCase()
		.split(' ')
		.map((word, index) => {
			// Always capitalize first and last word
			if (index === 0 || !smallWords.has(word)) {
				return word.charAt(0).toUpperCase() + word.slice(1);
			}
			return word;
		})
		.join(' ');
}

/**
 * Convert text to Sentence case
 */
export function toSentenceCase(text: string): string {
	if (!text) return text;
	return text.charAt(0).toUpperCase() + text.slice(1).toLowerCase();
}

/**
 * Process format template with placeholders
 */
export function processTemplate(
	template: string,
	variables: Record<string, string>,
	settings: ImageSeoSettings
): string {
	let result = template;

	// Replace standard placeholders
	result = result.replace(/%filename%/gi, variables['filename'] || '');
	result = result.replace(/%title%/gi, variables['title'] || '');
	result = result.replace(/%site%/gi, settings.siteName);
	result = result.replace(/%sep%/gi, settings.separator);
	result = result.replace(/%name%/gi, variables['name'] || '');

	// Clean up extra spaces and separators
	result = result.replace(/\s+/g, ' ').trim();
	result = result.replace(new RegExp(`\\s*${settings.separator}\\s*$`), '');
	result = result.replace(new RegExp(`^\\s*${settings.separator}\\s*`), '');

	return result;
}

/**
 * Generate alt text for an image
 */
export function generateAltText(image: ImageMetadata, settings: ImageSeoSettings): string {
	if (!settings.altTextEnabled) {
		return image.alt || '';
	}

	// Don't overwrite existing if setting is disabled
	if (image.alt && !settings.overwriteExisting) {
		return image.alt;
	}

	const { filename } = extractFilename(image.src);
	const cleanedFilename = cleanFilename(
		filename,
		settings.altTextStripNumbers,
		settings.altTextStripSpecialChars
	);

	if (!cleanedFilename) {
		return settings.placeholderText;
	}

	const variables = {
		filename: cleanedFilename,
		title: image.title || cleanedFilename
	};

	let altText = processTemplate(settings.altTextFormat, variables, settings);
	altText = applyCase(altText, settings.altTextCase);

	// Add prefix/suffix
	if (settings.altTextPrefix) {
		altText = `${settings.altTextPrefix} ${altText}`;
	}
	if (settings.altTextSuffix) {
		altText = `${altText} ${settings.altTextSuffix}`;
	}

	return altText.trim();
}

/**
 * Generate title attribute for an image
 */
export function generateTitle(image: ImageMetadata, settings: ImageSeoSettings): string {
	if (!settings.titleEnabled) {
		return image.title || '';
	}

	// Don't overwrite existing if setting is disabled
	if (image.title && !settings.overwriteExisting) {
		return image.title;
	}

	const { filename } = extractFilename(image.src);
	const cleanedFilename = cleanFilename(filename, true, true);

	if (!cleanedFilename) {
		return '';
	}

	const variables = {
		filename: cleanedFilename,
		title: cleanedFilename
	};

	let title = processTemplate(settings.titleFormat, variables, settings);
	title = applyCase(title, settings.titleCase);

	// Add prefix/suffix
	if (settings.titlePrefix) {
		title = `${settings.titlePrefix} ${title}`;
	}
	if (settings.titleSuffix) {
		title = `${title} ${settings.titleSuffix}`;
	}

	return title.trim();
}

/**
 * Generate caption for an image
 */
export function generateCaption(image: ImageMetadata, settings: ImageSeoSettings): string {
	if (!settings.captionEnabled) {
		return image.caption || '';
	}

	// Don't overwrite existing if setting is disabled
	if (image.caption && !settings.overwriteExisting) {
		return image.caption;
	}

	const { filename } = extractFilename(image.src);
	const cleanedFilename = cleanFilename(filename, true, true);

	if (!cleanedFilename) {
		return '';
	}

	const variables = {
		filename: cleanedFilename,
		title: image.title || cleanedFilename
	};

	let caption = processTemplate(settings.captionFormat, variables, settings);
	caption = applyCase(caption, settings.captionCase);

	return caption.trim();
}

/**
 * Generate avatar alt text
 */
export function generateAvatarAlt(name: string, settings: ImageSeoSettings): string {
	if (!name) {
		return 'Avatar';
	}

	const variables = { name, filename: '', title: '' };
	return processTemplate(settings.avatarAltFormat, variables, settings);
}

/**
 * Generate SEO-friendly filename suggestion
 */
export function generateSeoFilename(altText: string, extension: string): string {
	if (!altText) return '';

	// Convert to lowercase and replace spaces with hyphens
	let filename = altText
		.toLowerCase()
		.replace(/[^a-z0-9\s-]/g, '')
		.replace(/\s+/g, '-')
		.replace(/-+/g, '-')
		.substring(0, 100); // Max length

	// Remove leading/trailing hyphens
	filename = filename.replace(/^-+|-+$/g, '');

	return extension ? `${filename}.${extension}` : filename;
}

/**
 * Calculate SEO score for an image
 */
export function calculateImageSeoScore(image: ProcessedImage): number {
	let score = 0;
	const maxScore = 100;

	// Alt text (40 points)
	if (image.generatedAlt) {
		score += 20;
		if (image.generatedAlt.length >= 10) score += 10;
		if (image.generatedAlt.length <= 125) score += 10;
	}

	// Title attribute (20 points)
	if (image.generatedTitle) {
		score += 10;
		if (image.generatedTitle.length >= 5) score += 10;
	}

	// Filename (20 points)
	const { filename } = extractFilename(image.src);
	if (filename) {
		// Check if filename is descriptive (not just numbers or random chars)
		if (/[a-zA-Z]{3,}/.test(filename)) score += 10;
		// Check if filename uses hyphens (SEO-friendly)
		if (filename.includes('-')) score += 5;
		// Check if filename is not too long
		if (filename.length <= 50) score += 5;
	}

	// Dimensions (10 points)
	if (image.width && image.height) {
		score += 10;
	}

	// Modern format (10 points)
	if (image.extension === 'webp' || image.extension === 'avif') {
		score += 10;
	} else if (image.extension === 'png' || image.extension === 'jpg' || image.extension === 'jpeg') {
		score += 5;
	}

	return Math.min(score, maxScore);
}

/**
 * Get SEO issues and recommendations for an image
 */
export function analyzeImageSeo(
	image: ImageMetadata,
	settings: ImageSeoSettings
): { issues: string[]; recommendations: string[] } {
	const issues: string[] = [];
	const recommendations: string[] = [];

	const { filename, extension } = extractFilename(image.src);

	// Check alt text
	if (!image.alt && !settings.altTextEnabled) {
		issues.push('Image has no alt text');
	}

	// Check title
	if (!image.title && settings.titleEnabled) {
		recommendations.push('Consider adding a title attribute for improved accessibility');
	}

	// Check filename
	if (/^\d+$/.test(filename)) {
		issues.push('Filename contains only numbers - use descriptive names');
	}

	if (filename.includes('_')) {
		recommendations.push('Use hyphens instead of underscores in filenames');
	}

	if (filename.length > 50) {
		recommendations.push('Consider shortening the filename for better SEO');
	}

	// Check format
	if (!['webp', 'avif'].includes(extension)) {
		recommendations.push('Consider converting to WebP or AVIF for better performance');
	}

	// Check dimensions
	if (!image.width || !image.height) {
		recommendations.push('Specify width and height attributes to prevent layout shifts');
	}

	return { issues, recommendations };
}

/**
 * Process a single image with all SEO optimizations
 */
export function processImage(
	image: ImageMetadata,
	settings: ImageSeoSettings = defaultImageSeoSettings
): ProcessedImage {
	const { filename, extension } = extractFilename(image.src);
	const generatedAlt = generateAltText(image, settings);
	const generatedTitle = generateTitle(image, settings);
	const generatedCaption = generateCaption(image, settings);
	const { issues, recommendations } = analyzeImageSeo(image, settings);

	const processed: ProcessedImage = {
		...image,
		filename,
		extension,
		generatedAlt,
		generatedTitle,
		generatedCaption,
		suggestedFilename: generateSeoFilename(generatedAlt, extension),
		seoScore: 0,
		issues,
		recommendations
	};

	processed.seoScore = calculateImageSeoScore(processed);

	return processed;
}

/**
 * Batch process multiple images
 */
export function processImages(
	images: ImageMetadata[],
	settings: ImageSeoSettings = defaultImageSeoSettings
): ProcessedImage[] {
	return images.map((image) => processImage(image, settings));
}

/**
 * Generate HTML img tag with SEO attributes
 */
export function generateImageHtml(
	processed: ProcessedImage,
	additionalClasses?: string,
	lazy: boolean = true
): string {
	const attrs: string[] = [];

	attrs.push(`src="${processed.src}"`);
	attrs.push(`alt="${escapeHtml(processed.generatedAlt)}"`);

	if (processed.generatedTitle) {
		attrs.push(`title="${escapeHtml(processed.generatedTitle)}"`);
	}

	if (processed.width) attrs.push(`width="${processed.width}"`);
	if (processed.height) attrs.push(`height="${processed.height}"`);
	if (additionalClasses) attrs.push(`class="${additionalClasses}"`);
	if (lazy) attrs.push('loading="lazy"');

	return `<img ${attrs.join(' ')} />`;
}

/**
 * Escape HTML special characters
 */
function escapeHtml(text: string): string {
	return text
		.replace(/&/g, '&amp;')
		.replace(/</g, '&lt;')
		.replace(/>/g, '&gt;')
		.replace(/"/g, '&quot;')
		.replace(/'/g, '&#39;');
}

// ═══════════════════════════════════════════════════════════════════════════
// Svelte Store for Settings
// ═══════════════════════════════════════════════════════════════════════════

import { writable, get } from 'svelte/store';
import { browser } from '$app/environment';

const STORAGE_KEY = 'image-seo-settings';

function createImageSeoStore() {
	// Load from localStorage if available
	let initial = defaultImageSeoSettings;
	if (browser) {
		const stored = localStorage.getItem(STORAGE_KEY);
		if (stored) {
			try {
				initial = { ...defaultImageSeoSettings, ...JSON.parse(stored) };
			} catch {
				// Use defaults on parse error
			}
		}
	}

	const { subscribe, set, update } = writable<ImageSeoSettings>(initial);

	return {
		subscribe,
		set: (value: ImageSeoSettings) => {
			if (browser) {
				localStorage.setItem(STORAGE_KEY, JSON.stringify(value));
			}
			set(value);
		},
		update: (fn: (value: ImageSeoSettings) => ImageSeoSettings) => {
			update((current) => {
				const newValue = fn(current);
				if (browser) {
					localStorage.setItem(STORAGE_KEY, JSON.stringify(newValue));
				}
				return newValue;
			});
		},
		reset: () => {
			if (browser) {
				localStorage.removeItem(STORAGE_KEY);
			}
			set(defaultImageSeoSettings);
		},
		get: () => get({ subscribe })
	};
}

export const imageSeoSettings = createImageSeoStore();

// ═══════════════════════════════════════════════════════════════════════════
// Export Default
// ═══════════════════════════════════════════════════════════════════════════

export default {
	extractFilename,
	cleanFilename,
	applyCase,
	toTitleCase,
	toSentenceCase,
	generateAltText,
	generateTitle,
	generateCaption,
	generateAvatarAlt,
	generateSeoFilename,
	calculateImageSeoScore,
	analyzeImageSeo,
	processImage,
	processImages,
	generateImageHtml,
	settings: imageSeoSettings,
	defaultSettings: defaultImageSeoSettings
};

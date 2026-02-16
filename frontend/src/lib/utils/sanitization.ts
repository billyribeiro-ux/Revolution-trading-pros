/**
 * Content Sanitization Utilities
 * ═══════════════════════════════════════════════════════════════════════════
 * Comprehensive security utilities for content sanitization, XSS prevention,
 * input validation, and Content Security Policy management.
 */

import DOMPurify, { type Config } from 'isomorphic-dompurify';
import { logger } from '$lib/utils/logger';

// ============================================================================
// Types
// ============================================================================

export type SanitizationMode = 'default' | 'strict' | 'custom';

export interface SanitizeOptions {
	/**
	 * Sanitization mode
	 * - 'default': Standard sanitization for rich content
	 * - 'strict': Minimal tags, text-only content
	 * - 'custom': Allow custom HTML (iframes, etc.) - use with caution
	 */
	mode?: SanitizationMode;

	/**
	 * Custom DOMPurify config to merge with defaults
	 */
	config?: Partial<Config>;

	/**
	 * Whether to add noopener/noreferrer to links
	 */
	secureLinks?: boolean;

	/**
	 * Maximum allowed length (truncate if exceeded)
	 */
	maxLength?: number;
}

export interface FileValidationOptions {
	/** Maximum file size in bytes (default: 10MB) */
	maxSize?: number;
	/** Allowed MIME types */
	allowedTypes?: string[];
	/** Allowed file extensions (without dot) */
	allowedExtensions?: string[];
}

export interface FileValidationResult {
	valid: boolean;
	error?: string;
	sanitizedName?: string;
}

export interface CSPConfig {
	/** Nonce for inline scripts/styles */
	nonce?: string;
	/** Additional allowed domains */
	allowedDomains?: string[];
	/** Report URI for CSP violations */
	reportUri?: string;
	/** Whether to use report-only mode */
	reportOnly?: boolean;
}

export interface CSPDirectives {
	'default-src'?: string[];
	'script-src'?: string[];
	'style-src'?: string[];
	'img-src'?: string[];
	'font-src'?: string[];
	'connect-src'?: string[];
	'frame-src'?: string[];
	'object-src'?: string[];
	'base-uri'?: string[];
	'form-action'?: string[];
	'frame-ancestors'?: string[];
	'report-uri'?: string[];
	[key: string]: string[] | undefined;
}

// ============================================================================
// Configuration
// ============================================================================

/** Safe protocols for URLs */
const SAFE_PROTOCOLS = ['http:', 'https:', 'mailto:', 'tel:', 'sms:'] as const;

/** Dangerous CSS patterns that could enable injection attacks */
const DANGEROUS_CSS_PATTERNS = [
	/javascript\s*:/gi,
	/expression\s*\(/gi,
	/behavior\s*:/gi,
	/@import/gi,
	/binding\s*:/gi,
	/-moz-binding/gi,
	/url\s*\(\s*["']?\s*data:/gi,
	/url\s*\(\s*["']?\s*javascript:/gi,
	/<script/gi,
	/<\/script/gi,
	/<!--/g,
	/-->/g,
	/\\[0-9a-f]{1,6}/gi // Encoded characters
] as const;

/** XSS attack patterns for detection */
const XSS_PATTERNS = [
	/<script[\s\S]*?>[\s\S]*?<\/script>/gi,
	/<script[\s\S]*?>/gi,
	/javascript\s*:/gi,
	/on\w+\s*=/gi,
	/<iframe[\s\S]*?>/gi,
	/<object[\s\S]*?>/gi,
	/<embed[\s\S]*?>/gi,
	/<link[\s\S]*?>/gi,
	/<meta[\s\S]*?>/gi,
	/<base[\s\S]*?>/gi,
	/data\s*:\s*text\/html/gi,
	/expression\s*\(/gi,
	/vbscript\s*:/gi
] as const;

/** Default DOMPurify configuration for standard content */
const DEFAULT_CONFIG: Config = {
	ALLOWED_TAGS: [
		'p',
		'br',
		'strong',
		'em',
		'u',
		's',
		'a',
		'ul',
		'ol',
		'li',
		'h1',
		'h2',
		'h3',
		'h4',
		'h5',
		'h6',
		'blockquote',
		'code',
		'pre',
		'div',
		'span',
		'img',
		'table',
		'thead',
		'tbody',
		'tfoot',
		'tr',
		'th',
		'td',
		'figure',
		'figcaption',
		'hr',
		'sub',
		'sup',
		'mark',
		'abbr',
		'cite',
		'del',
		'ins',
		'kbd',
		'samp',
		'var',
		'details',
		'summary'
	],
	ALLOWED_ATTR: [
		'href',
		'title',
		'target',
		'rel',
		'class',
		'id',
		'src',
		'alt',
		'width',
		'height',
		'style',
		'colspan',
		'rowspan',
		'scope',
		'datetime',
		'cite',
		'lang',
		'dir',
		'open'
	],
	ALLOWED_URI_REGEXP:
		/^(?:(?:(?:f|ht)tps?|mailto|tel|callto|sms|cid|xmpp):|[^a-z]|[a-z+.-]+(?:[^a-z+.-:]|$))/i,
	ALLOW_DATA_ATTR: false,
	ALLOW_UNKNOWN_PROTOCOLS: false,
	SAFE_FOR_TEMPLATES: true,
	RETURN_TRUSTED_TYPE: false
};

/** Strict DOMPurify configuration - text only with basic formatting */
const STRICT_CONFIG: Config = {
	ALLOWED_TAGS: ['p', 'br', 'strong', 'em', 'u', 's', 'sub', 'sup'],
	ALLOWED_ATTR: [],
	ALLOW_DATA_ATTR: false,
	ALLOW_UNKNOWN_PROTOCOLS: false,
	SAFE_FOR_TEMPLATES: true,
	KEEP_CONTENT: true
};

/** Custom HTML configuration - allows embeds with security restrictions */
const CUSTOM_HTML_CONFIG: Config = {
	...DEFAULT_CONFIG,
	ALLOWED_TAGS: [
		...(DEFAULT_CONFIG.ALLOWED_TAGS as string[]),
		'iframe',
		'video',
		'audio',
		'source',
		'track',
		'picture',
		'canvas'
	],
	ALLOWED_ATTR: [
		...(DEFAULT_CONFIG.ALLOWED_ATTR as string[]),
		'frameborder',
		'allowfullscreen',
		'controls',
		'autoplay',
		'muted',
		'loop',
		'playsinline',
		'poster',
		'preload',
		'srcset',
		'sizes',
		'media',
		'type',
		'kind',
		'srclang',
		'label',
		'default',
		'loading',
		'decoding',
		'crossorigin',
		'referrerpolicy',
		'sandbox',
		'allow'
	],
	ADD_TAGS: ['iframe', 'video', 'audio'],
	ADD_ATTR: ['allow', 'allowfullscreen', 'frameborder', 'sandbox']
};

// ============================================================================
// HTML Sanitization Functions
// ============================================================================

/**
 * Sanitize HTML content using DOMPurify with configurable security levels
 * @param html - The HTML string to sanitize
 * @param options - Sanitization options
 * @returns Sanitized HTML string
 */
export function sanitizeHTML(html: string, options: SanitizeOptions = {}): string {
	const { mode = 'default', config = {}, secureLinks = true, maxLength } = options;

	if (!html || typeof html !== 'string') {
		return '';
	}

	// Truncate if needed
	const content = maxLength ? html.slice(0, maxLength) : html;

	// Select base config based on mode
	let baseConfig: Config;
	switch (mode) {
		case 'strict':
			baseConfig = STRICT_CONFIG;
			break;
		case 'custom':
			baseConfig = CUSTOM_HTML_CONFIG;
			break;
		default:
			baseConfig = DEFAULT_CONFIG;
	}

	// Merge configs
	const finalConfig: Config = {
		...baseConfig,
		...config
	};

	// Sanitize
	let sanitized = DOMPurify.sanitize(content, finalConfig) as string;

	// Add security attributes to links
	if (secureLinks && mode !== 'strict') {
		sanitized = addSecureLinkAttributes(sanitized);
	}

	return sanitized;
}

/**
 * Sanitize plain text by stripping all HTML tags
 * @param text - The text to sanitize
 * @param maxLength - Optional maximum length
 * @returns Plain text with all HTML removed
 */
export function sanitizeText(text: string, maxLength?: number): string {
	if (!text || typeof text !== 'string') {
		return '';
	}

	// Strip all HTML tags while keeping content
	const stripped = DOMPurify.sanitize(text, {
		ALLOWED_TAGS: [],
		ALLOWED_ATTR: [],
		KEEP_CONTENT: true
	}) as string;

	return maxLength ? stripped.slice(0, maxLength) : stripped;
}

/**
 * Add secure attributes (noopener, noreferrer) to links with target="_blank"
 */
function addSecureLinkAttributes(html: string): string {
	return html.replace(/<a\s+([^>]*href=["'][^"']*["'][^>]*)>/gi, (match, attributes) => {
		// Check if target="_blank" exists
		if (attributes.includes('target="_blank"') || attributes.includes("target='_blank'")) {
			// Add rel if not present
			if (!attributes.includes('rel=')) {
				return `<a ${attributes} rel="noopener noreferrer">`;
			} else if (!attributes.includes('noopener') || !attributes.includes('noreferrer')) {
				// Update existing rel attribute
				return match.replace(/rel=["']([^"']*)["']/i, (_: string, relValue: string) => {
					const parts = new Set(relValue.split(/\s+/).filter(Boolean));
					parts.add('noopener');
					parts.add('noreferrer');
					return `rel="${Array.from(parts).join(' ')}"`;
				});
			}
		}
		return match;
	});
}

// ============================================================================
// URL Sanitization
// ============================================================================

/**
 * Sanitize and validate a URL
 * @param url - The URL to sanitize
 * @param additionalSafeProtocols - Additional protocols to allow beyond defaults
 * @returns Sanitized URL or empty string if invalid/unsafe
 */
export function sanitizeURL(url: string, additionalSafeProtocols: string[] = []): string {
	if (!url || typeof url !== 'string') {
		return '';
	}

	const trimmedUrl = url.trim();

	// Check for javascript: and other dangerous protocols before parsing
	const lowercaseUrl = trimmedUrl.toLowerCase();
	if (
		lowercaseUrl.startsWith('javascript:') ||
		lowercaseUrl.startsWith('vbscript:') ||
		lowercaseUrl.startsWith('data:text/html')
	) {
		logger.warn('Dangerous URL protocol blocked:', trimmedUrl.slice(0, 20));
		return '';
	}

	try {
		const urlObj = new URL(trimmedUrl);

		// Combine safe protocols
		const allSafeProtocols = [...SAFE_PROTOCOLS, ...additionalSafeProtocols];

		// Validate protocol
		if (!allSafeProtocols.includes(urlObj.protocol as (typeof SAFE_PROTOCOLS)[number])) {
			logger.warn('Unsafe URL protocol blocked:', urlObj.protocol);
			return '';
		}

		// Return normalized URL
		return urlObj.href;
	} catch {
		// If URL parsing fails, it might be a relative URL
		// Only allow relative URLs starting with / or alphanumeric
		if (/^[/a-z0-9]/i.test(trimmedUrl) && !lowercaseUrl.includes(':')) {
			return trimmedUrl;
		}
		return '';
	}
}

/**
 * Check if a protocol is considered safe
 */
export function isSafeProtocol(protocol: string): boolean {
	const normalizedProtocol = protocol.endsWith(':') ? protocol : `${protocol}:`;
	return SAFE_PROTOCOLS.includes(normalizedProtocol as (typeof SAFE_PROTOCOLS)[number]);
}

// ============================================================================
// CSS Sanitization
// ============================================================================

/**
 * Sanitize CSS to prevent injection attacks
 * @param css - The CSS string to sanitize
 * @returns Sanitized CSS string
 */
export function sanitizeCSS(css: string): string {
	if (!css || typeof css !== 'string') {
		return '';
	}

	let cleaned = css;

	// Remove dangerous patterns
	for (const pattern of DANGEROUS_CSS_PATTERNS) {
		cleaned = cleaned.replace(pattern, '');
	}

	// Remove null bytes
	cleaned = cleaned.replace(/\0/g, '');

	// Remove unicode escape sequences that could be used to bypass filters
	cleaned = cleaned.replace(/\\[0-9a-f]{1,6}\s?/gi, '');

	// Validate remaining content doesn't contain dangerous constructs
	if (containsDangerousCSS(cleaned)) {
		logger.warn('CSS contained dangerous constructs after sanitization');
		return '';
	}

	return cleaned.trim();
}

/**
 * Check if CSS contains dangerous constructs
 */
function containsDangerousCSS(css: string): boolean {
	const lowercaseCSS = css.toLowerCase();
	return (
		lowercaseCSS.includes('javascript') ||
		lowercaseCSS.includes('expression') ||
		lowercaseCSS.includes('behavior') ||
		lowercaseCSS.includes('binding') ||
		lowercaseCSS.includes('<script') ||
		lowercaseCSS.includes('</script')
	);
}

/**
 * Sanitize inline style attribute value
 * @param style - The style attribute value to sanitize
 * @returns Sanitized style string
 */
export function sanitizeInlineStyle(style: string): string {
	if (!style || typeof style !== 'string') {
		return '';
	}

	// Split into individual declarations
	const declarations = style.split(';').filter(Boolean);
	const safeDeclarations: string[] = [];

	for (const declaration of declarations) {
		const [property, ...valueParts] = declaration.split(':');
		if (!property || valueParts.length === 0) continue;

		const propName = property.trim().toLowerCase();
		const value = valueParts.join(':').trim();

		// Skip dangerous properties
		if (
			propName.startsWith('behavior') ||
			propName.startsWith('-moz-binding') ||
			propName.includes('expression')
		) {
			continue;
		}

		// Sanitize the value
		const sanitizedValue = sanitizeCSS(value);
		if (sanitizedValue) {
			safeDeclarations.push(`${propName}: ${sanitizedValue}`);
		}
	}

	return safeDeclarations.join('; ');
}

// ============================================================================
// File Validation
// ============================================================================

/** Default maximum file size (10MB) */
const DEFAULT_MAX_FILE_SIZE = 10 * 1024 * 1024;

/** Common safe image MIME types */
export const SAFE_IMAGE_TYPES = [
	'image/jpeg',
	'image/png',
	'image/gif',
	'image/webp',
	'image/svg+xml',
	'image/avif'
] as const;

/** Common safe document MIME types */
export const SAFE_DOCUMENT_TYPES = [
	'application/pdf',
	'application/msword',
	'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
	'application/vnd.ms-excel',
	'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
	'text/plain',
	'text/csv'
] as const;

/**
 * Validate and check file for upload
 * @param file - The File object to validate
 * @param options - Validation options
 * @returns Validation result with sanitized filename
 */
export function validateFile(
	file: File,
	options: FileValidationOptions = {}
): FileValidationResult {
	const { maxSize = DEFAULT_MAX_FILE_SIZE, allowedTypes, allowedExtensions } = options;

	// Check if file exists
	if (!file || !(file instanceof File)) {
		return {
			valid: false,
			error: 'Invalid file object'
		};
	}

	// Check file size
	if (file.size > maxSize) {
		const maxSizeMB = (maxSize / 1024 / 1024).toFixed(2);
		const fileSizeMB = (file.size / 1024 / 1024).toFixed(2);
		return {
			valid: false,
			error: `File too large (${fileSizeMB}MB). Maximum size: ${maxSizeMB}MB`
		};
	}

	// Check file size is not zero
	if (file.size === 0) {
		return {
			valid: false,
			error: 'File is empty'
		};
	}

	// Check MIME type
	if (allowedTypes && allowedTypes.length > 0) {
		if (!allowedTypes.includes(file.type)) {
			return {
				valid: false,
				error: `Invalid file type "${file.type}". Allowed: ${allowedTypes.join(', ')}`
			};
		}
	}

	// Get and check file extension
	const ext = getFileExtension(file.name);
	if (allowedExtensions && allowedExtensions.length > 0) {
		if (!ext || !allowedExtensions.includes(ext)) {
			return {
				valid: false,
				error: `Invalid file extension "${ext || 'none'}". Allowed: ${allowedExtensions.join(', ')}`
			};
		}
	}

	// Sanitize filename
	const sanitizedName = sanitizeFilename(file.name);

	return {
		valid: true,
		sanitizedName
	};
}

/**
 * Get file extension from filename
 */
function getFileExtension(filename: string): string | null {
	const parts = filename.split('.');
	if (parts.length < 2) return null;
	return parts.pop()?.toLowerCase() || null;
}

// ============================================================================
// Filename Sanitization
// ============================================================================

/**
 * Sanitize a filename to prevent path traversal and other attacks
 * @param filename - The filename to sanitize
 * @returns Sanitized filename safe for storage
 */
export function sanitizeFilename(filename: string): string {
	if (!filename || typeof filename !== 'string') {
		return 'file';
	}

	// Get original extension
	const lastDotIndex = filename.lastIndexOf('.');
	const hasExtension = lastDotIndex > 0 && lastDotIndex < filename.length - 1;
	const extension = hasExtension ? filename.slice(lastDotIndex + 1).toLowerCase() : '';
	const baseName = hasExtension ? filename.slice(0, lastDotIndex) : filename;

	// Sanitize base name
	let cleaned = baseName
		// Remove path separators (prevents path traversal)
		.replace(/[/\\]/g, '-')
		// Remove null bytes
		.replace(/\0/g, '')
		// Remove control characters
		.replace(/[\x00-\x1f\x7f]/g, '')
		// Remove Windows reserved characters
		.replace(/[<>:"|?*]/g, '-')
		// Remove percentage signs (prevents URL encoding attacks)
		.replace(/%/g, '-')
		// Replace multiple consecutive dots with single (prevents hidden file creation)
		.replace(/\.{2,}/g, '.')
		// Remove leading dots (prevents hidden files)
		.replace(/^\.+/, '')
		// Replace whitespace with underscores
		.replace(/\s+/g, '_')
		// Remove consecutive dashes/underscores
		.replace(/[-_]{2,}/g, '-')
		// Convert to lowercase
		.toLowerCase()
		// Trim dashes and underscores from ends
		.replace(/^[-_]+|[-_]+$/g, '');

	// Ensure we have a valid base name
	if (!cleaned || cleaned.length === 0) {
		cleaned = 'file';
	}

	// Sanitize extension
	const cleanExtension = extension.replace(/[^a-z0-9]/g, '').slice(0, 10); // Limit extension length

	// Combine and limit total length (255 is common filesystem limit)
	const maxBaseLength = 255 - (cleanExtension ? cleanExtension.length + 1 : 0);
	const finalBase = cleaned.slice(0, maxBaseLength);

	return cleanExtension ? `${finalBase}.${cleanExtension}` : finalBase;
}

/**
 * Check if a filename contains path traversal attempts
 */
export function containsPathTraversal(path: string): boolean {
	if (!path || typeof path !== 'string') {
		return false;
	}

	const normalized = path.toLowerCase();
	return (
		normalized.includes('..') ||
		normalized.includes('./') ||
		normalized.includes('.\\') ||
		normalized.startsWith('/') ||
		normalized.startsWith('\\') ||
		/^[a-z]:/i.test(normalized) || // Windows drive letter
		normalized.includes('\0') // Null byte
	);
}

// ============================================================================
// CSP (Content Security Policy) Utilities
// ============================================================================

/**
 * Generate a cryptographically secure CSP nonce
 * @returns Base64-encoded nonce string
 */
export function generateCSPNonce(): string {
	const array = new Uint8Array(16);
	crypto.getRandomValues(array);
	// Convert to base64-like string
	return Array.from(array, (byte) => byte.toString(16).padStart(2, '0')).join('');
}

/**
 * Build a Content Security Policy header string
 * @param config - CSP configuration options
 * @returns CSP header value string
 */
export function buildCSP(config: CSPConfig = {}): string {
	const { nonce, allowedDomains = [], reportUri } = config;

	const directives: CSPDirectives = {
		'default-src': ["'self'"],
		'script-src': nonce ? ["'self'", `'nonce-${nonce}'`] : ["'self'"],
		'style-src': nonce
			? ["'self'", `'nonce-${nonce}'`, "'unsafe-inline'"]
			: ["'self'", "'unsafe-inline'"],
		'img-src': ["'self'", 'data:', 'https:', ...allowedDomains],
		'font-src': ["'self'", 'data:'],
		'connect-src': ["'self'", ...allowedDomains],
		'frame-src': allowedDomains.length > 0 ? allowedDomains : ["'none'"],
		'object-src': ["'none'"],
		'base-uri': ["'self'"],
		'form-action': ["'self'"],
		'frame-ancestors': ["'none'"]
	};

	// Add report URI if specified
	if (reportUri) {
		directives['report-uri'] = [reportUri];
	}

	// Build the header string
	const directiveStrings = Object.entries(directives)
		.filter(([, values]) => values && values.length > 0)
		.map(([key, values]) => `${key} ${values!.join(' ')}`);

	// Add upgrade-insecure-requests
	directiveStrings.push('upgrade-insecure-requests');

	const cspValue = directiveStrings.join('; ');

	// Note: The actual header name would be set by the server
	// This returns just the value
	return cspValue;
}

/**
 * Build CSP header with custom directives
 * @param directives - Custom CSP directives
 * @returns CSP header value string
 */
export function buildCustomCSP(directives: CSPDirectives): string {
	return Object.entries(directives)
		.filter(([, values]) => values && values.length > 0)
		.map(([key, values]) => `${key} ${values!.join(' ')}`)
		.join('; ');
}

// ============================================================================
// XSS Prevention Utilities
// ============================================================================

/**
 * HTML entity map for escaping
 */
const HTML_ESCAPE_MAP: Record<string, string> = {
	'&': '&amp;',
	'<': '&lt;',
	'>': '&gt;',
	'"': '&quot;',
	"'": '&#39;',
	'/': '&#x2F;',
	'`': '&#x60;',
	'=': '&#x3D;'
};

/**
 * Reverse map for unescaping
 */
const HTML_UNESCAPE_MAP: Record<string, string> = {
	'&amp;': '&',
	'&lt;': '<',
	'&gt;': '>',
	'&quot;': '"',
	'&#39;': "'",
	'&#x27;': "'",
	'&#x2F;': '/',
	'&#x60;': '`',
	'&#x3D;': '=',
	'&apos;': "'"
};

/**
 * Escape HTML special characters to prevent XSS
 * @param str - The string to escape
 * @returns Escaped string safe for HTML insertion
 */
export function escapeHTML(str: string): string {
	if (!str || typeof str !== 'string') {
		return '';
	}

	return str.replace(/[&<>"'`=/]/g, (char) => HTML_ESCAPE_MAP[char] || char);
}

/**
 * Unescape HTML entities back to their original characters
 * @param str - The escaped string to unescape
 * @returns Unescaped string
 */
export function unescapeHTML(str: string): string {
	if (!str || typeof str !== 'string') {
		return '';
	}

	return str.replace(
		/&(?:amp|lt|gt|quot|#39|#x27|#x2F|#x60|#x3D|apos);/gi,
		(entity) => HTML_UNESCAPE_MAP[entity.toLowerCase()] || entity
	);
}

/**
 * Check if a string contains potential XSS attack patterns
 * @param str - The string to check
 * @returns True if XSS patterns detected, false otherwise
 */
export function containsXSS(str: string): boolean {
	if (!str || typeof str !== 'string') {
		return false;
	}

	// Normalize the string for checking
	const normalized = str
		// Decode HTML entities
		.replace(/&#x([0-9a-f]+);?/gi, (_, hex) => String.fromCharCode(parseInt(hex, 16)))
		.replace(/&#(\d+);?/g, (_, dec) => String.fromCharCode(parseInt(dec, 10)))
		// Decode URL encoding
		.replace(/%([0-9a-f]{2})/gi, (_, hex) => String.fromCharCode(parseInt(hex, 16)));

	// Check against XSS patterns
	for (const pattern of XSS_PATTERNS) {
		if (pattern.test(normalized)) {
			return true;
		}
	}

	// Additional checks for encoded attacks
	const lowercased = normalized.toLowerCase();
	return (
		lowercased.includes('javascript:') ||
		lowercased.includes('vbscript:') ||
		lowercased.includes('data:text/html') ||
		lowercased.includes('expression(') ||
		/on\w+\s*=/.test(lowercased)
	);
}

/**
 * Strip all potentially dangerous content from a string
 * @param str - The string to strip
 * @returns Safe string with all dangerous content removed
 */
export function stripDangerous(str: string): string {
	if (!str || typeof str !== 'string') {
		return '';
	}

	// Remove null bytes
	let cleaned = str.replace(/\0/g, '');

	// Remove script tags and content
	cleaned = cleaned.replace(/<script[\s\S]*?>[\s\S]*?<\/script>/gi, '');

	// Remove event handlers
	cleaned = cleaned.replace(/\s*on\w+\s*=\s*["'][^"']*["']/gi, '');
	cleaned = cleaned.replace(/\s*on\w+\s*=\s*[^\s>]*/gi, '');

	// Remove javascript: URLs
	cleaned = cleaned.replace(/javascript\s*:/gi, '');

	// Remove vbscript: URLs
	cleaned = cleaned.replace(/vbscript\s*:/gi, '');

	// Remove data: URLs with HTML content
	cleaned = cleaned.replace(/data\s*:\s*text\/html[^"'\s>]*/gi, '');

	return cleaned;
}

// ============================================================================
// Input Validation Helpers
// ============================================================================

/**
 * Validate email address format
 * @param email - The email to validate
 * @returns True if valid email format
 */
export function isValidEmail(email: string): boolean {
	if (!email || typeof email !== 'string') {
		return false;
	}

	// RFC 5322 compliant email regex (simplified but robust)
	const emailRegex =
		/^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$/;

	// Basic length checks
	if (email.length > 254) return false;

	const [local, domain] = email.split('@');
	if (!local || !domain) return false;
	if (local.length > 64) return false;

	return emailRegex.test(email);
}

/**
 * Validate URL format
 * @param url - The URL to validate
 * @param requireHTTPS - Whether to require HTTPS protocol
 * @returns True if valid URL format
 */
export function isValidURL(url: string, requireHTTPS = false): boolean {
	if (!url || typeof url !== 'string') {
		return false;
	}

	try {
		const urlObj = new URL(url);

		// Check protocol
		if (requireHTTPS && urlObj.protocol !== 'https:') {
			return false;
		}

		// Only allow http and https
		if (!['http:', 'https:'].includes(urlObj.protocol)) {
			return false;
		}

		// Basic hostname validation
		if (!urlObj.hostname || urlObj.hostname.length === 0) {
			return false;
		}

		return true;
	} catch {
		return false;
	}
}

/**
 * Validate hexadecimal color format
 * @param color - The color string to validate
 * @returns True if valid hex color (with or without #)
 */
export function isValidHexColor(color: string): boolean {
	if (!color || typeof color !== 'string') {
		return false;
	}

	// Match 3, 4, 6, or 8 character hex colors with optional #
	const hexColorRegex = /^#?([0-9A-Fa-f]{3}|[0-9A-Fa-f]{4}|[0-9A-Fa-f]{6}|[0-9A-Fa-f]{8})$/;

	return hexColorRegex.test(color.trim());
}

/**
 * Sanitize and parse a number from string input
 * @param value - The value to parse
 * @param options - Sanitization options
 * @returns Sanitized number or default value
 */
export function sanitizeNumber(
	value: string | number | null | undefined,
	options: {
		min?: number;
		max?: number;
		defaultValue?: number;
		allowFloat?: boolean;
		precision?: number;
	} = {}
): number {
	const {
		min = -Infinity,
		max = Infinity,
		defaultValue = 0,
		allowFloat = true,
		precision
	} = options;

	// Handle null/undefined
	if (value === null || value === undefined) {
		return defaultValue;
	}

	// Parse the value
	let num: number;
	if (typeof value === 'number') {
		num = value;
	} else {
		// Remove any non-numeric characters except . and -
		const cleaned = String(value).replace(/[^\d.-]/g, '');
		num = allowFloat ? parseFloat(cleaned) : parseInt(cleaned, 10);
	}

	// Check for NaN
	if (Number.isNaN(num)) {
		return defaultValue;
	}

	// Check for Infinity
	if (!Number.isFinite(num)) {
		return defaultValue;
	}

	// Clamp to range
	num = Math.max(min, Math.min(max, num));

	// Apply precision if specified
	if (precision !== undefined && allowFloat) {
		num = Number(num.toFixed(precision));
	}

	return num;
}

/**
 * Validate and sanitize a slug (URL-safe identifier)
 * @param slug - The slug to validate
 * @returns Sanitized slug or null if invalid
 */
export function sanitizeSlug(slug: string): string | null {
	if (!slug || typeof slug !== 'string') {
		return null;
	}

	// Convert to lowercase and sanitize
	let cleaned = slug
		.toLowerCase()
		.trim()
		// Replace spaces with hyphens
		.replace(/\s+/g, '-')
		// Remove non-alphanumeric characters except hyphens
		.replace(/[^a-z0-9-]/g, '')
		// Remove consecutive hyphens
		.replace(/-{2,}/g, '-')
		// Remove leading/trailing hyphens
		.replace(/^-+|-+$/g, '');

	// Must have at least one character
	if (cleaned.length === 0) {
		return null;
	}

	// Limit length
	if (cleaned.length > 100) {
		cleaned = cleaned.slice(0, 100).replace(/-+$/, '');
	}

	return cleaned;
}

/**
 * Validate JSON string
 * @param jsonString - The JSON string to validate
 * @returns Parsed JSON object or null if invalid
 */
export function parseJSONSafe<T = unknown>(jsonString: string): T | null {
	if (!jsonString || typeof jsonString !== 'string') {
		return null;
	}

	try {
		return JSON.parse(jsonString) as T;
	} catch {
		return null;
	}
}

// ============================================================================
// Export all utilities
// ============================================================================

export {
	DEFAULT_CONFIG as defaultSanitizationConfig,
	STRICT_CONFIG as strictSanitizationConfig,
	CUSTOM_HTML_CONFIG as customHTMLConfig,
	SAFE_PROTOCOLS,
	DANGEROUS_CSS_PATTERNS,
	XSS_PATTERNS
};

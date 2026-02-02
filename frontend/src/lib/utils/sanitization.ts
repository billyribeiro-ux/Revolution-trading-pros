/**
 * Content Sanitization Utilities
 * ═══════════════════════════════════════════════════════════════════════════
 * DOMPurify-based sanitization for user-generated content
 */

import DOMPurify, { type Config } from 'isomorphic-dompurify';

// ============================================================================
// Configuration
// ============================================================================

const DEFAULT_CONFIG: Config = {
	ALLOWED_TAGS: [
		'p',
		'br',
		'strong',
		'em',
		'u',
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
		'tr',
		'th',
		'td'
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
		'style'
	],
	ALLOWED_URI_REGEXP:
		/^(?:(?:(?:f|ht)tps?|mailto|tel|callto|sms|cid|xmpp):|[^a-z]|[a-z+.\-]+(?:[^a-z+.\-:]|$))/i,
	ALLOW_DATA_ATTR: false,
	ALLOW_UNKNOWN_PROTOCOLS: false,
	SAFE_FOR_TEMPLATES: true,
	RETURN_TRUSTED_TYPE: false
};

const STRICT_CONFIG: Config = {
	ALLOWED_TAGS: ['p', 'br', 'strong', 'em', 'u'],
	ALLOWED_ATTR: [],
	ALLOW_DATA_ATTR: false,
	ALLOW_UNKNOWN_PROTOCOLS: false,
	SAFE_FOR_TEMPLATES: true
};

const CUSTOM_HTML_CONFIG: Config = {
	...DEFAULT_CONFIG,
	ALLOWED_TAGS: [
		...(DEFAULT_CONFIG.ALLOWED_TAGS as string[]),
		'iframe',
		'video',
		'audio',
		'source',
		'track'
	],
	ALLOWED_ATTR: [
		...(DEFAULT_CONFIG.ALLOWED_ATTR as string[]),
		'frameborder',
		'allowfullscreen',
		'controls'
	],
	ADD_TAGS: ['iframe'],
	ADD_ATTR: ['allow', 'allowfullscreen', 'frameborder']
};

// ============================================================================
// Sanitization Functions
// ============================================================================

export interface SanitizeOptions {
	/**
	 * Sanitization mode
	 * - 'default': Standard sanitization for rich content
	 * - 'strict': Minimal tags, text-only content
	 * - 'custom': Allow custom HTML (iframes, etc.) - use with caution
	 */
	mode?: 'default' | 'strict' | 'custom';

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

/**
 * Sanitize HTML content
 */
export function sanitizeHTML(html: string, options: SanitizeOptions = {}): string {
	const { mode = 'default', config = {}, secureLinks = true, maxLength } = options;

	if (!html || typeof html !== 'string') {
		return '';
	}

	// Truncate if needed
	const content = maxLength ? html.slice(0, maxLength) : html;

	// Select base config
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
 * Sanitize plain text (strip all HTML)
 */
export function sanitizeText(text: string, maxLength?: number): string {
	if (!text || typeof text !== 'string') {
		return '';
	}

	// Strip all HTML tags
	const stripped = DOMPurify.sanitize(text, {
		ALLOWED_TAGS: [],
		ALLOWED_ATTR: [],
		KEEP_CONTENT: true
	}) as string;

	return maxLength ? stripped.slice(0, maxLength) : stripped;
}

/**
 * Sanitize URL
 */
export function sanitizeURL(url: string): string {
	if (!url || typeof url !== 'string') {
		return '';
	}

	try {
		const urlObj = new URL(url);

		// Only allow safe protocols
		const safeProtocols = ['http:', 'https:', 'mailto:', 'tel:'];
		if (!safeProtocols.includes(urlObj.protocol)) {
			console.warn('Unsafe URL protocol blocked:', urlObj.protocol);
			return '';
		}

		return urlObj.href;
	} catch {
		// Invalid URL
		return '';
	}
}

/**
 * Sanitize CSS (for inline styles)
 */
export function sanitizeCSS(css: string): string {
	if (!css || typeof css !== 'string') {
		return '';
	}

	// Basic CSS injection prevention
	const dangerous = ['javascript:', 'expression(', 'behavior:', '@import', '<', '>', '\\', '&'];

	let cleaned = css;
	for (const pattern of dangerous) {
		cleaned = cleaned.replace(new RegExp(pattern, 'gi'), '');
	}

	return cleaned;
}

/**
 * Validate and sanitize file upload
 */
export interface FileValidationOptions {
	maxSize?: number; // in bytes
	allowedTypes?: string[]; // MIME types
	allowedExtensions?: string[];
}

export interface FileValidationResult {
	valid: boolean;
	error?: string;
	sanitizedName?: string;
}

export function validateFile(
	file: File,
	options: FileValidationOptions = {}
): FileValidationResult {
	const { maxSize = 10 * 1024 * 1024, allowedTypes, allowedExtensions } = options;

	// Check size
	if (file.size > maxSize) {
		return {
			valid: false,
			error: `File too large. Maximum size: ${(maxSize / 1024 / 1024).toFixed(2)}MB`
		};
	}

	// Check MIME type
	if (allowedTypes && !allowedTypes.includes(file.type)) {
		return {
			valid: false,
			error: `Invalid file type. Allowed: ${allowedTypes.join(', ')}`
		};
	}

	// Check extension
	const ext = file.name.split('.').pop()?.toLowerCase();
	if (allowedExtensions && ext && !allowedExtensions.includes(ext)) {
		return {
			valid: false,
			error: `Invalid file extension. Allowed: ${allowedExtensions.join(', ')}`
		};
	}

	// Sanitize filename
	const sanitizedName = sanitizeFilename(file.name);

	return {
		valid: true,
		sanitizedName
	};
}

/**
 * Sanitize filename
 */
export function sanitizeFilename(filename: string): string {
	if (!filename || typeof filename !== 'string') {
		return 'file';
	}

	// Remove path separators and dangerous characters
	let cleaned = filename
		.replace(/[/\\?%*:|"<>]/g, '-')
		.replace(/\s+/g, '_')
		.replace(/\.{2,}/g, '.')
		.replace(/^\.+/, '')
		.toLowerCase();

	// Ensure it has an extension
	if (!cleaned.includes('.')) {
		cleaned += '.tmp';
	}

	// Limit length
	if (cleaned.length > 255) {
		const ext = cleaned.split('.').pop();
		const name = cleaned.slice(0, 255 - (ext?.length ?? 0) - 1);
		cleaned = `${name}.${ext}`;
	}

	return cleaned;
}

// ============================================================================
// Helper Functions
// ============================================================================

function addSecureLinkAttributes(html: string): string {
	// This is a simple regex-based approach
	// In production, consider using a proper HTML parser
	return html.replace(/<a\s+([^>]*href=["'][^"']*["'][^>]*)>/gi, (match, attributes) => {
		// Check if target="_blank" exists
		if (attributes.includes('target="_blank"') || attributes.includes("target='_blank'")) {
			// Add rel if not present
			if (!attributes.includes('rel=')) {
				return `<a ${attributes} rel="noopener noreferrer">`;
			} else if (!attributes.includes('noopener') || !attributes.includes('noreferrer')) {
				// Add missing attributes
				return match.replace(/rel=["']([^"']*)["']/i, 'rel="$1 noopener noreferrer"');
			}
		}
		return match;
	});
}

// ============================================================================
// Content Security Policy Helpers
// ============================================================================

export function generateCSPNonce(): string {
	const array = new Uint8Array(16);
	crypto.getRandomValues(array);
	return Array.from(array, (byte) => byte.toString(16).padStart(2, '0')).join('');
}

export interface CSPConfig {
	nonce?: string;
	allowedDomains?: string[];
}

export function buildCSP(config: CSPConfig = {}): string {
	const { nonce, allowedDomains = [] } = config;

	const directives = [
		"default-src 'self'",
		nonce ? `script-src 'self' 'nonce-${nonce}'` : "script-src 'self'",
		nonce
			? `style-src 'self' 'nonce-${nonce}' 'unsafe-inline'`
			: "style-src 'self' 'unsafe-inline'",
		`img-src 'self' data: https: ${allowedDomains.join(' ')}`,
		`font-src 'self' data:`,
		`connect-src 'self' ${allowedDomains.join(' ')}`,
		`frame-src ${allowedDomains.join(' ')}`,
		"object-src 'none'",
		"base-uri 'self'",
		"form-action 'self'",
		"frame-ancestors 'none'",
		'upgrade-insecure-requests'
	];

	return directives.join('; ');
}

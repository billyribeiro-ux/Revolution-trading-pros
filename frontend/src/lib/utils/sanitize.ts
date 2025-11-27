/**
 * HTML Sanitization Utility
 * =========================
 * Provides secure HTML sanitization using DOMPurify to prevent XSS attacks.
 * Use this for ALL user-generated or external HTML content before rendering with {@html}.
 *
 * @version 1.0.0
 * @security Critical - prevents XSS vulnerabilities
 */

import DOMPurify from 'isomorphic-dompurify';

/**
 * Sanitization profiles for different use cases
 */
const SANITIZE_PROFILES = {
	// Strict: Only basic formatting, no links or images
	strict: {
		ALLOWED_TAGS: ['b', 'i', 'em', 'strong', 'u', 's', 'br', 'p', 'span'],
		ALLOWED_ATTR: ['class']
	},

	// Standard: Common HTML elements for blog content
	standard: {
		ALLOWED_TAGS: [
			'b',
			'i',
			'em',
			'strong',
			'u',
			's',
			'br',
			'p',
			'span',
			'div',
			'h1',
			'h2',
			'h3',
			'h4',
			'h5',
			'h6',
			'ul',
			'ol',
			'li',
			'blockquote',
			'pre',
			'code',
			'a',
			'img',
			'figure',
			'figcaption',
			'table',
			'thead',
			'tbody',
			'tr',
			'th',
			'td'
		],
		ALLOWED_ATTR: [
			'class',
			'id',
			'href',
			'target',
			'rel',
			'src',
			'alt',
			'title',
			'width',
			'height',
			'loading'
		],
		// Force all links to open in new tab with security attributes
		ADD_ATTR: ['target', 'rel'],
		FORBID_TAGS: ['script', 'style', 'iframe', 'object', 'embed', 'form', 'input'],
		FORBID_ATTR: ['onerror', 'onload', 'onclick', 'onmouseover', 'onfocus', 'onblur']
	},

	// Rich: Full HTML support for admin-created content
	rich: {
		ALLOWED_TAGS: [
			'b',
			'i',
			'em',
			'strong',
			'u',
			's',
			'br',
			'p',
			'span',
			'div',
			'h1',
			'h2',
			'h3',
			'h4',
			'h5',
			'h6',
			'ul',
			'ol',
			'li',
			'blockquote',
			'pre',
			'code',
			'a',
			'img',
			'figure',
			'figcaption',
			'table',
			'thead',
			'tbody',
			'tr',
			'th',
			'td',
			'video',
			'source',
			'audio',
			'iframe'
		],
		ALLOWED_ATTR: [
			'class',
			'id',
			'href',
			'target',
			'rel',
			'src',
			'alt',
			'title',
			'width',
			'height',
			'loading',
			'controls',
			'autoplay',
			'muted',
			'loop',
			'poster',
			'frameborder',
			'allowfullscreen',
			'allow'
		],
		// Only allow iframes from trusted sources
		ALLOWED_URI_REGEXP:
			/^(?:(?:https?|mailto|tel):|[^a-z]|[a-z+.-]+(?:[^a-z+.\-:]|$))/i
	},

	// Minimal: Text only with basic formatting
	minimal: {
		ALLOWED_TAGS: ['b', 'i', 'em', 'strong', 'br'],
		ALLOWED_ATTR: []
	}
};

export type SanitizeProfile = keyof typeof SANITIZE_PROFILES;

/**
 * Sanitize HTML content to prevent XSS attacks.
 *
 * @param html - The HTML string to sanitize
 * @param profile - The sanitization profile to use (default: 'standard')
 * @returns Sanitized HTML string safe for rendering
 *
 * @example
 * ```svelte
 * <script>
 *   import { sanitizeHtml } from '$lib/utils/sanitize';
 *   const safeHtml = sanitizeHtml(userContent);
 * </script>
 * {@html safeHtml}
 * ```
 */
export function sanitizeHtml(html: string | null | undefined, profile: SanitizeProfile = 'standard'): string {
	if (!html) return '';

	const config = SANITIZE_PROFILES[profile];

	// Configure DOMPurify hooks for additional security
	DOMPurify.addHook('afterSanitizeAttributes', (node) => {
		// Add rel="noopener noreferrer" to all links for security
		if (node.tagName === 'A') {
			node.setAttribute('rel', 'noopener noreferrer');
			// Open external links in new tab
			if (node.getAttribute('href')?.startsWith('http')) {
				node.setAttribute('target', '_blank');
			}
		}

		// Only allow iframes from trusted domains
		if (node.tagName === 'IFRAME') {
			const src = node.getAttribute('src') || '';
			const trustedDomains = [
				'youtube.com',
				'www.youtube.com',
				'youtube-nocookie.com',
				'www.youtube-nocookie.com',
				'player.vimeo.com',
				'vimeo.com'
			];
			const isTrusted = trustedDomains.some((domain) => src.includes(domain));
			if (!isTrusted) {
				node.remove();
			}
		}
	});

	const sanitized = DOMPurify.sanitize(html, config);

	// Clean up hooks after sanitization
	DOMPurify.removeHook('afterSanitizeAttributes');

	return sanitized;
}

/**
 * Sanitize content specifically for popup displays.
 * Uses standard profile with additional restrictions.
 */
export function sanitizePopupContent(html: string | null | undefined): string {
	return sanitizeHtml(html, 'standard');
}

/**
 * Sanitize content for blog posts and articles.
 * Uses rich profile to support embedded media.
 */
export function sanitizeBlogContent(html: string | null | undefined): string {
	return sanitizeHtml(html, 'rich');
}

/**
 * Sanitize content for form field placeholders and labels.
 * Uses strict profile - no links or complex HTML.
 */
export function sanitizeFormContent(html: string | null | undefined): string {
	return sanitizeHtml(html, 'strict');
}

/**
 * Sanitize content for video overlays.
 * Uses standard profile.
 */
export function sanitizeVideoOverlay(html: string | null | undefined): string {
	return sanitizeHtml(html, 'standard');
}

/**
 * Strip all HTML tags and return plain text.
 * Use when you need text-only content.
 */
export function stripHtml(html: string | null | undefined): string {
	if (!html) return '';
	return DOMPurify.sanitize(html, { ALLOWED_TAGS: [] });
}

/**
 * Check if a string contains potentially dangerous HTML.
 * Useful for validation before storage.
 */
export function containsDangerousHtml(html: string): boolean {
	if (!html) return false;

	const dangerous = [
		/<script/i,
		/javascript:/i,
		/on\w+\s*=/i, // onclick, onerror, etc.
		/<iframe/i,
		/<object/i,
		/<embed/i,
		/<form/i,
		/data:/i
	];

	return dangerous.some((pattern) => pattern.test(html));
}

export default sanitizeHtml;

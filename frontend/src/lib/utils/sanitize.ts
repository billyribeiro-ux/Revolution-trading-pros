/**
 * HTML Sanitization Utility
 * =========================
 * Sanitizes HTML before it is rendered with {@html}. Use this for ALL
 * user-generated or external HTML content.
 *
 * Runs identically during SSR (Node, Cloudflare workerd) and in the browser —
 * see `#lib/utils/html-sanitizer/engine.ts` — so server-rendered output
 * hydrates without a mismatch. The profiles are defined in
 * `#lib/utils/html-sanitizer/policy.ts`.
 *
 * @security Critical - prevents XSS vulnerabilities
 */

import { extractText, isSafeUrl, sanitizeWithPolicy } from '#lib/utils/html-sanitizer/engine.js';
import { PROFILES } from '#lib/utils/html-sanitizer/policy.js';

export type SanitizeProfile = keyof typeof PROFILES;

/**
 * Sanitize HTML content to prevent XSS attacks.
 *
 * Every link gets `rel="noopener noreferrer"` (and `target="_blank"` when it
 * points at an http(s) URL); iframes survive only in the `rich` profile and
 * only when they embed a trusted player host over https.
 *
 * @param html - The HTML string to sanitize
 * @param profile - The sanitization profile to use (default: 'standard')
 * @returns Sanitized HTML string safe for rendering
 *
 * @example
 * ```svelte
 * <script>
 *   import { sanitizeHtml } from '#lib/utils/sanitize.js';
 *   const safeHtml = sanitizeHtml(userContent);
 * </script>
 * {@html safeHtml}
 * ```
 */
export function sanitizeHtml(
	html: string | null | undefined,
	profile: SanitizeProfile = 'standard'
): string {
	if (!html) return '';
	return sanitizeWithPolicy(html, PROFILES[profile]);
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
 * Use when you need text-only content. The result is text, not HTML: render it
 * with `{text}`, never with `{@html}`.
 */
export function stripHtml(html: string | null | undefined): string {
	if (!html) return '';
	return extractText(html);
}

/** Schemes a bare link may use; relative URLs are always allowed. */
const LINK_SCHEMES = ['http', 'https', 'mailto', 'tel'] as const;

/**
 * Validate a URL that goes straight into an `href` or `src` attribute (not
 * through the HTML sanitizer). Returns the URL unchanged when it is relative —
 * `/path`, `#anchor`, `../up`, `?query` — or uses http(s), mailto or tel, and
 * an empty string otherwise (so `javascript:` in any spelling never reaches
 * the attribute).
 */
export function safeUrl(url: string | null | undefined): string {
	if (!url) return '';
	return isSafeUrl(url, LINK_SCHEMES) ? url : '';
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

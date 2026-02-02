/**
 * String Utilities
 * ═══════════════════════════════════════════════════════════════════════════
 */

/**
 * Truncate string to specified length
 */
export function truncate(str: string, length: number, suffix = '...'): string {
	if (str.length <= length) return str;
	return str.substring(0, length - suffix.length) + suffix;
}

/**
 * Slugify text for URLs
 */
export function slugify(text: string): string {
	return text
		.toLowerCase()
		.trim()
		.replace(/[^\w\s-]/g, '')
		.replace(/[\s_-]+/g, '-')
		.replace(/^-+|-+$/g, '');
}

/**
 * Capitalize first letter
 */
export function capitalize(str: string): string {
	return str.charAt(0).toUpperCase() + str.slice(1);
}

/**
 * Convert to title case
 */
export function titleCase(str: string): string {
	return str
		.toLowerCase()
		.split(' ')
		.map((word) => capitalize(word))
		.join(' ');
}

/**
 * Convert camelCase to kebab-case
 */
export function camelToKebab(str: string): string {
	return str.replace(/([a-z0-9])([A-Z])/g, '$1-$2').toLowerCase();
}

/**
 * Convert kebab-case to camelCase
 */
export function kebabToCamel(str: string): string {
	return str.replace(/-([a-z])/g, (g) => g[1].toUpperCase());
}

/**
 * Strip HTML tags
 */
export function stripHtml(html: string): string {
	return html.replace(/<[^>]*>/g, '');
}

/**
 * Extract excerpt from text
 */
export function excerpt(text: string, length = 150): string {
	const stripped = stripHtml(text);
	return truncate(stripped, length);
}

/**
 * Count words
 */
export function wordCount(text: string): number {
	return text.trim().split(/\s+/).filter(Boolean).length;
}

/**
 * Generate random string
 */
export function randomString(length = 10): string {
	const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
	let result = '';
	for (let i = 0; i < length; i++) {
		result += chars.charAt(Math.floor(Math.random() * chars.length));
	}
	return result;
}

/**
 * SEO Plugin Layer - Canonical Engine
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Normalizes canonical URLs with deterministic rules:
 * - Always absolute URL
 * - Force HTTPS (configurable)
 * - Lowercase host
 * - Strip hash fragments
 * - Remove tracking params by denylist
 * - Preserve allowlisted query params
 * - Enforce trailing slash policy
 * - Normalize duplicate slashes
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @since February 2026
 */

import type { CanonicalConfig } from './types';

/**
 * Default tracking param denylist.
 * These are stripped from canonical URLs to prevent duplicate content.
 */
const DEFAULT_DENY_PARAMS = [
	'utm_source',
	'utm_medium',
	'utm_campaign',
	'utm_term',
	'utm_content',
	'gclid',
	'fbclid',
	'msclkid',
	'dclid',
	'twclid',
	'li_fat_id',
	'mc_cid',
	'mc_eid',
	'ref',
	'_ga',
	'_gl',
	'_hsenc',
	'_hsmi'
];

/**
 * Normalize a canonical URL according to the provided config.
 *
 * @param input - Raw URL string (absolute or relative path)
 * @param config - Canonical normalization config
 * @returns Fully normalized canonical URL string
 */
export function normalizeCanonical(input: string, config: CanonicalConfig): string {
	let url: URL;

	try {
		// Handle relative paths by prepending siteUrl
		if (input.startsWith('/')) {
			url = new URL(input, config.siteUrl);
		} else {
			url = new URL(input);
		}
	} catch {
		// Fallback: treat as path relative to siteUrl
		url = new URL('/' + input.replace(/^\/+/, ''), config.siteUrl);
	}

	// Force HTTPS
	if (config.forceHttps) {
		url.protocol = 'https:';
	}

	// Lowercase host
	url.hostname = url.hostname.toLowerCase();

	// Strip hash
	url.hash = '';

	// Build denylist set (config denylist + defaults)
	const denySet = new Set([
		...DEFAULT_DENY_PARAMS,
		...config.queryParamDenylist
	].map((p) => p.toLowerCase()));

	// Build allowlist set
	const allowSet = new Set(config.queryParamAllowlist.map((p) => p.toLowerCase()));

	// Filter query params
	const filteredParams = new URLSearchParams();
	const sortedKeys: string[] = [];

	url.searchParams.forEach((_value, key) => {
		sortedKeys.push(key);
	});
	sortedKeys.sort();

	for (const key of sortedKeys) {
		const lowerKey = key.toLowerCase();
		// If in denylist, skip (unless explicitly in allowlist)
		if (denySet.has(lowerKey) && !allowSet.has(lowerKey)) {
			continue;
		}
		const val = url.searchParams.get(key);
		if (val !== null) {
			filteredParams.set(key, val);
		}
	}

	url.search = filteredParams.toString() ? `?${filteredParams.toString()}` : '';

	// Normalize duplicate slashes in pathname
	url.pathname = url.pathname.replace(/\/{2,}/g, '/');

	// Trailing slash policy
	if (config.trailingSlash === 'always') {
		if (!url.pathname.endsWith('/')) {
			url.pathname += '/';
		}
	} else if (config.trailingSlash === 'never') {
		if (url.pathname !== '/' && url.pathname.endsWith('/')) {
			url.pathname = url.pathname.slice(0, -1);
		}
	}
	// 'ignore' = leave as-is

	return url.toString();
}

/**
 * Build a canonical URL from a pathname and config.
 * Convenience wrapper for the common case.
 */
export function buildCanonical(pathname: string, config: CanonicalConfig): string {
	return normalizeCanonical(pathname, config);
}

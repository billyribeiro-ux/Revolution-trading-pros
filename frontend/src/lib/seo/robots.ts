/**
 * SEO Plugin Layer - Robots Engine
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Environment-aware robots directive builder.
 * - Production public pages: index,follow by default
 * - Dev/staging: noindex,nofollow unless explicitly overridden
 * - Private/account/auth/checkout/internal/search/filter: noindex,nofollow
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @since February 2026
 */

import type { RobotsDirectives, RouteSEOContext, SEODefaults } from './types';

/**
 * Determine if a pathname matches any prefix in the given list.
 */
function matchesPrefix(pathname: string, prefixes: string[]): boolean {
	const normalized = pathname.toLowerCase();
	return prefixes.some((prefix) => normalized.startsWith(prefix.toLowerCase()));
}

/**
 * Build robots directives based on context, overrides, and defaults.
 *
 * Priority:
 * 1. Explicit page-level overrides always win
 * 2. Private/search/error pages force noindex,nofollow
 * 3. Non-production environments default to noindex,nofollow
 * 4. Production public pages use defaults (index,follow)
 *
 * @returns Object with parsed directives and final content string
 */
export function buildRobots(
	context: RouteSEOContext,
	overrides: RobotsDirectives | null | undefined,
	defaults: SEODefaults
): { directives: RobotsDirectives; content: string } {
	// Step 1: Determine base directives from context
	let base: RobotsDirectives;

	const isPrivatePath =
		context.isPrivate || matchesPrefix(context.pathname, defaults.privatePathPrefixes);
	const isSearchPath =
		context.isSearchPage || matchesPrefix(context.pathname, defaults.searchPathPrefixes);
	const isNonProduction = context.env !== 'production';

	if (isPrivatePath || isSearchPath || context.isErrorPage) {
		// Private, search, and error pages are always noindex
		base = {
			index: false,
			follow: false
		};
	} else if (isNonProduction) {
		// Non-production environments default to noindex
		base = {
			index: false,
			follow: false
		};
	} else {
		// Production public pages use site defaults
		base = { ...defaults.robots };
	}

	// Step 2: Apply explicit overrides (page-level always wins)
	if (overrides && overrides !== null) {
		// For non-production, only allow override if explicitly setting index:true
		// on a safe allowlist (prevents accidental indexing of staging)
		if (isNonProduction && overrides.index === true) {
			// Ignore index:true override on non-production — safety guard
			const { index: _ignored, ...safeOverrides } = overrides;
			base = { ...base, ...safeOverrides };
		} else if (isPrivatePath || context.isErrorPage) {
			// Private/error pages: ignore index override, apply others
			const { index: _ignored, ...safeOverrides } = overrides;
			base = { ...base, ...safeOverrides };
		} else {
			base = { ...base, ...overrides };
		}
	}

	// Step 3: Build content string
	const content = directivesToString(base);

	return { directives: base, content };
}

/**
 * Convert RobotsDirectives to a meta content string.
 *
 * @example
 * directivesToString({ index: true, follow: true, 'max-snippet': -1 })
 * // => "index, follow, max-snippet:-1"
 */
export function directivesToString(directives: RobotsDirectives): string {
	const parts: string[] = [];

	// index/noindex
	if (directives.index === true) {
		parts.push('index');
	} else if (directives.index === false) {
		parts.push('noindex');
	}

	// follow/nofollow
	if (directives.follow === true) {
		parts.push('follow');
	} else if (directives.follow === false) {
		parts.push('nofollow');
	}

	// Boolean directives
	if (directives.noarchive) parts.push('noarchive');
	if (directives.nosnippet) parts.push('nosnippet');
	if (directives.noimageindex) parts.push('noimageindex');

	// Numeric/enum directives
	if (directives['max-snippet'] !== undefined) {
		parts.push(`max-snippet:${directives['max-snippet']}`);
	}
	if (directives['max-image-preview'] !== undefined) {
		parts.push(`max-image-preview:${directives['max-image-preview']}`);
	}
	if (directives['max-video-preview'] !== undefined) {
		parts.push(`max-video-preview:${directives['max-video-preview']}`);
	}

	return parts.join(', ');
}

/**
 * Parse a robots content string back into directives.
 * Useful for testing and audit tooling.
 */
export function parseRobotsString(content: string): RobotsDirectives {
	const directives: RobotsDirectives = {};
	const parts = content.split(',').map((p) => p.trim().toLowerCase());

	for (const part of parts) {
		if (part === 'index') directives.index = true;
		else if (part === 'noindex') directives.index = false;
		else if (part === 'follow') directives.follow = true;
		else if (part === 'nofollow') directives.follow = false;
		else if (part === 'noarchive') directives.noarchive = true;
		else if (part === 'nosnippet') directives.nosnippet = true;
		else if (part === 'noimageindex') directives.noimageindex = true;
		else if (part.startsWith('max-snippet:')) {
			directives['max-snippet'] = parseInt(part.split(':')[1], 10);
		} else if (part.startsWith('max-image-preview:')) {
			directives['max-image-preview'] = part.split(':')[1] as 'none' | 'standard' | 'large';
		} else if (part.startsWith('max-video-preview:')) {
			directives['max-video-preview'] = parseInt(part.split(':')[1], 10);
		}
	}

	return directives;
}

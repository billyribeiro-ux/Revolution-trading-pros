/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Standardized Breakpoints - Repository-Wide System
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Consistent breakpoint values for responsive design across the entire repo
 * @version 1.0.0 - January 2026
 * @standards Apple Principal Engineer ICT 7+ Standards
 *
 * MOBILE FIRST APPROACH:
 * - Use min-width for progressive enhancement (mobile → desktop)
 * - Use max-width for graceful degradation (desktop → mobile)
 *
 * These values align with Tailwind CSS defaults for consistency.
 */

/**
 * Breakpoint pixel values
 * Use these in JavaScript/TypeScript for programmatic responsive logic
 */
export const BREAKPOINTS = {
	/** Small devices (landscape phones) - 640px and up */
	sm: 640,
	/** Medium devices (tablets) - 768px and up */
	md: 768,
	/** Large devices (desktops) - 1024px and up */
	lg: 1024,
	/** Extra large devices (large desktops) - 1280px and up */
	xl: 1280,
	/** 2X large devices (larger desktops) - 1536px and up */
	'2xl': 1536
} as const;

/**
 * Media query strings for use in CSS-in-JS or matchMedia
 * Mobile-first (min-width) approach
 */
export const MEDIA_QUERIES = {
	/** @media (min-width: 640px) */
	sm: `(min-width: ${BREAKPOINTS.sm}px)`,
	/** @media (min-width: 768px) */
	md: `(min-width: ${BREAKPOINTS.md}px)`,
	/** @media (min-width: 1024px) */
	lg: `(min-width: ${BREAKPOINTS.lg}px)`,
	/** @media (min-width: 1280px) */
	xl: `(min-width: ${BREAKPOINTS.xl}px)`,
	/** @media (min-width: 1536px) */
	'2xl': `(min-width: ${BREAKPOINTS['2xl']}px)`
} as const;

/**
 * Max-width media query strings for desktop-first approach
 * Use when you need to target smaller screens specifically
 */
export const MEDIA_QUERIES_MAX = {
	/** @media (max-width: 639px) - Mobile phones */
	sm: `(max-width: ${BREAKPOINTS.sm - 1}px)`,
	/** @media (max-width: 767px) - Small tablets */
	md: `(max-width: ${BREAKPOINTS.md - 1}px)`,
	/** @media (max-width: 1023px) - Tablets */
	lg: `(max-width: ${BREAKPOINTS.lg - 1}px)`,
	/** @media (max-width: 1279px) - Small desktops */
	xl: `(max-width: ${BREAKPOINTS.xl - 1}px)`,
	/** @media (max-width: 1535px) - Large desktops */
	'2xl': `(max-width: ${BREAKPOINTS['2xl'] - 1}px)`
} as const;

/**
 * Type for breakpoint keys
 */
export type BreakpointKey = keyof typeof BREAKPOINTS;

/**
 * Check if viewport matches a breakpoint (client-side only)
 * @param breakpoint - The breakpoint to check
 * @param type - 'min' for mobile-first, 'max' for desktop-first
 * @returns boolean indicating if viewport matches
 */
export function matchesBreakpoint(breakpoint: BreakpointKey, type: 'min' | 'max' = 'min'): boolean {
	if (typeof window === 'undefined') return false;
	const query = type === 'min' ? MEDIA_QUERIES[breakpoint] : MEDIA_QUERIES_MAX[breakpoint];
	return window.matchMedia(query).matches;
}

/**
 * Get current breakpoint name based on viewport width (client-side only)
 * @returns The current breakpoint name or 'xs' for extra small
 */
export function getCurrentBreakpoint(): BreakpointKey | 'xs' {
	if (typeof window === 'undefined') return 'xs';
	const width = window.innerWidth;
	if (width >= BREAKPOINTS['2xl']) return '2xl';
	if (width >= BREAKPOINTS.xl) return 'xl';
	if (width >= BREAKPOINTS.lg) return 'lg';
	if (width >= BREAKPOINTS.md) return 'md';
	if (width >= BREAKPOINTS.sm) return 'sm';
	return 'xs';
}

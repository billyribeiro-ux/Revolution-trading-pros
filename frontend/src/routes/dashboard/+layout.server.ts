/**
 * Dashboard Layout Server - SSR Configuration
 * ═══════════════════════════════════════════════════════════════════════════
 * Apple ICT 11+ Principal Engineer Implementation
 *
 * ARCHITECTURE DECISION:
 * Authentication is handled CLIENT-SIDE because:
 * 1. JWT tokens are stored in memory (not httpOnly cookies)
 * 2. SSR cannot access client-side memory/localStorage
 * 3. Protected routes don't benefit from SSR (no SEO needed)
 * 4. Client-side auth is the industry standard for SPAs
 *
 * The +layout.svelte handles:
 * - Auth state checking via isAuthenticated store
 * - Redirect to login if not authenticated
 * - Loading memberships data client-side
 *
 * This file only configures SSR behavior for the dashboard routes.
 *
 * @version 3.0.0 - ICT 11+ Client-Side Auth Pattern
 * @author Revolution Trading Pros
 */

import type { LayoutServerLoad } from './$types';

/**
 * Disable SSR for dashboard routes
 * Auth is handled client-side, SSR would cause hydration mismatches
 */
export const ssr = false;

/**
 * Disable prerendering for dashboard routes
 * These are dynamic, authenticated pages
 */
export const prerender = false;

/**
 * Server load function - minimal, auth is client-side
 */
export const load: LayoutServerLoad = async () => {
	// No server-side auth check - handled in +layout.svelte client-side
	// This prevents the redirect loop caused by locals.user being undefined
	return {};
};

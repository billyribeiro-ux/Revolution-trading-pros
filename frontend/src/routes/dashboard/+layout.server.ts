/**
 * Dashboard Layout Server - ICT 11+ Server-Side Auth Pattern
 * ═══════════════════════════════════════════════════════════════════════════
 * Apple ICT 11+ Principal Engineer Implementation
 * Svelte 5 (2025) Best Practice: Server-Side Route Protection
 *
 * ARCHITECTURE (Updated December 2025):
 * 1. Authentication is now handled SERVER-SIDE in hooks.server.ts
 * 2. hooks.server.ts validates JWT and redirects to login if needed
 * 3. This load function receives authenticated user from event.locals
 * 4. User data is passed to client for hydration
 *
 * Benefits:
 * - Auth check happens BEFORE any data loading
 * - Prevents unauthorized API calls
 * - More secure than client-side guards
 * - Proper redirects with return URLs
 *
 * @version 4.0.0 - ICT 11+ Server-Side Auth Pattern
 * @author Revolution Trading Pros
 */

import type { LayoutServerLoad } from './$types';

/**
 * SSR enabled for dashboard - server-side auth is now secure
 * The hooks.server.ts handles auth before this load runs
 */
export const ssr = true;

/**
 * Disable prerendering for dashboard routes
 * These are dynamic, authenticated pages
 */
export const prerender = false;

/**
 * Server load function - receives authenticated user from hooks.server.ts
 * If user is not authenticated, hooks.server.ts already redirected to login
 */
export const load: LayoutServerLoad = async ({ locals }) => {
	// User is guaranteed to exist here because hooks.server.ts
	// redirects to login if not authenticated
	return {
		user: locals.user ?? null
	};
};

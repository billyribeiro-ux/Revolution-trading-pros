/**
 * Dashboard Layout Server - ICT 11+ Server-Side Auth Pattern
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
 */

import type { LayoutServerLoad } from './$types';
import { getMemberships } from './memberships.remote';

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

export const load: LayoutServerLoad = async ({ locals }) => {
	// User is guaranteed to exist here because hooks.server.ts
	// redirects to login if not authenticated.
	const user =
		(locals as { user?: { id: string; email: string; name?: string; role?: string } }).user ?? null;

	// CLS FIX (measured 0.12 → 0.0016 on /dashboard): pre-fetch the membership list
	// on the server so DashboardSidebar renders its nav on first paint instead of
	// popping in after a client fetch. Now delegated to the `getMemberships` remote
	// query — type-safe, deduped, and reusable from the client (kit ≥ 2.61).
	const memberships = await getMemberships();

	return {
		user,
		accessToken: (locals as { accessToken?: string | null }).accessToken ?? null,
		memberships
	};
};

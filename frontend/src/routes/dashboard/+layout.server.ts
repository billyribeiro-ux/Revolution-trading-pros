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
/**
 * Minimal flat membership shape the sidebar needs to render its dynamic nav.
 * Kept intentionally small — categorization happens by `type` in the sidebar.
 */
interface SidebarMembership {
	id: string;
	name: string;
	type: string;
	slug: string;
	status: string;
	icon?: string;
}

export const load: LayoutServerLoad = async ({ locals, fetch }) => {
	// User is guaranteed to exist here because hooks.server.ts
	// redirects to login if not authenticated
	// ICT 7 FIX: Also pass accessToken for server-side API calls in child pages
	const user =
		(locals as { user?: { id: string; email: string; name?: string; role?: string } }).user ??
		null;

	// CLS FIX (evidence: measured 0.12 CLS on /dashboard from the sidebar nav
	// popping in after a client-side fetch): pre-fetch the membership list on the
	// server so DashboardSidebar renders its trading-room/mentorship/scanner nav
	// sections on first paint. The client still refreshes via getUserMemberships()
	// — identical data, so no further layout shift. The internal `fetch` forwards
	// the auth cookie through the /api proxy.
	let memberships: SidebarMembership[] = [];
	if (user) {
		try {
			const res = await fetch('/api/user/memberships');
			if (res.ok) {
				const body = await res.json();
				const raw = body?.data?.memberships ?? body?.memberships ?? body?.data ?? [];
				if (Array.isArray(raw)) {
					// Sort into the SAME category order the client derives
					// (tradingRooms → alertServices → courses → indicators → scanners
					// → weeklyWatchlist → premiumReports). Without this, a backend that
					// returns memberships in a different/interleaved order would make the
					// sidebar nav links re-order once the client fetch resolves —
					// measured as a ~0.01 CLS regression. A stable sort preserves the
					// backend's within-category order, matching categorizeMemberships().
					const TYPE_ORDER: Record<string, number> = {
						'trading-room': 0,
						'alert-service': 1,
						course: 2,
						indicator: 3,
						scanner: 4,
						'weekly-watchlist': 5,
						'premium-report': 6
					};
					memberships = raw
						.filter((m) => m && (m.status === 'active' || m.status === 'expiring'))
						.map((m) => ({
							id: String(m.id ?? m.slug ?? ''),
							name: m.name ?? '',
							type: m.type ?? '',
							slug: m.slug ?? '',
							status: m.status ?? 'active',
							icon: m.icon ?? undefined
						}))
						.map((m, i) => ({ m, i })) // index for stable tiebreak
						.sort((a, b) => {
							const d = (TYPE_ORDER[a.m.type] ?? 99) - (TYPE_ORDER[b.m.type] ?? 99);
							return d !== 0 ? d : a.i - b.i;
						})
						.map(({ m }) => m);
				}
			}
		} catch {
			// Non-fatal: the client fetch will populate the sidebar. Worst case is
			// the pre-fix behaviour (a single late paint), never a broken page.
			memberships = [];
		}
	}

	return {
		user,
		accessToken: (locals as { accessToken?: string | null }).accessToken ?? null,
		memberships
	};
};

/**
 * Dashboard Layout Server - SSR Auth Guard + Membership Data
 * Apple ICT 11+ Principal Engineer Implementation
 * Matches backend API: GET /api/user/memberships
 *
 * @version 2.0.0
 * @author Revolution Trading Pros
 */

import { redirect } from '@sveltejs/kit';
import type { ServerLoadEvent } from '@sveltejs/kit';
import { getUserMemberships } from '$lib/api/user-memberships';

export const load = async ({ locals, url }: ServerLoadEvent) => {
	// Check if user is authenticated via locals
	const user = locals.user;

	if (!user) {
		// Store the intended destination for redirect after login
		const redirectTo = url.pathname + url.search;
		throw redirect(303, `/login?redirect=${encodeURIComponent(redirectTo)}`);
	}

	// Fetch user memberships from API
	// getUserMemberships() calls backend /api/user/memberships
	// Backend returns: { memberships: [] }
	// Frontend categorizes into: { memberships, tradingRooms, alertServices, courses, etc. }
	const membershipsData = await getUserMemberships();

	// Extract membership slugs for sidebar filtering
	const membershipSlugs = membershipsData.memberships
		.filter(m => m.status === 'active')
		.map(m => m.slug);

	// Return user data with categorized memberships for the dashboard
	return {
		user: {
			id: user.id,
			email: user.email,
			name: user.name || user.email?.split('@')[0] || 'Member',
			avatar: null, // Avatar will be fetched client-side from user profile
			memberships: membershipSlugs
		},
		membershipsData: {
			memberships: membershipsData.memberships,
			tradingRooms: membershipsData.tradingRooms,
			alertServices: membershipsData.alertServices,
			courses: membershipsData.courses,
			indicators: membershipsData.indicators,
			weeklyWatchlist: membershipsData.weeklyWatchlist,
			premiumReports: membershipsData.premiumReports,
			stats: membershipsData.stats
		}
	};
};

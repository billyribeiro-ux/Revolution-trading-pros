/**
 * Dashboard Layout Server - SSR Auth Guard
 * Ensures user is authenticated before accessing dashboard pages
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */

import { redirect } from '@sveltejs/kit';
import type { ServerLoadEvent } from '@sveltejs/kit';

export const load = async ({ locals, url }: ServerLoadEvent) => {
	// Check if user is authenticated via locals
	const user = locals.user;

	if (!user) {
		// Store the intended destination for redirect after login
		const redirectTo = url.pathname + url.search;
		throw redirect(303, `/login?redirect=${encodeURIComponent(redirectTo)}`);
	}

	// Return user data for the dashboard
	return {
		user: {
			id: user.id,
			email: user.email,
			name: user.name || user.email?.split('@')[0] || 'Member',
			avatar: null, // Will be populated when avatar support is added
			memberships: [] // Will be populated when membership data is available
		}
	};
};

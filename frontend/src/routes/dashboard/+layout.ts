/**
 * Dashboard Layout Load Function
 *
 * Svelte 5 / SvelteKit best practices:
 * - SSR-safe data loading
 * - Auth guard with redirect
 * - Preload user memberships for sidebar
 */

import { redirect } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ parent, url }) => {
	// Get parent data (includes auth state from root layout)
	const parentData = await parent();

	// Auth guard - redirect to login if not authenticated
	// This runs on both server and client
	if (!parentData.user) {
		const redirectUrl = encodeURIComponent(url.pathname);
		throw redirect(302, `/login?redirect=${redirectUrl}`);
	}

	// Return user data for dashboard
	return {
		user: parentData.user,
		// Memberships will be loaded from API in components
		// This keeps the layout load fast
	};
};

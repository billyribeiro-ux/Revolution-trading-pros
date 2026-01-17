/**
 * Start Here Page - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 * 
 * SSR Configuration (SvelteKit Official Docs):
 * - ssr: true - Server-side render for SEO and fast initial load
 * - csr: true - Enable client-side hydration for accordion interactivity
 * - prerender: false - Contains dynamic watchlist data
 * 
 * SSR pre-fetch for 0ms loading delay on Weekly Watchlist.
 * 
 * @version 1.0.0
 */

import { getLatestWatchlist } from '$lib/server/watchlist';
import type { ServerLoad } from '@sveltejs/kit';

// SSR/SSG Configuration - Per SvelteKit Official Docs
export const ssr = true;       // Enable server-side rendering
export const csr = true;       // Enable client-side hydration for interactivity
export const prerender = false; // Dynamic watchlist content - cannot prerender

export const load: ServerLoad = async ({ fetch }) => {
	const watchlist = await getLatestWatchlist('small-account-mentorship', fetch);

	return {
		watchlist
	};
};

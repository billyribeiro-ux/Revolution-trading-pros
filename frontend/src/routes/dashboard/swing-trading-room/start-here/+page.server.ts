/**
 * Start Here Page - Server Load Function (Swing Trading Room)
 * ===============================================================================
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

import type { PageServerLoad } from './$types';
import { getLatestWatchlist } from '$lib/server/watchlist';

// SSR/SSG Configuration - Per SvelteKit Official Docs
export const ssr = true; // Enable server-side rendering
export const csr = true; // Enable client-side hydration for interactivity
export const prerender = false; // Dynamic watchlist content - cannot prerender

export const load: PageServerLoad = async ({ fetch }) => {
	// Pre-fetch latest watchlist for swing-trading-room (SSR)
	const watchlist = await getLatestWatchlist('swing-trading-room', fetch);

	return {
		watchlist
	};
};

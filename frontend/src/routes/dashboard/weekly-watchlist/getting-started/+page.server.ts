/**
 * Weekly Watchlist Getting Started - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 * 
 * Svelte 5 / SvelteKit 2.0 Best Practices:
 * - satisfies PageServerLoad for type inference
 * - Typed return value
 */

import { getLatestWatchlist } from '$lib/server/watchlist';
import type { PageServerLoad } from './$types';

export const load = (async ({ fetch }) => {
	const watchlist = await getLatestWatchlist('weekly-watchlist', fetch);
	return { watchlist };
}) satisfies PageServerLoad;

/**
 * Day Trading Room Dashboard - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 * 
 * SSR pre-fetch for 0ms loading delay on Weekly Watchlist.
 * 
 * @version 1.0.0
 */

import { getLatestWatchlist } from '$lib/server/watchlist';

/** @type {import('./$types').PageServerLoad} */
export async function load({ fetch }) {
	// Pre-fetch latest watchlist for day-trading-room (or global if none)
	const watchlist = await getLatestWatchlist('day-trading-room', fetch);

	return {
		watchlist
	};
}

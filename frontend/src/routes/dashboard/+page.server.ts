/**
 * Member Dashboard - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 * 
 * SSR pre-fetch for 0ms loading delay on Weekly Watchlist.
 * 
 * @version 1.0.0
 */

import type { ServerLoadEvent } from '@sveltejs/kit';
import { getLatestWatchlist } from '$lib/server/watchlist';

export async function load({ fetch }: ServerLoadEvent) {
	// Pre-fetch latest watchlist for 0ms render
	const watchlist = await getLatestWatchlist(undefined, fetch);

	return {
		watchlist
	};
}

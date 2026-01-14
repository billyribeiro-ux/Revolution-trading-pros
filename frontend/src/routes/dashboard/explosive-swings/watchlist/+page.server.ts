/**
 * Explosive Swings Watchlist - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * SSR pre-fetch for watchlist data
 * 
 * @version 1.0.0
 */

import { getLatestWatchlist } from '$lib/server/watchlist';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	const watchlist = await getLatestWatchlist('explosive-swings', fetch);

	return {
		watchlist
	};
};

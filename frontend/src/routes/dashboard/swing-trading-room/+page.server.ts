/**
 * Swing Trading Room Dashboard - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 *
 * SSR pre-fetch for 0ms loading delay on Weekly Watchlist.
 *
 * @version 1.0.0
 */

import { getLatestWatchlist } from '$lib/server/watchlist';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, locals }) => {
	// ICT 7 FIX: Pass access token from locals for authenticated API calls
	const accessToken = locals.accessToken ?? undefined;
	
	// Pre-fetch latest watchlist for swing-trading-room (or global if none)
	const watchlist = await getLatestWatchlist('swing-trading-room', fetch, undefined, accessToken);

	return {
		watchlist
	};
};

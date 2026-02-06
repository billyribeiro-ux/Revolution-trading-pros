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

import { getLatestWatchlist } from '$lib/server/watchlist';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, locals }) => {
	// ICT 7 FIX: Pass access token from locals (set by hooks.server.ts auth handler)
	const accessToken = locals.accessToken ?? undefined;

	// Pre-fetch latest watchlist for 0ms render
	const watchlist = await getLatestWatchlist(undefined, fetch, undefined, accessToken);

	return {
		watchlist
	};
};

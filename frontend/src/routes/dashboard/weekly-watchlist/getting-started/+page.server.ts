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

export const load = (async ({ fetch, locals }) => {
	// ICT 7 FIX: Pass access token from locals for authenticated API calls
	const accessToken = locals.accessToken ?? undefined;
	
	const watchlist = await getLatestWatchlist('weekly-watchlist', fetch, undefined, accessToken);
	return { watchlist };
}) satisfies PageServerLoad;

/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Explosive Swings Watchlist - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description SSR pre-fetch for watchlist data with graceful degradation
 * @version 2.0.0 - Apple Principal Engineer ICT Level 7 Grade
 * @author Revolution Trading Pros Engineering
 * @standards Apple ICT 7+ | SvelteKit 2.0+ | Svelte 5 (January 2026)
 *
 * @security
 * - Server-side only (no client exposure of API keys)
 * - Environment variables via $env/dynamic/private
 */

import { env } from '$env/dynamic/private';
import { getLatestWatchlist, type WatchlistData } from '$lib/server/watchlist';
import type { PageServerLoad } from './$types';
import { ROOM_SLUG } from '../constants';

// ═══════════════════════════════════════════════════════════════════════════════
// CONSTANTS - ICT 7 Single Source of Truth
// ═══════════════════════════════════════════════════════════════════════════════

const LOG_PREFIX = '[explosive-swings/watchlist]';
const DEFAULT_API_URL = 'https://revolution-trading-pros-api.fly.dev';

// ═══════════════════════════════════════════════════════════════════════════════
// TYPES - ICT 7 Type Safety
// ═══════════════════════════════════════════════════════════════════════════════

/** Page data returned to client */
interface WatchlistPageData {
	watchlist: WatchlistData | null;
}

// ═══════════════════════════════════════════════════════════════════════════════
// LOAD FUNCTION - ICT 7 Server-Side Rendering
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Server load function for Explosive Swings watchlist page
 *
 * @description Fetches latest watchlist with graceful degradation.
 *              Never throws - returns null on failure.
 *
 * @param {Object} context - SvelteKit load context
 * @param {typeof fetch} context.fetch - SvelteKit fetch with credentials
 * @returns {Promise<WatchlistPageData>} Page data for client hydration
 */
export const load: PageServerLoad = async ({ fetch }): Promise<WatchlistPageData> => {
	const baseUrl = env.API_BASE_URL || DEFAULT_API_URL;

	try {
		const watchlist = await getLatestWatchlist(ROOM_SLUG, fetch, baseUrl);

		return {
			watchlist: watchlist ?? null
		};
	} catch (error) {
		// ICT 7: Never throw 500 - graceful degradation
		console.error(`${LOG_PREFIX} FATAL ERROR:`, error);
		
		return {
			watchlist: null
		};
	}
};

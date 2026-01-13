import { getLatestWatchlist } from '$lib/server/watchlist';

/** @type {import('./$types').PageServerLoad} */
export async function load({ fetch }) {
	// Pre-fetch latest watchlist for weekly-watchlist (or global if none)
	const watchlist = await getLatestWatchlist('weekly-watchlist', fetch);

	return {
		watchlist
	};
}

import { getLatestWatchlist } from '$lib/server/watchlist';
import type { ServerLoadEvent } from '@sveltejs/kit';

export async function load({ fetch }: ServerLoadEvent) {
	// Pre-fetch latest watchlist for weekly-watchlist (or global if none)
	const watchlist = await getLatestWatchlist('weekly-watchlist', fetch);

	return {
		watchlist
	};
}

import type { PageServerLoad } from './$types';
import { fetchWeeklyWatchlist } from '$lib/api/weekly-watchlist';

export const load: PageServerLoad = async ({ fetch }) => {
	try {
		const watchlist = await fetchWeeklyWatchlist(fetch);
		
		return {
			watchlist
		};
	} catch (error) {
		console.error('Error loading Weekly Watchlist for Getting Started page:', error);
		return {
			watchlist: null
		};
	}
};

/**
 * Weekly Watchlist Server-Side Fetch Helper
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 * 
 * Server-side data fetching for 0ms loading delay.
 * Use in +page.server.ts load functions to pre-fetch watchlist data.
 * 
 * @version 1.0.0
 */

const API_BASE = 'https://revolution-trading-pros-api.fly.dev';

export interface WatchlistData {
	id: number;
	slug: string;
	title: string;
	trader: string;
	weekOf: string;
	video: {
		src: string;
		poster: string;
		title: string;
	};
	description?: string;
}

export interface WatchlistResponse {
	success: boolean;
	data: WatchlistData;
}

/**
 * Fetch the latest published watchlist entry
 * @param roomSlug - Optional room filter (e.g., 'day-trading-room')
 * @param fetchFn - SvelteKit's fetch function from load context
 * @returns Watchlist data or null if not found
 */
export async function getLatestWatchlist(
	roomSlug?: string,
	fetchFn: typeof fetch = fetch
): Promise<WatchlistData | null> {
	try {
		const params = new URLSearchParams({
			per_page: '1',
			status: 'published'
		});
		
		if (roomSlug) {
			params.set('room', roomSlug);
		}

		const response = await fetchFn(`${API_BASE}/api/watchlist?${params}`, {
			headers: {
				'Accept': 'application/json',
				'Content-Type': 'application/json'
			}
		});

		if (!response.ok) {
			console.warn('[getLatestWatchlist] API returned:', response.status);
			return null;
		}

		const result = await response.json();
		
		if (result.success && result.data?.length > 0) {
			return result.data[0];
		}

		return null;
	} catch (err) {
		console.error('[getLatestWatchlist] Failed to fetch:', err);
		return null;
	}
}

/**
 * Fetch watchlist by slug
 * @param slug - Watchlist entry slug
 * @param fetchFn - SvelteKit's fetch function from load context
 * @returns Watchlist data or null if not found
 */
export async function getWatchlistBySlug(
	slug: string,
	fetchFn: typeof fetch = fetch
): Promise<WatchlistData | null> {
	try {
		const response = await fetchFn(`${API_BASE}/api/watchlist/${slug}`, {
			headers: {
				'Accept': 'application/json',
				'Content-Type': 'application/json'
			}
		});

		if (!response.ok) {
			return null;
		}

		const result = await response.json();
		
		if (result.success && result.data) {
			return result.data;
		}

		return null;
	} catch (err) {
		console.error('[getWatchlistBySlug] Failed to fetch:', err);
		return null;
	}
}

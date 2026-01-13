/**
 * Watchlist Rundown Archive - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 * 
 * Svelte 5 / SvelteKit 2.0 Best Practices:
 * - Typed load function with explicit return type
 * - Proper error handling with structured responses
 * - Type-safe API response mapping
 */

import type { PageServerLoad } from './$types';

/** API response structure from backend */
interface WatchlistApiEntry {
	id: string | number;
	slug: string;
	title: string;
	publish_date?: string;
	week_of?: string;
	thumbnail_url?: string;
	image_url?: string;
	description?: string;
}

interface WatchlistApiResponse {
	entries?: WatchlistApiEntry[];
}

/** Transformed video for frontend consumption */
export interface WatchlistVideo {
	id: string;
	slug: string;
	title: string;
	date: string;
	weekOf: string;
	image: string;
	href: string;
	description: string;
}

export const load = (async ({ fetch }) => {
	try {
		const response = await fetch('https://revolution-trading-pros-api.fly.dev/api/watchlist/entries');
		
		if (!response.ok) {
			console.error('[WatchlistRundown] API error:', response.status, response.statusText);
			return { videos: [] as WatchlistVideo[] };
		}

		const data: WatchlistApiResponse = await response.json();
		
		const videos: WatchlistVideo[] = (data.entries ?? []).map((entry) => ({
			id: String(entry.id),
			slug: entry.slug,
			title: entry.title,
			date: entry.publish_date ?? '',
			weekOf: entry.week_of ?? '',
			image: entry.thumbnail_url ?? entry.image_url ?? '',
			href: `/watchlist/${entry.slug}`,
			description: entry.description ?? entry.week_of ?? ''
		}));

		return { videos };
	} catch (error) {
		console.error('[WatchlistRundown] Load error:', error);
		return { videos: [] as WatchlistVideo[] };
	}
}) satisfies PageServerLoad;

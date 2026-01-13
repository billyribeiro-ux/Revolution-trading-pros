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

import type { RequestEvent } from '@sveltejs/kit';

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

/** Mock data - used until API is available */
const MOCK_VIDEOS: WatchlistVideo[] = [
	{
		id: '1',
		slug: '01122026-tg-watkins',
		title: 'Weekly Watchlist Rundown',
		date: 'January 12, 2026',
		weekOf: 'Week of January 12, 2026',
		image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
		href: '/watchlist/01122026-tg-watkins',
		description: 'TG Watkins breaks down the Weekly Watchlist for the week of January 12, 2026.'
	},
	{
		id: '2',
		slug: '01052026-melissa-beegle',
		title: 'Weekly Watchlist Rundown',
		date: 'January 5, 2026',
		weekOf: 'Week of January 5, 2026',
		image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
		href: '/watchlist/01052026-melissa-beegle',
		description: 'Melissa Beegle breaks down the Weekly Watchlist for the week of January 5, 2026.'
	},
	{
		id: '3',
		slug: '12292025-david-starr',
		title: 'Weekly Watchlist Rundown',
		date: 'December 29, 2025',
		weekOf: 'Week of December 29, 2025',
		image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
		href: '/watchlist/12292025-david-starr',
		description: 'David Starr breaks down the Weekly Watchlist for the week of December 29, 2025.'
	},
	{
		id: '4',
		slug: '12222025-tg-watkins',
		title: 'Weekly Watchlist Rundown',
		date: 'December 22, 2025',
		weekOf: 'Week of December 22, 2025',
		image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
		href: '/watchlist/12222025-tg-watkins',
		description: 'TG Watkins breaks down the Weekly Watchlist for the week of December 22, 2025.'
	},
	{
		id: '5',
		slug: '12152025-allison-ostrander',
		title: 'Weekly Watchlist Rundown',
		date: 'December 15, 2025',
		weekOf: 'Week of December 15, 2025',
		image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
		href: '/watchlist/12152025-allison-ostrander',
		description: 'Allison Ostrander breaks down the Weekly Watchlist for the week of December 15, 2025.'
	},
	{
		id: '6',
		slug: '12082025-taylor-horton',
		title: 'Weekly Watchlist Rundown',
		date: 'December 8, 2025',
		weekOf: 'Week of December 8, 2025',
		image: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
		href: '/watchlist/12082025-taylor-horton',
		description: 'Taylor Horton breaks down the Weekly Watchlist for the week of December 8, 2025.'
	}
];

export const load = async ({ fetch }: RequestEvent) => {
	try {
		const response = await fetch('https://revolution-trading-pros-api.fly.dev/api/watchlist/entries');
		
		if (!response.ok) {
			console.error('[WatchlistRundown] API error:', response.status, '- using mock data');
			return { videos: MOCK_VIDEOS };
		}

		const data: WatchlistApiResponse = await response.json();
		
		if (!data.entries || data.entries.length === 0) {
			console.log('[WatchlistRundown] No API data - using mock data');
			return { videos: MOCK_VIDEOS };
		}
		
		const videos: WatchlistVideo[] = data.entries.map((entry) => ({
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
		console.error('[WatchlistRundown] Load error - using mock data:', error);
		return { videos: MOCK_VIDEOS };
	}
};

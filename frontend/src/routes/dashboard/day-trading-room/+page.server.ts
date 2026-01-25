/**
 * Day Trading Room Dashboard - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * SvelteKit 2.0+ / Svelte 5 Best Practices - January 2026
 *
 * SSR pre-fetch for 0ms loading:
 * - Tutorial video (featured)
 * - Latest updates (daily videos)
 * - Weekly watchlist
 * - PDFs/Documents
 *
 * @version 4.0.0 - SvelteKit 2.0+ satisfies pattern
 */

import { env } from '$env/dynamic/private';
import { getLatestWatchlist } from '$lib/server/watchlist';
import type { PageServerLoad } from './$types';
import type { RoomResource } from '$lib/api/room-resources';

const DAY_TRADING_ROOM_ID = 1;

export const load = (async ({ fetch }) => {
	const baseUrl = env.API_BASE_URL || 'https://revolution-trading-pros-api.fly.dev';

	// Parallel fetch for optimal performance
	const [watchlist, tutorialRes, updatesRes, documentsRes] = await Promise.all([
		// Weekly watchlist
		getLatestWatchlist('day-trading-room', fetch, baseUrl),

		// Featured tutorial video
		fetch(
			`${baseUrl}/api/room-resources?room_id=${DAY_TRADING_ROOM_ID}&resource_type=video&content_type=tutorial&is_featured=true&per_page=1`
		)
			.then((r: Response) => (r.ok ? r.json() : { data: [] }))
			.catch(() => ({ data: [] })),

		// Latest daily videos
		fetch(
			`${baseUrl}/api/room-resources?room_id=${DAY_TRADING_ROOM_ID}&resource_type=video&content_type=daily_video&per_page=6`
		)
			.then((r: Response) => (r.ok ? r.json() : { data: [] }))
			.catch(() => ({ data: [] })),

		// PDFs and documents
		fetch(
			`${baseUrl}/api/room-resources?room_id=${DAY_TRADING_ROOM_ID}&resource_type=pdf&per_page=10`
		)
			.then((r: Response) => (r.ok ? r.json() : { data: [] }))
			.catch(() => ({ data: [] }))
	]);

	return {
		watchlist,
		tutorialVideo: (tutorialRes.data?.[0] || null) as RoomResource | null,
		latestUpdates: (updatesRes.data || []) as RoomResource[],
		documents: (documentsRes.data || []) as RoomResource[],
		roomId: DAY_TRADING_ROOM_ID
	};
}) satisfies PageServerLoad;

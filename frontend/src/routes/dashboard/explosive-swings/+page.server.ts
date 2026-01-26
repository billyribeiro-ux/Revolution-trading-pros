/**
 * Explosive Swings Dashboard - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * SvelteKit 2.0+ / Svelte 5 Best Practices - January 2026
 *
 * SSR pre-fetch for unified room resources:
 * - Tutorial video (featured)
 * - Latest updates (daily videos with tags)
 * - Weekly watchlist
 * - PDFs/Documents
 *
 * @version 4.0.0 - SvelteKit 2.0+ satisfies pattern
 */

import { env } from '$env/dynamic/private';
import { getLatestWatchlist } from '$lib/server/watchlist';
import type { PageServerLoad } from './$types';
import type { RoomResource } from '$lib/api/room-resources';
import { ROOM_RESOURCES_ID, ROOM_SLUG } from './constants';

export const load = (async ({ fetch }) => {
	try {
		const baseUrl = env.API_BASE_URL || 'https://revolution-trading-pros-api.fly.dev';
		
		console.log('[explosive-swings] Loading page with baseUrl:', baseUrl);
		console.log('[explosive-swings] ROOM_RESOURCES_ID:', ROOM_RESOURCES_ID);

		// Parallel fetch for optimal performance
		const [watchlist, tutorialRes, updatesRes, documentsRes] = await Promise.all([
			// Weekly watchlist
			getLatestWatchlist(ROOM_SLUG, fetch, baseUrl).catch((err) => {
				console.error('[explosive-swings] Watchlist fetch error:', err);
				return null;
			}),

			// Featured tutorial video
			fetch(
				`${baseUrl}/api/room-resources?room_id=${ROOM_RESOURCES_ID}&resource_type=video&content_type=tutorial&is_featured=true&per_page=1`
			)
				.then((r: Response) => {
					if (!r.ok) {
						console.warn('[explosive-swings] Tutorial fetch returned:', r.status);
					}
					return r.ok ? r.json() : { data: [] };
				})
				.catch((err) => {
					console.error('[explosive-swings] Tutorial fetch error:', err);
					return { data: [] };
				}),

			// Latest daily videos with tags
			fetch(
				`${baseUrl}/api/room-resources?room_id=${ROOM_RESOURCES_ID}&resource_type=video&content_type=daily_video&per_page=6`
			)
				.then((r: Response) => {
					if (!r.ok) {
						console.warn('[explosive-swings] Updates fetch returned:', r.status);
					}
					return r.ok ? r.json() : { data: [] };
				})
				.catch((err) => {
					console.error('[explosive-swings] Updates fetch error:', err);
					return { data: [] };
				}),

			// PDFs and documents
			fetch(
				`${baseUrl}/api/room-resources?room_id=${ROOM_RESOURCES_ID}&resource_type=pdf&per_page=10`
			)
				.then((r: Response) => {
					if (!r.ok) {
						console.warn('[explosive-swings] Documents fetch returned:', r.status);
					}
					return r.ok ? r.json() : { data: [] };
				})
				.catch((err) => {
					console.error('[explosive-swings] Documents fetch error:', err);
					return { data: [] };
				})
		]);

		console.log('[explosive-swings] All fetches completed successfully');

		return {
			watchlist: watchlist ?? null,
			tutorialVideo: (tutorialRes?.data?.[0] || null) as RoomResource | null,
			latestUpdates: (updatesRes?.data || []) as RoomResource[],
			documents: (documentsRes?.data || []) as RoomResource[],
			roomId: ROOM_RESOURCES_ID
		};
	} catch (error) {
		console.error('[explosive-swings] FATAL ERROR in load function:', error);
		// Return safe fallback data to prevent 500 error
		return {
			watchlist: null,
			tutorialVideo: null,
			latestUpdates: [],
			documents: [],
			roomId: ROOM_RESOURCES_ID
		};
	}
}) satisfies PageServerLoad;

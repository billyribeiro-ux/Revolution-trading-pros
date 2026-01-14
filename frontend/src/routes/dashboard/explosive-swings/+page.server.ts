/**
 * Explosive Swings Dashboard - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 * 
 * SSR pre-fetch for unified room resources:
 * - Tutorial video (featured)
 * - Latest updates (daily videos with tags)
 * - Weekly watchlist
 * - PDFs/Documents
 * 
 * @version 3.0.0 - Unified Room Resources API
 */

import { env } from '$env/dynamic/private';
import { getLatestWatchlist } from '$lib/server/watchlist';
import type { PageServerLoad } from './$types';

const EXPLOSIVE_SWINGS_ROOM_ID = 2;

export const load: PageServerLoad = async ({ fetch }) => {
	const baseUrl = env.API_BASE_URL || 'https://revolution-trading-pros-api.fly.dev';
	
	// Parallel fetch for optimal performance
	const [watchlist, tutorialRes, updatesRes, documentsRes] = await Promise.all([
		// Weekly watchlist
		getLatestWatchlist('explosive-swings', fetch),
		
		// Featured tutorial video
		fetch(`${baseUrl}/api/room-resources?room_id=${EXPLOSIVE_SWINGS_ROOM_ID}&resource_type=video&content_type=tutorial&is_featured=true&per_page=1`)
			.then(r => r.ok ? r.json() : { data: [] })
			.catch(() => ({ data: [] })),
		
		// Latest daily videos with tags
		fetch(`${baseUrl}/api/room-resources?room_id=${EXPLOSIVE_SWINGS_ROOM_ID}&resource_type=video&content_type=daily_video&per_page=6`)
			.then(r => r.ok ? r.json() : { data: [] })
			.catch(() => ({ data: [] })),
		
		// PDFs and documents
		fetch(`${baseUrl}/api/room-resources?room_id=${EXPLOSIVE_SWINGS_ROOM_ID}&resource_type=pdf&per_page=10`)
			.then(r => r.ok ? r.json() : { data: [] })
			.catch(() => ({ data: [] }))
	]);

	return {
		watchlist,
		tutorialVideo: tutorialRes.data?.[0] || null,
		latestUpdates: updatesRes.data || [],
		documents: documentsRes.data || [],
		roomId: EXPLOSIVE_SWINGS_ROOM_ID
	};
};

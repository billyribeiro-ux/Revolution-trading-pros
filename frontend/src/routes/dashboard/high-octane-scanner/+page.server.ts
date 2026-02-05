/**
 * High Octane Scanner Dashboard - Server Load Function
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

const HIGH_OCTANE_SCANNER_ROOM_ID = 3;

export const load = (async ({ fetch, locals }) => {
	const baseUrl = env.API_BASE_URL || 'https://revolution-trading-pros-api.fly.dev';
	// ICT 7 FIX: Pass access token from locals for authenticated API calls
	const accessToken = locals.accessToken ?? undefined;

	// ICT 7 FIX: Build headers with Authorization for authenticated requests
	const headers: Record<string, string> = {
		Accept: 'application/json',
		'Content-Type': 'application/json'
	};
	if (accessToken) {
		headers['Authorization'] = `Bearer ${accessToken}`;
	}

	// Parallel fetch for optimal performance
	const [watchlist, tutorialRes, updatesRes, documentsRes] = await Promise.all([
		// Weekly watchlist
		getLatestWatchlist('high-octane-scanner', fetch, baseUrl, accessToken),

		// Featured tutorial video
		fetch(
			`${baseUrl}/api/room-resources?room_id=${HIGH_OCTANE_SCANNER_ROOM_ID}&resource_type=video&content_type=tutorial&is_featured=true&per_page=1`,
			{ headers }
		)
			.then((r: Response) => (r.ok ? r.json() : { data: [] }))
			.catch(() => ({ data: [] })),

		// Latest daily videos with tags
		fetch(
			`${baseUrl}/api/room-resources?room_id=${HIGH_OCTANE_SCANNER_ROOM_ID}&resource_type=video&content_type=daily_video&per_page=6`,
			{ headers }
		)
			.then((r: Response) => (r.ok ? r.json() : { data: [] }))
			.catch(() => ({ data: [] })),

		// PDFs and documents
		fetch(
			`${baseUrl}/api/room-resources?room_id=${HIGH_OCTANE_SCANNER_ROOM_ID}&resource_type=pdf&per_page=10`,
			{ headers }
		)
			.then((r: Response) => (r.ok ? r.json() : { data: [] }))
			.catch(() => ({ data: [] }))
	]);

	return {
		watchlist,
		tutorialVideo: (tutorialRes.data?.[0] || null) as RoomResource | null,
		latestUpdates: (updatesRes.data || []) as RoomResource[],
		documents: (documentsRes.data || []) as RoomResource[],
		roomId: HIGH_OCTANE_SCANNER_ROOM_ID
	};
}) satisfies PageServerLoad;

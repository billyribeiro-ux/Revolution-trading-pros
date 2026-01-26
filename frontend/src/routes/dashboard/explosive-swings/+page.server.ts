/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Explosive Swings Dashboard - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description SSR pre-fetch for unified room resources with graceful degradation
 * @version 5.0.0 - Apple Principal Engineer ICT Level 7 Grade
 * @author Revolution Trading Pros Engineering
 * @standards Apple ICT 7+ | SvelteKit 2.0+ | Svelte 5 (January 2026)
 *
 * @architecture
 * ┌─────────────────────────────────────────────────────────────────────────────┐
 * │ Server Load Function (SSR)                                                  │
 * ├─────────────────────────────────────────────────────────────────────────────┤
 * │ 1. Parallel fetch for optimal performance (Promise.all)                     │
 * │ 2. Individual error handling per resource (graceful degradation)            │
 * │ 3. Type-safe return with explicit null handling                             │
 * │ 4. Fallback data on complete failure (never throws 500)                     │
 * └─────────────────────────────────────────────────────────────────────────────┘
 *
 * @resources
 * - Weekly watchlist (from $lib/server/watchlist)
 * - Tutorial video (featured, room-resources API)
 * - Latest updates (daily videos with tags)
 * - PDFs/Documents
 *
 * @security
 * - Server-side only (no client exposure of API keys)
 * - Environment variables via $env/dynamic/private
 * - Credentials included via SvelteKit fetch
 */

import { env } from '$env/dynamic/private';
import { getLatestWatchlist, type WatchlistData } from '$lib/server/watchlist';
import type { PageServerLoad } from './$types';
import type { RoomResource } from '$lib/api/room-resources';
import { ROOM_RESOURCES_ID, ROOM_SLUG } from './constants';

// ═══════════════════════════════════════════════════════════════════════════════
// TYPES - ICT 7 Type Safety
// ═══════════════════════════════════════════════════════════════════════════════

/** API response wrapper for room resources */
interface RoomResourcesResponse {
	success?: boolean;
	data?: RoomResource[];
	error?: string;
}

/** Page data returned to client */
interface ExplosiveSwingsPageData {
	watchlist: WatchlistData | null;
	tutorialVideo: RoomResource | null;
	latestUpdates: RoomResource[];
	documents: RoomResource[];
	roomId: number;
}

// ═══════════════════════════════════════════════════════════════════════════════
// CONSTANTS - ICT 7 Single Source of Truth
// ═══════════════════════════════════════════════════════════════════════════════

const LOG_PREFIX = '[explosive-swings]';
const DEFAULT_API_URL = 'https://revolution-trading-pros-api.fly.dev';

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS - ICT 7 Clean Architecture
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Safely fetch room resources with error handling
 * @param fetchFn - SvelteKit fetch function
 * @param url - Full API URL
 * @param resourceType - Resource type for logging
 * @returns Promise<RoomResourcesResponse> - Always resolves, never throws
 */
async function fetchRoomResources(
	fetchFn: typeof fetch,
	url: string,
	resourceType: string
): Promise<RoomResourcesResponse> {
	try {
		const response = await fetchFn(url);
		
		if (!response.ok) {
			console.warn(`${LOG_PREFIX} ${resourceType} fetch returned:`, response.status);
			return { data: [] };
		}
		
		return await response.json();
	} catch (error) {
		console.error(`${LOG_PREFIX} ${resourceType} fetch error:`, error);
		return { data: [] };
	}
}

/**
 * Build room resources API URL
 * @param baseUrl - API base URL
 * @param params - Query parameters
 * @returns Full URL string
 */
function buildResourceUrl(
	baseUrl: string,
	params: { resourceType: string; contentType?: string; isFeatured?: boolean; perPage?: number }
): string {
	const searchParams = new URLSearchParams({
		room_id: String(ROOM_RESOURCES_ID),
		resource_type: params.resourceType,
		per_page: String(params.perPage ?? 10)
	});
	
	if (params.contentType) {
		searchParams.set('content_type', params.contentType);
	}
	
	if (params.isFeatured) {
		searchParams.set('is_featured', 'true');
	}
	
	return `${baseUrl}/api/room-resources?${searchParams}`;
}

// ═══════════════════════════════════════════════════════════════════════════════
// LOAD FUNCTION - ICT 7 Server-Side Rendering
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Server load function for Explosive Swings dashboard
 *
 * @description Fetches all required resources in parallel with graceful degradation.
 *              Never throws - returns fallback data on complete failure.
 *
 * @param {Object} context - SvelteKit load context
 * @param {typeof fetch} context.fetch - SvelteKit fetch with credentials
 * @returns {Promise<ExplosiveSwingsPageData>} Page data for client hydration
 */
export const load = (async ({ fetch }): Promise<ExplosiveSwingsPageData> => {
	const baseUrl = env.API_BASE_URL || DEFAULT_API_URL;

	try {
		// ICT 7: Parallel fetch for optimal performance
		const [watchlist, tutorialRes, updatesRes, documentsRes] = await Promise.all([
			// Weekly watchlist
			getLatestWatchlist(ROOM_SLUG, fetch, baseUrl).catch((err): null => {
				console.error(`${LOG_PREFIX} Watchlist fetch error:`, err);
				return null;
			}),

			// Featured tutorial video
			fetchRoomResources(
				fetch,
				buildResourceUrl(baseUrl, {
					resourceType: 'video',
					contentType: 'tutorial',
					isFeatured: true,
					perPage: 1
				}),
				'Tutorial'
			),

			// Latest daily videos
			fetchRoomResources(
				fetch,
				buildResourceUrl(baseUrl, {
					resourceType: 'video',
					contentType: 'daily_video',
					perPage: 6
				}),
				'Updates'
			),

			// PDFs and documents
			fetchRoomResources(
				fetch,
				buildResourceUrl(baseUrl, {
					resourceType: 'pdf',
					perPage: 10
				}),
				'Documents'
			)
		]);

		return {
			watchlist: watchlist ?? null,
			tutorialVideo: tutorialRes.data?.[0] ?? null,
			latestUpdates: updatesRes.data ?? [],
			documents: documentsRes.data ?? [],
			roomId: ROOM_RESOURCES_ID
		};
	} catch (error) {
		// ICT 7: Never throw 500 - graceful degradation with fallback data
		console.error(`${LOG_PREFIX} FATAL ERROR in load function:`, error);
		
		return {
			watchlist: null,
			tutorialVideo: null,
			latestUpdates: [],
			documents: [],
			roomId: ROOM_RESOURCES_ID
		};
	}
}) satisfies PageServerLoad;

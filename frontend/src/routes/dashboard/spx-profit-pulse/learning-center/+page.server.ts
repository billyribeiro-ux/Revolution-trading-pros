/**
 * SPX Profit Pulse Learning Center Server-Side Loader
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 *
 * Fetches learning center videos with pagination
 * Supports filtering by category/tags
 */

import { env } from '$env/dynamic/private';
import type { PageServerLoad } from './$types';

// FIX-2026-04-26: canonical private-env URL pattern (CLAUDE.md house style).
const API_ROOT =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';
const ROOM_SLUG = 'spx-profit-pulse';

export interface VideoResponse {
	id: number;
	title: string;
	slug: string;
	description: string | null;
	video_url: string;
	embed_url: string;
	video_platform: string;
	thumbnail_url: string | null;
	duration: number | null;
	formatted_duration: string;
	content_type: string;
	video_date: string;
	formatted_date: string;
	is_published: boolean;
	is_featured: boolean;
	tags: string[];
	tag_details: Array<{ slug: string; name: string; color: string }>;
	views_count: number;
	trader: { id: number; name: string; slug: string } | null;
	rooms: Array<{ id: number; name: string; slug: string }>;
	created_at: string;
}

export interface ApiResponse {
	success: boolean;
	data: VideoResponse[];
	meta: {
		current_page: number;
		per_page: number;
		total: number;
		last_page: number;
	};
}

export const load: PageServerLoad = async ({ url, fetch }) => {
	const page = parseInt(url.searchParams.get('page') || '1');
	const category = url.searchParams.get('category');
	const perPage = 9;

	// FIX-2026-04-26: was a hard-coded empty stub with `// TODO: Implement new
	// video fetching approach`. Now wired to /api/videos with content_type=
	// learning_center, room=spx-profit-pulse. Old body kept commented per
	// CLAUDE.md "comment-don't-delete" rule.
	//
	// // OLD STUB (do not delete — see FIX-2026-04-26):
	// // return {
	// //     videos: [],
	// //     meta: { current_page: 1, per_page: perPage, total: 0, last_page: 1 },
	// //     activeFilter: category || 'all',
	// //     error: null
	// // };

	const baseReturn = {
		activeFilter: category || 'all',
		error: null as string | null
	};

	try {
		const roomRes = await fetch(`${API_ROOT}/api/trading-rooms/${ROOM_SLUG}`);
		if (!roomRes.ok) {
			return {
				...baseReturn,
				videos: [] as VideoResponse[],
				meta: { current_page: page, per_page: perPage, total: 0, last_page: 1 },
				dataUnavailable: true,
				reason: `Room lookup returned ${roomRes.status}`
			};
		}
		const roomBody = (await roomRes.json()) as { data?: { id?: number } };
		const roomId = roomBody?.data?.id;
		if (!roomId) {
			return {
				...baseReturn,
				videos: [] as VideoResponse[],
				meta: { current_page: page, per_page: perPage, total: 0, last_page: 1 },
				dataUnavailable: true,
				reason: 'Trading room not found'
			};
		}

		const qs = new URLSearchParams({
			room_id: String(roomId),
			content_type: 'learning_center',
			page: String(page),
			per_page: String(perPage)
		});
		if (category && category !== 'all' && category !== '0') {
			qs.set('tags', category);
		}

		const vidRes = await fetch(`${API_ROOT}/api/videos?${qs.toString()}`);
		if (!vidRes.ok) {
			return {
				...baseReturn,
				videos: [] as VideoResponse[],
				meta: { current_page: page, per_page: perPage, total: 0, last_page: 1 },
				dataUnavailable: true,
				reason: `Backend returned ${vidRes.status}`
			};
		}
		const body = (await vidRes.json()) as ApiResponse;
		return {
			...baseReturn,
			videos: body.data ?? [],
			meta:
				body.meta ?? { current_page: page, per_page: perPage, total: 0, last_page: 1 },
			dataUnavailable: false
		};
	} catch (e) {
		console.error('[dashboard/spx-profit-pulse/learning-center] video fetch failed:', e);
		return {
			...baseReturn,
			videos: [] as VideoResponse[],
			meta: { current_page: page, per_page: perPage, total: 0, last_page: 1 },
			dataUnavailable: true,
			reason: 'Network error contacting video backend'
		};
	}
};

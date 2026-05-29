/**
 * Learning Center Server-Side Loader - Dynamic Room
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 *
 * Fetches learning center videos with pagination
 * Works with any trading room via [room_slug] parameter
 *
 * @version 2.0.0
 */

import { env } from '$env/dynamic/private';
import type { PageServerLoad } from './$types';

// FIX-2026-04-26: canonical private-env URL pattern (CLAUDE.md house style).
const API_ROOT = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

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
	[key: string]: unknown; // Index signature for compatibility with VideoData
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

function getRoomName(slug: string): string {
	const roomNames: Record<string, string> = {
		'day-trading-room': 'Day Trading Room',
		'swing-trading-room': 'Swing Trading Room',
		'small-accounts-room': 'Small Accounts Room',
		'options-room': 'Options Room',
		'high-octane-scanner': 'High Octane Scanner'
	};
	return (
		roomNames[slug] ||
		slug
			.split('-')
			.map((w) => w.charAt(0).toUpperCase() + w.slice(1))
			.join(' ')
	);
}

export const load: PageServerLoad = async ({ params, url, fetch }) => {
	const { room_slug } = params;
	const page = parseInt(url.searchParams.get('page') || '1');
	const category = url.searchParams.get('category');
	const perPage = 9;

	// FIX-2026-04-26: was a hard-coded empty stub with `// TODO: Implement new
	// video fetching approach`. Now wired to /api/videos via room-slug → room-id
	// resolution against /api/trading-rooms/:slug. Old body kept commented as the
	// audit (CLAUDE.md / no_delete_comment_first) requires we never silently
	// remove a fallback path while the new code is settling.
	//
	// // OLD STUB (do not delete — see FIX-2026-04-26):
	// // return {
	// //     videos: [],
	// //     meta: { current_page: 1, per_page: perPage, total: 0, last_page: 1 },
	// //     activeFilter: category || 'all',
	// //     roomSlug: room_slug,
	// //     roomName: getRoomName(room_slug as string),
	// //     error: null
	// // };

	const baseReturn = {
		activeFilter: category || 'all',
		roomSlug: room_slug,
		roomName: getRoomName(room_slug as string),
		error: null as string | null,
		dataUnavailable: false,
		reason: undefined as string | undefined
	};

	try {
		// Step 1: resolve room slug → room id (backend videos endpoint takes room_id only).
		const roomRes = await fetch(`${API_ROOT}/api/trading-rooms/${encodeURIComponent(room_slug)}`);
		if (!roomRes.ok) {
			return {
				...baseReturn,
				videos: [],
				meta: { current_page: 1, per_page: perPage, total: 0, last_page: 1 },
				dataUnavailable: true,
				reason: `Room lookup returned ${roomRes.status}`
			};
		}
		const roomBody = (await roomRes.json()) as { data?: { id?: number } };
		const roomId = roomBody?.data?.id;
		if (!roomId) {
			return {
				...baseReturn,
				videos: [],
				meta: { current_page: 1, per_page: perPage, total: 0, last_page: 1 },
				dataUnavailable: true,
				reason: 'Trading room not found'
			};
		}

		// Step 2: fetch learning_center videos for that room.
		const qs = new URLSearchParams({
			room_id: String(roomId),
			content_type: 'learning_center',
			page: String(page),
			per_page: String(perPage)
		});
		// Treat WordPress numeric category id as a tag filter when present.
		if (category && category !== 'all' && category !== '0') {
			qs.set('tags', category);
		}
		const vidRes = await fetch(`${API_ROOT}/api/videos?${qs.toString()}`);
		if (!vidRes.ok) {
			return {
				...baseReturn,
				videos: [],
				meta: { current_page: page, per_page: perPage, total: 0, last_page: 1 },
				dataUnavailable: true,
				reason: `Backend returned ${vidRes.status}`
			};
		}
		const body = (await vidRes.json()) as ApiResponse;
		return {
			...baseReturn,
			videos: body.data ?? [],
			meta: body.meta ?? { current_page: page, per_page: perPage, total: 0, last_page: 1 }
		};
	} catch (e) {
		console.error('[dashboard/[room_slug]/learning-center] video fetch failed:', e);
		return {
			...baseReturn,
			videos: [],
			meta: { current_page: page, per_page: perPage, total: 0, last_page: 1 },
			dataUnavailable: true,
			reason: 'Network error contacting video backend'
		};
	}
};

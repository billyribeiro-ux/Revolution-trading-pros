/**
 * Trading Room Archive Page Server (Dynamic Route)
 * ═══════════════════════════════════════════════════════════════════════════
 * Apple ICT 11+ Principal Engineer Implementation
 *
 * Fetches room archive videos for any trading room.
 * Supports: day-trading-room, swing-trading-room, small-account-mentorship
 *
 * @version 3.0.0
 */

import { env } from '$env/dynamic/private';
import type { PageServerLoad } from './$types';

// FIX-2026-04-26: canonical private-env URL pattern (CLAUDE.md house style).
const API_ROOT =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

// Room configuration
const ROOM_CONFIG: Record<string, { name: string; startHereUrl: string }> = {
	'day-trading-room': {
		name: 'Day Trading Room',
		startHereUrl: '/dashboard/day-trading-room/start-here'
	},
	'swing-trading-room': {
		name: 'Swing Trading Room',
		startHereUrl: '/dashboard/swing-trading-room/start-here'
	},
	'small-account-mentorship': {
		name: 'Small Account Mentorship',
		startHereUrl: '/dashboard/small-account-mentorship/start-here'
	}
};

// Video response from API
interface VideoResponse {
	id: number;
	title: string;
	slug: string;
	description: string | null;
	video_url: string;
	thumbnail_url: string | null;
	duration: number | null;
	formatted_duration: string;
	content_type: string;
	video_date: string;
	formatted_date: string;
	is_published: boolean;
	trader: {
		id: number;
		name: string;
		slug: string;
	} | null;
	rooms: Array<{
		id: number;
		name: string;
		slug: string;
	}>;
	tags: string[];
	[key: string]: unknown;
}

// Page data type export for +page.svelte
export interface DynamicArchivePageData {
	roomSlug: string;
	roomName: string;
	startHereUrl: string;
	videos: VideoResponse[];
	meta: {
		current_page: number;
		per_page: number;
		total: number;
		last_page: number;
	};
	search: string;
	error: string | null;
	dataUnavailable?: boolean;
	reason?: string;
}

export const load: PageServerLoad = async ({
	url,
	params,
	fetch
}): Promise<DynamicArchivePageData> => {
	const roomSlug = params.room_slug;

	// Get room config
	const roomConfig = ROOM_CONFIG[roomSlug] || {
		name: roomSlug
			.split('-')
			.map((w) => w.charAt(0).toUpperCase() + w.slice(1))
			.join(' '),
		startHereUrl: `/dashboard/${roomSlug}/start-here`
	};

	// Get query params
	const page = parseInt(url.searchParams.get('page') || '1');
	const search = url.searchParams.get('search') || '';
	const perPage = 50;

	// FIX-2026-04-26: was a hard-coded empty stub with `// TODO: Implement new
	// video fetching approach`. Now wired to /api/videos with content_type=archive.
	// Old body kept commented per CLAUDE.md "comment-don't-delete" rule.
	//
	// // OLD STUB (do not delete — see FIX-2026-04-26):
	// // return {
	// //     roomSlug,
	// //     roomName: roomConfig.name,
	// //     startHereUrl: roomConfig.startHereUrl,
	// //     videos: [],
	// //     meta: { current_page: 1, per_page: 50, total: 0, last_page: 1 },
	// //     search,
	// //     error: null
	// // };

	const baseReturn = {
		roomSlug,
		roomName: roomConfig.name,
		startHereUrl: roomConfig.startHereUrl,
		search,
		error: null as string | null
	};

	try {
		const roomRes = await fetch(`${API_ROOT}/api/trading-rooms/${encodeURIComponent(roomSlug)}`);
		if (!roomRes.ok) {
			return {
				...baseReturn,
				videos: [],
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
				videos: [],
				meta: { current_page: page, per_page: perPage, total: 0, last_page: 1 },
				dataUnavailable: true,
				reason: 'Trading room not found'
			};
		}

		const qs = new URLSearchParams({
			room_id: String(roomId),
			content_type: 'archive',
			page: String(page),
			per_page: String(perPage)
		});
		if (search) qs.set('search', search);

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
		const body = (await vidRes.json()) as {
			data?: VideoResponse[];
			meta?: { current_page: number; per_page: number; total: number; last_page: number };
		};
		return {
			...baseReturn,
			videos: body.data ?? [],
			meta: body.meta ?? { current_page: page, per_page: perPage, total: 0, last_page: 1 },
			dataUnavailable: false
		};
	} catch (e) {
		console.error('[dashboard/[room_slug]/trading-room-archive] video fetch failed:', e);
		return {
			...baseReturn,
			videos: [],
			meta: { current_page: page, per_page: perPage, total: 0, last_page: 1 },
			dataUnavailable: true,
			reason: 'Network error contacting video backend'
		};
	}
};

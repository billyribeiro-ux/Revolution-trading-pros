/**
 * Premium Daily Videos - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * ICT 11+ Principal Engineer Grade
 * Fetches videos with pagination support
 * NOW SUPPORTS DYNAMIC ROOM SELECTION via [room_slug] parameter
 *
 * @version 5.0.0 - January 2026
 */

import { env } from '$env/dynamic/private';
import type { PageServerLoad } from './$types';

// FIX-2026-04-26: canonical private-env URL pattern (CLAUDE.md house style).
const API_ROOT = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

export interface DailyVideo {
	id: number;
	title: string;
	slug: string;
	date: string;
	trader: string;
	excerpt: string;
	thumbnail: string;
	duration?: string;
	isVideo: boolean;
}

export interface PageData {
	videos: DailyVideo[];
	pagination: {
		page: number;
		perPage: number;
		total: number;
		totalPages: number;
	};
	roomSlug: string;
	roomName: string;
	error?: string;
	dataUnavailable?: boolean;
	reason?: string;
}

// FIX-2026-04-26: thin shape of the backend video row we actually consume here.
// (Full shape lives in api/src/models/video.rs::VideoResponse.)
interface BackendVideo {
	id: number;
	title: string;
	slug: string;
	description: string | null;
	thumbnail_url: string | null;
	formatted_date: string;
	formatted_duration: string;
	trader: { id: number; name: string; slug: string } | null;
}

export const load: PageServerLoad = async ({ params, url, fetch }): Promise<PageData> => {
	const { room_slug } = params;
	const page = parseInt(url.searchParams.get('page') || '1');
	const search = url.searchParams.get('search') || '';
	const perPage = 12;

	// FIX-2026-04-26: was mock data with `// TODO: Implement new video fetching
	// approach`. Now wired to /api/videos with content_type=daily_video filter,
	// resolving room slug → room id via /api/trading-rooms/:slug.
	// Old mock-data path kept (commented) below per CLAUDE.md "comment-don't-delete"
	// rule until production traffic confirms wiring.
	//
	// // OLD STUB (do not delete — see FIX-2026-04-26):
	// // return getMockData(room_slug as string, page, perPage, search);

	const roomName = getRoomName(room_slug as string);
	const baseError = (reason: string, actualPage = page): PageData => ({
		videos: [],
		pagination: { page: actualPage, perPage, total: 0, totalPages: 1 },
		roomSlug: room_slug as string,
		roomName,
		dataUnavailable: true,
		reason
	});

	try {
		const roomRes = await fetch(
			`${API_ROOT}/api/trading-rooms/${encodeURIComponent(room_slug as string)}`
		);
		if (!roomRes.ok) return baseError(`Room lookup returned ${roomRes.status}`);
		const roomBody = (await roomRes.json()) as { data?: { id?: number } };
		const roomId = roomBody?.data?.id;
		if (!roomId) return baseError('Trading room not found');

		const qs = new URLSearchParams({
			room_id: String(roomId),
			content_type: 'daily_video',
			page: String(page),
			per_page: String(perPage)
		});
		if (search) qs.set('search', search);

		const vidRes = await fetch(`${API_ROOT}/api/videos?${qs.toString()}`);
		if (!vidRes.ok) return baseError(`Backend returned ${vidRes.status}`);

		const body = (await vidRes.json()) as {
			data?: BackendVideo[];
			meta?: { current_page: number; per_page: number; total: number; last_page: number };
		};

		const videos: DailyVideo[] = (body.data ?? []).map((v) => ({
			id: v.id,
			title: v.title,
			slug: v.slug,
			date: v.formatted_date,
			trader: v.trader?.name ?? '',
			excerpt: v.description ?? '',
			thumbnail:
				v.thumbnail_url ||
				'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			duration: v.formatted_duration || undefined,
			isVideo: true
		}));

		const meta = body.meta ?? { current_page: page, per_page: perPage, total: 0, last_page: 1 };
		return {
			videos,
			pagination: {
				page: meta.current_page,
				perPage: meta.per_page,
				total: meta.total,
				totalPages: meta.last_page
			},
			roomSlug: room_slug as string,
			roomName,
			dataUnavailable: false
		};
	} catch (e) {
		console.error('[dashboard/[room_slug]/daily-videos] video fetch failed:', e);
		return baseError('Network error contacting video backend');
	}
};

function getRoomName(slug: string): string {
	const roomNames: Record<string, string> = {
		'day-trading-room': 'Day Trading Room',
		'high-octane-scanner': 'High Octane Scanner',
		'swing-trading-room': 'Swing Trading Room',
		'options-room': 'Options Room'
	};
	return (
		roomNames[slug] ||
		slug
			.split('-')
			.map((w) => w.charAt(0).toUpperCase() + w.slice(1))
			.join(' ')
	);
}

// FIX-2026-04-26: getMockData kept (commented usage above) per CLAUDE.md
// "comment-don't-delete" rule. Suppress unused-warning until proven safe to remove.
// eslint-disable-next-line @typescript-eslint/no-unused-vars
function getMockData(roomSlug: string, page: number, perPage: number, search: string): PageData {
	// Mock video data for development when backend is unavailable
	const allVideos: DailyVideo[] = [
		{
			id: 1,
			title: 'How to use Bookmap to make more informed trades',
			slug: 'bookmap',
			date: 'January 2, 2026',
			trader: 'Kody Ashmore',
			excerpt:
				"You asked for it, you got it. Here are Kody's Bookmap tools and how he uses them to make better informed trades.",
			thumbnail: 'https://cdn.simplertrading.com/2025/02/07135413/SimplerCentral_KA.jpg',
			isVideo: true
		},
		{
			id: 2,
			title: 'Cautious entry into 2026',
			slug: 'cautious-entry-into-2026',
			date: 'December 31, 2025',
			trader: 'Henry Gambell',
			excerpt:
				"If Santa doesn't show up, the first bit of 2026 may be a little precarious. With that in mind, let's dive in to some of the most important charts for the new year.",
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			isVideo: true
		},
		{
			id: 3,
			title: 'SPX Snoozefest',
			slug: 'spx-snoozefest',
			date: 'December 30, 2025',
			trader: 'Heather',
			excerpt:
				"We've had two days of some very narrow ranges in the indices. It's almost as though the market has had an amazing year and just needs to rest a bit before making its next move!",
			thumbnail: 'https://cdn.simplertrading.com/2025/11/18171423/MTT_HV.jpg',
			isVideo: true
		}
	];

	// Filter by search if provided
	const filteredVideos = search
		? allVideos.filter((v) => v.title.toLowerCase().includes(search.toLowerCase()))
		: allVideos;

	const total = filteredVideos.length;
	const totalPages = Math.ceil(total / perPage);
	const start = (page - 1) * perPage;
	const videos = filteredVideos.slice(start, start + perPage);

	return {
		videos,
		pagination: {
			page,
			perPage,
			total,
			totalPages
		},
		roomSlug,
		roomName: getRoomName(roomSlug)
	};
}

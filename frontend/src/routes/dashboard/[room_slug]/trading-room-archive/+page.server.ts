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
}

export const load: PageServerLoad = async ({
	url,
	params
}): Promise<DynamicArchivePageData> => {
	void env;
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
	const page = url.searchParams.get('page') || '1';
	void page;
	const search = url.searchParams.get('search') || '';

	// TODO: Implement new video fetching approach
	// Returning empty data until new implementation is ready
	return {
		roomSlug,
		roomName: roomConfig.name,
		startHereUrl: roomConfig.startHereUrl,
		videos: [],
		meta: { current_page: 1, per_page: 50, total: 0, last_page: 1 },
		search,
		error: null
	};
};

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

import type { PageServerLoad } from './$types';


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

export const load: PageServerLoad = async ({ params, url }) => {
	const { room_slug } = params;
	const page = parseInt(url.searchParams.get('page') || '1');
	void page;
	const category = url.searchParams.get('category');
	const perPage = 9;

	// TODO: Implement new video fetching approach
	// Returning empty data until new implementation is ready
	return {
		videos: [],
		meta: { current_page: 1, per_page: perPage, total: 0, last_page: 1 },
		activeFilter: category || 'all',
		roomSlug: room_slug,
		roomName: getRoomName(room_slug as string),
		error: null
	};
};

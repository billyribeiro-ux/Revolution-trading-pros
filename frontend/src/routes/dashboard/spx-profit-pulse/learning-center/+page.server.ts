/**
 * SPX Profit Pulse Learning Center Server-Side Loader
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 *
 * Fetches learning center videos with pagination
 * Supports filtering by category/tags
 */

import type { PageServerLoad } from './$types';

const _API_BASE = 'https://revolution-trading-pros-api.fly.dev';

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

// Category mapping from WordPress IDs to tags (SPX-specific categories)
const _CATEGORY_TAG_MAP: Record<string, string> = {
	'2929': 'charting-indicators-tools',
	'329': 'member-webinar',
	'528': 'methodology',
	'529': 'trade-setups'
};

export const load: PageServerLoad = async ({ url }) => {
	const _page = parseInt(url.searchParams.get('page') || '1');
	const category = url.searchParams.get('category');
	const perPage = 9;

	// TODO: Implement new video fetching approach
	// Returning empty data until new implementation is ready
	return {
		videos: [],
		meta: { current_page: 1, per_page: perPage, total: 0, last_page: 1 },
		activeFilter: category || 'all',
		error: null
	};
};

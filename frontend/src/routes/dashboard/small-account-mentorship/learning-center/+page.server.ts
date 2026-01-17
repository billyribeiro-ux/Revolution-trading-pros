/**
 * Learning Center Server-Side Loader
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 *
 * Fetches learning center videos with pagination
 * Supports filtering by category/tags
 */

import type { PageServerLoad } from './$types';

const API_BASE = 'https://revolution-trading-pros-api.fly.dev';

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

// Category mapping from WordPress IDs to tags
const CATEGORY_TAG_MAP: Record<string, string> = {
	'529': 'trade-setups',
	'528': 'methodology',
	'329': 'member-webinar',
	'2932': 'trade-management',
	'531': 'indicators',
	'3260': 'options',
	'469': 'foundation',
	'527': 'fundamentals',
	'522': 'simpler-tech',
	'2929': 'charting-indicators-tools',
	'530': 'charting',
	'3515': 'drama-free-daytrades',
	'3516': 'quick-hits-daytrades',
	'537': 'psychology',
	'775': 'trading-platform',
	'3055': 'calls',
	'447': 'thinkorswim',
	'446': 'tradestation',
	'776': 'charting-software',
	'772': 'trading-computer',
	'3057': 'calls-puts-credit-spreads',
	'3056': 'puts',
	'3514': 'profit-recycling',
	'791': 'trade-strategies',
	'774': 'website-support',
	'2927': 'options-strategies',
	'457': 'crypto',
	'2931': 'fibonacci-options',
	'2928': 'pricing-volatility',
	'459': 'crypto-indicators',
	'771': 'browser-support',
	'2930': 'earnings-expiration'
};

export const load: PageServerLoad = async ({ url, fetch, cookies }) => {
	const page = parseInt(url.searchParams.get('page') || '1');
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

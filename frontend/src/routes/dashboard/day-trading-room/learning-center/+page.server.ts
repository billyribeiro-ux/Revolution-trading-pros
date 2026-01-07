/**
 * Learning Center Server-Side Loader
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 * 
 * Fetches learning center videos from the unified videos API
 * Supports filtering by category/tags and pagination
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

	// Build API URL
	const apiUrl = new URL(`${API_BASE}/api/unified-videos`);
	apiUrl.searchParams.set('content_type', 'learning_center');
	apiUrl.searchParams.set('is_published', 'true');
	apiUrl.searchParams.set('page', page.toString());
	apiUrl.searchParams.set('per_page', perPage.toString());
	apiUrl.searchParams.set('sort_by', 'video_date');
	apiUrl.searchParams.set('sort_dir', 'desc');

	// Add tag filter if category selected
	if (category && category !== '0' && CATEGORY_TAG_MAP[category]) {
		apiUrl.searchParams.set('tags', CATEGORY_TAG_MAP[category]);
	}

	// Add room filter for day trading room
	apiUrl.searchParams.set('room_id', '1'); // Day Trading Room ID

	try {
		const token = cookies.get('access_token');
		const headers: Record<string, string> = {
			'Content-Type': 'application/json',
			'Accept': 'application/json'
		};

		if (token) {
			headers['Authorization'] = `Bearer ${token}`;
		}

		const response = await fetch(apiUrl.toString(), { headers });

		if (!response.ok) {
			console.error(`[LearningCenter] API error: ${response.status}`);
			return {
				videos: [],
				meta: { current_page: 1, per_page: perPage, total: 0, last_page: 1 },
				activeFilter: category || 'all',
				error: `Failed to fetch videos: ${response.status}`
			};
		}

		const data: ApiResponse = await response.json();

		return {
			videos: data.data || [],
			meta: data.meta || { current_page: 1, per_page: perPage, total: 0, last_page: 1 },
			activeFilter: category || 'all',
			error: null
		};
	} catch (error) {
		console.error('[LearningCenter] Fetch error:', error);
		return {
			videos: [],
			meta: { current_page: 1, per_page: perPage, total: 0, last_page: 1 },
			activeFilter: category || 'all',
			error: 'Failed to connect to API'
		};
	}
};

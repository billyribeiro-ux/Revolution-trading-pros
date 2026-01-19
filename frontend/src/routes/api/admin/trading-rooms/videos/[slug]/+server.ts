/**
 * Trading Room Videos by Room Slug API Endpoint
 *
 * Returns videos for a specific trading room.
 *
 * @version 1.0.0 - December 2025
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';


// Room slug to ID mapping
const roomSlugToId: Record<string, number> = {
	'day-trading-room': 1,
	'swing-trading-room': 2,
	'small-account-mentorship': 3,
	'spx-profit-pulse': 4,
	'explosive-swing': 5
};

// Mock videos data (same as parent endpoint)
const mockVideos = [
	{
		id: 1,
		trading_room_id: 1,
		trader_id: 1,
		title: 'Morning Market Analysis - SPY Setup',
		description: "Analysis of today's market conditions and SPY trading opportunities.",
		video_url: 'https://vimeo.com/123456789',
		video_platform: 'vimeo',
		video_id: '123456789',
		thumbnail_url: null,
		duration: 1800,
		video_date: '2025-12-10',
		is_featured: true,
		is_published: true,
		views_count: 245,
		tags: ['market-review', 'technical-analysis'],
		metadata: null,
		trader: { id: 1, name: 'Mike Thompson', photo_url: null },
		created_at: '2025-12-10T09:00:00Z',
		updated_at: '2025-12-10T09:00:00Z'
	},
	{
		id: 2,
		trading_room_id: 1,
		trader_id: 1,
		title: 'Options Flow Analysis - December 10th',
		description: 'Reviewing options flow and unusual activity.',
		video_url: 'https://vimeo.com/123456790',
		video_platform: 'vimeo',
		video_id: '123456790',
		thumbnail_url: null,
		duration: 2400,
		video_date: '2025-12-10',
		is_featured: false,
		is_published: true,
		views_count: 189,
		tags: ['options-strategies', 'trade-setups'],
		metadata: null,
		trader: { id: 1, name: 'Mike Thompson', photo_url: null },
		created_at: '2025-12-10T14:00:00Z',
		updated_at: '2025-12-10T14:00:00Z'
	},
	{
		id: 3,
		trading_room_id: 2,
		trader_id: 2,
		title: 'Swing Trading Weekly Watchlist',
		description: 'Top stocks to watch for swing trading this week.',
		video_url: 'https://vimeo.com/123456791',
		video_platform: 'vimeo',
		video_id: '123456791',
		thumbnail_url: null,
		duration: 1500,
		video_date: '2025-12-09',
		is_featured: true,
		is_published: true,
		views_count: 312,
		tags: ['trade-setups', 'technical-analysis'],
		metadata: null,
		trader: { id: 2, name: 'Sarah Chen', photo_url: null },
		created_at: '2025-12-09T10:00:00Z',
		updated_at: '2025-12-09T10:00:00Z'
	},
	{
		id: 4,
		trading_room_id: 3,
		trader_id: 4,
		title: 'Small Account Strategy - Risk Management',
		description: 'How to properly manage risk with a small trading account.',
		video_url: 'https://vimeo.com/123456792',
		video_platform: 'vimeo',
		video_id: '123456792',
		thumbnail_url: null,
		duration: 2100,
		video_date: '2025-12-08',
		is_featured: false,
		is_published: true,
		views_count: 456,
		tags: ['risk-management', 'small-accounts', 'position-sizing'],
		metadata: null,
		trader: { id: 4, name: 'Emily Davis', photo_url: null },
		created_at: '2025-12-08T11:00:00Z',
		updated_at: '2025-12-08T11:00:00Z'
	},
	{
		id: 5,
		trading_room_id: 4,
		trader_id: 3,
		title: 'SPX 0DTE Trade Breakdown',
		description: 'Detailed breakdown of SPX 0DTE trading strategy.',
		video_url: 'https://vimeo.com/123456793',
		video_platform: 'vimeo',
		video_id: '123456793',
		thumbnail_url: null,
		duration: 1200,
		video_date: '2025-12-09',
		is_featured: true,
		is_published: true,
		views_count: 523,
		tags: ['options-strategies', 'trade-setups'],
		metadata: null,
		trader: { id: 3, name: 'James Wilson', photo_url: null },
		created_at: '2025-12-09T15:00:00Z',
		updated_at: '2025-12-09T15:00:00Z'
	}
];

// Try to fetch from backend
async function fetchFromBackend(endpoint: string, options?: RequestInit): Promise<any | null> {
	const BACKEND_URL = PROD_BACKEND;
	if (!BACKEND_URL) return null;

	try {
		const response = await fetch(`${BACKEND_URL}${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...options?.headers
			}
		});

		if (!response.ok) return null;
		return await response.json();
	} catch (error) {
		return null;
	}
}

// GET - List videos for a specific room by slug
export const GET: RequestHandler = async ({ params, url, request }) => {
	const { slug } = params;
	const traderId = url.searchParams.get('trader_id');
	const search = url.searchParams.get('search');
	const publishedOnly = url.searchParams.get('published_only') !== 'false';
	const page = parseInt(url.searchParams.get('page') || '1');
	const perPage = parseInt(url.searchParams.get('per_page') || '20');

	// Try backend first
	const backendData = await fetchFromBackend(
		`/api/admin/trading-rooms/videos/${slug}?${url.searchParams.toString()}`,
		{
			headers: { Authorization: request.headers.get('Authorization') || '' }
		}
	);

	if (backendData?.success) {
		return json(backendData);
	}

	// Fallback to mock data
	const roomId = roomSlugToId[slug ?? ''];
	if (!roomId) {
		throw error(404, `Trading room '${slug}' not found`);
	}

	let filteredVideos = mockVideos.filter((v) => v.trading_room_id === roomId);

	if (traderId && traderId !== 'all') {
		filteredVideos = filteredVideos.filter((v) => v.trader_id?.toString() === traderId);
	}

	if (search) {
		const searchLower = search.toLowerCase();
		filteredVideos = filteredVideos.filter(
			(v) =>
				v.title.toLowerCase().includes(searchLower) ||
				v.description?.toLowerCase().includes(searchLower)
		);
	}

	if (publishedOnly) {
		filteredVideos = filteredVideos.filter((v) => v.is_published);
	}

	// Sort by date descending
	filteredVideos.sort(
		(a, b) => new Date(b.video_date).getTime() - new Date(a.video_date).getTime()
	);

	// Paginate
	const total = filteredVideos.length;
	const start = (page - 1) * perPage;
	const paginatedVideos = filteredVideos.slice(start, start + perPage);

	return json({
		success: true,
		data: {
			data: paginatedVideos,
			current_page: page,
			per_page: perPage,
			total,
			last_page: Math.ceil(total / perPage)
		},
		_mock: true
	});
};

/**
 * Trading Room Videos API Endpoint
 *
 * Handles video management for trading rooms.
 * Returns mock data when backend is not connected.
 *
 * @version 1.0.0 - December 2025
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

// Production fallback - Rust API on Fly.io
import { env } from '$env/dynamic/private';
const BACKEND_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

// Mock videos data — PRESERVED as a local-dev reference only (audit
// 2026-05-16). The real backend (`admin_list_videos`, trading_rooms.rs)
// is implemented and the proxy now translates params to it, so this is
// no longer a live data path. `_`-prefixed so the config's
// varsIgnorePattern (/^_/) silences no-unused-vars without deleting it.
const _mockVideos = [
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
		trader: {
			id: 1,
			name: 'Mike Thompson',
			photo_url: null
		},
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
		trader: {
			id: 1,
			name: 'Mike Thompson',
			photo_url: null
		},
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
		trader: {
			id: 2,
			name: 'Sarah Chen',
			photo_url: null
		},
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
		trader: {
			id: 4,
			name: 'Emily Davis',
			photo_url: null
		},
		created_at: '2025-12-08T11:00:00Z',
		updated_at: '2025-12-08T11:00:00Z'
	}
];

// Try to fetch from backend
async function fetchFromBackend(endpoint: string, options?: RequestInit): Promise<any | null> {
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
	} catch (_error) {
		return null;
	}
}

// GET - List videos
export const GET: RequestHandler = async ({ url, request, cookies }) => {
	// FIX (audit 2026-05-16, TRADING_ROOMS_BACKEND_GAPS Gap 2): the old code
	// forwarded `url.searchParams.toString()` verbatim. The frontend sends
	// `published_only` / `trading_room_id` / `trader_id`, but the Rust
	// `VideosQuery` (trading_rooms.rs) only deserializes
	// `page` / `per_page` / `room_slug` / `content_type` / `is_published`,
	// so every filter except pagination was silently dropped by Axum.
	//
	// Translate to the backend's actual contract instead of pretending:
	//  - published_only -> is_published (default true unless "false")
	//  - page / per_page -> pass through (names already align)
	//  - content_type / room_slug -> pass through if the caller sends them
	//  - trading_room_id / trader_id -> NOT forwardable: the admin video
	//    LIST endpoint has no room/trader column filter (room scoping lives
	//    on the separate path handler `/videos/:slug`). Forwarding them
	//    only created the illusion of filtering. Dropped explicitly; a
	//    real room/trader filter is tracked in the gaps doc as future work.
	const backendQuery = new URLSearchParams();
	const page = parseInt(url.searchParams.get('page') || '1');
	const perPage = parseInt(url.searchParams.get('per_page') || '20');
	backendQuery.set('page', String(page));
	backendQuery.set('per_page', String(perPage));

	const publishedOnly = url.searchParams.get('published_only');
	if (publishedOnly !== null) {
		backendQuery.set('is_published', publishedOnly === 'false' ? 'false' : 'true');
	}
	const contentType = url.searchParams.get('content_type');
	if (contentType) backendQuery.set('content_type', contentType);
	const roomSlug = url.searchParams.get('room_slug');
	if (roomSlug) backendQuery.set('room_slug', roomSlug);

	// FIX-2026-04-26: prefer canonical rtp_access_token cookie, fall back to header.
	// Old: headers: { Authorization: request.headers.get('Authorization') || '' }
	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) error(401, 'Unauthorized');

	// Try backend first
	const backendData = await fetchFromBackend(
		`/api/admin/trading-rooms/videos?${backendQuery.toString()}`,
		{
			headers: { Authorization: `Bearer ${token}` }
		}
	);

	if (backendData?.success) {
		return json(backendData);
	}

	// FIX-2026-04-26-audit (P1-11): surface backend failure instead of returning phantom
	// mock videos. Admins were seeing fake data with no indication of a real failure.
	// TODO(2026-04-26-audit): gate mock on env.ENABLE_MOCK_DATA if needed for local dev.
	console.error('[Trading-rooms videos proxy] Backend unavailable or non-success:', backendData);
	return json(
		{
			success: false,
			error: 'Unable to load videos from backend. Please check the API connection.',
			_mock: false
		},
		{ status: 502 }
	);
};

// POST - Create video
export const POST: RequestHandler = async ({ request, cookies }) => {
	const body = await request.json();

	// FIX-2026-04-26: prefer canonical rtp_access_token cookie, fall back to header.
	// Old: headers: { Authorization: request.headers.get('Authorization') || '' }
	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) error(401, 'Unauthorized');

	// Try backend first
	const backendData = await fetchFromBackend('/api/admin/trading-rooms/videos', {
		method: 'POST',
		headers: { Authorization: `Bearer ${token}` },
		body: JSON.stringify(body)
	});

	if (backendData?.success) {
		return json(backendData);
	}

	// FIX-2026-04-26-audit (P1-11): surface POST failure — a silent mock-create would
	// persist nothing to the DB while appearing to succeed.
	console.error('[Trading-rooms videos proxy POST] Backend unavailable or non-success:', backendData);
	return json(
		{
			success: false,
			error: 'Unable to create video — backend is unavailable. Please try again.'
		},
		{ status: 502 }
	);
};

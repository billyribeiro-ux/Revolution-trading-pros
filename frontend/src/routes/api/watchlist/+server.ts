/**
 * Weekly Watchlist API Endpoint
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * CRUD operations for weekly watchlist items.
 * Supports room-specific targeting for all 6 services.
 * Falls back to mock data when backend is unavailable.
 *
 * @version 2.0.0 - December 2025 - Added room targeting
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { ALL_ROOM_IDS } from '$lib/config/rooms';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface WatchlistDate {
	date: string;
	spreadsheetUrl: string;
}

export interface WatchlistItem {
	id: number;
	slug: string;
	title: string;
	subtitle: string;
	trader: string;
	traderImage?: string;
	datePosted: string;
	weekOf: string;
	video: {
		src: string;
		poster: string;
		title: string;
	};
	spreadsheet: {
		src: string;
	};
	watchlistDates?: WatchlistDate[]; // NEW: Multiple date versions
	description: string;
	status: 'published' | 'draft' | 'archived';
	rooms: string[]; // Room targeting
	createdAt: string;
	updatedAt: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// MOCK DATA - All rooms by default
// ═══════════════════════════════════════════════════════════════════════════

const mockWatchlistItems: WatchlistItem[] = [
	{
		id: 1,
		slug: '01032026-melissa-beegle',
		title: 'Weekly Watchlist with Melissa Beegle',
		subtitle: 'Week of January 3, 2026',
		trader: 'Melissa Beegle',
		traderImage: 'https://cdn.simplertrading.com/2025/03/09130833/Melissa-WeeklyWatchlist.jpg',
		datePosted: 'January 3, 2026',
		weekOf: '2026-01-03',
		video: {
			src: 'https://cloud-streaming.s3.amazonaws.com/WeeklyWatchlist/WW-MB-01032026.mp4',
			poster: 'https://cdn.simplertrading.com/2025/03/09130833/Melissa-WeeklyWatchlist.jpg',
			title: 'Weekly Watchlist with Melissa Beegle'
		},
		spreadsheet: {
			src: 'https://docs.google.com/spreadsheets/d/e/2PACX-1vS0DkJXxG0rA7tUzJl2-au8DRWf486KZyPbbjeaVNp4fJ1ZyO0qPWLUkHia-TWEdDRCnPFPMJjFc-1V/pubhtml'
		},
		watchlistDates: [
			{
				date: '1/3/2026',
				spreadsheetUrl:
					'https://docs.google.com/spreadsheets/d/e/2PACX-1vS0DkJXxG0rA7tUzJl2-au8DRWf486KZyPbbjeaVNp4fJ1ZyO0qPWLUkHia-TWEdDRCnPFPMJjFc-1V/pubhtml'
			},
			{
				date: '5/28/2025',
				spreadsheetUrl:
					'https://docs.google.com/spreadsheets/d/e/2PACX-1vS0DkJXxG0rA7tUzJl2-au8DRWf486KZyPbbjeaVNp4fJ1ZyO0qPWLUkHia-TWEdDRCnPFPMJjFc-1V/pubhtml'
			},
			{
				date: '3/9/2025',
				spreadsheetUrl:
					'https://docs.google.com/spreadsheets/d/e/2PACX-1vS0DkJXxG0rA7tUzJl2-au8DRWf486KZyPbbjeaVNp4fJ1ZyO0qPWLUkHia-TWEdDRCnPFPMJjFc-1V/pubhtml'
			}
		],
		description: 'Week starting on January 3, 2026.',
		status: 'published',
		rooms: ALL_ROOM_IDS,
		createdAt: '2026-01-03T09:00:00Z',
		updatedAt: '2026-01-03T09:00:00Z'
	},
	{
		id: 2,
		slug: '12292025-david-starr',
		title: 'Weekly Watchlist with David Starr',
		subtitle: 'Week of December 29, 2025',
		trader: 'David Starr',
		traderImage:
			'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/David-Watchlist-Rundown.jpg',
		datePosted: 'December 29, 2025',
		weekOf: '2025-12-29',
		video: {
			src: 'https://cloud-streaming.s3.amazonaws.com/WeeklyWatchlist/WW-DS-12292025.mp4',
			poster:
				'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/David-Watchlist-Rundown.jpg',
			title: 'Weekly Watchlist with David Starr'
		},
		spreadsheet: {
			src: 'https://docs.google.com/spreadsheets/d/e/2PACX-1vSsdQCEUZLymwpLK8j35e5B6qjdRPz1k2tX8U2yL0z30EsEv06i-74m7V-cPgCyxZe528DA_3gdMUKy/pubhtml'
		},
		description: 'Week of December 29, 2025.',
		status: 'published',
		rooms: ALL_ROOM_IDS,
		createdAt: '2025-12-29T09:00:00Z',
		updatedAt: '2025-12-29T09:00:00Z'
	},
	{
		id: 3,
		slug: '12222025-tg-watkins',
		title: 'Weekly Watchlist with TG Watkins',
		subtitle: 'Week of December 22, 2025',
		trader: 'TG Watkins',
		traderImage: 'https://cdn.simplertrading.com/2025/05/07135326/SimplerCentral_TG.jpg',
		datePosted: 'December 22, 2025',
		weekOf: '2025-12-22',
		video: {
			src: 'https://cloud-streaming.s3.amazonaws.com/WeeklyWatchlist/WW-TG-12222025.mp4',
			poster:
				'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg',
			title: 'Weekly Watchlist with TG Watkins'
		},
		spreadsheet: {
			src: 'https://docs.google.com/spreadsheets/d/e/2PACX-1vSsdQCEUZLymwpLK8j35e5B6qjdRPz1k2tX8U2yL0z30EsEv06i-74m7V-cPgCyxZe528DA_3gdMUKy/pubhtml'
		},
		description: 'Week of December 22, 2025.',
		status: 'published',
		rooms: ALL_ROOM_IDS, // Available to all rooms
		createdAt: '2025-12-22T09:00:00Z',
		updatedAt: '2025-12-22T09:00:00Z'
	},
	{
		id: 2,
		slug: '12152025-allison-ostrander',
		title: 'Weekly Watchlist with Allison Ostrander',
		subtitle: 'Week of December 15, 2025',
		trader: 'Allison Ostrander',
		traderImage: 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg',
		datePosted: 'December 15, 2025',
		weekOf: '2025-12-15',
		video: {
			src: 'https://cloud-streaming.s3.amazonaws.com/WeeklyWatchlist/WW-AO-12152025.mp4',
			poster:
				'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/Allison-Watchlist-Rundown.jpg',
			title: 'Weekly Watchlist with Allison Ostrander'
		},
		spreadsheet: {
			src: 'https://docs.google.com/spreadsheets/d/e/2PACX-1vSsdQCEUZLymwpLK8j35e5B6qjdRPz1k2tX8U2yL0z30EsEv06i-74m7V-cPgCyxZe528DA_3gdMUKy/pubhtml'
		},
		description: 'Week of December 15, 2025.',
		status: 'published',
		rooms: ['day-trading-room', 'swing-trading-room', 'small-account-mentorship'], // Only live trading rooms
		createdAt: '2025-12-15T09:00:00Z',
		updatedAt: '2025-12-15T09:00:00Z'
	},
	{
		id: 3,
		slug: '12082025-taylor-horton',
		title: 'Weekly Watchlist with Taylor Horton',
		subtitle: 'Week of December 8, 2025',
		trader: 'Taylor Horton',
		traderImage: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
		datePosted: 'December 8, 2025',
		weekOf: '2025-12-08',
		video: {
			src: 'https://cloud-streaming.s3.amazonaws.com/WeeklyWatchlist/WW-TH-12082025.mp4',
			poster:
				'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/Taylor-Watchlist-Rundown.jpg',
			title: 'Weekly Watchlist with Taylor Horton'
		},
		spreadsheet: {
			src: 'https://docs.google.com/spreadsheets/d/e/2PACX-1vSsdQCEUZLymwpLK8j35e5B6qjdRPz1k2tX8U2yL0z30EsEv06i-74m7V-cPgCyxZe528DA_3gdMUKy/pubhtml'
		},
		description: 'Week of December 8, 2025.',
		status: 'published',
		rooms: ['small-account-mentorship', 'explosive-swings'], // Specific rooms only
		createdAt: '2025-12-08T09:00:00Z',
		updatedAt: '2025-12-08T09:00:00Z'
	}
];

// ═══════════════════════════════════════════════════════════════════════════
// BACKEND FETCH
// ═══════════════════════════════════════════════════════════════════════════

async function fetchFromBackend(endpoint: string, options: RequestInit = {}): Promise<any | null> {
	const BACKEND_URL = env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

	try {
		const response = await fetch(`${BACKEND_URL}${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...options.headers
			}
		});

		if (!response.ok) return null;
		return await response.json();
	} catch {
		return null;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// GET HANDLER - List all watchlist items
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ url, request }) => {
	// Parse query params
	const page = parseInt(url.searchParams.get('page') || '1');
	const perPage = parseInt(url.searchParams.get('per_page') || '20');
	const status = url.searchParams.get('status');
	const search = url.searchParams.get('search');
	const room = url.searchParams.get('room'); // Room filter

	// Try backend first
	const authHeader = request.headers.get('Authorization');
	const headers: Record<string, string> = {};
	if (authHeader) headers['Authorization'] = authHeader;

	const backendData = await fetchFromBackend(`/api/watchlist?${url.searchParams.toString()}`, {
		headers
	});

	if (backendData?.success) {
		return json(backendData);
	}

	// Fallback to mock data
	let items = [...mockWatchlistItems];

	// Filter by status
	if (status) {
		items = items.filter((item) => item.status === status);
	}

	// Filter by room - only return items that include this room
	if (room) {
		items = items.filter((item) => item.rooms.includes(room));
	}

	// Search filter
	if (search) {
		const searchLower = search.toLowerCase();
		items = items.filter(
			(item) =>
				item.title.toLowerCase().includes(searchLower) ||
				item.trader.toLowerCase().includes(searchLower) ||
				item.description.toLowerCase().includes(searchLower)
		);
	}

	// Sort by date (newest first)
	items.sort((a, b) => new Date(b.weekOf).getTime() - new Date(a.weekOf).getTime());

	// Paginate
	const total = items.length;
	const start = (page - 1) * perPage;
	const paginatedItems = items.slice(start, start + perPage);

	// Add previous/next links
	const itemsWithNav = paginatedItems.map((item) => {
		const globalIndex = items.findIndex((i) => i.id === item.id);
		return {
			...item,
			previous:
				globalIndex < items.length - 1
					? {
							slug: items[globalIndex + 1].slug,
							title: items[globalIndex + 1].title
						}
					: null,
			next:
				globalIndex > 0
					? {
							slug: items[globalIndex - 1].slug,
							title: items[globalIndex - 1].title
						}
					: null
		};
	});

	return json({
		success: true,
		data: itemsWithNav,
		pagination: {
			current_page: page,
			per_page: perPage,
			total,
			last_page: Math.ceil(total / perPage)
		},
		_mock: true
	});
};

// ═══════════════════════════════════════════════════════════════════════════
// POST HANDLER - Create new watchlist item
// ═══════════════════════════════════════════════════════════════════════════

export const POST: RequestHandler = async ({ request }) => {
	const authHeader = request.headers.get('Authorization');
	if (!authHeader) {
		error(401, 'Authentication required');
	}

	const body = await request.json();

	// Validate required fields
	if (!body.title || !body.trader || !body.weekOf) {
		error(400, 'Title, trader, and weekOf are required');
	}

	// Default to all rooms if not specified
	const rooms = body.rooms || ALL_ROOM_IDS;

	// Try backend first
	const backendData = await fetchFromBackend('/api/admin/watchlist', {
		method: 'POST',
		headers: { Authorization: authHeader },
		body: JSON.stringify({ ...body, rooms })
	});

	if (backendData?.success) {
		return json(backendData, { status: 201 });
	}

	// Mock create - generate new item
	const newId = Math.max(...mockWatchlistItems.map((i) => i.id), 0) + 1;
	const weekOfDate = new Date(body.weekOf);
	const dateStr = weekOfDate.toLocaleDateString('en-US', {
		month: 'long',
		day: 'numeric',
		year: 'numeric'
	});
	const slugDate = body.weekOf.replace(/-/g, '').substring(4) + body.weekOf.substring(0, 4);
	const traderSlug = body.trader.toLowerCase().replace(/\s+/g, '-');

	const newItem: WatchlistItem = {
		id: newId,
		slug: body.slug || `${slugDate}-${traderSlug}`,
		title: body.title,
		subtitle: `Week of ${dateStr}`,
		trader: body.trader,
		traderImage: body.traderImage,
		datePosted: dateStr,
		weekOf: body.weekOf,
		video: {
			src: body.videoSrc || '',
			poster:
				body.videoPoster ||
				'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg',
			title: body.title
		},
		spreadsheet: {
			src: body.spreadsheetSrc || ''
		},
		watchlistDates: body.watchlistDates || undefined,
		description: body.description || `Week of ${dateStr}.`,
		status: body.status || 'draft',
		rooms: rooms,
		createdAt: new Date().toISOString(),
		updatedAt: new Date().toISOString()
	};

	return json(
		{
			success: true,
			data: newItem,
			_mock: true
		},
		{ status: 201 }
	);
};

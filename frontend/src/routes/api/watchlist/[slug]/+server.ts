/**
 * Weekly Watchlist Item API Endpoint
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET, PUT, DELETE operations for individual watchlist items.
 * Supports room-specific targeting.
 *
 * @version 2.0.0 - December 2025 - Added room targeting
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { ALL_ROOM_IDS } from '$lib/config/rooms';
// R20-A: migrated off local `fetchFromBackend` helper to shared
// `$lib/server/proxy-fetch` (CLAUDE.md URL-fallback pinned once,
// `Promise<unknown>` return, narrowing primitives consolidated).
import { fetchBackend, hasSuccess } from '$lib/server/proxy-fetch';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

interface WatchlistItem {
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
	description: string;
	status: 'published' | 'draft' | 'archived';
	rooms: string[];
	previous: { slug: string; title: string } | null;
	next: { slug: string; title: string } | null;
	createdAt: string;
	updatedAt: string;
}

/** PUT body — partial update of the item; admin form sends only changed fields. */
interface WatchlistPutBody {
	title?: string;
	trader?: string;
	traderImage?: string;
	description?: string;
	status?: 'published' | 'draft' | 'archived';
	rooms?: string[];
	videoSrc?: string;
	videoPoster?: string;
	spreadsheetSrc?: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// MOCK DATA (shared reference)
// ═══════════════════════════════════════════════════════════════════════════

const mockWatchlistItems: Record<string, WatchlistItem> = {
	'12222025-tg-watkins': {
		id: 1,
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
		rooms: ALL_ROOM_IDS,
		previous: {
			slug: '12152025-allison-ostrander',
			title: 'Weekly Watchlist with Allison Ostrander'
		},
		next: null,
		createdAt: '2025-12-22T09:00:00Z',
		updatedAt: '2025-12-22T09:00:00Z'
	},
	'12152025-allison-ostrander': {
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
		rooms: ['day-trading-room', 'swing-trading-room', 'small-account-mentorship'],
		previous: {
			slug: '12082025-taylor-horton',
			title: 'Weekly Watchlist with Taylor Horton'
		},
		next: {
			slug: '12222025-tg-watkins',
			title: 'Weekly Watchlist with TG Watkins'
		},
		createdAt: '2025-12-15T09:00:00Z',
		updatedAt: '2025-12-15T09:00:00Z'
	},
	'12082025-taylor-horton': {
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
		rooms: ['small-account-mentorship', 'explosive-swings'],
		previous: null,
		next: {
			slug: '12152025-allison-ostrander',
			title: 'Weekly Watchlist with Allison Ostrander'
		},
		createdAt: '2025-12-08T09:00:00Z',
		updatedAt: '2025-12-08T09:00:00Z'
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// GET HANDLER - Get single watchlist item
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, request }) => {
	const { slug } = params;

	if (!slug) {
		error(400, 'Slug is required');
	}

	// Try backend first
	const authHeader = request.headers.get('Authorization');
	const headers: Record<string, string> = {};
	if (authHeader) headers['Authorization'] = authHeader;

	const backendData = await fetchBackend(
		`/api/watchlist/${slug}`,
		{ headers },
		'[Watchlist API]'
	);

	if (hasSuccess(backendData) && backendData.success) {
		return json(backendData);
	}

	// Fallback to mock data
	const item = mockWatchlistItems[slug];

	if (!item) {
		error(404, `Watchlist item '${slug}' not found`);
	}

	return json({
		success: true,
		data: item,
		_mock: true
	});
};

// ═══════════════════════════════════════════════════════════════════════════
// PUT HANDLER - Update watchlist item
// ═══════════════════════════════════════════════════════════════════════════

export const PUT: RequestHandler = async ({ params, request }) => {
	const { slug } = params;
	const authHeader = request.headers.get('Authorization');

	if (!authHeader) {
		error(401, 'Authentication required');
	}

	if (!slug) {
		error(400, 'Slug is required');
	}

	const body = (await request.json()) as WatchlistPutBody;

	// Try backend first
	const backendData = await fetchBackend(
		`/api/admin/watchlist/${slug}`,
		{
			method: 'PUT',
			headers: { Authorization: authHeader },
			body: JSON.stringify(body)
		},
		'[Watchlist API]'
	);

	if (hasSuccess(backendData) && backendData.success) {
		return json(backendData);
	}

	// Mock update
	const existingItem = mockWatchlistItems[slug];

	if (!existingItem) {
		error(404, `Watchlist item '${slug}' not found`);
	}

	const updatedItem: WatchlistItem = {
		...existingItem,
		title: body.title ?? existingItem.title,
		trader: body.trader ?? existingItem.trader,
		traderImage: body.traderImage ?? existingItem.traderImage,
		description: body.description ?? existingItem.description,
		status: body.status ?? existingItem.status,
		rooms: body.rooms ?? existingItem.rooms,
		video: {
			src: body.videoSrc ?? existingItem.video.src,
			poster: body.videoPoster ?? existingItem.video.poster,
			title: body.title ?? existingItem.video.title
		},
		spreadsheet: {
			src: body.spreadsheetSrc ?? existingItem.spreadsheet.src
		},
		updatedAt: new Date().toISOString()
	};

	return json({
		success: true,
		data: updatedItem,
		_mock: true
	});
};

// ═══════════════════════════════════════════════════════════════════════════
// DELETE HANDLER - Delete watchlist item
// ═══════════════════════════════════════════════════════════════════════════

export const DELETE: RequestHandler = async ({ params, request }) => {
	const { slug } = params;
	const authHeader = request.headers.get('Authorization');

	if (!authHeader) {
		error(401, 'Authentication required');
	}

	if (!slug) {
		error(400, 'Slug is required');
	}

	// Try backend first
	const backendData = await fetchBackend(
		`/api/admin/watchlist/${slug}`,
		{
			method: 'DELETE',
			headers: { Authorization: authHeader }
		},
		'[Watchlist API]'
	);

	if (hasSuccess(backendData) && backendData.success) {
		return json(backendData);
	}

	// Mock delete
	const existingItem = mockWatchlistItems[slug];

	if (!existingItem) {
		error(404, `Watchlist item '${slug}' not found`);
	}

	return json({
		success: true,
		message: `Watchlist item '${slug}' deleted`,
		_mock: true
	});
};

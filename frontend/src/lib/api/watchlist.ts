/**
 * Weekly Watchlist API Client
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Client for CRUD operations on weekly watchlist items.
 * Supports room-specific targeting for all 6 services.
 *
 * @version 2.0.0 - December 2025 - Added room targeting
 */

import { getAuthToken } from '$lib/stores/auth.svelte';
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
	watchlistDates?: WatchlistDate[];
	description: string;
	status: 'published' | 'draft' | 'archived';
	// Room targeting - which rooms/services can see this watchlist
	rooms: string[];
	previous?: { slug: string; title: string } | null;
	next?: { slug: string; title: string } | null;
	createdAt: string;
	updatedAt: string;
}

export interface WatchlistResponse {
	success: boolean;
	data: WatchlistItem[];
	pagination?: {
		current_page: number;
		per_page: number;
		total: number;
		last_page: number;
	};
	_mock?: boolean;
}

export interface SingleWatchlistResponse {
	success: boolean;
	data: WatchlistItem;
	_mock?: boolean;
}

export interface WatchlistParams {
	page?: number;
	per_page?: number;
	status?: 'published' | 'draft' | 'archived';
	search?: string;
	room?: string; // Filter by specific room
}

export interface CreateWatchlistData {
	title: string;
	trader: string;
	traderImage?: string;
	weekOf: string;
	slug?: string;
	description?: string;
	videoSrc?: string;
	videoPoster?: string;
	spreadsheetSrc?: string;
	watchlistDates?: WatchlistDate[];
	status?: 'published' | 'draft' | 'archived';
	rooms?: string[]; // Target rooms (defaults to all)
}

export interface UpdateWatchlistData {
	title?: string;
	trader?: string;
	traderImage?: string;
	description?: string;
	videoSrc?: string;
	videoPoster?: string;
	spreadsheetSrc?: string;
	status?: 'published' | 'draft' | 'archived';
	rooms?: string[];
}

// ═══════════════════════════════════════════════════════════════════════════
// API FETCH WRAPPER
// ═══════════════════════════════════════════════════════════════════════════

async function apiFetch<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
	const token = typeof window !== 'undefined' ? getAuthToken() : null;

	const headers: Record<string, string> = {
		'Content-Type': 'application/json',
		Accept: 'application/json',
		...((options.headers as Record<string, string>) || {})
	};

	if (token) {
		headers['Authorization'] = `Bearer ${token}`;
	}

	const response = await fetch(endpoint, {
		...options,
		headers
	});

	if (!response.ok) {
		const errorData = await response.json().catch(() => ({ message: 'Request failed' }));
		throw new Error(errorData.message || `API Error: ${response.status}`);
	}

	return response.json();
}

// ═══════════════════════════════════════════════════════════════════════════
// WATCHLIST API
// ═══════════════════════════════════════════════════════════════════════════

export const watchlistApi = {
	/**
	 * Get all watchlist items with optional filtering
	 */
	getAll: async (params?: WatchlistParams): Promise<WatchlistResponse> => {
		const queryParams = new URLSearchParams();

		if (params) {
			if (params.page) queryParams.set('page', params.page.toString());
			if (params.per_page) queryParams.set('per_page', params.per_page.toString());
			if (params.status) queryParams.set('status', params.status);
			if (params.search) queryParams.set('search', params.search);
			if (params.room) queryParams.set('room', params.room);
		}

		const qs = queryParams.toString();
		const url = `/api/watchlist${qs ? `?${qs}` : ''}`;

		return apiFetch<WatchlistResponse>(url);
	},

	/**
	 * Get published watchlist items only
	 */
	getPublished: async (params?: Omit<WatchlistParams, 'status'>): Promise<WatchlistResponse> => {
		return watchlistApi.getAll({ ...params, status: 'published' });
	},

	/**
	 * Get watchlist items for a specific room
	 */
	getByRoom: async (
		roomId: string,
		params?: Omit<WatchlistParams, 'room'>
	): Promise<WatchlistResponse> => {
		return watchlistApi.getAll({ ...params, room: roomId, status: 'published' });
	},

	/**
	 * Get a single watchlist item by slug
	 */
	getBySlug: async (slug: string): Promise<SingleWatchlistResponse> => {
		return apiFetch<SingleWatchlistResponse>(`/api/watchlist/${slug}`);
	},

	/**
	 * Get the latest/current watchlist item (global or for a specific room)
	 */
	getLatest: async (roomId?: string): Promise<SingleWatchlistResponse> => {
		const params: WatchlistParams = { per_page: 1, status: 'published' };
		if (roomId) params.room = roomId;

		const response = await watchlistApi.getAll(params);
		if (response.data.length === 0) {
			throw new Error('No watchlist items found');
		}
		return {
			success: true,
			data: response.data[0],
			_mock: response._mock
		};
	},

	/**
	 * Create a new watchlist item (admin only)
	 */
	create: async (data: CreateWatchlistData): Promise<SingleWatchlistResponse> => {
		// Default to all rooms if not specified
		const payload = {
			...data,
			rooms: data.rooms || ALL_ROOM_IDS
		};

		return apiFetch<SingleWatchlistResponse>('/api/watchlist', {
			method: 'POST',
			body: JSON.stringify(payload)
		});
	},

	/**
	 * Update a watchlist item (admin only)
	 */
	update: async (slug: string, data: UpdateWatchlistData): Promise<SingleWatchlistResponse> => {
		return apiFetch<SingleWatchlistResponse>(`/api/watchlist/${slug}`, {
			method: 'PUT',
			body: JSON.stringify(data)
		});
	},

	/**
	 * Delete a watchlist item (admin only)
	 */
	delete: async (slug: string): Promise<{ success: boolean; message: string }> => {
		return apiFetch(`/api/watchlist/${slug}`, {
			method: 'DELETE'
		});
	},

	/**
	 * Publish a draft watchlist item
	 */
	publish: async (slug: string): Promise<SingleWatchlistResponse> => {
		return watchlistApi.update(slug, { status: 'published' });
	},

	/**
	 * Archive a watchlist item
	 */
	archive: async (slug: string): Promise<SingleWatchlistResponse> => {
		return watchlistApi.update(slug, { status: 'archived' });
	}
};

export default watchlistApi;

/**
 * Weekly Watchlist API Client
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Client for CRUD operations on weekly watchlist items.
 *
 * @version 1.0.0 - December 2025
 */

import { getAuthToken } from '$lib/stores/auth';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

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
	description: string;
	status: 'published' | 'draft' | 'archived';
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
	status?: 'published' | 'draft' | 'archived';
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
}

// ═══════════════════════════════════════════════════════════════════════════
// API FETCH WRAPPER
// ═══════════════════════════════════════════════════════════════════════════

async function apiFetch<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
	const token = typeof window !== 'undefined' ? getAuthToken() : null;

	const headers: Record<string, string> = {
		'Content-Type': 'application/json',
		Accept: 'application/json',
		...(options.headers as Record<string, string> || {})
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
	 * Get a single watchlist item by slug
	 */
	getBySlug: async (slug: string): Promise<SingleWatchlistResponse> => {
		return apiFetch<SingleWatchlistResponse>(`/api/watchlist/${slug}`);
	},

	/**
	 * Get the latest/current watchlist item
	 */
	getLatest: async (): Promise<SingleWatchlistResponse> => {
		const response = await watchlistApi.getPublished({ per_page: 1 });
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
		return apiFetch<SingleWatchlistResponse>('/api/watchlist', {
			method: 'POST',
			body: JSON.stringify(data)
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

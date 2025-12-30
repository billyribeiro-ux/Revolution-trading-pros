/**
 * Dashboard Articles API
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * API client for fetching dashboard articles (daily videos + chatroom archives).
 * Provides a unified interface for the dashboard content feed.
 *
 * @version 1.0.0 - December 2025
 */

import { getAuthToken } from '$lib/stores/auth';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export type ArticleType = 'daily-video' | 'chatroom-archive';

export interface Article {
	id: number;
	type: ArticleType;
	label?: string;
	title: string;
	date: string;
	excerpt?: string;
	traderName?: string;
	href: string;
	image: string;
	videoUrl?: string;
	videoPlatform?: 'vimeo' | 'youtube' | 'bunny' | 'wistia' | 'direct';
	videoId?: string;
	duration?: number;
	isVideo?: boolean;
}

export interface ArticlesResponse {
	success: boolean;
	data: Article[];
	pagination?: {
		current_page: number;
		per_page: number;
		total: number;
		last_page: number;
	};
	_mock?: boolean;
}

export interface ArticleParams {
	page?: number;
	per_page?: number;
	type?: ArticleType;
	search?: string;
	trader_id?: number;
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
// ARTICLES API
// ═══════════════════════════════════════════════════════════════════════════

export const articlesApi = {
	/**
	 * Get articles for a specific trading room/membership
	 */
	getByRoom: async (slug: string, params?: ArticleParams): Promise<ArticlesResponse> => {
		let url = `/api/articles/${slug}`;

		if (params) {
			const queryParams = new URLSearchParams();
			if (params.page) queryParams.set('page', params.page.toString());
			if (params.per_page) queryParams.set('per_page', params.per_page.toString());
			if (params.type) queryParams.set('type', params.type);
			if (params.search) queryParams.set('search', params.search);
			if (params.trader_id) queryParams.set('trader_id', params.trader_id.toString());

			const qs = queryParams.toString();
			if (qs) url += `?${qs}`;
		}

		return apiFetch<ArticlesResponse>(url);
	},

	/**
	 * Get only daily videos for a room
	 */
	getDailyVideos: async (slug: string, params?: Omit<ArticleParams, 'type'>): Promise<ArticlesResponse> => {
		return articlesApi.getByRoom(slug, { ...params, type: 'daily-video' });
	},

	/**
	 * Get only chatroom archives for a room
	 */
	getChatroomArchives: async (slug: string, params?: Omit<ArticleParams, 'type'>): Promise<ArticlesResponse> => {
		return articlesApi.getByRoom(slug, { ...params, type: 'chatroom-archive' });
	},

	/**
	 * Get a single article by ID
	 */
	getById: async (id: number): Promise<{ success: boolean; data: Article }> => {
		return apiFetch(`/api/articles/item/${id}`);
	}
};

export default articlesApi;

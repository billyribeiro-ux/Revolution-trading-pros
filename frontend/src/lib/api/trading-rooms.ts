/**
 * Trading Rooms & Alert Services API
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * API client for managing trading rooms, alert services, traders, and videos.
 * Uses SvelteKit API routes with backend fallback for resilient data loading.
 *
 * @version 1.1.0 - December 2025 - Enhanced with SvelteKit fallback
 */

import { getAuthToken } from '$lib/stores/auth';

// Custom fetch wrapper that uses relative URLs for SvelteKit endpoints
async function apiFetch<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
	const token = typeof window !== 'undefined' ? getAuthToken() : null;

	const headers: Record<string, string> = {
		'Content-Type': 'application/json',
		'Accept': 'application/json',
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

// Simple API helper for relative endpoints
const api = {
	get: <T>(endpoint: string, params?: Record<string, any>): Promise<T> => {
		let url = endpoint;
		if (params) {
			const queryString = new URLSearchParams(
				Object.entries(params)
					.filter(([, v]) => v !== undefined && v !== null)
					.map(([k, v]) => [k, String(v)])
			).toString();
			if (queryString) url += `?${queryString}`;
		}
		return apiFetch<T>(url, { method: 'GET' });
	},
	post: <T>(endpoint: string, data?: any): Promise<T> =>
		apiFetch<T>(endpoint, { method: 'POST', body: data ? JSON.stringify(data) : undefined }),
	put: <T>(endpoint: string, data?: any): Promise<T> =>
		apiFetch<T>(endpoint, { method: 'PUT', body: data ? JSON.stringify(data) : undefined }),
	delete: <T>(endpoint: string): Promise<T> =>
		apiFetch<T>(endpoint, { method: 'DELETE' })
};

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface TradingRoom {
	id: number;
	name: string;
	slug: string;
	type: 'trading_room' | 'alert_service';
	description?: string;
	short_description?: string;
	icon?: string;
	color?: string;
	image_url?: string;
	logo_url?: string;
	is_active: boolean;
	is_featured: boolean;
	sort_order: number;
	features?: string[];
	schedule?: Record<string, any>;
	metadata?: Record<string, any>;
	traders?: Trader[];
	daily_videos_count?: number;
	learning_content_count?: number;
	archives_count?: number;
	created_at: string;
	updated_at: string;
}

export interface Trader {
	id: number;
	name: string;
	slug: string;
	title?: string;
	bio?: string;
	photo_url?: string;
	email?: string;
	social_links?: Record<string, string>;
	specialties?: string[];
	is_active: boolean;
	sort_order: number;
	trading_rooms?: TradingRoom[];
	daily_videos_count?: number;
	created_at: string;
	updated_at: string;
}

export interface DailyVideo {
	id: number;
	trading_room_id: number;
	trader_id?: number;
	title: string;
	description?: string;
	video_url: string;
	video_platform: 'vimeo' | 'youtube' | 'bunny' | 'wistia' | 'direct';
	video_id?: string;
	thumbnail_url?: string;
	duration?: number;
	video_date: string;
	is_featured: boolean;
	is_published: boolean;
	views_count: number;
	tags?: string[]; // Category IDs stored as tags in backend
	metadata?: Record<string, any>;
	trading_room?: TradingRoom;
	trader?: Trader;
	created_at: string;
	updated_at: string;
}

export interface LearningContent {
	id: number;
	trading_room_id: number;
	trader_id?: number;
	title: string;
	description?: string;
	content_type: string;
	content_url: string;
	thumbnail_url?: string;
	duration?: number;
	difficulty_level?: string;
	category?: string;
	sort_order: number;
	is_published: boolean;
	views_count: number;
	metadata?: Record<string, any>;
	trading_room?: TradingRoom;
	trader?: Trader;
	created_at: string;
	updated_at: string;
}

export interface RoomArchive {
	id: number;
	trading_room_id: number;
	title: string;
	description?: string;
	video_url: string;
	thumbnail_url?: string;
	duration?: number;
	recording_date: string;
	session_type?: string;
	is_published: boolean;
	views_count: number;
	timestamps?: Record<string, any>[];
	metadata?: Record<string, any>;
	trading_room?: TradingRoom;
	created_at: string;
	updated_at: string;
}

export interface ApiResponse<T> {
	success: boolean;
	data: T;
	message?: string;
}

export interface PaginatedResponse<T> {
	success: boolean;
	data: {
		data: T[];
		current_page: number;
		last_page: number;
		per_page: number;
		total: number;
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// API ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════

const ENDPOINTS = {
	// Public (member) routes
	rooms: '/api/trading-rooms',
	roomBySlug: (slug: string) => `/api/trading-rooms/${slug}`,
	roomVideos: (slug: string) => `/api/trading-rooms/${slug}/videos`,
	traders: '/api/trading-rooms/traders',

	// Admin routes
	admin: {
		rooms: '/api/admin/trading-rooms',
		roomById: (id: number) => `/api/admin/trading-rooms/${id}`,
		traders: '/api/admin/trading-rooms/traders',
		traderById: (id: number) => `/api/admin/trading-rooms/traders/${id}`,
		videos: '/api/admin/trading-rooms/videos',
		videosByRoom: (slug: string) => `/api/admin/trading-rooms/videos/${slug}`,
		videoById: (id: number) => `/api/admin/trading-rooms/videos/${id}`,
		videosBulk: '/api/admin/trading-rooms/videos/bulk'
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// TRADING ROOMS API
// ═══════════════════════════════════════════════════════════════════════════

export const tradingRoomsApi = {
	/**
	 * List all trading rooms
	 */
	list: async (params?: {
		type?: 'trading_room' | 'alert_service';
		active_only?: boolean;
		with_traders?: boolean;
		with_counts?: boolean;
	}): Promise<ApiResponse<TradingRoom[]>> => {
		return api.get(ENDPOINTS.rooms, params);
	},

	/**
	 * Get a single trading room by slug
	 */
	getBySlug: async (slug: string): Promise<ApiResponse<TradingRoom>> => {
		return api.get(ENDPOINTS.roomBySlug(slug));
	},

	/**
	 * Create a new trading room (admin)
	 */
	create: async (data: Partial<TradingRoom>): Promise<ApiResponse<TradingRoom>> => {
		return api.post(ENDPOINTS.admin.rooms, data);
	},

	/**
	 * Update a trading room (admin)
	 */
	update: async (id: number, data: Partial<TradingRoom>): Promise<ApiResponse<TradingRoom>> => {
		return api.put(ENDPOINTS.admin.roomById(id), data);
	},

	/**
	 * Delete a trading room (admin)
	 */
	delete: async (id: number): Promise<ApiResponse<void>> => {
		return api.delete(ENDPOINTS.admin.roomById(id));
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// TRADERS API
// ═══════════════════════════════════════════════════════════════════════════

export const tradersApi = {
	/**
	 * List all traders
	 */
	list: async (params?: {
		room_slug?: string;
		active_only?: boolean;
	}): Promise<ApiResponse<Trader[]>> => {
		return api.get(ENDPOINTS.admin.traders, params);
	},

	/**
	 * Create a new trader (admin)
	 */
	create: async (data: Partial<Trader> & { trading_room_ids?: number[] }): Promise<ApiResponse<Trader>> => {
		return api.post(ENDPOINTS.admin.traders, data);
	},

	/**
	 * Update a trader (admin)
	 */
	update: async (id: number, data: Partial<Trader> & { trading_room_ids?: number[] }): Promise<ApiResponse<Trader>> => {
		return api.put(ENDPOINTS.admin.traderById(id), data);
	},

	/**
	 * Delete a trader (admin)
	 */
	delete: async (id: number): Promise<ApiResponse<void>> => {
		return api.delete(ENDPOINTS.admin.traderById(id));
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// DAILY VIDEOS API
// ═══════════════════════════════════════════════════════════════════════════

export const dailyVideosApi = {
	/**
	 * List videos for a room (member access)
	 */
	listByRoom: async (
		roomSlug: string,
		params?: {
			search?: string;
			trader_id?: number;
			published_only?: boolean;
			per_page?: number;
			page?: number;
		}
	): Promise<PaginatedResponse<DailyVideo>> => {
		return api.get(ENDPOINTS.roomVideos(roomSlug), params);
	},

	/**
	 * List videos for admin (with more options)
	 */
	adminListByRoom: async (
		roomSlug: string,
		params?: {
			search?: string;
			trader_id?: number;
			published_only?: boolean;
			per_page?: number;
			page?: number;
		}
	): Promise<PaginatedResponse<DailyVideo>> => {
		return api.get(ENDPOINTS.admin.videosByRoom(roomSlug), params);
	},

	/**
	 * Create a new video (admin)
	 */
	create: async (data: {
		trading_room_id: number;
		trader_id?: number | null;
		title: string;
		description?: string;
		video_url: string;
		video_platform: string;
		video_id?: string;
		thumbnail_url?: string;
		duration?: number;
		video_date: string;
		is_featured?: boolean;
		is_published?: boolean;
		tags?: string[]; // Categories sent as tags
		metadata?: Record<string, any>;
	}): Promise<ApiResponse<DailyVideo>> => {
		return api.post(ENDPOINTS.admin.videos, data);
	},

	/**
	 * Update a video (admin)
	 */
	update: async (id: number, data: Partial<{
		trading_room_id: number;
		trader_id: number | null;
		title: string;
		description: string;
		video_url: string;
		video_platform: string;
		video_id: string;
		thumbnail_url: string;
		duration: number;
		video_date: string;
		is_featured: boolean;
		is_published: boolean;
		tags: string[];
		metadata: Record<string, any>;
	}>): Promise<ApiResponse<DailyVideo>> => {
		return api.put(ENDPOINTS.admin.videoById(id), data);
	},

	/**
	 * Delete a video (admin)
	 */
	delete: async (id: number): Promise<ApiResponse<void>> => {
		return api.delete(ENDPOINTS.admin.videoById(id));
	},

	/**
	 * Bulk create videos (admin)
	 */
	bulkCreate: async (videos: Array<{
		trading_room_id: number;
		trader_id?: number | null;
		title: string;
		description?: string;
		video_url: string;
		video_platform: string;
		video_date: string;
		is_published?: boolean;
		tags?: string[];
	}>): Promise<ApiResponse<DailyVideo[]>> => {
		return api.post(ENDPOINTS.admin.videosBulk, { videos });
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// COMBINED EXPORT
// ═══════════════════════════════════════════════════════════════════════════

export const tradingRoomApi = {
	rooms: tradingRoomsApi,
	traders: tradersApi,
	videos: dailyVideosApi
};

export default tradingRoomApi;

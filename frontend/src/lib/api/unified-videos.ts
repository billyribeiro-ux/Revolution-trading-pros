/**
 * Unified Video API Client - Revolution Trading Pros
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 * 
 * Complete API client for unified video management with Bunny.net integration.
 * Supports: Daily Videos, Weekly Watchlist, Learning Center, Room Archives
 */

import { apiClient } from './client';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface TagDetail {
	slug: string;
	name: string;
	color: string;
}

export interface TraderInfo {
	id: number;
	name: string;
	slug: string;
}

export interface RoomInfo {
	id: number;
	name: string;
	slug: string;
}

export interface UnifiedVideo {
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
	content_type: 'daily_video' | 'weekly_watchlist' | 'learning_center' | 'room_archive';
	video_date: string;
	formatted_date: string;
	is_published: boolean;
	is_featured: boolean;
	tags: string[];
	tag_details: TagDetail[];
	views_count: number;
	trader: TraderInfo | null;
	rooms: RoomInfo[];
	created_at: string;
}

export interface VideoListResponse {
	success: boolean;
	data: UnifiedVideo[];
	meta: {
		current_page: number;
		per_page: number;
		total: number;
		last_page: number;
	};
}

export interface VideoStatsResponse {
	success: boolean;
	data: {
		total: number;
		published: number;
		by_type: {
			daily_video: number;
			weekly_watchlist: number;
			learning_center: number;
			room_archive: number;
		};
		total_views: number;
	};
}

export interface VideoOptionsResponse {
	success: boolean;
	data: {
		content_types: { value: string; label: string }[];
		platforms: { value: string; label: string }[];
		difficulty_levels: { value: string; label: string }[];
		tags: TagDetail[];
		trading_rooms: RoomInfo[];
		traders: TraderInfo[];
	};
}

export interface CreateVideoRequest {
	title: string;
	description?: string;
	video_url: string;
	video_platform?: string;
	content_type: string;
	video_date: string;
	trader_id?: number | null;
	is_published?: boolean;
	is_featured?: boolean;
	tags?: string[];
	room_ids?: number[];
	upload_to_all?: boolean;
	thumbnail_url?: string;
	difficulty_level?: string;
	category?: string;
	session_type?: string;
	duration?: number;
}

export interface UpdateVideoRequest extends Partial<CreateVideoRequest> {}

export interface VideoListParams {
	page?: number;
	per_page?: number;
	content_type?: string;
	room_id?: number;
	room_slug?: string;
	trader_id?: number;
	tags?: string;
	is_published?: boolean;
	is_featured?: boolean;
	search?: string;
	sort_by?: string;
	sort_dir?: 'asc' | 'desc';
}

export interface BulkAssignRequest {
	video_ids: number[];
	room_ids: number[];
	clear_existing?: boolean;
}

export interface BulkPublishRequest {
	video_ids: number[];
	publish: boolean;
}

export interface BulkDeleteRequest {
	video_ids: number[];
	force?: boolean;
}

export interface UploadUrlResponse {
	success: boolean;
	data: {
		video_guid: string;
		library_id: number;
		upload_url: string;
		embed_url: string;
		play_url: string;
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// API CLIENT
// ═══════════════════════════════════════════════════════════════════════════

const BASE_URL = '/admin/unified-videos';

export const unifiedVideoApi = {
	/**
	 * List videos with filters and pagination
	 */
	async list(params: VideoListParams = {}): Promise<VideoListResponse> {
		const searchParams = new URLSearchParams();
		
		if (params.page) searchParams.set('page', params.page.toString());
		if (params.per_page) searchParams.set('per_page', params.per_page.toString());
		if (params.content_type) searchParams.set('content_type', params.content_type);
		if (params.room_id) searchParams.set('room_id', params.room_id.toString());
		if (params.room_slug) searchParams.set('room_slug', params.room_slug);
		if (params.trader_id) searchParams.set('trader_id', params.trader_id.toString());
		if (params.tags) searchParams.set('tags', params.tags);
		if (params.is_published !== undefined) searchParams.set('is_published', params.is_published.toString());
		if (params.is_featured !== undefined) searchParams.set('is_featured', params.is_featured.toString());
		if (params.search) searchParams.set('search', params.search);
		if (params.sort_by) searchParams.set('sort_by', params.sort_by);
		if (params.sort_dir) searchParams.set('sort_dir', params.sort_dir);

		const query = searchParams.toString();
		return apiClient.get(`${BASE_URL}${query ? `?${query}` : ''}`);
	},

	/**
	 * Get a single video by ID
	 */
	async get(id: number): Promise<{ success: boolean; data: UnifiedVideo }> {
		return apiClient.get(`${BASE_URL}/${id}`);
	},

	/**
	 * Create a new video
	 */
	async create(data: CreateVideoRequest): Promise<{ success: boolean; message: string; data: { id: number; slug: string } }> {
		return apiClient.post(BASE_URL, data);
	},

	/**
	 * Update an existing video
	 */
	async update(id: number, data: UpdateVideoRequest): Promise<{ success: boolean; message: string }> {
		return apiClient.put(`${BASE_URL}/${id}`, data);
	},

	/**
	 * Delete a video
	 */
	async delete(id: number): Promise<{ success: boolean; message: string }> {
		return apiClient.delete(`${BASE_URL}/${id}`);
	},

	/**
	 * Get video statistics
	 */
	async stats(): Promise<VideoStatsResponse> {
		return apiClient.get(`${BASE_URL}/stats`);
	},

	/**
	 * Get form options (content types, platforms, tags, rooms, traders)
	 */
	async options(): Promise<VideoOptionsResponse> {
		return apiClient.get(`${BASE_URL}/options`);
	},

	/**
	 * Get Bunny.net direct upload URL
	 */
	async getUploadUrl(title: string): Promise<UploadUrlResponse> {
		return apiClient.post(`${BASE_URL}/upload-url`, { title });
	},

	/**
	 * Bulk assign videos to rooms
	 */
	async bulkAssign(data: BulkAssignRequest): Promise<{ success: boolean; message: string; count: number }> {
		return apiClient.post(`${BASE_URL}/bulk-assign`, data);
	},

	/**
	 * Bulk publish/unpublish videos
	 */
	async bulkPublish(data: BulkPublishRequest): Promise<{ success: boolean; message: string; count: number }> {
		return apiClient.post(`${BASE_URL}/bulk-publish`, data);
	},

	/**
	 * Bulk delete videos
	 */
	async bulkDelete(data: BulkDeleteRequest): Promise<{ success: boolean; message: string; count: number }> {
		return apiClient.post(`${BASE_URL}/bulk-delete`, data);
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════

export const CONTENT_TYPES = [
	{ value: 'daily_video', label: 'Daily Video', icon: 'video', color: '#6366f1' },
	{ value: 'weekly_watchlist', label: 'Weekly Watchlist', icon: 'list-check', color: '#10b981' },
	{ value: 'learning_center', label: 'Learning Center', icon: 'school', color: '#f59e0b' },
	{ value: 'room_archive', label: 'Room Archive', icon: 'archive', color: '#8b5cf6' }
] as const;

export const AVAILABLE_TAGS: TagDetail[] = [
	{ slug: 'risk-management', name: 'Risk Management', color: '#ef4444' },
	{ slug: 'options-strategies', name: 'Options Strategies', color: '#f59e0b' },
	{ slug: 'macro-structure', name: 'Macro Structure', color: '#10b981' },
	{ slug: 'micro-structure', name: 'Micro Structure', color: '#06b6d4' },
	{ slug: 'psychology', name: 'Psychology', color: '#8b5cf6' },
	{ slug: 'technical-analysis', name: 'Technical Analysis', color: '#3b82f6' },
	{ slug: 'fundamentals', name: 'Fundamentals', color: '#ec4899' },
	{ slug: 'trade-setups', name: 'Trade Setups', color: '#14b8a6' },
	{ slug: 'market-review', name: 'Market Review', color: '#6366f1' },
	{ slug: 'earnings', name: 'Earnings', color: '#f97316' },
	{ slug: 'futures', name: 'Futures', color: '#84cc16' },
	{ slug: 'forex', name: 'Forex', color: '#22c55e' },
	{ slug: 'crypto', name: 'Crypto', color: '#a855f7' },
	{ slug: 'small-accounts', name: 'Small Accounts', color: '#eab308' },
	{ slug: 'position-sizing', name: 'Position Sizing', color: '#0ea5e9' },
	{ slug: 'entry-exit', name: 'Entry & Exit', color: '#d946ef' },
	{ slug: 'scanner-setups', name: 'Scanner Setups', color: '#64748b' },
	{ slug: 'indicators', name: 'Indicators', color: '#fb7185' }
];

export function getTagBySlug(slug: string): TagDetail | undefined {
	return AVAILABLE_TAGS.find(t => t.slug === slug);
}

export function getContentTypeLabel(value: string): string {
	return CONTENT_TYPES.find(ct => ct.value === value)?.label || value;
}

export function getContentTypeColor(value: string): string {
	return CONTENT_TYPES.find(ct => ct.value === value)?.color || '#6366f1';
}

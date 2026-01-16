/**
 * Room Content API - Trade Plans, Alerts, Weekly Videos
 * ═══════════════════════════════════════════════════════════════════════════════════
 * Apple Principal Engineer ICT 7+ Grade - January 13, 2026
 *
 * TypeScript API client for managing trading room content:
 * - Trade Plan entries (ticker, bias, entry, targets, stop, options, notes)
 * - Alerts (Entry/Exit/Update with expandable notes)
 * - Weekly Videos (featured video per room with auto-archive)
 * - Room Stats (auto-calculated performance metrics)
 *
 * @version 1.0.0
 */

import { apiClient } from './client.svelte';

// ═══════════════════════════════════════════════════════════════════════════════════
// TYPE DEFINITIONS
// ═══════════════════════════════════════════════════════════════════════════════════

export type Bias = 'BULLISH' | 'BEARISH' | 'NEUTRAL';
export type AlertType = 'ENTRY' | 'EXIT' | 'UPDATE';

export interface TradePlanEntry {
	id: number;
	room_id: number;
	room_slug: string;
	week_of: string;
	ticker: string;
	bias: Bias;
	entry: string | null;
	target1: string | null;
	target2: string | null;
	target3: string | null;
	runner: string | null;
	stop: string | null;
	options_strike: string | null;
	options_exp: string | null;
	notes: string | null;
	sort_order: number;
	is_active: boolean;
	created_at: string;
	updated_at: string;
}

export interface RoomAlert {
	id: number;
	room_id: number;
	room_slug: string;
	alert_type: AlertType;
	ticker: string;
	title: string;
	message: string;
	notes: string | null;
	is_new: boolean;
	is_published: boolean;
	is_pinned: boolean;
	published_at: string;
	created_at: string;
	updated_at: string;
}

export interface WeeklyVideo {
	id: number;
	room_id: number;
	room_slug: string;
	week_of: string;
	week_title: string;
	video_title: string;
	video_url: string;
	video_platform: string | null;
	thumbnail_url: string | null;
	duration: string | null;
	description: string | null;
	is_current: boolean;
	is_published: boolean;
	published_at: string;
	created_at: string;
	updated_at: string;
}

export interface RoomStats {
	id: number;
	room_id: number;
	room_slug: string;
	win_rate: number | null;
	weekly_profit: string | null;
	monthly_profit: string | null;
	active_trades: number | null;
	closed_this_week: number | null;
	total_trades: number | null;
	wins: number | null;
	losses: number | null;
	calculated_at: string;
}

export interface PaginatedResponse<T> {
	data: T[];
	meta: {
		current_page: number;
		per_page: number;
		total: number;
		total_pages?: number;
	};
}

// ═══════════════════════════════════════════════════════════════════════════════════
// REQUEST TYPES
// ═══════════════════════════════════════════════════════════════════════════════════

export interface CreateTradePlanRequest {
	room_slug: string;
	week_of?: string;
	ticker: string;
	bias: Bias;
	entry?: string;
	target1?: string;
	target2?: string;
	target3?: string;
	runner?: string;
	stop?: string;
	options_strike?: string;
	options_exp?: string;
	notes?: string;
	sort_order?: number;
}

export interface UpdateTradePlanRequest {
	ticker?: string;
	bias?: Bias;
	entry?: string;
	target1?: string;
	target2?: string;
	target3?: string;
	runner?: string;
	stop?: string;
	options_strike?: string;
	options_exp?: string;
	notes?: string;
	sort_order?: number;
	is_active?: boolean;
}

export interface CreateAlertRequest {
	room_slug: string;
	alert_type: AlertType;
	ticker: string;
	title: string;
	message: string;
	notes?: string;
	is_new?: boolean;
	is_published?: boolean;
}

export interface UpdateAlertRequest {
	alert_type?: AlertType;
	ticker?: string;
	title?: string;
	message?: string;
	notes?: string;
	is_new?: boolean;
	is_published?: boolean;
	is_pinned?: boolean;
}

export interface CreateWeeklyVideoRequest {
	room_slug: string;
	week_of: string;
	week_title: string;
	video_title: string;
	video_url: string;
	video_platform?: string;
	thumbnail_url?: string;
	duration?: string;
	description?: string;
}

export interface ReorderRequest {
	items: Array<{ id: number; sort_order: number }>;
}

export interface ListParams {
	page?: number;
	per_page?: number;
	week_of?: string;
}

// ═══════════════════════════════════════════════════════════════════════════════════
// TRADE PLAN API
// ═══════════════════════════════════════════════════════════════════════════════════

export const tradePlanApi = {
	/**
	 * List trade plan entries for a room
	 */
	list: (roomSlug: string, params?: ListParams): Promise<PaginatedResponse<TradePlanEntry>> =>
		apiClient.get(`/room-content/rooms/${roomSlug}/trade-plan`, { params }),

	/**
	 * Create a new trade plan entry
	 */
	create: (data: CreateTradePlanRequest): Promise<TradePlanEntry> =>
		apiClient.post('/admin/room-content/trade-plan', data),

	/**
	 * Update a trade plan entry
	 */
	update: (id: number, data: UpdateTradePlanRequest): Promise<TradePlanEntry> =>
		apiClient.put(`/admin/room-content/trade-plan/${id}`, data),

	/**
	 * Delete a trade plan entry
	 */
	delete: (id: number): Promise<{ success: boolean; message: string }> =>
		apiClient.delete(`/admin/room-content/trade-plan/${id}`),

	/**
	 * Reorder trade plan entries
	 */
	reorder: (roomSlug: string, items: ReorderRequest['items']): Promise<{ success: boolean }> =>
		apiClient.put(`/admin/room-content/rooms/${roomSlug}/trade-plan/reorder`, { items })
};

// ═══════════════════════════════════════════════════════════════════════════════════
// ALERTS API
// ═══════════════════════════════════════════════════════════════════════════════════

export const alertsApi = {
	/**
	 * List alerts for a room
	 */
	list: (roomSlug: string, params?: ListParams): Promise<PaginatedResponse<RoomAlert>> =>
		apiClient.get(`/room-content/rooms/${roomSlug}/alerts`, { params }),

	/**
	 * Create a new alert
	 */
	create: (data: CreateAlertRequest): Promise<RoomAlert> =>
		apiClient.post('/admin/room-content/alerts', data),

	/**
	 * Update an alert
	 */
	update: (id: number, data: UpdateAlertRequest): Promise<RoomAlert> =>
		apiClient.put(`/admin/room-content/alerts/${id}`, data),

	/**
	 * Delete an alert
	 */
	delete: (id: number): Promise<{ success: boolean; message: string }> =>
		apiClient.delete(`/admin/room-content/alerts/${id}`),

	/**
	 * Mark alert as read (not new)
	 */
	markRead: (id: number): Promise<{ success: boolean }> =>
		apiClient.post(`/room-content/rooms/any/alerts/${id}/read`)
};

// ═══════════════════════════════════════════════════════════════════════════════════
// WEEKLY VIDEO API
// ═══════════════════════════════════════════════════════════════════════════════════

export const weeklyVideoApi = {
	/**
	 * Get current weekly video for a room
	 */
	getCurrent: (roomSlug: string): Promise<{ data: WeeklyVideo | null }> =>
		apiClient.get(`/room-content/rooms/${roomSlug}/weekly-video`),

	/**
	 * List all weekly videos (including archived)
	 */
	list: (roomSlug: string, params?: ListParams): Promise<PaginatedResponse<WeeklyVideo>> =>
		apiClient.get(`/room-content/rooms/${roomSlug}/weekly-videos`, { params }),

	/**
	 * Create/publish a new weekly video (archives previous)
	 */
	create: (data: CreateWeeklyVideoRequest): Promise<WeeklyVideo> =>
		apiClient.post('/admin/room-content/weekly-video', data)
};

// ═══════════════════════════════════════════════════════════════════════════════════
// ROOM STATS API
// ═══════════════════════════════════════════════════════════════════════════════════

export const roomStatsApi = {
	/**
	 * Get room stats
	 */
	get: (roomSlug: string): Promise<{ data: RoomStats | null }> =>
		apiClient.get(`/room-content/rooms/${roomSlug}/stats`)
};

// ═══════════════════════════════════════════════════════════════════════════════════
// COMBINED EXPORT
// ═══════════════════════════════════════════════════════════════════════════════════

export const roomContentApi = {
	tradePlan: tradePlanApi,
	alerts: alertsApi,
	weeklyVideo: weeklyVideoApi,
	stats: roomStatsApi
};

export default roomContentApi;

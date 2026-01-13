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

import { getAuthToken } from '$lib/stores/auth';

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
// API CLIENT
// ═══════════════════════════════════════════════════════════════════════════════════

const API_BASE = '/api';

async function apiFetch<T>(
	endpoint: string,
	options: RequestInit = {}
): Promise<T> {
	const token = typeof window !== 'undefined' ? getAuthToken() : null;

	const headers: Record<string, string> = {
		'Content-Type': 'application/json',
		Accept: 'application/json',
		...((options.headers as Record<string, string>) || {})
	};

	if (token) {
		headers['Authorization'] = `Bearer ${token}`;
	}

	const response = await fetch(`${API_BASE}${endpoint}`, {
		...options,
		headers,
		credentials: 'include'
	});

	if (!response.ok) {
		const errorData = await response.json().catch(() => ({ message: 'Request failed' }));
		throw new Error(errorData.message || errorData.error || `API Error: ${response.status}`);
	}

	return response.json();
}

// ═══════════════════════════════════════════════════════════════════════════════════
// TRADE PLAN API
// ═══════════════════════════════════════════════════════════════════════════════════

export const tradePlanApi = {
	/**
	 * List trade plan entries for a room
	 */
	list: (roomSlug: string, params?: ListParams): Promise<PaginatedResponse<TradePlanEntry>> => {
		const queryParams = new URLSearchParams();
		if (params?.page) queryParams.set('page', params.page.toString());
		if (params?.per_page) queryParams.set('per_page', params.per_page.toString());
		if (params?.week_of) queryParams.set('week_of', params.week_of);
		const query = queryParams.toString();
		return apiFetch(`/room-content/rooms/${roomSlug}/trade-plan${query ? `?${query}` : ''}`);
	},

	/**
	 * Create a new trade plan entry
	 */
	create: (data: CreateTradePlanRequest): Promise<TradePlanEntry> =>
		apiFetch('/admin/room-content/trade-plan', {
			method: 'POST',
			body: JSON.stringify(data)
		}),

	/**
	 * Update a trade plan entry
	 */
	update: (id: number, data: UpdateTradePlanRequest): Promise<TradePlanEntry> =>
		apiFetch(`/admin/room-content/trade-plan/${id}`, {
			method: 'PUT',
			body: JSON.stringify(data)
		}),

	/**
	 * Delete a trade plan entry
	 */
	delete: (id: number): Promise<{ success: boolean; message: string }> =>
		apiFetch(`/admin/room-content/trade-plan/${id}`, {
			method: 'DELETE'
		}),

	/**
	 * Reorder trade plan entries
	 */
	reorder: (roomSlug: string, items: ReorderRequest['items']): Promise<{ success: boolean }> =>
		apiFetch(`/admin/room-content/rooms/${roomSlug}/trade-plan/reorder`, {
			method: 'PUT',
			body: JSON.stringify({ items })
		})
};

// ═══════════════════════════════════════════════════════════════════════════════════
// ALERTS API
// ═══════════════════════════════════════════════════════════════════════════════════

export const alertsApi = {
	/**
	 * List alerts for a room
	 */
	list: (roomSlug: string, params?: ListParams): Promise<PaginatedResponse<RoomAlert>> => {
		const queryParams = new URLSearchParams();
		if (params?.page) queryParams.set('page', params.page.toString());
		if (params?.per_page) queryParams.set('per_page', params.per_page.toString());
		const query = queryParams.toString();
		return apiFetch(`/room-content/rooms/${roomSlug}/alerts${query ? `?${query}` : ''}`);
	},

	/**
	 * Create a new alert
	 */
	create: (data: CreateAlertRequest): Promise<RoomAlert> =>
		apiFetch('/admin/room-content/alerts', {
			method: 'POST',
			body: JSON.stringify(data)
		}),

	/**
	 * Update an alert
	 */
	update: (id: number, data: UpdateAlertRequest): Promise<RoomAlert> =>
		apiFetch(`/admin/room-content/alerts/${id}`, {
			method: 'PUT',
			body: JSON.stringify(data)
		}),

	/**
	 * Delete an alert
	 */
	delete: (id: number): Promise<{ success: boolean; message: string }> =>
		apiFetch(`/admin/room-content/alerts/${id}`, {
			method: 'DELETE'
		}),

	/**
	 * Mark alert as read (not new)
	 */
	markRead: (id: number): Promise<{ success: boolean }> =>
		apiFetch(`/room-content/rooms/any/alerts/${id}/read`, {
			method: 'POST'
		})
};

// ═══════════════════════════════════════════════════════════════════════════════════
// WEEKLY VIDEO API
// ═══════════════════════════════════════════════════════════════════════════════════

export const weeklyVideoApi = {
	/**
	 * Get current weekly video for a room
	 */
	getCurrent: (roomSlug: string): Promise<{ data: WeeklyVideo | null }> =>
		apiFetch(`/room-content/rooms/${roomSlug}/weekly-video`),

	/**
	 * List all weekly videos (including archived)
	 */
	list: (roomSlug: string, params?: ListParams): Promise<PaginatedResponse<WeeklyVideo>> => {
		const queryParams = new URLSearchParams();
		if (params?.page) queryParams.set('page', params.page.toString());
		if (params?.per_page) queryParams.set('per_page', params.per_page.toString());
		const query = queryParams.toString();
		return apiFetch(`/room-content/rooms/${roomSlug}/weekly-videos${query ? `?${query}` : ''}`);
	},

	/**
	 * Create/publish a new weekly video (archives previous)
	 */
	create: (data: CreateWeeklyVideoRequest): Promise<WeeklyVideo> =>
		apiFetch('/admin/room-content/weekly-video', {
			method: 'POST',
			body: JSON.stringify(data)
		})
};

// ═══════════════════════════════════════════════════════════════════════════════════
// ROOM STATS API
// ═══════════════════════════════════════════════════════════════════════════════════

export const roomStatsApi = {
	/**
	 * Get room stats
	 */
	get: (roomSlug: string): Promise<{ data: RoomStats | null }> =>
		apiFetch(`/room-content/rooms/${roomSlug}/stats`)
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

/**
 * Advanced Video API - Revolution Trading Pros
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple Principal Engineer ICT 7 Grade - January 2026
 *
 * API client for:
 * - Video Analytics
 * - Video Series/Playlists
 * - Video Chapters
 * - Scheduled Publishing
 * - Bulk Upload Queue
 * - Video Cloning
 * - Export to CSV
 * - Bulk Edit
 * - Drag & Drop Reorder
 */

const API_BASE = 'https://revolution-trading-pros-api.fly.dev';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface AnalyticsEvent {
	video_id: number;
	session_id?: string;
	event_type:
		| 'view'
		| 'play'
		| 'pause'
		| 'complete'
		| 'progress'
		| 'seek'
		| 'quality_change'
		| 'speed_change'
		| 'buffer';
	event_data?: Record<string, any>;
	watch_time_seconds?: number;
	progress_percent?: number;
	buffer_count?: number;
	quality_level?: string;
	playback_speed?: number;
	device_type?: 'desktop' | 'mobile' | 'tablet' | 'tv';
	browser?: string;
	os?: string;
	screen_width?: number;
	screen_height?: number;
	connection_type?: string;
}

export interface AnalyticsDashboard {
	period: string;
	total_videos: number;
	total_views: number;
	total_watch_time_hours: number;
	avg_completion_rate: number;
	unique_viewers: number;
	top_videos: TopVideo[];
	views_by_day: DailyView[];
	device_breakdown: Record<string, number>;
}

export interface TopVideo {
	video_id: number;
	title: string;
	thumbnail_url?: string;
	views: number;
}

export interface DailyView {
	date: string;
	views: number;
	unique_viewers?: number;
}

export interface VideoAnalytics {
	video_id: number;
	title: string;
	thumbnail_url?: string;
	period: string;
	total_views: number;
	unique_viewers: number;
	avg_completion_percent: number;
	total_watch_time_hours: number;
	daily_views: DailyView[];
}

export interface VideoSeries {
	id: number;
	title: string;
	slug: string;
	description?: string;
	thumbnail_url?: string;
	content_type: string;
	difficulty_level?: string;
	is_published: boolean;
	is_premium: boolean;
	video_count: number;
	estimated_duration_minutes?: number;
	tags: string[];
	videos?: SeriesVideo[];
	created_at: string;
}

export interface SeriesVideo {
	video_id: number;
	title: string;
	thumbnail_url?: string;
	duration?: number;
	sort_order: number;
	section_title?: string;
	is_preview: boolean;
}

export interface CreateSeriesRequest {
	title: string;
	description?: string;
	thumbnail_url?: string;
	content_type: string;
	difficulty_level?: string;
	category?: string;
	is_published?: boolean;
	is_premium?: boolean;
	required_plan_id?: number;
	tags?: string[];
	video_ids?: number[];
}

export interface VideoChapter {
	id: number;
	title: string;
	description?: string;
	start_time_seconds: number;
	end_time_seconds?: number;
	formatted_start_time: string;
	formatted_end_time?: string;
	thumbnail_url?: string;
	chapter_number: number;
}

export interface CreateChapterRequest {
	title: string;
	description?: string;
	start_time_seconds: number;
	end_time_seconds?: number;
	thumbnail_url?: string;
}

export interface ScheduledJob {
	id: number;
	resource_type: string;
	resource_id: number;
	resource_title?: string;
	scheduled_at: string;
	timezone: string;
	action: string;
	status: string;
	notify_on_publish: boolean;
	created_at: string;
}

export interface CreateScheduledJobRequest {
	resource_type: 'video' | 'series';
	resource_id: number;
	scheduled_at: string;
	timezone?: string;
	action: 'publish' | 'unpublish' | 'feature' | 'unfeature';
	notify_on_publish?: boolean;
	notification_recipients?: string[];
}

export interface BulkUploadFile {
	filename: string;
	file_size_bytes?: number;
	content_type?: string;
	title?: string;
	description?: string;
}

export interface BulkUploadRequest {
	files: BulkUploadFile[];
	default_metadata: {
		content_type: string;
		video_date?: string;
		trader_id?: number;
		room_ids?: number[];
		upload_to_all?: boolean;
		is_published?: boolean;
		tags?: string[];
	};
}

export interface BulkUploadResponse {
	batch_id: string;
	uploads: UploadQueueItem[];
}

export interface UploadQueueItem {
	id: number;
	filename: string;
	upload_url: string;
	video_guid: string;
	status: string;
}

export interface BatchStatus {
	batch_id: string;
	total_files: number;
	completed: number;
	failed: number;
	in_progress: number;
	pending: number;
	uploads: UploadStatusItem[];
}

export interface UploadStatusItem {
	id: number;
	filename: string;
	status: string;
	progress_percent: number;
	error_message?: string;
	created_video_id?: number;
}

export interface CloneVideoRequest {
	new_title?: string;
	new_video_date?: string;
	content_type?: string;
	room_ids?: number[];
	upload_to_all?: boolean;
	include_chapters?: boolean;
	is_published?: boolean;
}

export interface BulkEditRequest {
	video_ids: number[];
	updates: {
		content_type?: string;
		trader_id?: number;
		is_published?: boolean;
		is_featured?: boolean;
		difficulty_level?: string;
		category?: string;
		add_tags?: string[];
		remove_tags?: string[];
		add_room_ids?: number[];
		remove_room_ids?: number[];
	};
}

export interface VideoOrder {
	video_id: number;
	sort_order: number;
	is_pinned?: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════
// API CLIENT
// ═══════════════════════════════════════════════════════════════════════════

async function apiRequest<T>(
	endpoint: string,
	options: RequestInit = {}
): Promise<{ success: boolean; data?: T; error?: string }> {
	try {
		const response = await fetch(`${API_BASE}/api/video-advanced${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				...options.headers
			},
			credentials: 'include'
		});

		const data = await response.json();

		if (!response.ok) {
			return { success: false, error: data.error || 'Request failed' };
		}

		return { success: true, data: data.data || data };
	} catch (error) {
		console.error('API Error:', error);
		return { success: false, error: error instanceof Error ? error.message : 'Network error' };
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// ANALYTICS API
// ═══════════════════════════════════════════════════════════════════════════

export const analyticsApi = {
	/**
	 * Track a single analytics event
	 */
	async trackEvent(event: AnalyticsEvent): Promise<{ success: boolean }> {
		return apiRequest('/analytics/track', {
			method: 'POST',
			body: JSON.stringify(event)
		});
	},

	/**
	 * Track multiple events (batch)
	 */
	async trackEventsBatch(
		events: AnalyticsEvent[]
	): Promise<{ success: boolean; tracked?: number }> {
		return apiRequest('/analytics/track-batch', {
			method: 'POST',
			body: JSON.stringify({ events })
		});
	},

	/**
	 * Get analytics dashboard
	 */
	async getDashboard(params?: {
		period?: '7d' | '30d' | '90d';
		content_type?: string;
		room_id?: number;
	}): Promise<{ success: boolean; data?: AnalyticsDashboard; error?: string }> {
		const searchParams = new URLSearchParams();
		if (params?.period) searchParams.set('period', params.period);
		if (params?.content_type) searchParams.set('content_type', params.content_type);
		if (params?.room_id) searchParams.set('room_id', params.room_id.toString());

		const query = searchParams.toString();
		return apiRequest(`/analytics/dashboard${query ? `?${query}` : ''}`);
	},

	/**
	 * Get analytics for a specific video
	 */
	async getVideoAnalytics(
		videoId: number,
		period?: '7d' | '30d' | '90d'
	): Promise<{ success: boolean; data?: VideoAnalytics; error?: string }> {
		const query = period ? `?period=${period}` : '';
		return apiRequest(`/analytics/video/${videoId}${query}`);
	},

	/**
	 * Update user watch progress
	 */
	async updateWatchProgress(
		videoId: number,
		data: {
			user_id?: number;
			position_seconds: number;
			watch_time_seconds: number;
			completion_percent: number;
		}
	): Promise<{ success: boolean }> {
		return apiRequest(`/analytics/progress/${videoId}`, {
			method: 'POST',
			body: JSON.stringify(data)
		});
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// SERIES API
// ═══════════════════════════════════════════════════════════════════════════

export const seriesApi = {
	/**
	 * List all series
	 */
	async list(params?: {
		page?: number;
		per_page?: number;
		content_type?: string;
		difficulty_level?: string;
		is_published?: boolean;
		search?: string;
	}): Promise<{ success: boolean; data?: VideoSeries[]; meta?: any }> {
		const searchParams = new URLSearchParams();
		if (params?.page) searchParams.set('page', params.page.toString());
		if (params?.per_page) searchParams.set('per_page', params.per_page.toString());
		if (params?.content_type) searchParams.set('content_type', params.content_type);
		if (params?.difficulty_level) searchParams.set('difficulty_level', params.difficulty_level);
		if (params?.is_published !== undefined)
			searchParams.set('is_published', params.is_published.toString());
		if (params?.search) searchParams.set('search', params.search);

		const query = searchParams.toString();
		return apiRequest(`/series${query ? `?${query}` : ''}`);
	},

	/**
	 * Get a single series with videos
	 */
	async get(id: number): Promise<{ success: boolean; data?: VideoSeries }> {
		return apiRequest(`/series/${id}`);
	},

	/**
	 * Create a new series
	 */
	async create(
		data: CreateSeriesRequest
	): Promise<{ success: boolean; data?: { id: number; slug: string } }> {
		return apiRequest('/series', {
			method: 'POST',
			body: JSON.stringify(data)
		});
	},

	/**
	 * Update a series
	 */
	async update(id: number, data: Partial<CreateSeriesRequest>): Promise<{ success: boolean }> {
		return apiRequest(`/series/${id}`, {
			method: 'PUT',
			body: JSON.stringify(data)
		});
	},

	/**
	 * Delete a series
	 */
	async delete(id: number): Promise<{ success: boolean }> {
		return apiRequest(`/series/${id}`, { method: 'DELETE' });
	},

	/**
	 * Add videos to series
	 */
	async addVideos(
		seriesId: number,
		videoIds: number[],
		sectionTitle?: string
	): Promise<{ success: boolean }> {
		return apiRequest(`/series/${seriesId}/videos`, {
			method: 'POST',
			body: JSON.stringify({ video_ids: videoIds, section_title: sectionTitle })
		});
	},

	/**
	 * Remove video from series
	 */
	async removeVideo(seriesId: number, videoId: number): Promise<{ success: boolean }> {
		return apiRequest(`/series/${seriesId}/videos/${videoId}`, { method: 'DELETE' });
	},

	/**
	 * Reorder videos in series
	 */
	async reorderVideos(
		seriesId: number,
		videoOrders: {
			video_id: number;
			sort_order: number;
			section_title?: string;
			section_order?: number;
		}[]
	): Promise<{ success: boolean }> {
		return apiRequest(`/series/${seriesId}/reorder`, {
			method: 'POST',
			body: JSON.stringify({ video_orders: videoOrders })
		});
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// CHAPTERS API
// ═══════════════════════════════════════════════════════════════════════════

export const chaptersApi = {
	/**
	 * Get chapters for a video
	 */
	async list(
		videoId: number
	): Promise<{ success: boolean; data?: VideoChapter[]; error?: string }> {
		return apiRequest(`/videos/${videoId}/chapters`);
	},

	/**
	 * Create a chapter
	 */
	async create(
		videoId: number,
		data: CreateChapterRequest
	): Promise<{ success: boolean; data?: { id: number }; error?: string }> {
		return apiRequest(`/videos/${videoId}/chapters`, {
			method: 'POST',
			body: JSON.stringify(data)
		});
	},

	/**
	 * Bulk create/replace chapters
	 */
	async bulkCreate(
		videoId: number,
		chapters: {
			title: string;
			description?: string;
			start_time_seconds: number;
			end_time_seconds?: number;
		}[]
	): Promise<{ success: boolean; error?: string }> {
		return apiRequest(`/videos/${videoId}/chapters/bulk`, {
			method: 'POST',
			body: JSON.stringify({ chapters })
		});
	},

	/**
	 * Update a chapter
	 */
	async update(
		videoId: number,
		chapterId: number,
		data: Partial<CreateChapterRequest>
	): Promise<{ success: boolean; error?: string }> {
		return apiRequest(`/videos/${videoId}/chapters/${chapterId}`, {
			method: 'PUT',
			body: JSON.stringify(data)
		});
	},

	/**
	 * Delete a chapter
	 */
	async delete(videoId: number, chapterId: number): Promise<{ success: boolean; error?: string }> {
		return apiRequest(`/videos/${videoId}/chapters/${chapterId}`, { method: 'DELETE' });
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// SCHEDULED PUBLISHING API
// ═══════════════════════════════════════════════════════════════════════════

export const scheduledApi = {
	/**
	 * List scheduled jobs
	 */
	async list(params?: {
		status?: string;
		resource_type?: string;
		page?: number;
		per_page?: number;
	}): Promise<{ success: boolean; data?: ScheduledJob[]; error?: string }> {
		const searchParams = new URLSearchParams();
		if (params?.status) searchParams.set('status', params.status);
		if (params?.resource_type) searchParams.set('resource_type', params.resource_type);
		if (params?.page) searchParams.set('page', params.page.toString());
		if (params?.per_page) searchParams.set('per_page', params.per_page.toString());

		const query = searchParams.toString();
		return apiRequest(`/scheduled-jobs${query ? `?${query}` : ''}`);
	},

	/**
	 * Create a scheduled job
	 */
	async create(
		data: CreateScheduledJobRequest
	): Promise<{ success: boolean; data?: { id: number; scheduled_at: string }; error?: string }> {
		return apiRequest('/scheduled-jobs', {
			method: 'POST',
			body: JSON.stringify(data)
		});
	},

	/**
	 * Cancel a scheduled job
	 */
	async cancel(id: number): Promise<{ success: boolean; error?: string }> {
		return apiRequest(`/scheduled-jobs/${id}/cancel`, { method: 'POST' });
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// BULK UPLOAD API
// ═══════════════════════════════════════════════════════════════════════════

export const bulkUploadApi = {
	/**
	 * Initialize bulk upload
	 */
	async init(
		data: BulkUploadRequest
	): Promise<{ success: boolean; data?: BulkUploadResponse; error?: string }> {
		return apiRequest('/bulk-upload', {
			method: 'POST',
			body: JSON.stringify(data)
		});
	},

	/**
	 * Get batch status
	 */
	async getBatchStatus(batchId: string): Promise<{ success: boolean; data?: BatchStatus }> {
		return apiRequest(`/bulk-upload/${batchId}`);
	},

	/**
	 * Update upload item status
	 */
	async updateItemStatus(
		itemId: number,
		data: { status: string; progress_percent?: number; error_message?: string }
	): Promise<{ success: boolean }> {
		return apiRequest(`/bulk-upload/item/${itemId}`, {
			method: 'PUT',
			body: JSON.stringify(data)
		});
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// VIDEO OPERATIONS API
// ═══════════════════════════════════════════════════════════════════════════

export const videoOpsApi = {
	/**
	 * Clone a video
	 */
	async clone(
		videoId: number,
		data: CloneVideoRequest
	): Promise<{ success: boolean; data?: { id: number; slug: string } }> {
		return apiRequest(`/videos/${videoId}/clone`, {
			method: 'POST',
			body: JSON.stringify(data)
		});
	},

	/**
	 * Fetch video duration from Bunny.net
	 */
	async fetchDuration(
		videoId: number
	): Promise<{
		success: boolean;
		data?: { duration: number; width?: number; height?: number; formatted_duration: string };
	}> {
		return apiRequest(`/videos/${videoId}/duration`, { method: 'POST' });
	},

	/**
	 * Fetch all video durations
	 */
	async fetchAllDurations(): Promise<{
		success: boolean;
		data?: { updated: number; total_processed: number };
	}> {
		return apiRequest('/videos/fetch-durations', { method: 'POST' });
	},

	/**
	 * Bulk edit videos
	 */
	async bulkEdit(data: BulkEditRequest): Promise<{ success: boolean; error?: string }> {
		return apiRequest('/bulk-edit', {
			method: 'POST',
			body: JSON.stringify(data)
		});
	},

	/**
	 * Export videos to CSV
	 */
	exportCsvUrl(params?: {
		content_type?: string;
		room_id?: number;
		trader_id?: number;
		is_published?: boolean;
		start_date?: string;
		end_date?: string;
	}): string {
		const searchParams = new URLSearchParams();
		if (params?.content_type) searchParams.set('content_type', params.content_type);
		if (params?.room_id) searchParams.set('room_id', params.room_id.toString());
		if (params?.trader_id) searchParams.set('trader_id', params.trader_id.toString());
		if (params?.is_published !== undefined)
			searchParams.set('is_published', params.is_published.toString());
		if (params?.start_date) searchParams.set('start_date', params.start_date);
		if (params?.end_date) searchParams.set('end_date', params.end_date);

		const query = searchParams.toString();
		return `${API_BASE}/api/video-advanced/export/csv${query ? `?${query}` : ''}`;
	},

	/**
	 * Reorder videos in a room
	 */
	async reorderInRoom(roomId: number, videoOrders: VideoOrder[]): Promise<{ success: boolean }> {
		return apiRequest(`/rooms/${roomId}/reorder`, {
			method: 'POST',
			body: JSON.stringify({ video_orders: videoOrders })
		});
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// CDN API
// ═══════════════════════════════════════════════════════════════════════════

export const cdnApi = {
	/**
	 * Purge CDN cache for a video
	 */
	async purgeVideo(videoId: number): Promise<{ success: boolean }> {
		return apiRequest(`/cdn/purge/${videoId}`, { method: 'POST' });
	},

	/**
	 * Purge all CDN cache
	 */
	async purgeAll(): Promise<{ success: boolean }> {
		return apiRequest('/cdn/purge-all', { method: 'POST' });
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Format seconds to HH:MM:SS or MM:SS
 */
export function formatDuration(seconds: number): string {
	const hours = Math.floor(seconds / 3600);
	const minutes = Math.floor((seconds % 3600) / 60);
	const secs = seconds % 60;

	if (hours > 0) {
		return `${hours}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
	}
	return `${minutes}:${secs.toString().padStart(2, '0')}`;
}

/**
 * Parse time string (HH:MM:SS or MM:SS) to seconds
 */
export function parseTimeToSeconds(timeStr: string): number | null {
	const parts = timeStr.split(':').map((p) => parseInt(p, 10));

	if (parts.some(isNaN)) return null;

	if (parts.length === 2) {
		return parts[0] * 60 + parts[1];
	} else if (parts.length === 3) {
		return parts[0] * 3600 + parts[1] * 60 + parts[2];
	}

	return null;
}

/**
 * Generate session ID for analytics
 */
export function generateSessionId(): string {
	return `${Date.now()}-${Math.random().toString(36).substring(2, 11)}`;
}

/**
 * Get device type
 */
export function getDeviceType(): 'desktop' | 'mobile' | 'tablet' {
	if (typeof window === 'undefined') return 'desktop';

	const ua = navigator.userAgent;
	if (/tablet|ipad|playbook|silk/i.test(ua)) return 'tablet';
	if (/mobile|iphone|ipod|android|blackberry|opera mini|iemobile/i.test(ua)) return 'mobile';
	return 'desktop';
}

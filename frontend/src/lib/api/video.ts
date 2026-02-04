/**
 * Video API Client
 * Handles video management and analytics tracking
 */

import { apiClient } from './client.svelte';

export interface Video {
	id: number;
	title: string;
	description?: string;
	url: string;
	platform: 'youtube' | 'vimeo' | 'wistia' | 'dailymotion' | 'twitch' | 'html5';
	video_id?: string;
	thumbnail_url?: string;
	duration?: number;
	quality: string;
	is_active: boolean;
	user_id?: number;
	metadata?: {
		chapters?: VideoChapter[];
		overlays?: VideoOverlay[];
		subtitles?: SubtitleTrack[];
		playlist?: VideoPlaylistItem[];
	};
	created_at: string;
	updated_at: string;
}

export interface VideoChapter {
	id: string;
	title: string;
	startTime: number;
	endTime?: number;
	thumbnail?: string;
}

export interface VideoOverlay {
	id: string;
	type: 'text' | 'image' | 'button' | 'html';
	content: string;
	startTime: number;
	endTime: number;
	position: {
		top?: string;
		bottom?: string;
		left?: string;
		right?: string;
	};
	style?: string;
}

export interface SubtitleTrack {
	label: string;
	language: string;
	src: string;
	default?: boolean;
}

export interface VideoPlaylistItem {
	id: string;
	title: string;
	url: string;
	thumbnail?: string;
	duration?: number;
}

export interface VideoEventData {
	playbackRate?: number;
	previousQuality?: string;
	newQuality?: string;
	errorCode?: string;
	errorMessage?: string;
	seekFrom?: number;
	seekTo?: number;
	milestone?: number;
	[key: string]: string | number | boolean | undefined;
}

export interface VideoAnalytic {
	id: number;
	video_id: number;
	user_id?: number;
	session_id: string;
	event_type: string;
	timestamp_seconds?: number;
	watch_time: number;
	completion_rate: number;
	interactions: number;
	quality?: string;
	buffer_events: number;
	event_data?: VideoEventData;
	ip_address?: string;
	user_agent?: string;
	referrer?: string;
	created_at: string;
	updated_at: string;
}

export interface VideoStats {
	total_views: number;
	total_plays: number;
	total_completions: number;
	average_completion_rate: number;
	total_watch_time: number;
	unique_sessions: number;
	total_errors: number;
	average_buffer_events: number;
}

export interface VideoHeatmap {
	seek_heatmap: Record<number, number>;
	progress_milestones: Record<number, number>;
}

export interface PaginationMeta {
	current_page: number;
	from: number;
	last_page: number;
	per_page: number;
	to: number;
	total: number;
}

export interface VideoTrackingEvent {
	session_id: string;
	event_type:
		| 'view'
		| 'play'
		| 'pause'
		| 'complete'
		| 'progress'
		| 'seek'
		| 'error'
		| 'quality_change'
		| 'speed_change'
		| 'fullscreen_enter'
		| 'fullscreen_exit';
	timestamp_seconds?: number;
	watch_time?: number;
	completion_rate?: number;
	interactions?: number;
	quality?: string;
	buffer_events?: number;
	event_data?: VideoEventData;
}

/**
 * Get all videos
 */
export async function getVideos(params?: {
	platform?: string;
	search?: string;
	per_page?: number;
	page?: number;
}) {
	return apiClient.get<{ data: Video[]; meta: PaginationMeta }>('/videos', { params });
}

/**
 * Get a single video by ID
 */
export async function getVideo(id: number) {
	return apiClient.get<{
		video: Video;
		stats: {
			total_views: number;
			average_completion_rate: number;
			total_watch_time: number;
		};
	}>(`/videos/${id}`);
}

/**
 * Create a new video (admin only)
 */
export async function createVideo(data: Partial<Video>) {
	return apiClient.post<{ message: string; video: Video }>('/admin/videos', data);
}

/**
 * Update a video (admin only)
 */
export async function updateVideo(id: number, data: Partial<Video>) {
	return apiClient.put<{ message: string; video: Video }>(`/admin/videos/${id}`, data);
}

/**
 * Delete a video (admin only)
 */
export async function deleteVideo(id: number) {
	return apiClient.delete<{ message: string }>(`/admin/videos/${id}`);
}

/**
 * Track a video analytics event
 */
export async function trackVideoEvent(videoId: number, data: VideoTrackingEvent) {
	return apiClient.post<{ message: string; analytic: VideoAnalytic }>(
		`/videos/${videoId}/track`,
		data
	);
}

/**
 * Get analytics for a video (admin only)
 */
export async function getVideoAnalytics(
	videoId: number,
	params?: {
		event_type?: string;
		session_id?: string;
		from_date?: string;
		to_date?: string;
		per_page?: number;
		page?: number;
	}
) {
	return apiClient.get<{
		analytics: { data: VideoAnalytic[]; meta: PaginationMeta };
		stats: VideoStats;
	}>(`/admin/videos/${videoId}/analytics`, { params });
}

/**
 * Get video engagement heatmap (admin only)
 */
export async function getVideoHeatmap(videoId: number) {
	return apiClient.get<VideoHeatmap>(`/admin/videos/${videoId}/heatmap`);
}

/**
 * Generate a unique session ID for tracking
 */
export function generateSessionId(): string {
	return `session_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}

/**
 * Queued event for batch processing
 */
interface QueuedEvent {
	session_id: string;
	event_type: string;
	timestamp_seconds?: number;
	watch_time?: number;
	completion_rate?: number;
	interactions?: number;
	quality?: string;
	buffer_events?: number;
	event_data?: VideoEventData;
}

/**
 * Video tracking helper class
 */
export class VideoTracker {
	private videoId: number;
	private sessionId: string;
	private watchStartTime: number = 0;
	private totalWatchTime: number = 0;
	private eventQueue: QueuedEvent[] = [];
	private flushInterval: number | null = null;

	constructor(videoId: number, sessionId?: string) {
		this.videoId = videoId;
		this.sessionId = sessionId || generateSessionId();
		this.startFlushInterval();
	}

	/**
	 * Track a video event
	 */
	async track(
		eventType: string,
		data?: {
			timestamp_seconds?: number;
			watch_time?: number;
			completion_rate?: number;
			interactions?: number;
			quality?: string;
			buffer_events?: number;
			event_data?: VideoEventData;
		}
	) {
		const event: QueuedEvent = {
			session_id: this.sessionId,
			event_type: eventType,
			...data
		};

		this.eventQueue.push(event);

		// Flush immediately for critical events
		if (['view', 'complete', 'error'].includes(eventType)) {
			await this.flush();
		}
	}

	/**
	 * Start watch time tracking
	 */
	startWatchTime() {
		this.watchStartTime = Date.now();
	}

	/**
	 * Stop watch time tracking and update total
	 */
	stopWatchTime() {
		if (this.watchStartTime > 0) {
			this.totalWatchTime += Math.floor((Date.now() - this.watchStartTime) / 1000);
			this.watchStartTime = 0;
		}
	}

	/**
	 * Get total watch time in seconds
	 */
	getTotalWatchTime(): number {
		let total = this.totalWatchTime;
		if (this.watchStartTime > 0) {
			total += Math.floor((Date.now() - this.watchStartTime) / 1000);
		}
		return total;
	}

	/**
	 * Flush queued events to the server
	 */
	async flush() {
		if (this.eventQueue.length === 0) return;

		const events = [...this.eventQueue];
		this.eventQueue = [];

		try {
			// Send events in parallel for better performance
			await Promise.all(events.map(event => trackVideoEvent(this.videoId, event as VideoTrackingEvent)));
		} catch (error: unknown) {
			console.error('Failed to flush video analytics:', error);
			// Re-queue failed events
			this.eventQueue.unshift(...events);
		}
	}

	/**
	 * Start automatic flush interval
	 */
	private startFlushInterval() {
		this.flushInterval = window.setInterval(() => {
			this.flush();
		}, 30000); // Flush every 30 seconds
	}

	/**
	 * Clean up and flush remaining events
	 */
	async destroy() {
		if (this.flushInterval) {
			clearInterval(this.flushInterval);
			this.flushInterval = null;
		}
		this.stopWatchTime();
		await this.flush();
	}
}

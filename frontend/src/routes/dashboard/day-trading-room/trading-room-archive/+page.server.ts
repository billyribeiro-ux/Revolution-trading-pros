/**
 * Trading Room Archive Page Server
 * ═══════════════════════════════════════════════════════════════════════════
 * Apple ICT 11+ Principal Engineer Implementation
 *
 * Fetches room archive videos
 *
 * @version 3.0.0
 */

import { env } from '$env/dynamic/private';
import type { PageServerLoad } from './$types';
import { logger } from '$lib/utils/logger';

// Video response from API
interface VideoResponse {
	id: number;
	title: string;
	slug: string;
	description: string | null;
	video_url: string;
	thumbnail_url: string | null;
	duration: number | null;
	formatted_duration: string;
	content_type: string;
	video_date: string;
	formatted_date: string;
	is_published: boolean;
	trader: {
		id: number;
		name: string;
		slug: string;
	} | null;
	rooms: Array<{
		id: number;
		name: string;
		slug: string;
	}>;
	tags: string[];
	[key: string]: unknown;
}

// API response structure
interface ApiResponse {
	success: boolean;
	data: VideoResponse[];
	meta: {
		current_page: number;
		per_page: number;
		total: number;
		last_page: number;
	};
}

// Page data type export for +page.svelte
export interface ArchivePageData {
	videos: VideoResponse[];
	meta: {
		current_page: number;
		per_page: number;
		total: number;
		last_page: number;
	};
	search: string;
	error: string | null;
}

export const load: PageServerLoad = async ({ url, fetch }): Promise<ArchivePageData> => {
	const API_URL = env.API_URL || 'https://revolution-trading-pros-api.fly.dev';
	const ROOM_SLUG = 'day-trading-room';

	// Get query params
	const page = url.searchParams.get('page') || '1';
	const perPage = url.searchParams.get('per_page') || '50';
	const search = url.searchParams.get('search') || '';

	try {
		// Build query string for videos API
		const queryParams = new URLSearchParams({
			page,
			per_page: perPage,
			content_type: 'room_archive'
		});

		if (search) {
			queryParams.set('search', search);
		}

		// Fetch videos from the admin trading-rooms videos endpoint
		const videosResponse = await fetch(
			`${API_URL}/api/admin/trading-rooms/videos/${ROOM_SLUG}?${queryParams.toString()}`,
			{
				headers: {
					'Content-Type': 'application/json',
					Accept: 'application/json'
				}
			}
		);

		if (!videosResponse.ok) {
			// Fallback to public videos endpoint with room filter
			const roomResponse = await fetch(`${API_URL}/api/trading-rooms/${ROOM_SLUG}`);
			let roomId: number | null = null;

			if (roomResponse.ok) {
				const roomData = await roomResponse.json();
				roomId = roomData.data?.id || null;
			}

			if (roomId) {
				queryParams.set('room_id', roomId.toString());
				const publicVideosResponse = await fetch(`${API_URL}/api/videos?${queryParams.toString()}`);

				if (publicVideosResponse.ok) {
					const data: ApiResponse = await publicVideosResponse.json();
					return {
						videos: data.data || [],
						meta: data.meta || { current_page: 1, per_page: 50, total: 0, last_page: 1 },
						search,
						error: null
					};
				}
			}

			// If all fails, return empty with error
			return {
				videos: [],
				meta: { current_page: 1, per_page: 50, total: 0, last_page: 1 },
				search,
				error: 'Unable to load archive videos. Please try again later.'
			};
		}

		const data = await videosResponse.json();

		// Transform the response to match expected format
		const videos: VideoResponse[] = (data.data || []).map((video: Record<string, unknown>) => ({
			id: video.id as number,
			title: video.title as string,
			slug: video.slug as string,
			description: (video.description as string) || null,
			video_url: video.video_url as string,
			thumbnail_url: (video.thumbnail_url as string) || null,
			duration: (video.duration as number) || null,
			formatted_duration: formatDuration((video.duration as number) || 0),
			content_type: (video.content_type as string) || 'room_archive',
			video_date: (video.created_at as string) || new Date().toISOString(),
			formatted_date: formatDate((video.created_at as string) || new Date().toISOString()),
			is_published: (video.is_published as boolean) ?? true,
			trader: null,
			rooms: [],
			tags: []
		}));

		return {
			videos,
			meta: {
				current_page: data.meta?.current_page || parseInt(page),
				per_page: data.meta?.per_page || parseInt(perPage),
				total: data.meta?.total || videos.length,
				last_page: data.meta?.total_pages || data.meta?.last_page || 1
			},
			search,
			error: null
		};
	} catch (err) {
		logger.error('Trading Room Archive load error:', err);
		return {
			videos: [],
			meta: { current_page: 1, per_page: 50, total: 0, last_page: 1 },
			search,
			error: 'Failed to load archive videos. Please try again later.'
		};
	}
};

// Helper function to format duration (seconds to MM:SS or HH:MM:SS)
function formatDuration(seconds: number): string {
	if (!seconds || seconds <= 0) return '';
	const hours = Math.floor(seconds / 3600);
	const minutes = Math.floor((seconds % 3600) / 60);
	const secs = seconds % 60;
	if (hours > 0) {
		return `${hours}:${String(minutes).padStart(2, '0')}:${String(secs).padStart(2, '0')}`;
	}
	return `${minutes}:${String(secs).padStart(2, '0')}`;
}

// Helper function to format date
function formatDate(dateStr: string): string {
	try {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	} catch {
		return dateStr;
	}
}

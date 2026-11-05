import {
	recordedVideoDate,
	formatRecordedVideoDate,
	formatRecordedVideoDuration
} from '#lib/utils/unified-video-metadata.js';
import { memberVideoHeaders, MEMBER_VIDEO_CACHE } from '#lib/server/member-videos.js';
/**
 * Trading Room Archive Page Server
 * ═══════════════════════════════════════════════════════════════════════════
 * Apple ICT 11+ Principal Engineer Implementation
 *
 * Fetches room archive videos
 *
 * @version 3.0.0
 */

import { API_URL as ENV_API_URL } from '$app/env/private';
import type { PageServerLoad } from './$types';

// Video response from API
interface VideoResponse {
	id: number;
	title: string;
	slug: string;
	description: string | null;
	video_url: string | null;
	thumbnail_url: string | null;
	duration: number | null;
	formatted_duration: string | null;
	content_type: string;
	video_date: string | null;
	formatted_date: string | null;
	is_published: boolean | null;
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

export const load: PageServerLoad = async ({
	url,
	fetch,
	locals,
	setHeaders
}): Promise<ArchivePageData> => {
	const videoHeaders = memberVideoHeaders(locals);
	setHeaders(MEMBER_VIDEO_CACHE);
	const API_URL = ENV_API_URL || 'http://localhost:8080';
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
				const publicVideosResponse = await fetch(
					`${API_URL}/api/videos?${queryParams.toString()}`,
					{ headers: videoHeaders }
				);

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
			video_url: typeof video.video_url === 'string' ? video.video_url : null,
			thumbnail_url: (video.thumbnail_url as string) || null,
			duration: typeof video.duration === 'number' ? video.duration : null,
			formatted_duration: formatRecordedVideoDuration(
				typeof video.duration === 'number' ? video.duration : null
			),
			content_type: (video.content_type as string) || 'room_archive',
			video_date: recordedVideoDate(typeof video.video_date === 'string' ? video.video_date : null),
			formatted_date: formatRecordedVideoDate(
				typeof video.video_date === 'string' ? video.video_date : null
			),
			is_published: typeof video.is_published === 'boolean' ? video.is_published : null,
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
		console.error('Trading Room Archive load error:', err);
		return {
			videos: [],
			meta: { current_page: 1, per_page: 50, total: 0, last_page: 1 },
			search,
			error: 'Failed to load archive videos. Please try again later.'
		};
	}
};

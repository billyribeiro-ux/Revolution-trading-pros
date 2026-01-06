/**
 * Video Detail Page - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * ICT 11+ Principal Engineer Grade
 * Fetches single video with Previous/Next navigation from backend API
 * NOW SUPPORTS DYNAMIC ROOM SELECTION via [room_slug] parameter
 * 
 * @version 3.0.0 - January 2026 - Multi-room support
 */

import type { ServerLoadEvent } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';

const BACKEND_URL = import.meta.env.VITE_API_URL || 'https://revolution-trading-pros-api.fly.dev/api';

export interface VideoNavReference {
	id: string;
	title: string;
	slug: string;
}

export interface TraderInfo {
	id: string;
	name: string;
	slug: string;
	avatar_url?: string;
}

export interface VideoDetail {
	id: string;
	title: string;
	slug: string;
	description: string;
	videoUrl: string;
	thumbnailUrl: string;
	duration?: number;
	publishedAt: string;
	author: string;
	trader?: TraderInfo;
	previousVideo?: VideoNavReference;
	nextVideo?: VideoNavReference;
}

export interface PageData {
	video: VideoDetail;
	roomSlug: string;
	roomName: string;
}

export async function load({ params, fetch, cookies }: ServerLoadEvent): Promise<PageData> {
	const { room_slug, slug } = params;

	if (!slug || !room_slug) {
		throw error(404, 'Video not found');
	}

	try {
		// Fetch video with navigation from backend API using dynamic room_slug
		const apiUrl = `${BACKEND_URL}/trading-rooms/${room_slug}/videos/${slug}`;

		// Get auth token if available
		const token = cookies.get('auth_token');
		const headers: HeadersInit = {
			'Accept': 'application/json',
			'Content-Type': 'application/json',
		};
		if (token) {
			headers['Authorization'] = `Bearer ${token}`;
		}

		const response = await fetch(apiUrl, { headers });

		if (response.ok) {
			const data = await response.json();
			const videoData = data.data;

			// Transform backend response to frontend format
			const video: VideoDetail = {
				id: videoData.video.id,
				title: videoData.video.title,
				slug: videoData.video.slug,
				description: videoData.video.description || '',
				videoUrl: videoData.video.video_url,
				thumbnailUrl: videoData.video.thumbnail_url || 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg',
				duration: videoData.video.duration,
				publishedAt: formatDate(videoData.video.published_at),
				author: videoData.video.trader?.name || 'Trading Team',
				trader: videoData.video.trader,
				previousVideo: videoData.previous_video ? {
					id: videoData.previous_video.id,
					title: videoData.previous_video.title,
					slug: videoData.previous_video.slug,
				} : undefined,
				nextVideo: videoData.next_video ? {
					id: videoData.next_video.id,
					title: videoData.next_video.title,
					slug: videoData.next_video.slug,
				} : undefined,
			};

			return {
				video,
				roomSlug: room_slug,
				roomName: getRoomName(room_slug),
			};
		}

		// If API fails, return mock data for development
		console.warn('Backend API unavailable, using mock data for video:', slug);
		return getMockVideoData(room_slug, slug);

	} catch (err) {
		console.error('Failed to fetch video:', err);
		// Return mock data as fallback
		return getMockVideoData(room_slug, slug);
	}
}

function formatDate(dateStr: string): string {
	try {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric',
		});
	} catch {
		return dateStr;
	}
}

function getRoomName(slug: string): string {
	const roomNames: Record<string, string> = {
		'day-trading-room': 'Day Trading Room',
		'high-octane-scanner': 'High Octane Scanner',
		'swing-trading-room': 'Swing Trading Room',
		'options-room': 'Options Room',
	};
	return roomNames[slug] || slug.split('-').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join(' ');
}

function getMockVideoData(roomSlug: string, slug: string): PageData {
	// Mock video data for development when backend is unavailable
	const mockVideos = [
		{
			id: '1',
			slug: 'bookmap',
			title: 'How to use Bookmap to make more informed trades',
			description: '<p>You asked for it, you got it. Here are Kody\'s Bookmap tools and how he uses them to make better informed trades.</p>',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/nightlyvids/2025/dec29DSG.mp4',
			thumbnailUrl: 'https://cdn.simplertrading.com/2025/02/07135413/SimplerCentral_KA.jpg',
			publishedAt: 'January 2, 2026',
			author: 'Kody Ashmore',
		},
		{
			id: '2',
			slug: 'cautious-entry-into-2026',
			title: 'Cautious entry into 2026',
			description: '<p>As we head into the new year, here are some key considerations for your trading strategy.</p>',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/nightlyvids/2025/dec29DSG.mp4',
			thumbnailUrl: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			publishedAt: 'December 31, 2025',
			author: 'Henry Gambell',
		},
		{
			id: '3',
			slug: 'market-analysis',
			title: 'Market Analysis & Trading Strategies',
			description: '<p>Things can always change, but given how the market closed on Tuesday, it looks like Santa\'s on his way.</p>',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/nightlyvids/2025/dec29DSG.mp4',
			thumbnailUrl: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			publishedAt: 'December 23, 2025',
			author: 'HG',
		},
	];

	// Find current video index
	const currentIndex = mockVideos.findIndex(v => v.slug === slug);
	const currentVideo = currentIndex >= 0 ? mockVideos[currentIndex] : mockVideos[0];

	// Calculate Previous/Next based on position
	// Previous = older video (higher index in date-sorted list)
	// Next = newer video (lower index in date-sorted list)
	const previousVideo = currentIndex < mockVideos.length - 1 ? {
		id: mockVideos[currentIndex + 1].id,
		title: mockVideos[currentIndex + 1].title,
		slug: mockVideos[currentIndex + 1].slug,
	} : undefined;

	const nextVideo = currentIndex > 0 ? {
		id: mockVideos[currentIndex - 1].id,
		title: mockVideos[currentIndex - 1].title,
		slug: mockVideos[currentIndex - 1].slug,
	} : undefined;

	return {
		video: {
			...currentVideo,
			previousVideo,
			nextVideo,
		},
		roomSlug,
		roomName: getRoomName(roomSlug),
	};
}

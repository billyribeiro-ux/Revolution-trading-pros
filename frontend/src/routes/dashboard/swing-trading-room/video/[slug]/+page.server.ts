/**
 * Video Detail Page - Server Load Function (Swing Trading Room)
 * ===============================================================================
 *
 * ICT 11+ Principal Engineer Grade
 * Fetches single video with Previous/Next navigation from backend API
 *
 * @version 1.0.0 - January 2026
 */

import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

// ICT 7 FIX: VITE_API_URL does NOT include /api suffix - we add it here
const API_ROOT = import.meta.env.VITE_API_URL || 'https://revolution-trading-pros-api.fly.dev';
const BACKEND_URL = `${API_ROOT}/api`;

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
}

export const load: PageServerLoad = async ({ params, fetch, cookies }): Promise<PageData> => {
	const { slug } = params;

	if (!slug) {
		error(404, 'Video not found');
	}

	try {
		// Fetch video with navigation from backend API
		const apiUrl = `${BACKEND_URL}/trading-rooms/swing-trading-room/videos/${slug}`;

		// Get auth token if available
		const token = cookies.get('auth_token');
		const headers: HeadersInit = {
			Accept: 'application/json',
			'Content-Type': 'application/json'
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
				thumbnailUrl:
					videoData.video.thumbnail_url ||
					'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg',
				duration: videoData.video.duration,
				publishedAt: formatDate(videoData.video.published_at),
				author: videoData.video.trader?.name || 'Trading Team',
				trader: videoData.video.trader,
				previousVideo: videoData.previous_video
					? {
							id: videoData.previous_video.id,
							title: videoData.previous_video.title,
							slug: videoData.previous_video.slug
						}
					: undefined,
				nextVideo: videoData.next_video
					? {
							id: videoData.next_video.id,
							title: videoData.next_video.title,
							slug: videoData.next_video.slug
						}
					: undefined
			};

			return { video };
		}

		// If API fails, return mock data for development
		console.warn('Backend API unavailable, using mock data for video:', slug);
		return getMockVideoData(slug);
	} catch (err) {
		console.error('Failed to fetch video:', err);
		// Return mock data as fallback
		return getMockVideoData(slug);
	}
};

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

function getMockVideoData(slug: string): PageData {
	// Mock video data for development when backend is unavailable
	const mockVideos = [
		{
			id: '1',
			slug: 'swing-trading-basics',
			title: 'Swing Trading Fundamentals',
			description:
				'<p>Learn the core concepts of swing trading and how to identify profitable opportunities.</p>',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/nightlyvids/2025/dec29DSG.mp4',
			thumbnailUrl: 'https://cdn.simplertrading.com/2025/02/07135413/SimplerCentral_KA.jpg',
			publishedAt: 'January 2, 2026',
			author: 'Trading Team'
		},
		{
			id: '2',
			slug: 'market-structure-analysis',
			title: 'Understanding Market Structure',
			description:
				'<p>Deep dive into market structure and how it affects swing trading decisions.</p>',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/nightlyvids/2025/dec29DSG.mp4',
			thumbnailUrl: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			publishedAt: 'December 31, 2025',
			author: 'Trading Team'
		},
		{
			id: '3',
			slug: 'weekly-trading-plan',
			title: 'Weekly Trading Plan Setup',
			description:
				'<p>How to set up your weekly trading plan for consistent swing trading success.</p>',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/nightlyvids/2025/dec29DSG.mp4',
			thumbnailUrl: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			publishedAt: 'December 23, 2025',
			author: 'Trading Team'
		}
	];

	// Find current video index
	const currentIndex = mockVideos.findIndex((v) => v.slug === slug);
	const currentVideo = currentIndex >= 0 ? mockVideos[currentIndex] : mockVideos[0];

	// Calculate Previous/Next based on position
	const previousVideo =
		currentIndex < mockVideos.length - 1
			? {
					id: mockVideos[currentIndex + 1].id,
					title: mockVideos[currentIndex + 1].title,
					slug: mockVideos[currentIndex + 1].slug
				}
			: undefined;

	const nextVideo =
		currentIndex > 0
			? {
					id: mockVideos[currentIndex - 1].id,
					title: mockVideos[currentIndex - 1].title,
					slug: mockVideos[currentIndex - 1].slug
				}
			: undefined;

	return {
		video: {
			...currentVideo,
			previousVideo,
			nextVideo
		}
	};
}

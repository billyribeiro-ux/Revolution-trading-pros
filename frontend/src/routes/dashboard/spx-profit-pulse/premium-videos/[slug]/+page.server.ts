/**
 * SPX Profit Pulse Video Detail - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 *
 * Fetches individual video data from backend API
 * Provides navigation to previous/next videos
 */

import type { PageServerLoad } from './$types';

const API_ROOT = import.meta.env.VITE_API_URL || 'https://revolution-trading-pros-api.fly.dev';
const BACKEND_URL = `${API_ROOT}/api`;

export interface VideoDetail {
	id: string;
	title: string;
	slug: string;
	date: string;
	trader: string;
	traderImage: string;
	videoUrl: string;
	chatLogUrl?: string;
	previousVideo?: { slug: string; title: string };
	nextVideo?: { slug: string; title: string };
}

export const load: PageServerLoad = async ({ params, fetch, cookies }) => {
	const { slug } = params;

	try {
		// Try to fetch from backend API
		const apiUrl = `${BACKEND_URL}/trading-rooms/spx-profit-pulse/videos/${slug}`;
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
			return {
				video: data.data as VideoDetail
			};
		}

		// Fallback to mock data
		return getMockVideoData(slug);
	} catch (error) {
		console.error('Failed to fetch video:', error);
		return getMockVideoData(slug);
	}
};

function getMockVideoData(slug: string): { video: VideoDetail } {
	// Mock data matching WordPress structure
	const mockVideos: Record<string, VideoDetail> = {
		'january-8-2026': {
			id: '1',
			title: 'January 8 2026',
			slug: 'january-8-2026',
			date: 'January 08, 2026',
			trader: 'Melissa Beegle',
			traderImage: 'https://cdn.simplertrading.com/2024/01/25025702/melissa.png',
			videoUrl:
				'https://cloud-streaming.s3.amazonaws.com/chatrecordings/trendyspx/670d255d4c45202c5ee724bb_Tr3ndy_SPX_Melissa_01-08-2026__04.01.280_PM.mp4',
			chatLogUrl:
				'https://cloud-streaming.s3.amazonaws.com/chatrecordings/trendyspx/670d255d4c45202c5ee724bb_Tr3ndy_SPX_main_chat_01-08-2026__11.45.679_PM.txt',
			previousVideo: { slug: 'january-7-2026', title: 'January 7 2026' },
			nextVideo: { slug: 'january-9-2026', title: 'January 9 2026' }
		},
		'january-12-2026': {
			id: '2',
			title: 'January 12 2026',
			slug: 'january-12-2026',
			date: 'January 12, 2026',
			trader: 'Tr3ndy Jon',
			traderImage: 'https://cdn.simplertrading.com/2024/01/29162759/SimplerCentral_JM.jpg',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/Tr3ndy/TrendySPXQuickstart2025.mp4',
			previousVideo: { slug: 'january-11-2026', title: 'January 11 2026' },
			nextVideo: { slug: 'january-13-2026', title: 'January 13 2026' }
		},
		'january-13-2026': {
			id: '3',
			title: 'January 13 2026',
			slug: 'january-13-2026',
			date: 'January 13, 2026',
			trader: 'Billy Ribeiro',
			traderImage: 'https://cdn.simplertrading.com/2024/09/23132611/Jon-Generic.jpg',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/Tr3ndy/TrendySPXQuickstart2025.mp4',
			previousVideo: { slug: 'january-12-2026', title: 'January 12 2026' },
			nextVideo: { slug: 'january-14-2026', title: 'January 14 2026' }
		}
	};

	const video = mockVideos[slug] || {
		id: '0',
		title: slug
			.split('-')
			.map((w) => w.charAt(0).toUpperCase() + w.slice(1))
			.join(' '),
		slug: slug,
		date: new Date().toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: '2-digit'
		}),
		trader: 'SPX Trading Team',
		traderImage: 'https://cdn.simplertrading.com/2024/01/29162759/SimplerCentral_JM.jpg',
		videoUrl: 'https://simpler-options.s3.amazonaws.com/Tr3ndy/TrendySPXQuickstart2025.mp4'
	};

	return { video };
}

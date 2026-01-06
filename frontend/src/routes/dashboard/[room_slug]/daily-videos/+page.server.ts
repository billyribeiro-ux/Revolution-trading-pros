/**
 * Premium Daily Videos - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * ICT 11+ Principal Engineer Grade
 * Fetches videos from backend API with pagination support
 * NOW SUPPORTS DYNAMIC ROOM SELECTION via [room_slug] parameter
 * 
 * @version 3.0.0 - January 2026 - Multi-room support
 */

import type { ServerLoadEvent, RequestEvent } from '@sveltejs/kit';

const BACKEND_URL = import.meta.env.VITE_API_URL || 'https://revolution-trading-pros-api.fly.dev/api';

export interface DailyVideo {
	id: string;
	title: string;
	slug: string;
	date: string;
	trader: string;
	excerpt: string;
	thumbnail: string;
	isVideo: boolean;
}

export interface PageData {
	videos: DailyVideo[];
	pagination: {
		page: number;
		perPage: number;
		total: number;
		totalPages: number;
	};
	roomSlug: string;
	roomName: string;
	error?: string;
}

export async function load({ params, url, fetch, cookies }: ServerLoadEvent): Promise<PageData> {
	const { room_slug } = params;
	const page = parseInt(url.searchParams.get('page') || '1');
	const search = url.searchParams.get('search') || '';
	const perPage = 12;

	try {
		// Try to fetch from backend API using dynamic room_slug
		const apiUrl = new URL(`${BACKEND_URL}/trading-rooms/${room_slug}/videos`);
		apiUrl.searchParams.set('page', page.toString());
		apiUrl.searchParams.set('per_page', perPage.toString());
		if (search) {
			apiUrl.searchParams.set('search', search);
		}

		// Get auth token if available
		const token = cookies.get('auth_token');
		const headers: HeadersInit = {
			'Accept': 'application/json',
			'Content-Type': 'application/json',
		};
		if (token) {
			headers['Authorization'] = `Bearer ${token}`;
		}

		const response = await fetch(apiUrl.toString(), { headers });

		if (response.ok) {
			const data = await response.json();
			
			// Transform backend response to frontend format
			const videos: DailyVideo[] = data.data.videos.map((video: any) => ({
				id: video.id,
				title: video.title,
				slug: video.slug,
				date: formatDate(video.published_at),
				trader: video.trader?.name || 'Trading Team',
				excerpt: video.description || '',
				thumbnail: video.thumbnail_url || 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg',
				isVideo: true,
			}));

			return {
				videos,
				pagination: {
					page: data.data.pagination.page,
					perPage: data.data.pagination.per_page,
					total: data.data.pagination.total,
					totalPages: data.data.pagination.total_pages,
				},
				roomSlug: room_slug,
				roomName: getRoomName(room_slug),
			};
		}

		// If API fails, return mock data for development
		console.warn('Backend API unavailable, using mock data for room:', room_slug);
		return getMockData(room_slug, page, perPage, search);

	} catch (err) {
		console.error('Failed to fetch videos:', err);
		// Return mock data as fallback
		return getMockData(room_slug, page, perPage, search);
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

function getMockData(roomSlug: string, page: number, perPage: number, search: string): PageData {
	// Mock video data for development when backend is unavailable
	const allVideos: DailyVideo[] = [
		{
			id: '1',
			title: 'How to use Bookmap to make more informed trades',
			slug: 'bookmap',
			date: 'January 2, 2026',
			trader: 'Kody Ashmore',
			excerpt: 'You asked for it, you got it. Here are Kody\'s Bookmap tools and how he uses them to make better informed trades.',
			thumbnail: 'https://cdn.simplertrading.com/2025/02/07135413/SimplerCentral_KA.jpg',
			isVideo: true,
		},
		{
			id: '2',
			title: 'Cautious entry into 2026',
			slug: 'cautious-entry-into-2026',
			date: 'December 31, 2025',
			trader: 'Henry Gambell',
			excerpt: 'If Santa doesn\'t show up, the first bit of 2026 may be a little precarious. With that in mind, let\'s dive in to some of the most important charts for the new year.',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			isVideo: true,
		},
		{
			id: '3',
			title: 'SPX Snoozefest',
			slug: 'spx-snoozefest',
			date: 'December 30, 2025',
			trader: 'Heather',
			excerpt: 'We\'ve had two days of some very narrow ranges in the indices. It\'s almost as though the market has had an amazing year and just needs to rest a bit before making its next move!',
			thumbnail: 'https://cdn.simplertrading.com/2025/11/18171423/MTT_HV.jpg',
			isVideo: true,
		},
	];

	// Filter by search if provided
	const filteredVideos = search
		? allVideos.filter(v => v.title.toLowerCase().includes(search.toLowerCase()))
		: allVideos;

	const total = filteredVideos.length;
	const totalPages = Math.ceil(total / perPage);
	const start = (page - 1) * perPage;
	const videos = filteredVideos.slice(start, start + perPage);

	return {
		videos,
		pagination: {
			page,
			perPage,
			total,
			totalPages,
		},
		roomSlug,
		roomName: getRoomName(roomSlug),
	};
}

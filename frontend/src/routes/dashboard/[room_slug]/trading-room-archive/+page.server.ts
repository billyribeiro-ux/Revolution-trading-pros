/**
 * Trading Room Archive Page Server (Dynamic Route)
 * ═══════════════════════════════════════════════════════════════════════════
 * Apple ICT 11+ Principal Engineer Implementation
 * 
 * Fetches room archive videos from unified videos API for any trading room.
 * Supports: day-trading-room, swing-trading-room, small-account-mentorship
 * 
 * @version 2.0.0
 */

import { env } from '$env/dynamic/private';
import type { ServerLoadEvent } from '@sveltejs/kit';

// Room configuration
const ROOM_CONFIG: Record<string, { name: string; startHereUrl: string }> = {
	'day-trading-room': {
		name: 'Day Trading Room',
		startHereUrl: '/dashboard/day-trading-room/start-here'
	},
	'swing-trading-room': {
		name: 'Swing Trading Room',
		startHereUrl: '/dashboard/swing-trading-room/start-here'
	},
	'small-account-mentorship': {
		name: 'Small Account Mentorship',
		startHereUrl: '/dashboard/small-account-mentorship/start-here'
	}
};

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
export interface DynamicArchivePageData {
	roomSlug: string;
	roomName: string;
	startHereUrl: string;
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

export const load = async ({ url, fetch, params }: ServerLoadEvent): Promise<DynamicArchivePageData> => {
	const API_URL = env.API_URL || 'https://api.revolutiontradingpros.com';
	const roomSlug = params.room_slug;
	
	// Get room config
	const roomConfig = ROOM_CONFIG[roomSlug] || {
		name: roomSlug.split('-').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join(' '),
		startHereUrl: `/dashboard/${roomSlug}/start-here`
	};
	
	// Get query params
	const page = url.searchParams.get('page') || '1';
	const search = url.searchParams.get('search') || '';
	
	try {
		// Fetch room archives from unified videos API
		const apiParams = new URLSearchParams({
			content_type: 'room_archive',
			room_slug: roomSlug,
			page,
			per_page: '50',
		});
		
		if (search) {
			apiParams.set('search', search);
		}
		
		const response = await fetch(`${API_URL}/api/videos?${apiParams.toString()}`);
		
		if (!response.ok) {
			console.error('Failed to fetch archives:', response.status);
			return {
				roomSlug,
				roomName: roomConfig.name,
				startHereUrl: roomConfig.startHereUrl,
				videos: [],
				meta: { current_page: 1, per_page: 50, total: 0, last_page: 1 },
				search,
				error: 'Failed to load archives'
			};
		}
		
		const data: ApiResponse = await response.json();
		
		return {
			roomSlug,
			roomName: roomConfig.name,
			startHereUrl: roomConfig.startHereUrl,
			videos: data.data || [],
			meta: data.meta || { current_page: 1, per_page: 50, total: 0, last_page: 1 },
			search,
			error: null
		};
	} catch (error) {
		console.error('Error fetching archives:', error);
		return {
			roomSlug,
			roomName: roomConfig.name,
			startHereUrl: roomConfig.startHereUrl,
			videos: [],
			meta: { current_page: 1, per_page: 50, total: 0, last_page: 1 },
			search,
			error: 'Failed to load archives'
		};
	}
};

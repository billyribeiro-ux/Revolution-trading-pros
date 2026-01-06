/**
 * Trading Room Archive Page Server
 * ═══════════════════════════════════════════════════════════════════════════
 * Apple ICT 11+ Principal Engineer Implementation
 * 
 * Fetches room archive videos from unified videos API
 * 
 * @version 2.0.0
 */

import { env } from '$env/dynamic/private';
import type { ServerLoadEvent } from '@sveltejs/kit';

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

export const load = async ({ url, fetch }: ServerLoadEvent): Promise<ArchivePageData> => {
	const API_URL = env.API_URL || 'https://api.revolutiontradingpros.com';
	
	// Get query params
	const page = url.searchParams.get('page') || '1';
	const search = url.searchParams.get('search') || '';
	
	try {
		// Fetch room archives from unified videos API
		const params = new URLSearchParams({
			content_type: 'room_archive',
			room_slug: 'day-trading-room',
			page,
			per_page: '50',
		});
		
		if (search) {
			params.set('search', search);
		}
		
		const response = await fetch(`${API_URL}/api/videos?${params.toString()}`);
		
		if (!response.ok) {
			console.error('Failed to fetch archives:', response.status);
			return {
				videos: [],
				meta: { current_page: 1, per_page: 50, total: 0, last_page: 1 },
				search,
				error: 'Failed to load archives'
			};
		}
		
		const data: ApiResponse = await response.json();
		
		return {
			videos: data.data || [],
			meta: data.meta || { current_page: 1, per_page: 50, total: 0, last_page: 1 },
			search,
			error: null
		};
	} catch (error) {
		console.error('Error fetching archives:', error);
		return {
			videos: [],
			meta: { current_page: 1, per_page: 50, total: 0, last_page: 1 },
			search,
			error: 'Failed to load archives'
		};
	}
};

/**
 * Chatroom Archive Video Detail Page Server
 * ═══════════════════════════════════════════════════════════════════════════
 * Apple ICT 11+ Principal Engineer Implementation
 * 
 * Fetches videos for a specific date from the chatroom archive.
 * URL: /chatroom-archive/{room_slug}/{date_slug}
 * Example: /chatroom-archive/day-trading-room/01052026
 * 
 * @version 1.0.0
 */

import { env } from '$env/dynamic/private';
import { error } from '@sveltejs/kit';

// Room configuration
const ROOM_CONFIG: Record<string, { name: string; archiveUrl: string }> = {
	'day-trading-room': {
		name: 'Day Trading Room',
		archiveUrl: '/dashboard/day-trading-room/trading-room-archive'
	},
	'swing-trading-room': {
		name: 'Swing Trading Room',
		archiveUrl: '/dashboard/swing-trading-room/trading-room-archive'
	},
	'small-account-mentorship': {
		name: 'Small Account Mentorship',
		archiveUrl: '/dashboard/small-account-mentorship/trading-room-archive'
	}
};

// Video from API
interface VideoData {
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
		photo_url?: string;
	} | null;
	rooms: Array<{
		id: number;
		name: string;
		slug: string;
	}>;
	tags: string[];
}

// API response
interface ApiResponse {
	success: boolean;
	data: VideoData[];
	meta: {
		current_page: number;
		per_page: number;
		total: number;
		last_page: number;
	};
}

// Page data export
export interface ArchiveDetailPageData {
	roomSlug: string;
	roomName: string;
	archiveUrl: string;
	dateSlug: string;
	displayDate: string;
	videos: VideoData[];
	chatLogUrl: string | null;
	previousDate: string | null;
	nextDate: string | null;
	error: string | null;
}

// Parse date from slug format (MMDDYYYY) to Date object
function parseDateSlug(slug: string): Date | null {
	if (slug.length !== 8) return null;
	const month = parseInt(slug.substring(0, 2), 10) - 1;
	const day = parseInt(slug.substring(2, 4), 10);
	const year = parseInt(slug.substring(4, 8), 10);
	const date = new Date(year, month, day);
	return isNaN(date.getTime()) ? null : date;
}

// Format date for display
function formatDisplayDate(date: Date): string {
	return date.toLocaleDateString('en-US', { 
		month: 'long', 
		day: 'numeric', 
		year: 'numeric' 
	});
}

// Format date to slug (MMDDYYYY)
function dateToSlug(date: Date): string {
	const month = String(date.getMonth() + 1).padStart(2, '0');
	const day = String(date.getDate()).padStart(2, '0');
	const year = date.getFullYear();
	return `${month}${day}${year}`;
}

/** @type {import('./$types').PageServerLoad} */
export const load = async ({ params, fetch }): Promise<ArchiveDetailPageData> => {
	const API_URL = env.API_URL || 'https://api.revolution-trading-pros.pages.dev';
	const roomSlug = params.room_slug as string;
	const dateSlug = params.date_slug as string;

	// Get room config
	const roomConfig = ROOM_CONFIG[roomSlug] || {
		name: roomSlug.split('-').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join(' '),
		archiveUrl: `/dashboard/${roomSlug}/trading-room-archive`
	};

	// Parse the date
	const archiveDate = parseDateSlug(dateSlug);
	if (!archiveDate) {
		throw error(404, 'Invalid date format');
	}

	const displayDate = formatDisplayDate(archiveDate);

	// Calculate previous and next dates
	const prevDate = new Date(archiveDate);
	prevDate.setDate(prevDate.getDate() - 1);
	const nextDate = new Date(archiveDate);
	nextDate.setDate(nextDate.getDate() + 1);

	// Format date for API query (YYYY-MM-DD)
	const apiDateStr = archiveDate.toISOString().split('T')[0];

	try {
		// Fetch videos for this specific date
		const apiParams = new URLSearchParams({
			content_type: 'room_archive',
			room_slug: roomSlug,
			video_date: apiDateStr,
			per_page: '50',
		});

		const response = await fetch(`${API_URL}/api/videos?${apiParams.toString()}`);

		if (!response.ok) {
			console.error('Failed to fetch archive videos:', response.status);
			return {
				roomSlug,
				roomName: roomConfig.name,
				archiveUrl: roomConfig.archiveUrl,
				dateSlug,
				displayDate,
				videos: [],
				chatLogUrl: null,
				previousDate: dateToSlug(prevDate),
				nextDate: dateToSlug(nextDate),
				error: 'Failed to load archive videos'
			};
		}

		const data: ApiResponse = await response.json();

		return {
			roomSlug,
			roomName: roomConfig.name,
			archiveUrl: roomConfig.archiveUrl,
			dateSlug,
			displayDate,
			videos: data.data || [],
			chatLogUrl: null, // TODO: Add chat log URL from API when available
			previousDate: dateToSlug(prevDate),
			nextDate: dateToSlug(nextDate),
			error: null
		};
	} catch (err) {
		console.error('Error fetching archive videos:', err);
		return {
			roomSlug,
			roomName: roomConfig.name,
			archiveUrl: roomConfig.archiveUrl,
			dateSlug,
			displayDate,
			videos: [],
			chatLogUrl: null,
			previousDate: dateToSlug(prevDate),
			nextDate: dateToSlug(nextDate),
			error: 'Failed to load archive videos'
		};
	}
};

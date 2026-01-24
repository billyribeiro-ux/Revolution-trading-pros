/**
 * Weekly Video API - Get current weekly video for a room
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/weekly-video/[room-slug] - Get current weekly video
 * POST /api/weekly-video/[room-slug] - Publish new weekly video (admin)
 *
 * Connects to backend at /api/room-content/rooms/:slug/weekly-video
 *
 * @version 1.0.0 - ICT 11 Principal Engineer Grade
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

// ═══════════════════════════════════════════════════════════════════════════
// BACKEND CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════

const BACKEND_URL = env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

interface WeeklyVideo {
	id: number;
	room_id: number;
	room_slug: string;
	week_of: string;
	week_title: string;
	video_title: string;
	video_url: string;
	video_platform: string | null;
	thumbnail_url: string | null;
	duration: string | null;
	description: string | null;
	is_current: boolean;
	is_published: boolean;
	published_at: string;
	created_at: string;
	updated_at: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// BACKEND FETCH HELPER
// ═══════════════════════════════════════════════════════════════════════════

async function fetchFromBackend(endpoint: string, options: RequestInit = {}): Promise<any | null> {
	try {
		console.log(`[Weekly Video API] Fetching: ${BACKEND_URL}${endpoint}`);
		const response = await fetch(`${BACKEND_URL}${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...options.headers
			}
		});

		if (!response.ok) {
			console.error(`[Weekly Video API] Backend error: ${response.status} ${response.statusText}`);
			return null;
		}

		const data = await response.json();
		console.log(`[Weekly Video API] Backend success`);
		return data;
	} catch (err) {
		console.error('[Weekly Video API] Backend fetch failed:', err);
		return null;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// ═══════════════════════════════════════════════════════════════════════════
// FALLBACK MOCK DATA - ICT 7 Standards
// ═══════════════════════════════════════════════════════════════════════════
// Purpose: Provide graceful degradation when backend is unavailable
// Behavior: Returns null data to trigger frontend "no video" state
// Security: Empty video_url prevents CSP violations from invalid URLs
// ═══════════════════════════════════════════════════════════════════════════

const mockWeeklyVideos: Record<string, WeeklyVideo | null> = {
	'explosive-swings': null, // No mock data - force frontend to show "no video" state
	'spx-profit-pulse': null,
	'weekly-watchlist': null
};

// ═══════════════════════════════════════════════════════════════════════════
// GET - Get current weekly video
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, request, cookies }) => {
	const { slug } = params;

	if (!slug) {
		throw error(400, 'Room slug is required');
	}

	// Get auth headers
	const authHeader = request.headers.get('Authorization');
	const sessionCookie = cookies.get('session');
	const headers: Record<string, string> = {};

	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (sessionCookie) {
		headers['Cookie'] = `session=${sessionCookie}`;
	}

	// Call backend at /api/room-content/rooms/:slug/weekly-video
	const backendData = await fetchFromBackend(`/api/room-content/rooms/${slug}/weekly-video`, {
		headers
	});

	if (backendData) {
		return json({
			success: true,
			data: backendData.data || backendData,
			_source: 'backend'
		});
	}

	// Fallback to mock data
	const video = mockWeeklyVideos[slug] || null;

	return json({
		success: true,
		data: video,
		_source: 'mock'
	});
};

// ═══════════════════════════════════════════════════════════════════════════
// POST - Publish new weekly video (archives previous)
// ═══════════════════════════════════════════════════════════════════════════

export const POST: RequestHandler = async ({ params, request, cookies }) => {
	const { slug } = params;
	const authHeader = request.headers.get('Authorization');
	const sessionCookie = cookies.get('session');

	if (!authHeader && !sessionCookie) {
		throw error(401, 'Authentication required');
	}

	if (!slug) {
		throw error(400, 'Room slug is required');
	}

	const body = await request.json();

	// Validate required fields
	if (!body.video_url || !body.video_title) {
		throw error(400, 'Video URL and title are required');
	}

	// Build headers
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (sessionCookie) {
		headers['Cookie'] = `session=${sessionCookie}`;
	}

	// Call backend at /api/admin/room-content/weekly-video
	const backendData = await fetchFromBackend(`/api/admin/room-content/weekly-video`, {
		method: 'POST',
		headers,
		body: JSON.stringify({
			room_slug: slug,
			week_of: body.week_of || new Date().toISOString().split('T')[0],
			week_title: body.week_title || `Week of ${new Date().toLocaleDateString('en-US', { month: 'long', day: 'numeric', year: 'numeric' })}`,
			video_title: body.video_title,
			video_url: body.video_url,
			video_platform: body.video_platform || 'bunny',
			thumbnail_url: body.thumbnail_url || '',
			duration: body.duration || '',
			description: body.description || ''
		})
	});

	if (backendData) {
		return json({
			success: true,
			data: backendData,
			message: 'Weekly video published successfully',
			_source: 'backend'
		});
	}

	// Fallback to mock create
	const newVideo: WeeklyVideo = {
		id: Date.now(),
		room_id: 4,
		room_slug: slug,
		week_of: body.week_of || new Date().toISOString().split('T')[0],
		week_title: body.week_title || `Week of ${new Date().toLocaleDateString('en-US', { month: 'long', day: 'numeric', year: 'numeric' })}`,
		video_title: body.video_title,
		video_url: body.video_url,
		video_platform: body.video_platform || 'bunny',
		thumbnail_url: body.thumbnail_url || null,
		duration: body.duration || null,
		description: body.description || null,
		is_current: true,
		is_published: true,
		published_at: new Date().toISOString(),
		created_at: new Date().toISOString(),
		updated_at: new Date().toISOString()
	};

	mockWeeklyVideos[slug] = newVideo;

	return json({
		success: true,
		data: newVideo,
		message: 'Weekly video published successfully',
		_source: 'mock'
	});
};

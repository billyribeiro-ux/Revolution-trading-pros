/**
 * Dashboard Article by ID API Endpoint
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Returns a single article (daily video or chatroom archive) by ID.
 *
 * @version 1.0.0 - December 2025
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

interface Article {
	id: number;
	type: 'daily-video' | 'chatroom-archive';
	label?: string;
	title: string;
	date: string;
	excerpt?: string;
	traderName?: string;
	href: string;
	image: string;
	videoUrl?: string;
	videoPlatform?: string;
	videoId?: string;
	duration?: number;
	isVideo?: boolean;
	content?: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// MOCK DATA
// ═══════════════════════════════════════════════════════════════════════════

const mockArticles: Record<number, Article> = {
	1: {
		id: 1,
		type: 'daily-video',
		label: 'Daily Video',
		title: "Santa's On His Way",
		date: 'December 23, 2025 with HG',
		excerpt: "Things can always change, but given how the market closed, it looks like Santa's on his way.",
		href: '/dashboard/mastering-the-trade/daily-videos/12232025',
		image: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
		videoUrl: 'https://player.vimeo.com/video/123456789',
		videoPlatform: 'vimeo',
		videoId: '123456789',
		duration: 1800,
		isVideo: true,
		content: "Things can always change, but given how the market closed on Tuesday, it looks like Santa's on his way. Let's look at the facts, then also some preferences and opinions as we get into the end of 2025."
	},
	2: {
		id: 2,
		type: 'chatroom-archive',
		title: 'December 23, 2025',
		date: 'December 23, 2025',
		traderName: 'Danielle Shay',
		href: '/dashboard/mastering-the-trade/chatroom-archives/12232025',
		image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
		videoUrl: 'https://player.vimeo.com/video/123456790',
		videoPlatform: 'vimeo',
		videoId: '123456790',
		duration: 14400,
		isVideo: true
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// BACKEND FETCH
// ═══════════════════════════════════════════════════════════════════════════

async function fetchFromBackend(endpoint: string, authHeader?: string): Promise<any | null> {
	const BACKEND_URL = env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

	try {
		const headers: Record<string, string> = {
			'Content-Type': 'application/json',
			'Accept': 'application/json'
		};

		if (authHeader) {
			headers['Authorization'] = authHeader;
		}

		const response = await fetch(`${BACKEND_URL}${endpoint}`, { headers });

		if (!response.ok) return null;
		return await response.json();
	} catch {
		return null;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// GET HANDLER
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, request }) => {
	const id = parseInt(params.id || '0');

	if (!id || isNaN(id)) {
		throw error(400, 'Valid article ID is required');
	}

	// Try backend first
	const authHeader = request.headers.get('Authorization') || undefined;
	const backendData = await fetchFromBackend(`/api/dashboard/articles/item/${id}`, authHeader);

	if (backendData?.success) {
		return json(backendData);
	}

	// Fallback to mock data
	const article = mockArticles[id];

	if (!article) {
		throw error(404, `Article with ID ${id} not found`);
	}

	return json({
		success: true,
		data: article,
		_mock: true
	});
};

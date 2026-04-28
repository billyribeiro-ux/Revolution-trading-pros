/**
 * Course Player API - Proxy to Backend
 * ═══════════════════════════════════════════════════════════════════════════
 * Apple ICT 11+ Principal Engineer Implementation - January 2026
 *
 * Proxies requests from frontend to Rust backend API
 * Endpoint: GET /api/courses/slug/:slug/player
 * Backend:  GET /api/my/courses/:slug/player
 *
 * Returns: Course + Modules + Lessons (with bunny_video_guid and thumbnail_url)
 */

import { json, type RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

// FIX-2026-04-26: env.API_URL → canonical pattern
// const API_URL = env.API_URL || 'http://localhost:8080';
const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

export const GET: RequestHandler = async ({ params, cookies, fetch }) => {
	const { slug } = params;

	if (!slug) {
		return json({ success: false, error: 'Course slug is required' }, { status: 400 });
	}

	try {
		// Get auth token from cookies if available
		// FIX-2026-04-26: comment-out, verify, delete in follow-up. Wrong cookie name — login proxy sets rtp_access_token, not access_token.
		// const accessToken = cookies.get('access_token');
		const accessToken = cookies.get('rtp_access_token');

		const headers: Record<string, string> = {
			'Content-Type': 'application/json'
		};

		if (accessToken) {
			headers['Authorization'] = `Bearer ${accessToken}`;
		}

		// Proxy to backend API - member endpoint for course player
		const response = await fetch(`${API_URL}/api/my/courses/${slug}/player`, {
			method: 'GET',
			headers
		});

		if (!response.ok) {
			console.warn(`[API Proxy] Backend returned ${response.status} for course player: ${slug}`);

			// Return structured empty response
			return json(
				{
					success: false,
					error: 'Course not found or not accessible',
					data: null
				},
				{ status: response.status }
			);
		}

		const data = await response.json();
		return json(data);
	} catch (error) {
		console.error('[API Proxy] Error fetching course player:', error);

		return json(
			{
				success: false,
				error: 'Failed to load course data',
				data: null
			},
			{ status: 500 }
		);
	}
};

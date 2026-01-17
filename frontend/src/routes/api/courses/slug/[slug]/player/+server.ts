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

const API_URL = env.API_URL || 'https://revolution-trading-pros-api.fly.dev';

export const GET: RequestHandler = async ({ params, cookies, fetch }) => {
	const { slug } = params;

	if (!slug) {
		return json({ success: false, error: 'Course slug is required' }, { status: 400 });
	}

	try {
		// Get auth token from cookies if available
		const accessToken = cookies.get('access_token');

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

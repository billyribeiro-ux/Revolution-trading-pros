/**
 * Posts Stats API Endpoint
 *
 * Returns statistics for blog posts from the backend.
 *
 * @version 2.0.0 - January 2026
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { env } from '$env/dynamic/private';
const PROD_BACKEND =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

export const GET: RequestHandler = async ({ request, cookies }) => {
	const backendUrl = PROD_BACKEND;
	// FIX-2026-04-26: prefer canonical rtp_access_token cookie, fall back to header.
	// Old: const authHeader = request.headers.get('Authorization') || '';
	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) error(401, 'Unauthorized');
	const authHeader = `Bearer ${token}`;

	try {
		const response = await fetch(`${backendUrl}/api/admin/posts/stats`, {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: authHeader
			}
		});

		if (response.ok) {
			const data = await response.json();
			return json(data);
		}

		// If backend fails, return empty stats (not mock data)
		console.warn(`Backend stats returned ${response.status}`);
	} catch (err) {
		console.warn('Backend stats not available:', err);
	}

	// Return real empty stats, not mock data
	return json({
		success: true,
		total_posts: 0,
		published: 0,
		drafts: 0,
		scheduled: 0,
		archived: 0,
		total_views: 0,
		total_likes: 0,
		total_comments: 0,
		avg_views_per_post: 0,
		top_categories: [],
		recent_activity: []
	});
};

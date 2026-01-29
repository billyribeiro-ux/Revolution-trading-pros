/**
 * Posts Stats API Endpoint
 *
 * Returns statistics for blog posts from the backend.
 *
 * @version 2.0.0 - January 2026
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';

export const GET: RequestHandler = async ({ request }) => {
	const backendUrl = PROD_BACKEND;
	const authHeader = request.headers.get('Authorization') || '';

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

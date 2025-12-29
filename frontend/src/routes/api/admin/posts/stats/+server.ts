/**
 * Posts Stats API Endpoint
 *
 * Returns statistics for blog posts.
 *
 * @version 1.0.0 - December 2025
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async () => {
	return json({
		success: true,
		total_posts: 5,
		published: 3,
		drafts: 1,
		scheduled: 1,
		archived: 0,
		total_views: 4290,
		total_likes: 164,
		total_comments: 44,
		avg_views_per_post: 858,
		top_categories: [
			{ name: 'Psychology', count: 1, views: 2150 },
			{ name: 'Options Trading', count: 1, views: 1250 },
			{ name: 'Risk Management', count: 1, views: 890 }
		],
		recent_activity: [
			{ type: 'published', title: 'Getting Started with Options Trading', date: '2025-12-01' },
			{ type: 'scheduled', title: 'Weekly Market Analysis: December 2025', date: '2025-12-10' },
			{ type: 'draft', title: 'Technical Analysis 101: Chart Patterns', date: '2025-12-05' }
		]
	});
};

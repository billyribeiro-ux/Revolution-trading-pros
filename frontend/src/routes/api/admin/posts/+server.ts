/**
 * Posts API Endpoint
 *
 * Handles blog post management including listing, creating, updating, and deleting posts.
 * Returns mock data when backend is not connected.
 *
 * @version 1.0.0 - December 2025
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

// Mock data for posts
const mockPosts = [
	{
		id: 1,
		title: 'Getting Started with Options Trading',
		slug: 'getting-started-options-trading',
		excerpt: 'Learn the fundamentals of options trading and how to get started with your first trade.',
		content: '<p>Options trading can be a powerful tool in your investing arsenal...</p>',
		status: 'published',
		category_id: 1,
		category: { id: 1, name: 'Options Trading', slug: 'options-trading', color: '#f59e0b' },
		tags: ['options', 'beginner', 'education'],
		author: { id: 1, name: 'Admin', avatar: null },
		featured_image: null,
		view_count: 1250,
		like_count: 45,
		comment_count: 12,
		seo_title: 'Getting Started with Options Trading | Revolution Trading',
		seo_description: 'Learn the fundamentals of options trading.',
		is_featured: true,
		published_at: '2025-12-01T10:00:00Z',
		created_at: '2025-11-28T14:30:00Z',
		updated_at: '2025-12-01T10:00:00Z'
	},
	{
		id: 2,
		title: 'Risk Management Strategies for Day Traders',
		slug: 'risk-management-day-traders',
		excerpt: 'Protect your capital with proven risk management techniques used by professional traders.',
		content: '<p>Effective risk management is the cornerstone of successful trading...</p>',
		status: 'published',
		category_id: 2,
		category: { id: 2, name: 'Risk Management', slug: 'risk-management', color: '#ef4444' },
		tags: ['risk', 'day-trading', 'strategy'],
		author: { id: 1, name: 'Admin', avatar: null },
		featured_image: null,
		view_count: 890,
		like_count: 32,
		comment_count: 8,
		seo_title: 'Risk Management for Day Traders | Revolution Trading',
		seo_description: 'Protect your trading capital with proven risk management strategies.',
		is_featured: false,
		published_at: '2025-11-28T09:00:00Z',
		created_at: '2025-11-25T11:00:00Z',
		updated_at: '2025-11-28T09:00:00Z'
	},
	{
		id: 3,
		title: 'Technical Analysis 101: Chart Patterns',
		slug: 'technical-analysis-chart-patterns',
		excerpt: 'Master the most important chart patterns every trader needs to know.',
		content: '<p>Technical analysis involves analyzing price charts to predict future movements...</p>',
		status: 'draft',
		category_id: 3,
		category: { id: 3, name: 'Technical Analysis', slug: 'technical-analysis', color: '#6366f1' },
		tags: ['technical', 'charts', 'patterns'],
		author: { id: 1, name: 'Admin', avatar: null },
		featured_image: null,
		view_count: 0,
		like_count: 0,
		comment_count: 0,
		seo_title: 'Technical Analysis Chart Patterns | Revolution Trading',
		seo_description: 'Learn essential chart patterns for technical analysis.',
		is_featured: false,
		published_at: null,
		created_at: '2025-12-05T16:00:00Z',
		updated_at: '2025-12-05T16:00:00Z'
	},
	{
		id: 4,
		title: 'Weekly Market Analysis: December 2025',
		slug: 'weekly-market-analysis-december-2025',
		excerpt: 'Our comprehensive analysis of market trends and opportunities for the week ahead.',
		content: '<p>This week we are looking at several key market developments...</p>',
		status: 'scheduled',
		category_id: 4,
		category: { id: 4, name: 'Market Analysis', slug: 'market-analysis', color: '#3b82f6' },
		tags: ['market', 'weekly', 'analysis'],
		author: { id: 1, name: 'Admin', avatar: null },
		featured_image: null,
		view_count: 0,
		like_count: 0,
		comment_count: 0,
		seo_title: 'Weekly Market Analysis December 2025 | Revolution Trading',
		seo_description: 'Weekly market analysis and trading opportunities.',
		is_featured: true,
		published_at: '2025-12-15T08:00:00Z',
		created_at: '2025-12-10T14:00:00Z',
		updated_at: '2025-12-10T14:00:00Z'
	},
	{
		id: 5,
		title: 'Psychology of Trading: Managing Emotions',
		slug: 'psychology-trading-emotions',
		excerpt: 'How to keep your emotions in check and trade with discipline.',
		content: '<p>Trading psychology is often the difference between success and failure...</p>',
		status: 'published',
		category_id: 5,
		category: { id: 5, name: 'Psychology', slug: 'psychology', color: '#8b5cf6' },
		tags: ['psychology', 'emotions', 'discipline'],
		author: { id: 1, name: 'Admin', avatar: null },
		featured_image: null,
		view_count: 2150,
		like_count: 87,
		comment_count: 24,
		seo_title: 'Trading Psychology: Managing Emotions | Revolution Trading',
		seo_description: 'Master your trading psychology and manage emotions effectively.',
		is_featured: true,
		published_at: '2025-11-20T10:00:00Z',
		created_at: '2025-11-18T09:00:00Z',
		updated_at: '2025-11-20T10:00:00Z'
	}
];

// GET - List posts with filtering and pagination
export const GET: RequestHandler = async ({ url }) => {
	const status = url.searchParams.get('status');
	const category = url.searchParams.get('category');
	const search = url.searchParams.get('search');
	const sort = url.searchParams.get('sort') || 'created_at';
	const order = url.searchParams.get('order') || 'desc';
	const page = parseInt(url.searchParams.get('page') || '1');
	const perPage = parseInt(url.searchParams.get('per_page') || '20');

	let filteredPosts = [...mockPosts];

	// Apply filters
	if (status && status !== 'all') {
		filteredPosts = filteredPosts.filter(p => p.status === status);
	}

	if (category && category !== 'all') {
		filteredPosts = filteredPosts.filter(p =>
			p.category?.slug === category || p.category_id?.toString() === category
		);
	}

	if (search) {
		const searchLower = search.toLowerCase();
		filteredPosts = filteredPosts.filter(p =>
			p.title.toLowerCase().includes(searchLower) ||
			p.excerpt.toLowerCase().includes(searchLower) ||
			p.tags?.some(t => t.toLowerCase().includes(searchLower))
		);
	}

	// Apply sorting
	filteredPosts.sort((a, b) => {
		let aVal: any, bVal: any;

		switch (sort) {
			case 'title':
				aVal = a.title.toLowerCase();
				bVal = b.title.toLowerCase();
				break;
			case 'view_count':
				aVal = a.view_count;
				bVal = b.view_count;
				break;
			case 'published_at':
				aVal = a.published_at ? new Date(a.published_at).getTime() : 0;
				bVal = b.published_at ? new Date(b.published_at).getTime() : 0;
				break;
			case 'created_at':
			default:
				aVal = new Date(a.created_at).getTime();
				bVal = new Date(b.created_at).getTime();
		}

		if (order === 'asc') {
			return aVal > bVal ? 1 : -1;
		}
		return aVal < bVal ? 1 : -1;
	});

	// Paginate
	const total = filteredPosts.length;
	const start = (page - 1) * perPage;
	const paginatedPosts = filteredPosts.slice(start, start + perPage);

	return json({
		success: true,
		data: paginatedPosts,
		meta: {
			current_page: page,
			per_page: perPage,
			total,
			last_page: Math.ceil(total / perPage)
		}
	});
};

// POST - Create new post
export const POST: RequestHandler = async ({ request }) => {
	try {
		const body = await request.json();

		const newPost = {
			id: mockPosts.length + 1,
			...body,
			view_count: 0,
			like_count: 0,
			comment_count: 0,
			created_at: new Date().toISOString(),
			updated_at: new Date().toISOString()
		};

		mockPosts.push(newPost);

		return json({
			success: true,
			data: newPost,
			message: 'Post created successfully'
		});
	} catch (err) {
		throw error(400, 'Invalid request body');
	}
};

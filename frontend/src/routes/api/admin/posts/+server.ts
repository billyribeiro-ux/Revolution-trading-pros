/**
 * Posts API Endpoint
 *
 * Handles blog post management including listing, creating, updating, and deleting posts.
 * Connects to Rust API backend with fallback to mock data for development.
 *
 * @version 2.0.0 - December 2025
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

// Production fallback - Rust API on Fly.io
const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';

// Try to fetch from backend
async function fetchFromBackend(endpoint: string, options?: RequestInit): Promise<any | null> {
	const backendUrl = env.BACKEND_URL || PROD_BACKEND;

	try {
		const response = await fetch(`${backendUrl}/api${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				'Accept': 'application/json',
				...(options?.headers || {})
			}
		});

		if (!response.ok) {
			console.warn(`Backend returned ${response.status} for ${endpoint}`);
			return null;
		}
		return await response.json();
	} catch (err) {
		console.warn(`Backend not available for ${endpoint}:`, err);
		return null;
	}
}

// Mock data for development fallback
const mockPosts = [
	{
		id: 1,
		title: 'Getting Started with Options Trading',
		slug: 'getting-started-options-trading',
		excerpt: 'Learn the fundamentals of options trading and how to get started with your first trade.',
		content: '<p>Options trading can be a powerful tool in your investing arsenal...</p>',
		blocks: [],
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
		blocks: [],
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
		blocks: [],
		status: 'draft',
		category_id: 3,
		category: { id: 3, name: 'Technical Analysis', slug: 'technical-analysis', color: '#E6B800' },
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
	}
];

// GET - List posts with filtering and pagination
export const GET: RequestHandler = async ({ url, request }) => {
	const authHeader = request.headers.get('Authorization') || '';

	// Build query string
	const queryParams = url.searchParams.toString();
	const endpoint = `/admin/posts${queryParams ? `?${queryParams}` : ''}`;

	// Try backend first
	const backendData = await fetchFromBackend(endpoint, {
		headers: { Authorization: authHeader }
	});

	if (backendData?.data) {
		return json(backendData);
	}

	// Fallback to mock data
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
		},
		_mock: true,
		_message: 'Using mock data. Connect backend for real data.'
	});
};

// POST - Create new post
export const POST: RequestHandler = async ({ request }) => {
	const authHeader = request.headers.get('Authorization') || '';

	try {
		const body = await request.json();

		// Try backend first
		const backendData = await fetchFromBackend('/admin/posts', {
			method: 'POST',
			headers: { Authorization: authHeader },
			body: JSON.stringify(body)
		});

		if (backendData?.data) {
			return json(backendData);
		}

		// Fallback: create in mock
		const newPost = {
			id: Date.now(),
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
			message: 'Post created successfully (mock)',
			_mock: true
		});
	} catch (err) {
		console.error('POST /api/admin/posts error:', err);
		throw error(400, 'Invalid request body');
	}
};

// PUT - Update post
export const PUT: RequestHandler = async ({ request, url }) => {
	const authHeader = request.headers.get('Authorization') || '';
	const postId = url.searchParams.get('id');

	if (!postId) {
		throw error(400, 'Post ID required');
	}

	try {
		const body = await request.json();

		// Try backend first
		const backendData = await fetchFromBackend(`/admin/posts/${postId}`, {
			method: 'PUT',
			headers: { Authorization: authHeader },
			body: JSON.stringify(body)
		});

		if (backendData?.data) {
			return json(backendData);
		}

		// Fallback: update in mock
		const index = mockPosts.findIndex(p => p.id === parseInt(postId));
		if (index === -1) {
			throw error(404, 'Post not found');
		}

		mockPosts[index] = {
			...mockPosts[index],
			...body,
			updated_at: new Date().toISOString()
		};

		return json({
			success: true,
			data: mockPosts[index],
			message: 'Post updated successfully (mock)',
			_mock: true
		});
	} catch (err) {
		console.error('PUT /api/admin/posts error:', err);
		throw error(400, 'Invalid request body');
	}
};

// DELETE - Delete post
export const DELETE: RequestHandler = async ({ url, request }) => {
	const authHeader = request.headers.get('Authorization') || '';
	const postId = url.searchParams.get('id');

	if (!postId) {
		throw error(400, 'Post ID required');
	}

	// Try backend first
	const backendData = await fetchFromBackend(`/admin/posts/${postId}`, {
		method: 'DELETE',
		headers: { Authorization: authHeader }
	});

	if (backendData?.success !== undefined) {
		return json(backendData);
	}

	// Fallback: delete from mock
	const index = mockPosts.findIndex(p => p.id === parseInt(postId));
	if (index === -1) {
		throw error(404, 'Post not found');
	}

	mockPosts.splice(index, 1);

	return json({
		success: true,
		message: 'Post deleted successfully (mock)',
		_mock: true
	});
};

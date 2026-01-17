/**
 * Categories API Endpoint
 *
 * Handles blog category management.
 * Returns mock data when backend is not connected.
 *
 * @version 1.0.0 - December 2025
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

interface MockCategory {
	id: number;
	name: string;
	slug: string;
	description: string;
	color: string;
	icon: string;
	post_count: number;
	is_featured: boolean;
	sort_order: number;
	parent_id: number | null;
	created_at: string;
	updated_at: string;
}

// Mock categories matching the predefined list in the blog admin
const mockCategories: MockCategory[] = [
	{
		id: 1,
		name: 'Market Analysis',
		slug: 'market-analysis',
		description: 'In-depth market analysis and trading opportunities',
		color: '#3b82f6',
		icon: 'chart-line',
		post_count: 12,
		is_featured: true,
		sort_order: 1,
		parent_id: null,
		created_at: '2025-01-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 2,
		name: 'Trading Strategies',
		slug: 'trading-strategies',
		description: 'Proven trading strategies for all market conditions',
		color: '#10b981',
		icon: 'strategy',
		post_count: 8,
		is_featured: true,
		sort_order: 2,
		parent_id: null,
		created_at: '2025-01-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 3,
		name: 'Risk Management',
		slug: 'risk-management',
		description: 'Protect your capital with proper risk management',
		color: '#ef4444',
		icon: 'shield',
		post_count: 6,
		is_featured: true,
		sort_order: 3,
		parent_id: null,
		created_at: '2025-01-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 4,
		name: 'Options Trading',
		slug: 'options-trading',
		description: 'Options strategies and education',
		color: '#f59e0b',
		icon: 'options',
		post_count: 15,
		is_featured: true,
		sort_order: 4,
		parent_id: null,
		created_at: '2025-01-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 5,
		name: 'Technical Analysis',
		slug: 'technical-analysis',
		description: 'Chart patterns, indicators, and technical trading',
		color: '#E6B800',
		icon: 'chart',
		post_count: 10,
		is_featured: false,
		sort_order: 5,
		parent_id: null,
		created_at: '2025-01-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 6,
		name: 'Fundamental Analysis',
		slug: 'fundamental-analysis',
		description: 'Analyzing company fundamentals and valuations',
		color: '#ec4899',
		icon: 'document',
		post_count: 4,
		is_featured: false,
		sort_order: 6,
		parent_id: null,
		created_at: '2025-01-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 7,
		name: 'Psychology',
		slug: 'psychology',
		description: 'Trading psychology and mindset mastery',
		color: '#B38F00',
		icon: 'brain',
		post_count: 7,
		is_featured: true,
		sort_order: 7,
		parent_id: null,
		created_at: '2025-01-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 8,
		name: 'Education',
		slug: 'education',
		description: 'Trading education and learning resources',
		color: '#14b8a6',
		icon: 'book',
		post_count: 20,
		is_featured: true,
		sort_order: 8,
		parent_id: null,
		created_at: '2025-01-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 9,
		name: 'News & Updates',
		slug: 'news',
		description: 'Latest news and platform updates',
		color: '#06b6d4',
		icon: 'news',
		post_count: 25,
		is_featured: false,
		sort_order: 9,
		parent_id: null,
		created_at: '2025-01-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 10,
		name: 'Day Trading',
		slug: 'day-trading',
		description: 'Day trading strategies and tips',
		color: '#d946ef',
		icon: 'clock',
		post_count: 11,
		is_featured: false,
		sort_order: 10,
		parent_id: null,
		created_at: '2025-01-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 11,
		name: 'Swing Trading',
		slug: 'swing-trading',
		description: 'Swing trading setups and analysis',
		color: '#64748b',
		icon: 'trending',
		post_count: 9,
		is_featured: false,
		sort_order: 11,
		parent_id: null,
		created_at: '2025-01-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 12,
		name: 'Small Accounts',
		slug: 'small-accounts',
		description: 'Growing small trading accounts',
		color: '#eab308',
		icon: 'growth',
		post_count: 5,
		is_featured: false,
		sort_order: 12,
		parent_id: null,
		created_at: '2025-01-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	}
];

// GET - List categories
export const GET: RequestHandler = async ({ url }) => {
	const featured = url.searchParams.get('featured');
	const parent_id = url.searchParams.get('parent_id');

	let filteredCategories = [...mockCategories];

	if (featured === 'true') {
		filteredCategories = filteredCategories.filter((c) => c.is_featured);
	}

	if (parent_id) {
		filteredCategories = filteredCategories.filter((c) =>
			parent_id === 'null' ? c.parent_id === null : c.parent_id?.toString() === parent_id
		);
	}

	// Sort by sort_order
	filteredCategories.sort((a, b) => a.sort_order - b.sort_order);

	return json({
		success: true,
		data: filteredCategories,
		meta: {
			total: filteredCategories.length
		}
	});
};

// POST - Create new category
export const POST: RequestHandler = async ({ request }) => {
	try {
		const body = await request.json();

		const newCategory = {
			id: mockCategories.length + 1,
			...body,
			slug: body.slug || body.name.toLowerCase().replace(/\s+/g, '-'),
			post_count: 0,
			sort_order: mockCategories.length + 1,
			created_at: new Date().toISOString(),
			updated_at: new Date().toISOString()
		};

		mockCategories.push(newCategory);

		return json({
			success: true,
			data: newCategory,
			message: 'Category created successfully'
		});
	} catch (err) {
		throw error(400, 'Invalid request body');
	}
};

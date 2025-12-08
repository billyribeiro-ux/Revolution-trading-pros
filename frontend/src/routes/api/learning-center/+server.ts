/**
 * Learning Center API - Content Management
 *
 * Manages learning center content including:
 * - Video lessons
 * - Categories and topics
 * - Progress tracking
 * - Search and filtering
 *
 * @version 1.0.0 - December 2025
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

// Learning Center Content Types
interface LearningContent {
	id: string;
	title: string;
	slug: string;
	type: 'video' | 'article' | 'course' | 'webinar';
	description: string;
	excerpt: string;
	thumbnail_url: string;
	content_url: string;
	duration?: number; // minutes for videos
	read_time?: number; // minutes for articles
	category_id: string;
	categories: Category[];
	tags: string[];
	instructor: Instructor;
	membership_ids: string[]; // Which memberships have access
	is_premium: boolean;
	is_published: boolean;
	is_featured: boolean;
	sort_order: number;
	view_count: number;
	like_count: number;
	created_at: string;
	updated_at: string;
	published_at?: string;
}

interface Category {
	id: string;
	name: string;
	slug: string;
	color?: string;
}

interface Instructor {
	id: string;
	name: string;
	title?: string;
	avatar_url?: string;
	bio?: string;
}

// In-memory storage
const learningContent: Map<string, LearningContent> = new Map();
const categories: Map<string, Category> = new Map();
const instructors: Map<string, Instructor> = new Map();

// Initialize sample data
function initializeSampleData() {
	if (learningContent.size > 0) return;

	// Categories
	const sampleCategories: Category[] = [
		{ id: 'cat_1', name: 'Member Webinar', slug: 'member-webinar', color: '#3B82F6' },
		{ id: 'cat_2', name: 'Trade Setups & Strategies', slug: 'trade-setups-strategies', color: '#10B981' },
		{ id: 'cat_3', name: 'Trade & Money Management/Trading Plan', slug: 'trade-money-management', color: '#F59E0B' },
		{ id: 'cat_4', name: 'Technical Analysis', slug: 'technical-analysis', color: '#8B5CF6' },
		{ id: 'cat_5', name: 'Options Trading', slug: 'options-trading', color: '#EF4444' },
		{ id: 'cat_6', name: 'Futures Trading', slug: 'futures-trading', color: '#EC4899' }
	];
	sampleCategories.forEach(c => categories.set(c.id, c));

	// Instructors
	const sampleInstructors: Instructor[] = [
		{ id: 'inst_1', name: 'John Carter', title: 'Founder & CEO', avatar_url: '/images/instructors/john-carter.jpg' },
		{ id: 'inst_2', name: 'Chris Brecher', title: 'Lead Trader', avatar_url: '/images/instructors/chris-brecher.jpg' },
		{ id: 'inst_3', name: 'Taylor Horton', title: 'Options Specialist', avatar_url: '/images/instructors/taylor-horton.jpg' },
		{ id: 'inst_4', name: 'Sam Shames', title: 'Macro Analyst', avatar_url: '/images/instructors/sam-shames.jpg' },
		{ id: 'inst_5', name: 'Simpler Trading Team', title: '', avatar_url: '/images/instructors/team.jpg' }
	];
	sampleInstructors.forEach(i => instructors.set(i.id, i));

	// Sample content
	const sampleContent: LearningContent[] = [
		{
			id: 'lc_1',
			title: 'Mastering the Trade Room FAQs',
			slug: 'mastering-the-trade-room-faqs',
			type: 'webinar',
			description: 'Comprehensive FAQ session covering all aspects of the trading room experience.',
			excerpt: 'Mastering the Trade Room FAQs',
			thumbnail_url: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
			content_url: '',
			duration: 60,
			category_id: 'cat_1',
			categories: [{ id: 'cat_1', name: 'Member Webinar', slug: 'member-webinar', color: '#3B82F6' }],
			tags: ['FAQ', 'Getting Started', 'Trading Room'],
			instructor: sampleInstructors[4],
			membership_ids: ['mastering-the-trade'],
			is_premium: true,
			is_published: true,
			is_featured: false,
			sort_order: 1,
			view_count: 1250,
			like_count: 89,
			created_at: '2024-01-15T10:00:00Z',
			updated_at: '2024-06-01T12:00:00Z',
			published_at: '2024-01-15T10:00:00Z'
		},
		{
			id: 'lc_2',
			title: 'How to Find 3 Reasons to do a Trade',
			slug: '3-reasons-to-do-a-trade',
			type: 'webinar',
			description: 'Learn Chris Brecher\'s methodology for identifying three compelling reasons before entering any trade.',
			excerpt: 'How to Find 3 Reasons to do a Trade',
			thumbnail_url: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			content_url: '',
			duration: 45,
			category_id: 'cat_1',
			categories: [
				{ id: 'cat_1', name: 'Member Webinar', slug: 'member-webinar', color: '#3B82F6' },
				{ id: 'cat_3', name: 'Trade & Money Management/Trading Plan', slug: 'trade-money-management', color: '#F59E0B' }
			],
			tags: ['Trade Setup', 'Risk Management', 'Entry Strategy'],
			instructor: sampleInstructors[1],
			membership_ids: ['mastering-the-trade'],
			is_premium: true,
			is_published: true,
			is_featured: true,
			sort_order: 2,
			view_count: 890,
			like_count: 156,
			created_at: '2024-02-20T14:00:00Z',
			updated_at: '2024-06-15T09:00:00Z',
			published_at: '2024-02-20T14:00:00Z'
		},
		{
			id: 'lc_3',
			title: 'Analysis of A Trade',
			slug: 'analysis-of-a-trade',
			type: 'webinar',
			description: 'Deep dive into trade analysis methodology with Taylor Horton.',
			excerpt: 'Analysis of A Trade',
			thumbnail_url: 'https://cdn.simplertrading.com/2021/06/23152021/MemberWebinar-TaylorH.jpg',
			content_url: '',
			duration: 70,
			category_id: 'cat_2',
			categories: [
				{ id: 'cat_1', name: 'Member Webinar', slug: 'member-webinar', color: '#3B82F6' },
				{ id: 'cat_2', name: 'Trade Setups & Strategies', slug: 'trade-setups-strategies', color: '#10B981' }
			],
			tags: ['Analysis', 'Strategy', 'Trade Review'],
			instructor: sampleInstructors[2],
			membership_ids: ['mastering-the-trade'],
			is_premium: true,
			is_published: true,
			is_featured: false,
			sort_order: 3,
			view_count: 1560,
			like_count: 234,
			created_at: '2024-03-10T11:00:00Z',
			updated_at: '2024-07-01T16:00:00Z',
			published_at: '2024-03-10T11:00:00Z'
		},
		{
			id: 'lc_4',
			title: 'How to Use the ATR Trailing Stop Intraday',
			slug: 'atr-trailing-stop-intraday',
			type: 'webinar',
			description: 'Learn how to effectively use ATR trailing stops for intraday trading.',
			excerpt: 'How to Use the ATR Trailing Stop Intraday',
			thumbnail_url: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			content_url: '',
			duration: 55,
			category_id: 'cat_3',
			categories: [
				{ id: 'cat_1', name: 'Member Webinar', slug: 'member-webinar', color: '#3B82F6' },
				{ id: 'cat_3', name: 'Trade & Money Management/Trading Plan', slug: 'trade-money-management', color: '#F59E0B' }
			],
			tags: ['ATR', 'Trailing Stop', 'Intraday', 'Risk Management'],
			instructor: sampleInstructors[1],
			membership_ids: ['mastering-the-trade'],
			is_premium: true,
			is_published: true,
			is_featured: false,
			sort_order: 4,
			view_count: 720,
			like_count: 98,
			created_at: '2024-04-05T09:00:00Z',
			updated_at: '2024-07-10T14:00:00Z',
			published_at: '2024-04-05T09:00:00Z'
		},
		{
			id: 'lc_5',
			title: 'John Carter Bigger Picture Outlook',
			slug: 'john-carter-bigger-picture-outlook',
			type: 'webinar',
			description: 'Market update and review of what to expect moving forward for the overall market, commodities, Treasury yields, bonds, and more.',
			excerpt: 'John Carter provides an urgent market update and reviews the first quarter of 2025.',
			thumbnail_url: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
			content_url: '',
			duration: 90,
			category_id: 'cat_1',
			categories: [
				{ id: 'cat_1', name: 'Member Webinar', slug: 'member-webinar', color: '#3B82F6' },
				{ id: 'cat_3', name: 'Trade & Money Management/Trading Plan', slug: 'trade-money-management', color: '#F59E0B' }
			],
			tags: ['Market Outlook', 'Macro', 'Commodities', 'Bonds'],
			instructor: sampleInstructors[0],
			membership_ids: ['mastering-the-trade'],
			is_premium: true,
			is_published: true,
			is_featured: true,
			sort_order: 5,
			view_count: 2340,
			like_count: 445,
			created_at: '2024-04-14T15:00:00Z',
			updated_at: '2024-07-20T10:00:00Z',
			published_at: '2024-04-14T15:00:00Z'
		},
		{
			id: 'lc_6',
			title: 'Macroeconomic Recap',
			slug: 'macroeconomic-recap',
			type: 'webinar',
			description: 'Comprehensive macroeconomic analysis and market insights.',
			excerpt: 'Macroeconomic Recap',
			thumbnail_url: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg',
			content_url: '',
			duration: 65,
			category_id: 'cat_3',
			categories: [
				{ id: 'cat_3', name: 'Trade & Money Management/Trading Plan', slug: 'trade-money-management', color: '#F59E0B' }
			],
			tags: ['Macro', 'Economics', 'Market Analysis'],
			instructor: sampleInstructors[3],
			membership_ids: ['mastering-the-trade'],
			is_premium: true,
			is_published: true,
			is_featured: false,
			sort_order: 6,
			view_count: 980,
			like_count: 167,
			created_at: '2024-05-01T13:00:00Z',
			updated_at: '2024-08-01T11:00:00Z',
			published_at: '2024-05-01T13:00:00Z'
		}
	];
	sampleContent.forEach(c => learningContent.set(c.id, c));
}

// GET - List learning center content
export const GET: RequestHandler = async ({ url }) => {
	initializeSampleData();

	const page = parseInt(url.searchParams.get('page') || '1');
	const limit = parseInt(url.searchParams.get('limit') || '12');
	const membership_id = url.searchParams.get('membership_id');
	const category_id = url.searchParams.get('category_id');
	const type = url.searchParams.get('type');
	const search = url.searchParams.get('search')?.toLowerCase();
	const instructor_id = url.searchParams.get('instructor_id');
	const featured = url.searchParams.get('featured');
	const published_only = url.searchParams.get('published') !== 'false';

	let filtered = Array.from(learningContent.values());

	// Filter by membership
	if (membership_id) {
		filtered = filtered.filter(c => c.membership_ids.includes(membership_id));
	}

	// Filter by category
	if (category_id) {
		filtered = filtered.filter(c => c.category_id === category_id || c.categories.some(cat => cat.id === category_id));
	}

	// Filter by type
	if (type) {
		filtered = filtered.filter(c => c.type === type);
	}

	// Filter by instructor
	if (instructor_id) {
		filtered = filtered.filter(c => c.instructor.id === instructor_id);
	}

	// Filter by featured
	if (featured === 'true') {
		filtered = filtered.filter(c => c.is_featured);
	}

	// Filter by published status
	if (published_only) {
		filtered = filtered.filter(c => c.is_published);
	}

	// Search
	if (search) {
		filtered = filtered.filter(c =>
			c.title.toLowerCase().includes(search) ||
			c.description.toLowerCase().includes(search) ||
			c.instructor.name.toLowerCase().includes(search) ||
			c.tags.some(t => t.toLowerCase().includes(search))
		);
	}

	// Sort by most recent
	filtered.sort((a, b) => {
		// Featured items first
		if (a.is_featured && !b.is_featured) return -1;
		if (!a.is_featured && b.is_featured) return 1;
		// Then by date
		return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
	});

	// Pagination
	const total = filtered.length;
	const start = (page - 1) * limit;
	const paginated = filtered.slice(start, start + limit);

	return json({
		success: true,
		data: {
			content: paginated,
			categories: Array.from(categories.values()),
			instructors: Array.from(instructors.values()),
			pagination: {
				page,
				limit,
				total,
				total_pages: Math.ceil(total / limit)
			}
		}
	});
};

// POST - Create new learning center content
export const POST: RequestHandler = async ({ request }) => {
	try {
		const body = await request.json();

		// Validate required fields
		if (!body.title || !body.type) {
			throw error(400, 'Title and type are required');
		}

		const id = `lc_${Date.now()}`;
		const slug = body.title.toLowerCase().replace(/[^a-z0-9]+/g, '-');

		// Get category objects
		const contentCategories: Category[] = [];
		if (body.category_ids && Array.isArray(body.category_ids)) {
			body.category_ids.forEach((catId: string) => {
				const cat = categories.get(catId);
				if (cat) contentCategories.push(cat);
			});
		}

		// Get instructor
		let instructor: Instructor = { id: '', name: 'Unknown' };
		if (body.instructor_id) {
			const inst = instructors.get(body.instructor_id);
			if (inst) instructor = inst;
		}

		const newContent: LearningContent = {
			id,
			title: body.title,
			slug,
			type: body.type,
			description: body.description || '',
			excerpt: body.excerpt || body.description?.substring(0, 150) || '',
			thumbnail_url: body.thumbnail_url || '',
			content_url: body.content_url || '',
			duration: body.duration,
			read_time: body.read_time,
			category_id: body.category_id || contentCategories[0]?.id || '',
			categories: contentCategories,
			tags: body.tags || [],
			instructor,
			membership_ids: body.membership_ids || [],
			is_premium: body.is_premium ?? true,
			is_published: body.is_published ?? false,
			is_featured: body.is_featured ?? false,
			sort_order: body.sort_order ?? 0,
			view_count: 0,
			like_count: 0,
			created_at: new Date().toISOString(),
			updated_at: new Date().toISOString()
		};

		if (body.is_published) {
			newContent.published_at = new Date().toISOString();
		}

		learningContent.set(id, newContent);

		return json({
			success: true,
			data: newContent
		}, { status: 201 });
	} catch (err) {
		if (err instanceof Error && 'status' in err) {
			throw err;
		}
		throw error(500, 'Failed to create content');
	}
};

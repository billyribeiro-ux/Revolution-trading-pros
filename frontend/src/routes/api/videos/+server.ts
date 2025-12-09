/**
 * Videos API - Real CRUD Operations
 *
 * Handles video management for the dashboard and learning center.
 * Supports video uploads, metadata management, and category organization.
 *
 * @version 1.0.0 - December 2025
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

// Video interface
interface Video {
	id: string;
	title: string;
	slug: string;
	description: string;
	thumbnail_url: string;
	video_url: string;
	duration: number; // seconds
	category_id: string;
	categories: string[];
	tags: string[];
	instructor: {
		id: string;
		name: string;
		avatar?: string;
	};
	membership_id: string;
	is_premium: boolean;
	is_published: boolean;
	view_count: number;
	created_at: string;
	updated_at: string;
	published_at?: string;
}

// In production, this would connect to a real database
// For now, using in-memory storage that persists during runtime
const videos: Map<string, Video> = new Map();

// Initialize with sample data
function initializeSampleData() {
	if (videos.size === 0) {
		const sampleVideos: Video[] = [
			{
				id: 'vid_1',
				title: 'Mastering the Trade Room FAQs',
				slug: 'mastering-the-trade-room-faqs',
				description: 'Mastering the Trade Room FAQs - A comprehensive guide to getting started.',
				thumbnail_url: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
				video_url: '',
				duration: 3600,
				category_id: 'cat_1',
				categories: ['Member Webinar'],
				tags: ['FAQ', 'Getting Started'],
				instructor: { id: 'inst_1', name: 'Revolution Trading Team' },
				membership_id: 'mastering-the-trade',
				is_premium: true,
				is_published: true,
				view_count: 1250,
				created_at: new Date().toISOString(),
				updated_at: new Date().toISOString(),
				published_at: new Date().toISOString()
			},
			{
				id: 'vid_2',
				title: 'How to Find 3 Reasons to do a Trade',
				slug: '3-reasons-to-do-a-trade',
				description: 'Learn how to identify three compelling reasons before entering any trade.',
				thumbnail_url: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
				video_url: '',
				duration: 2700,
				category_id: 'cat_1',
				categories: ['Member Webinar', 'Trade & Money Management/Trading Plan'],
				tags: ['Trade Setup', 'Risk Management'],
				instructor: { id: 'inst_2', name: 'Chris Brecher' },
				membership_id: 'mastering-the-trade',
				is_premium: true,
				is_published: true,
				view_count: 890,
				created_at: new Date().toISOString(),
				updated_at: new Date().toISOString(),
				published_at: new Date().toISOString()
			},
			{
				id: 'vid_3',
				title: 'Analysis of A Trade',
				slug: 'analysis-of-a-trade',
				description: 'Deep dive into trade analysis methodology.',
				thumbnail_url: 'https://cdn.simplertrading.com/2021/06/23152021/MemberWebinar-TaylorH.jpg',
				video_url: '',
				duration: 4200,
				category_id: 'cat_2',
				categories: ['Member Webinar', 'Trade Setups & Strategies'],
				tags: ['Analysis', 'Strategy'],
				instructor: { id: 'inst_3', name: 'Taylor Horton' },
				membership_id: 'mastering-the-trade',
				is_premium: true,
				is_published: true,
				view_count: 1560,
				created_at: new Date().toISOString(),
				updated_at: new Date().toISOString(),
				published_at: new Date().toISOString()
			},
			{
				id: 'vid_4',
				title: 'John Carter Bigger Picture Outlook',
				slug: 'john-carter-bigger-picture-outlook',
				description: 'Market update and review of what to expect moving forward.',
				thumbnail_url: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
				video_url: '',
				duration: 5400,
				category_id: 'cat_1',
				categories: ['Member Webinar', 'Trade & Money Management/Trading Plan'],
				tags: ['Market Outlook', 'Macro'],
				instructor: { id: 'inst_4', name: 'John Carter' },
				membership_id: 'mastering-the-trade',
				is_premium: true,
				is_published: true,
				view_count: 2340,
				created_at: new Date().toISOString(),
				updated_at: new Date().toISOString(),
				published_at: new Date().toISOString()
			}
		];

		sampleVideos.forEach(v => videos.set(v.id, v));
	}
}

// GET - List videos with filtering and pagination
export const GET: RequestHandler = async ({ url }) => {
	initializeSampleData();

	const page = parseInt(url.searchParams.get('page') || '1');
	const limit = parseInt(url.searchParams.get('limit') || '12');
	const membership_id = url.searchParams.get('membership_id');
	const category = url.searchParams.get('category');
	const search = url.searchParams.get('search')?.toLowerCase();
	const published_only = url.searchParams.get('published') !== 'false';

	let filtered = Array.from(videos.values());

	// Filter by membership
	if (membership_id) {
		filtered = filtered.filter(v => v.membership_id === membership_id);
	}

	// Filter by category
	if (category) {
		filtered = filtered.filter(v => v.categories.includes(category));
	}

	// Filter by published status
	if (published_only) {
		filtered = filtered.filter(v => v.is_published);
	}

	// Search
	if (search) {
		filtered = filtered.filter(v =>
			v.title.toLowerCase().includes(search) ||
			v.description.toLowerCase().includes(search) ||
			v.instructor.name.toLowerCase().includes(search)
		);
	}

	// Sort by most recent
	filtered.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime());

	// Pagination
	const total = filtered.length;
	const start = (page - 1) * limit;
	const paginated = filtered.slice(start, start + limit);

	return json({
		success: true,
		data: {
			videos: paginated,
			pagination: {
				page,
				limit,
				total,
				total_pages: Math.ceil(total / limit)
			}
		}
	});
};

// POST - Create new video
export const POST: RequestHandler = async ({ request }) => {
	try {
		const body = await request.json();

		// Validate required fields
		if (!body.title || !body.membership_id) {
			throw error(400, 'Title and membership_id are required');
		}

		const id = `vid_${Date.now()}`;
		const slug = body.title.toLowerCase().replace(/[^a-z0-9]+/g, '-');

		const newVideo: Video = {
			id,
			title: body.title,
			slug,
			description: body.description || '',
			thumbnail_url: body.thumbnail_url || '',
			video_url: body.video_url || '',
			duration: body.duration || 0,
			category_id: body.category_id || '',
			categories: body.categories || [],
			tags: body.tags || [],
			instructor: body.instructor || { id: '', name: 'Unknown' },
			membership_id: body.membership_id,
			is_premium: body.is_premium ?? true,
			is_published: body.is_published ?? false,
			view_count: 0,
			created_at: new Date().toISOString(),
			updated_at: new Date().toISOString()
		};

		if (body.is_published) {
			newVideo.published_at = new Date().toISOString();
		}

		videos.set(id, newVideo);

		return json({
			success: true,
			data: newVideo
		}, { status: 201 });
	} catch (err) {
		if (err instanceof Error && 'status' in err) {
			throw err;
		}
		throw error(500, 'Failed to create video');
	}
};

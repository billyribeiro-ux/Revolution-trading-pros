/**
 * Learning Center Page Load Function
 *
 * Fetches learning center content from the API for the membership.
 *
 * @version 1.0.0 - December 2025
 */

import type { PageLoad } from './$types';

export interface LearningContent {
	id: string;
	title: string;
	slug: string;
	type: 'video' | 'article' | 'course' | 'webinar';
	description: string;
	excerpt: string;
	thumbnail_url: string;
	content_url: string;
	duration?: number;
	category_id: string;
	categories: Array<{ id: string; name: string; slug: string; color?: string }>;
	tags: string[];
	instructor: {
		id: string;
		name: string;
		title?: string;
		avatar_url?: string;
	};
	is_premium: boolean;
	is_published: boolean;
	is_featured: boolean;
	view_count: number;
	like_count: number;
	created_at: string;
}

export interface Category {
	id: string;
	name: string;
	slug: string;
	color?: string;
}

export interface Instructor {
	id: string;
	name: string;
	title?: string;
	avatar_url?: string;
}

export interface LearningCenterData {
	content: LearningContent[];
	categories: Category[];
	instructors: Instructor[];
	pagination: {
		page: number;
		limit: number;
		total: number;
		total_pages: number;
	};
}

export const load: PageLoad = async ({ params, fetch, url }) => {
	const { slug } = params;
	const page = parseInt(url.searchParams.get('page') || '1');
	const category = url.searchParams.get('category');
	const search = url.searchParams.get('search');

	try {
		// Build query string
		const queryParams = new URLSearchParams({
			membership_id: slug,
			page: page.toString(),
			limit: '12'
		});

		if (category && category !== 'all') {
			queryParams.set('category_id', category);
		}

		if (search) {
			queryParams.set('search', search);
		}

		const response = await fetch(`/api/learning-center?${queryParams}`);

		if (!response.ok) {
			throw new Error('Failed to load learning center content');
		}

		const data = await response.json();

		return {
			slug,
			learningContent: data.data.content as LearningContent[],
			categories: data.data.categories as Category[],
			instructors: data.data.instructors as Instructor[],
			pagination: data.data.pagination,
			currentPage: page,
			selectedCategory: category || 'all',
			searchQuery: search || ''
		};
	} catch (error) {
		console.error('[Learning Center] Load error:', error);

		// Return empty data on error
		return {
			slug,
			learningContent: [],
			categories: [],
			instructors: [],
			pagination: { page: 1, limit: 12, total: 0, total_pages: 0 },
			currentPage: 1,
			selectedCategory: 'all',
			searchQuery: '',
			error: 'Failed to load learning center content'
		};
	}
};

/**
 * Learning Center API - Proxy to Backend
 *
 * Proxies learning center content requests to the Laravel backend.
 * Uses the products API with type filtering for educational content.
 *
 * @version 2.0.0 - December 2025 - Connected to real backend
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

// Production fallback - Rust API on Fly.io
const PROD_API = 'https://revolution-trading-pros-api.fly.dev/api';
const BACKEND_URL = import.meta.env.VITE_API_URL || PROD_API;

/**
 * Get authorization headers from request
 */
function getAuthHeaders(request: Request): HeadersInit {
	const authHeader = request.headers.get('Authorization');
	const headers: HeadersInit = {
		'Content-Type': 'application/json',
		'Accept': 'application/json',
	};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	}
	return headers;
}

// GET - List learning center content (proxies to backend products/courses)
export const GET: RequestHandler = async ({ url, request }) => {
	try {
		// Forward query params to backend
		const queryParams = new URLSearchParams();

		const page = url.searchParams.get('page');
		const limit = url.searchParams.get('limit') || url.searchParams.get('per_page');
		const membership_id = url.searchParams.get('membership_id');
		const category_id = url.searchParams.get('category_id');
		const type = url.searchParams.get('type');
		const search = url.searchParams.get('search');
		const instructor_id = url.searchParams.get('instructor_id');
		const featured = url.searchParams.get('featured');
		const published = url.searchParams.get('published');

		if (page) queryParams.set('page', page);
		if (limit) queryParams.set('per_page', limit);
		if (membership_id) queryParams.set('membership_id', membership_id);
		if (category_id) queryParams.set('category_id', category_id);
		if (search) queryParams.set('search', search);
		if (instructor_id) queryParams.set('instructor_id', instructor_id);
		if (featured) queryParams.set('featured', featured);
		if (published) queryParams.set('is_published', published);

		// Default to courses/education type if not specified
		const contentType = type || 'course';
		queryParams.set('type', contentType);

		const backendUrl = `${BACKEND_URL}/admin/products${queryParams.toString() ? '?' + queryParams.toString() : ''}`;

		const response = await fetch(backendUrl, {
			method: 'GET',
			headers: getAuthHeaders(request),
		});

		if (!response.ok) {
			const errorData = await response.json().catch(() => ({ message: 'Failed to fetch learning content' }));
			return json({
				success: false,
				error: errorData.message || 'Failed to fetch learning content',
				data: {
					content: [],
					categories: [],
					instructors: [],
					pagination: { page: 1, limit: 12, total: 0, total_pages: 0 }
				}
			}, { status: response.status });
		}

		const data = await response.json();

		// Transform backend response to learning center format
		const content = (data.data || []).map((item: any) => ({
			id: item.id,
			title: item.name || item.title,
			slug: item.slug,
			type: item.type || 'course',
			description: item.description || '',
			excerpt: item.short_description || item.description?.substring(0, 150) || '',
			thumbnail_url: item.thumbnail_url || item.image_url || '',
			content_url: item.content_url || '',
			duration: item.duration,
			category_id: item.category_id,
			categories: item.categories || [],
			tags: item.tags || [],
			instructor: item.instructor || { id: '', name: 'Unknown' },
			membership_ids: item.membership_ids || [],
			is_premium: item.is_premium ?? true,
			is_published: item.is_published ?? true,
			is_featured: item.is_featured ?? false,
			sort_order: item.sort_order || 0,
			view_count: item.view_count || 0,
			like_count: item.like_count || 0,
			created_at: item.created_at,
			updated_at: item.updated_at,
			published_at: item.published_at
		}));

		return json({
			success: true,
			data: {
				content,
				categories: data.categories || [],
				instructors: data.instructors || [],
				pagination: {
					page: data.current_page || 1,
					limit: data.per_page || 12,
					total: data.total || 0,
					total_pages: data.last_page || 0
				}
			}
		});
	} catch (err) {
		console.error('Learning Center API proxy error:', err);
		return json({
			success: false,
			error: 'Failed to connect to backend',
			data: {
				content: [],
				categories: [],
				instructors: [],
				pagination: { page: 1, limit: 12, total: 0, total_pages: 0 }
			}
		}, { status: 503 });
	}
};

// POST - Create new learning center content (proxies to backend)
export const POST: RequestHandler = async ({ request }) => {
	try {
		const body = await request.json();

		// Validate required fields
		if (!body.title || !body.type) {
			throw error(400, 'Title and type are required');
		}

		const response = await fetch(`${BACKEND_URL}/admin/products`, {
			method: 'POST',
			headers: getAuthHeaders(request),
			body: JSON.stringify({
				name: body.title,
				slug: body.slug || body.title.toLowerCase().replace(/[^a-z0-9]+/g, '-'),
				type: body.type,
				description: body.description || '',
				short_description: body.excerpt || body.description?.substring(0, 150) || '',
				thumbnail_url: body.thumbnail_url || '',
				content_url: body.content_url || '',
				duration: body.duration,
				category_id: body.category_id,
				tags: body.tags || [],
				instructor_id: body.instructor_id,
				membership_ids: body.membership_ids || [],
				is_premium: body.is_premium ?? true,
				is_published: body.is_published ?? false,
				is_featured: body.is_featured ?? false,
				sort_order: body.sort_order ?? 0,
			}),
		});

		if (!response.ok) {
			const errorData = await response.json().catch(() => ({ message: 'Failed to create content' }));
			return json({
				success: false,
				error: errorData.message || 'Failed to create content',
				errors: errorData.errors
			}, { status: response.status });
		}

		const data = await response.json();

		return json({
			success: true,
			data: data
		}, { status: 201 });
	} catch (err) {
		if (err instanceof Error && 'status' in err) {
			throw err;
		}
		console.error('Learning Center API create error:', err);
		throw error(500, 'Failed to create content');
	}
};

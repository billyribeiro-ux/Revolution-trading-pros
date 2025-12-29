/**
 * Videos API - Proxy to Backend
 *
 * Proxies video management requests to the Rust API backend.
 * Handles video CRUD operations, listing, and filtering.
 *
 * @version 2.0.0 - December 2025 - Connected to real backend
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

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

// GET - List videos (proxies to backend)
export const GET: RequestHandler = async ({ url, request }) => {
	try {
		// Forward query params to backend
		const queryParams = new URLSearchParams();

		const page = url.searchParams.get('page');
		const limit = url.searchParams.get('limit') || url.searchParams.get('per_page');
		const membership_id = url.searchParams.get('membership_id');
		const category = url.searchParams.get('category');
		const search = url.searchParams.get('search');
		const platform = url.searchParams.get('platform');
		const published = url.searchParams.get('published');

		if (page) queryParams.set('page', page);
		if (limit) queryParams.set('per_page', limit);
		if (membership_id) queryParams.set('membership_id', membership_id);
		if (category) queryParams.set('category', category);
		if (search) queryParams.set('search', search);
		if (platform) queryParams.set('platform', platform);
		if (published) queryParams.set('is_published', published);

		const backendUrl = `${BACKEND_URL}/videos${queryParams.toString() ? '?' + queryParams.toString() : ''}`;

		const response = await fetch(backendUrl, {
			method: 'GET',
			headers: getAuthHeaders(request),
		});

		if (!response.ok) {
			// If backend returns error, return appropriate error response
			const errorData = await response.json().catch(() => ({ message: 'Failed to fetch videos' }));
			return json({
				success: false,
				error: errorData.message || 'Failed to fetch videos',
				data: { videos: [], pagination: { page: 1, limit: 12, total: 0, total_pages: 0 } }
			}, { status: response.status });
		}

		const data = await response.json();

		// Transform backend pagination response to match frontend expected format
		return json({
			success: true,
			data: {
				videos: data.data || [],
				pagination: {
					page: data.current_page || 1,
					limit: data.per_page || 12,
					total: data.total || 0,
					total_pages: data.last_page || 0
				}
			}
		});
	} catch (err) {
		console.error('Videos API proxy error:', err);
		return json({
			success: false,
			error: 'Failed to connect to backend',
			data: { videos: [], pagination: { page: 1, limit: 12, total: 0, total_pages: 0 } }
		}, { status: 503 });
	}
};

// POST - Create new video (proxies to backend)
export const POST: RequestHandler = async ({ request }) => {
	try {
		const body = await request.json();

		// Validate required fields before sending to backend
		if (!body.title) {
			throw error(400, 'Title is required');
		}
		if (!body.url) {
			throw error(400, 'Video URL is required');
		}

		const response = await fetch(`${BACKEND_URL}/admin/videos`, {
			method: 'POST',
			headers: getAuthHeaders(request),
			body: JSON.stringify({
				title: body.title,
				description: body.description || '',
				url: body.video_url || body.url,
				platform: body.platform || 'html5',
				thumbnail_url: body.thumbnail_url || '',
				duration: body.duration || 0,
				video_id: body.video_id || '',
				metadata: {
					categories: body.categories || [],
					tags: body.tags || [],
					instructor: body.instructor,
					membership_id: body.membership_id,
					is_premium: body.is_premium ?? true,
				},
			}),
		});

		if (!response.ok) {
			const errorData = await response.json().catch(() => ({ message: 'Failed to create video' }));
			return json({
				success: false,
				error: errorData.message || 'Failed to create video',
				errors: errorData.errors
			}, { status: response.status });
		}

		const data = await response.json();

		return json({
			success: true,
			data: data.video || data
		}, { status: 201 });
	} catch (err) {
		if (err instanceof Error && 'status' in err) {
			throw err;
		}
		console.error('Videos API create error:', err);
		throw error(500, 'Failed to create video');
	}
};

/**
 * Posts API Endpoint
 *
 * Handles blog post management - proxies to Rust API backend.
 * No mock data - always returns real backend data.
 *
 * @version 3.0.0 - January 2026
 */

import { json, error, isHttpError } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { env } from '$env/dynamic/private';
const PROD_BACKEND =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

async function fetchFromBackend(
	endpoint: string,
	options?: RequestInit
): Promise<{ data: unknown; status: number }> {
	const backendUrl = PROD_BACKEND;

	try {
		const response = await fetch(`${backendUrl}/api${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...(options?.headers || {})
			}
		});

		const data = await response.json();
		return { data, status: response.status };
	} catch (err) {
		console.error(`Backend error for ${endpoint}:`, err);
		return { data: null, status: 500 };
	}
}

// GET - List posts
export const GET: RequestHandler = async ({ url, request, cookies }) => {
	// FIX-2026-04-26: prefer canonical rtp_access_token cookie, fall back to
	// Authorization header for legacy adminFetch callers that still ship it.
	// Old: const authHeader = request.headers.get('Authorization') || '';
	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) error(401, 'Unauthorized');
	const authHeader = `Bearer ${token}`;
	const queryParams = url.searchParams.toString();
	const endpoint = `/admin/posts${queryParams ? `?${queryParams}` : ''}`;

	const { data, status } = await fetchFromBackend(endpoint, {
		headers: { Authorization: authHeader }
	});

	if (status >= 400) {
		// Return empty data on error, not mock data
		return json({
			success: false,
			data: [],
			meta: { current_page: 1, per_page: 20, total: 0, last_page: 0 },
			error: 'Failed to fetch posts from backend'
		});
	}

	return json(data);
};

// POST - Create new post
export const POST: RequestHandler = async ({ request, cookies }) => {
	// FIX-2026-04-26: prefer canonical rtp_access_token cookie, fall back to header.
	// Old: const authHeader = request.headers.get('Authorization') || '';
	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) error(401, 'Unauthorized');
	const authHeader = `Bearer ${token}`;

	try {
		const body = await request.json();

		const { data, status } = await fetchFromBackend('/admin/posts', {
			method: 'POST',
			headers: { Authorization: authHeader },
			body: JSON.stringify(body)
		});

		if (status >= 400) {
			// FIX-2026-04-26 (P0-1): pass through backend's actual error message/status
			// rather than collapsing into a generic 400. The browser reads result.message.
			const message =
				(data && typeof data === 'object' && ('message' in data || 'error' in data)
					? (data as { message?: string; error?: string }).message ||
						(data as { error?: string }).error
					: undefined) || 'Failed to create post';
			error(status, message);
		}

		return json(data);
	} catch (err) {
		// FIX-2026-04-26 (P0-1): rethrow HttpError so backend status/message reach the client.
		if (isHttpError(err)) throw err;
		console.error('POST /api/admin/posts error:', err);
		error(400, 'Invalid request body');
	}
};

// PUT - Update post
export const PUT: RequestHandler = async ({ request, url, cookies }) => {
	// FIX-2026-04-26: prefer canonical rtp_access_token cookie, fall back to header.
	// Old: const authHeader = request.headers.get('Authorization') || '';
	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) error(401, 'Unauthorized');
	const authHeader = `Bearer ${token}`;
	const postId = url.searchParams.get('id');

	if (!postId) {
		error(400, 'Post ID required');
	}

	try {
		const body = await request.json();

		const { data, status } = await fetchFromBackend(`/admin/posts/${postId}`, {
			method: 'PUT',
			headers: { Authorization: authHeader },
			body: JSON.stringify(body)
		});

		if (status >= 400) {
			// FIX-2026-04-26 (P0-1): forward backend status + message instead of generic 400.
			const message =
				(data && typeof data === 'object' && ('message' in data || 'error' in data)
					? (data as { message?: string; error?: string }).message ||
						(data as { error?: string }).error
					: undefined) || 'Failed to update post';
			error(status, message);
		}

		return json(data);
	} catch (err) {
		// FIX-2026-04-26 (P0-1): rethrow HttpError so backend status/message reach the client.
		if (isHttpError(err)) throw err;
		console.error('PUT /api/admin/posts error:', err);
		error(400, 'Invalid request body');
	}
};

// DELETE - Delete post
export const DELETE: RequestHandler = async ({ url, request, cookies }) => {
	// FIX-2026-04-26: prefer canonical rtp_access_token cookie, fall back to header.
	// Old: const authHeader = request.headers.get('Authorization') || '';
	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) error(401, 'Unauthorized');
	const authHeader = `Bearer ${token}`;
	const postId = url.searchParams.get('id');

	if (!postId) {
		error(400, 'Post ID required');
	}

	const { data, status } = await fetchFromBackend(`/admin/posts/${postId}`, {
		method: 'DELETE',
		headers: { Authorization: authHeader }
	});

	if (status >= 400) {
		error(status, 'Failed to delete post');
	}

	return json(data);
};

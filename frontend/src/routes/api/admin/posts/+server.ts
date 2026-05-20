/**
 * Posts API Endpoint
 *
 * Handles blog post management - proxies to Rust API backend.
 * No mock data - always returns real backend data.
 *
 * R21-A: migrated to shared `fetchBackendWithStatus` helper. POST/PUT bodies
 * now narrowed with `isObject` (was: implicit `any` from `request.json()`
 * passed straight to `JSON.stringify` — null/primitive bodies serialized to
 * "null"/literal and triggered backend-side validation 500s instead of
 * proxy-side 400). Backend-error-message extraction consolidated through
 * `extractBackendErrorMessage`.
 *
 * @version 3.1.0 — 2026-05-20
 */

import { json, error, isHttpError } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { requireAdmin } from '$lib/server/auth';
import {
	fetchBackendWithStatus,
	isObject,
	extractBackendErrorMessage
} from '$lib/server/proxy-fetch';

// GET - List posts
export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { url } = event;
	const authHeader = `Bearer ${token}`;
	const queryParams = url.searchParams.toString();
	const endpoint = `/api/admin/posts${queryParams ? `?${queryParams}` : ''}`;

	const { data, status } = await fetchBackendWithStatus(
		endpoint,
		{ headers: { Authorization: authHeader } },
		'[Posts API]'
	);

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
export const POST: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { request } = event;
	const authHeader = `Bearer ${token}`;

	try {
		const body: unknown = await request.json();

		if (!isObject(body)) {
			error(400, 'Invalid request body');
		}

		const { data, status } = await fetchBackendWithStatus(
			'/api/admin/posts',
			{
				method: 'POST',
				headers: { Authorization: authHeader },
				body: JSON.stringify(body)
			},
			'[Posts API]'
		);

		if (status >= 400) {
			// FIX-2026-04-26 (P0-1): pass through backend's actual error message/status
			// rather than collapsing into a generic 400. The browser reads result.message.
			error(status, extractBackendErrorMessage(data, 'Failed to create post'));
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
export const PUT: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { request, url } = event;
	const authHeader = `Bearer ${token}`;
	const postId = url.searchParams.get('id');

	if (!postId) {
		error(400, 'Post ID required');
	}

	try {
		const body: unknown = await request.json();

		if (!isObject(body)) {
			error(400, 'Invalid request body');
		}

		const { data, status } = await fetchBackendWithStatus(
			`/api/admin/posts/${postId}`,
			{
				method: 'PUT',
				headers: { Authorization: authHeader },
				body: JSON.stringify(body)
			},
			'[Posts API]'
		);

		if (status >= 400) {
			// FIX-2026-04-26 (P0-1): forward backend status + message instead of generic 400.
			error(status, extractBackendErrorMessage(data, 'Failed to update post'));
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
export const DELETE: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { url } = event;
	const authHeader = `Bearer ${token}`;
	const postId = url.searchParams.get('id');

	if (!postId) {
		error(400, 'Post ID required');
	}

	const { data, status } = await fetchBackendWithStatus(
		`/api/admin/posts/${postId}`,
		{
			method: 'DELETE',
			headers: { Authorization: authHeader }
		},
		'[Posts API]'
	);

	if (status >= 400) {
		error(status, extractBackendErrorMessage(data, 'Failed to delete post'));
	}

	return json(data);
};

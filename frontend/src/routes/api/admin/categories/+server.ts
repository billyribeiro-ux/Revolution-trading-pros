/**
 * Categories API Endpoint
 *
 * Proxies to backend for blog category management.
 *
 * R21-A: migrated to shared `fetchBackendWithStatus` helper. POST body
 * narrowed with `isObject`. Backend-error-message extraction consolidated.
 *
 * @version 2.1.0 — 2026-05-20
 */

import { json, error, isHttpError } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { requireAdmin } from '$lib/server/auth';
import {
	fetchBackendWithStatus,
	isObject,
	extractBackendErrorMessage
} from '$lib/server/proxy-fetch';

// GET - List categories
export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { url } = event;
	const authHeader = `Bearer ${token}`;
	const queryParams = url.searchParams.toString();
	const endpoint = `/api/admin/categories${queryParams ? `?${queryParams}` : ''}`;

	const { data, status } = await fetchBackendWithStatus(
		endpoint,
		{ headers: { Authorization: authHeader } },
		'[Categories API]'
	);

	if (status >= 400 || !data) {
		return json({
			success: false,
			data: [],
			meta: { total: 0 },
			error: 'Failed to fetch categories from backend'
		});
	}

	return json(data);
};

// POST - Create new category
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
			'/api/admin/categories',
			{
				method: 'POST',
				headers: { Authorization: authHeader },
				body: JSON.stringify(body)
			},
			'[Categories API]'
		);

		if (status >= 400) {
			// FIX-2026-04-26 (P0-1): forward backend status + message instead of generic 400.
			error(status, extractBackendErrorMessage(data, 'Failed to create category'));
		}

		return json(data);
	} catch (err) {
		// FIX-2026-04-26 (P0-1): rethrow HttpError so backend status/message reach the client.
		if (isHttpError(err)) throw err;
		console.error('POST /api/admin/categories error:', err);
		error(400, 'Invalid request body');
	}
};

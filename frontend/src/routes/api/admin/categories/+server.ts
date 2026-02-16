/**
 * Categories API Endpoint
 *
 * Proxies to backend for blog category management.
 *
 * @version 2.0.0 - January 2026
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { logger } from '$lib/utils/logger';

const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';

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
		logger.error(`Backend error for ${endpoint}:`, err);
		return { data: null, status: 500 };
	}
}

// GET - List categories
export const GET: RequestHandler = async ({ url, request }) => {
	const authHeader = request.headers.get('Authorization') || '';
	const queryParams = url.searchParams.toString();
	const endpoint = `/admin/categories${queryParams ? `?${queryParams}` : ''}`;

	const { data, status } = await fetchFromBackend(endpoint, {
		headers: { Authorization: authHeader }
	});

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
export const POST: RequestHandler = async ({ request }) => {
	const authHeader = request.headers.get('Authorization') || '';

	try {
		const body = await request.json();

		const { data, status } = await fetchFromBackend('/admin/categories', {
			method: 'POST',
			headers: { Authorization: authHeader },
			body: JSON.stringify(body)
		});

		if (status >= 400) {
			error(status, 'Failed to create category');
		}

		return json(data);
	} catch (err) {
		logger.error('POST /api/admin/categories error:', err);
		error(400, 'Invalid request body');
	}
};

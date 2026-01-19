/**
 * Member Management API Proxy - List & Create
 * Routes requests to Rust API backend
 *
 * @version 1.0.0 - January 2026
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

// Production fallback - Rust API on Fly.io
const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';

/**
 * GET /api/admin/member-management
 * List members with pagination and filters
 */
export const GET: RequestHandler = async ({ url, request }) => {
	const authHeader = request.headers.get('Authorization') || '';

	if (!authHeader) {
		return json({ error: 'Missing or invalid authorization header' }, { status: 401 });
	}

	try {
		const queryParams = url.searchParams.toString();
		const endpoint = queryParams
			? `/api/admin/member-management?${queryParams}`
			: '/api/admin/member-management';

		const response = await fetch(`${PROD_BACKEND}${endpoint}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: authHeader
			}
		});

		const text = await response.text();
		if (!text) {
			return json(
				{ error: 'Empty response from server' },
				{ status: response.status || 500 }
			);
		}

		try {
			const data = JSON.parse(text);
			return json(data, { status: response.status });
		} catch {
			return json({ error: 'Invalid response from server' }, { status: 500 });
		}
	} catch (error) {
		console.error('[API] List members error:', error);
		return json({ error: 'Failed to list members' }, { status: 500 });
	}
};

/**
 * POST /api/admin/member-management
 * Create new member
 */
export const POST: RequestHandler = async ({ request }) => {
	const authHeader = request.headers.get('Authorization') || '';
	const body = await request.json();

	if (!authHeader) {
		return json({ error: 'Missing or invalid authorization header' }, { status: 401 });
	}

	try {
		const response = await fetch(`${PROD_BACKEND}/api/admin/member-management`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: authHeader
			},
			body: JSON.stringify(body)
		});

		const text = await response.text();
		if (!text) {
			return json(
				{ error: 'Empty response from server' },
				{ status: response.status || 500 }
			);
		}

		try {
			const data = JSON.parse(text);
			return json(data, { status: response.status });
		} catch {
			return json({ error: 'Invalid response from server' }, { status: 500 });
		}
	} catch (error) {
		console.error('[API] Create member error:', error);
		return json({ error: 'Failed to create member' }, { status: 500 });
	}
};

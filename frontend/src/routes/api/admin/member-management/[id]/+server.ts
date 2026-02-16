/**
 * Member Management API Proxy - Individual Member
 * Routes DELETE requests to Rust API backend
 *
 * @version 1.0.0 - January 2026
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { logger } from '$lib/utils/logger';

// Production fallback - Rust API on Fly.io
const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';

/**
 * DELETE /api/admin/member-management/:id
 * Soft delete member (anonymizes data, keeps record for audit)
 */
export const DELETE: RequestHandler = async ({ params, request }) => {
	const memberId = params.id;
	const authHeader = request.headers.get('Authorization') || '';

	if (!authHeader) {
		return json({ error: 'Missing or invalid authorization header' }, { status: 401 });
	}

	try {
		const response = await fetch(`${PROD_BACKEND}/api/admin/member-management/${memberId}`, {
			method: 'DELETE',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: authHeader
			}
		});

		const text = await response.text();
		if (!text) {
			return json({ error: 'Empty response from server' }, { status: response.status || 500 });
		}

		try {
			const data = JSON.parse(text);
			return json(data, { status: response.status });
		} catch {
			return json({ error: 'Invalid response from server' }, { status: 500 });
		}
	} catch (error) {
		logger.error('[API] Delete member error:', error);
		return json({ error: 'Failed to delete member' }, { status: 500 });
	}
};

/**
 * GET /api/admin/member-management/:id
 * Get member with full details
 */
export const GET: RequestHandler = async ({ params, request }) => {
	const memberId = params.id;
	const authHeader = request.headers.get('Authorization') || '';

	if (!authHeader) {
		return json({ error: 'Missing or invalid authorization header' }, { status: 401 });
	}

	try {
		const response = await fetch(`${PROD_BACKEND}/api/admin/member-management/${memberId}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: authHeader
			}
		});

		const text = await response.text();
		if (!text) {
			return json({ error: 'Empty response from server' }, { status: response.status || 500 });
		}

		try {
			const data = JSON.parse(text);
			return json(data, { status: response.status });
		} catch {
			return json({ error: 'Invalid response from server' }, { status: 500 });
		}
	} catch (error) {
		logger.error('[API] Get member error:', error);
		return json({ error: 'Failed to get member' }, { status: 500 });
	}
};

/**
 * PUT /api/admin/member-management/:id
 * Update member details
 */
export const PUT: RequestHandler = async ({ params, request }) => {
	const memberId = params.id;
	const authHeader = request.headers.get('Authorization') || '';
	const body = await request.json();

	if (!authHeader) {
		return json({ error: 'Missing or invalid authorization header' }, { status: 401 });
	}

	try {
		const response = await fetch(`${PROD_BACKEND}/api/admin/member-management/${memberId}`, {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: authHeader
			},
			body: JSON.stringify(body)
		});

		const text = await response.text();
		if (!text) {
			return json({ error: 'Empty response from server' }, { status: response.status || 500 });
		}

		try {
			const data = JSON.parse(text);
			return json(data, { status: response.status });
		} catch {
			return json({ error: 'Invalid response from server' }, { status: 500 });
		}
	} catch (error) {
		logger.error('[API] Update member error:', error);
		return json({ error: 'Failed to update member' }, { status: 500 });
	}
};

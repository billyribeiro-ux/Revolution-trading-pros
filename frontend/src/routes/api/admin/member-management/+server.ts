/**
 * Member Management API Proxy - Create Member Only
 * Routes requests to Rust API backend
 *
 * NOTE: This endpoint only supports POST (create member).
 * For listing members, use /api/admin/members instead.
 * For individual member operations, use /api/admin/member-management/[id]
 *
 * @version 1.1.0 - January 2026
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { logger } from '$lib/utils/logger';

// Production fallback - Rust API on Fly.io
const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';

/**
 * GET /api/admin/member-management
 * NOT SUPPORTED - Backend doesn't have a list endpoint here.
 * Use /api/admin/members for listing members.
 */
export const GET: RequestHandler = async () => {
	return json(
		{
			error: 'Method not allowed. Use /api/admin/members for listing members.',
			hint: 'GET /api/admin/member-management/:id is supported for individual members'
		},
		{ status: 405 }
	);
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
			return json({ error: 'Empty response from server' }, { status: response.status || 500 });
		}

		try {
			const data = JSON.parse(text);
			return json(data, { status: response.status });
		} catch {
			return json({ error: 'Invalid response from server' }, { status: 500 });
		}
	} catch (error) {
		logger.error('[API] Create member error:', error);
		return json({ error: 'Failed to create member' }, { status: 500 });
	}
};

/**
 * Admin Course Analytics API Proxy
 * FIX-2026-04-26: New proxy backing CourseDetailDrawer Analytics tab.
 * Forwards to GET /api/admin/courses/:id/analytics on the Rust API.
 */

import { json, isHttpError } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { requireAdminToken } from '$lib/server/auth';

import { env } from '$env/dynamic/private';
const BACKEND_URL = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

export const GET: RequestHandler = async (event) => {
	// FIX-2026-04-26 (P1-1): require token via shared helper.
	const token = requireAdminToken(event);
	const { id } = event.params;

	try {
		const response = await event.fetch(`${BACKEND_URL}/api/admin/courses/${id}/analytics`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});

		if (!response.ok) {
			const errorBody = await response
				.json()
				.catch(() => ({ error: 'Failed to load course analytics' }));
			return json({ success: false, ...errorBody }, { status: response.status });
		}

		const data = await response.json();
		return json(data);
	} catch (err) {
		if (isHttpError(err)) throw err;
		console.error('[API] Admin course analytics error:', err);
		return json({ success: false, error: 'Failed to load course analytics' }, { status: 500 });
	}
};

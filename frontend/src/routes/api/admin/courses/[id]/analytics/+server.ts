/**
 * Admin Course Analytics API Proxy
 * FIX-2026-04-26: New proxy backing CourseDetailDrawer Analytics tab.
 * Forwards to GET /api/admin/courses/:id/analytics on the Rust API.
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { env } from '$env/dynamic/private';
const PROD_BACKEND =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';
const BACKEND_URL = PROD_BACKEND;

export const GET: RequestHandler = async ({ cookies, fetch, params }) => {
	try {
		// FIX-2026-04-26: login proxy sets rtp_access_token cookie; do not regress to auth_token.
		const token = cookies.get('rtp_access_token');
		const { id } = params;

		const response = await fetch(`${BACKEND_URL}/api/admin/courses/${id}/analytics`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...(token ? { Authorization: `Bearer ${token}` } : {})
			}
		});

		if (!response.ok) {
			const error = await response
				.json()
				.catch(() => ({ error: 'Failed to load course analytics' }));
			return json({ success: false, ...error }, { status: response.status });
		}

		const data = await response.json();
		return json(data);
	} catch (error) {
		console.error('[API] Admin course analytics error:', error);
		return json({ success: false, error: 'Failed to load course analytics' }, { status: 500 });
	}
};

/**
 * Admin Course Unpublish API Proxy
 * ICT 7 Grade - Routes requests to Rust API backend
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { env } from '$env/dynamic/private';
const PROD_BACKEND =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';
const BACKEND_URL = PROD_BACKEND;

export const POST: RequestHandler = async ({ cookies, fetch, params }) => {
	try {
		// FIX-2026-04-26: comment-out, verify, delete in follow-up. Wrong cookie name — login proxy sets rtp_access_token, not auth_token.
		// const token = cookies.get('auth_token');
		const token = cookies.get('rtp_access_token');
		const { id } = params;

		const response = await fetch(`${BACKEND_URL}/api/admin/courses/${id}/unpublish`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...(token ? { Authorization: `Bearer ${token}` } : {})
			}
		});

		const data = await response.json();
		return json(data, { status: response.status });
	} catch (error) {
		console.error('[API] Admin course unpublish error:', error);
		return json({ success: false, error: 'Failed to unpublish course' }, { status: 500 });
	}
};

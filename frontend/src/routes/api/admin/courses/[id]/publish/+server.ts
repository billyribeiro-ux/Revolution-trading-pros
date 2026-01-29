/**
 * Admin Course Publish API Proxy
 * ICT 7 Grade - Routes requests to Rust API backend
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';
const BACKEND_URL = PROD_BACKEND;

export const POST: RequestHandler = async ({ cookies, fetch, params }) => {
	try {
		const token = cookies.get('auth_token');
		const { id } = params;

		const response = await fetch(`${BACKEND_URL}/api/admin/courses/${id}/publish`, {
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
		console.error('[API] Admin course publish error:', error);
		return json({ success: false, error: 'Failed to publish course' }, { status: 500 });
	}
};

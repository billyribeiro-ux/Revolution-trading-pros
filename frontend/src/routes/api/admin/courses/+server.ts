/**
 * Admin Courses API Proxy
 * ICT 7 Grade - Routes requests to Rust API backend
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';
const BACKEND_URL = env.BACKEND_URL || env.VITE_API_URL || PROD_BACKEND;

export const GET: RequestHandler = async ({ cookies, fetch, url }) => {
	try {
		const token = cookies.get('auth_token');
		const queryString = url.search || '';

		const response = await fetch(`${BACKEND_URL}/api/admin/courses${queryString}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...(token ? { Authorization: `Bearer ${token}` } : {})
			}
		});

		if (!response.ok) {
			// Return empty data for auth/not-found errors to prevent console errors
			if (response.status === 401 || response.status === 404) {
				return json({
					success: true,
					data: {
						courses: [],
						total: 0,
						page: 1,
						per_page: 12,
						total_pages: 0
					}
				});
			}
			const error = await response.json().catch(() => ({ error: 'Request failed' }));
			return json({ success: false, ...error }, { status: response.status });
		}

		const data = await response.json();
		return json(data);
	} catch (error) {
		console.error('[API] Admin courses list error:', error);
		return json({
			success: true,
			data: {
				courses: [],
				total: 0,
				page: 1,
				per_page: 12,
				total_pages: 0
			}
		});
	}
};

export const POST: RequestHandler = async ({ cookies, fetch, request }) => {
	try {
		const token = cookies.get('auth_token');
		const body = await request.json();

		const response = await fetch(`${BACKEND_URL}/api/admin/courses`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...(token ? { Authorization: `Bearer ${token}` } : {})
			},
			body: JSON.stringify(body)
		});

		const data = await response.json();
		return json(data, { status: response.status });
	} catch (error) {
		console.error('[API] Admin courses create error:', error);
		return json({ success: false, error: 'Failed to create course' }, { status: 500 });
	}
};

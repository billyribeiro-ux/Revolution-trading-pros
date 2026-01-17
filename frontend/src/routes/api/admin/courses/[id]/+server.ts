/**
 * Admin Course Detail API Proxy
 * ICT 7 Grade - Routes requests to Rust API backend
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';
const BACKEND_URL = env.BACKEND_URL || env.VITE_API_URL || PROD_BACKEND;

export const GET: RequestHandler = async ({ cookies, fetch, params }) => {
	try {
		const token = cookies.get('auth_token');
		const { id } = params;

		const response = await fetch(`${BACKEND_URL}/api/admin/courses/${id}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...(token ? { Authorization: `Bearer ${token}` } : {})
			}
		});

		if (!response.ok) {
			const error = await response.json().catch(() => ({ error: 'Course not found' }));
			return json({ success: false, ...error }, { status: response.status });
		}

		const data = await response.json();
		return json(data);
	} catch (error) {
		console.error('[API] Admin course get error:', error);
		return json({ success: false, error: 'Failed to fetch course' }, { status: 500 });
	}
};

export const PUT: RequestHandler = async ({ cookies, fetch, params, request }) => {
	try {
		const token = cookies.get('auth_token');
		const { id } = params;
		const body = await request.json();

		const response = await fetch(`${BACKEND_URL}/api/admin/courses/${id}`, {
			method: 'PUT',
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
		console.error('[API] Admin course update error:', error);
		return json({ success: false, error: 'Failed to update course' }, { status: 500 });
	}
};

export const DELETE: RequestHandler = async ({ cookies, fetch, params }) => {
	try {
		const token = cookies.get('auth_token');
		const { id } = params;

		const response = await fetch(`${BACKEND_URL}/api/admin/courses/${id}`, {
			method: 'DELETE',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...(token ? { Authorization: `Bearer ${token}` } : {})
			}
		});

		const data = await response.json();
		return json(data, { status: response.status });
	} catch (error) {
		console.error('[API] Admin course delete error:', error);
		return json({ success: false, error: 'Failed to delete course' }, { status: 500 });
	}
};

/**
 * Coupons API Proxy
 * Routes requests to Rust API backend to avoid CORS issues
 *
 * @version 2.0.0 - January 2026
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

// Production fallback - Rust API on Fly.io
const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';

export const GET: RequestHandler = async ({ request }) => {
	// ICT 7 FIX: Return empty data for all error cases to prevent console errors
	const emptyData = {
		coupons: [],
		total: 0
	};

	const backendUrl = env.BACKEND_URL || PROD_BACKEND;
	const authHeader = request.headers.get('Authorization') || '';

	// If no auth, return empty data silently
	if (!authHeader) {
		return json(emptyData);
	}

	try {
		const response = await fetch(`${backendUrl}/api/admin/coupons`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: authHeader
			}
		});

		if (!response.ok) {
			// Return empty data for all error cases - graceful degradation
			return json(emptyData);
		}

		// Safely parse JSON response
		const text = await response.text();
		if (!text) {
			return json(emptyData);
		}

		try {
			const data = JSON.parse(text);
			return json(data);
		} catch {
			// Invalid JSON from backend
			return json(emptyData);
		}
	} catch {
		// Network or other error - silent fallback
		return json(emptyData);
	}
};

export const POST: RequestHandler = async ({ request }) => {
	const backendUrl = env.BACKEND_URL || PROD_BACKEND;
	const authHeader = request.headers.get('Authorization') || '';

	try {
		const body = await request.json();

		const response = await fetch(`${backendUrl}/api/admin/coupons`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: authHeader
			},
			body: JSON.stringify(body)
		});

		// Safely parse response
		const text = await response.text();
		if (!text) {
			return json({ message: 'Empty response from server' }, { status: response.status });
		}

		try {
			const data = JSON.parse(text);
			return json(data, { status: response.status });
		} catch {
			return json({ message: 'Invalid response from server' }, { status: 500 });
		}
	} catch (error) {
		console.error('[API] Create coupon error:', error);
		return json({ message: 'Failed to create coupon' }, { status: 500 });
	}
};

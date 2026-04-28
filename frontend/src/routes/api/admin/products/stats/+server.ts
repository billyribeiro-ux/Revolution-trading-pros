/**
 * Products Stats API Proxy
 * Routes requests to Rust API backend to avoid CORS issues
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

// Production fallback - Rust API on Fly.io
import { env } from '$env/dynamic/private';
const BACKEND_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

export const GET: RequestHandler = async ({ cookies, fetch }) => {
	// ICT 7 FIX: Return mock data for all error cases to prevent console errors
	const mockData = {
		total: 0,
		active: 0,
		draft: 0,
		featured: 0,
		total_revenue: 0,
		total_sales: 0,
		data: {
			total: 0
		}
	};

	try {
		const token = cookies.get('rtp_access_token');

		// If no token, return mock data silently (user not authenticated)
		if (!token) {
			return json(mockData);
		}

		const response = await fetch(`${BACKEND_URL}/api/admin/products/stats`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: `Bearer ${token}`
			}
		});

		// FIX-2026-04-26 (P2-2): forward auth failures (401/403) so the
		// admin shell can redirect to login instead of showing "0 products"
		// to a user whose token expired. Continue to mask 5xx as mock data
		// to keep the dashboard rendering when the backend hiccups.
		if (response.status === 401 || response.status === 403) {
			const text = await response.text();
			return new Response(text || JSON.stringify({ message: 'Unauthorized' }), {
				status: response.status,
				headers: { 'Content-Type': 'application/json' }
			});
		}

		if (!response.ok) {
			// Return mock data for non-auth error cases - graceful degradation
			return json(mockData);
		}

		const data = await response.json();
		return json(data);
	} catch (_error) {
		// Silent fallback - don't log expected errors
		return json(mockData);
	}
};

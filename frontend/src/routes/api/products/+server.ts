/**
 * Products API Proxy
 * Forwards requests to backend /api/products endpoint
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

export const GET: RequestHandler = async ({ url, cookies }) => {
	try {
		// FIX-2026-04-26: comment-out, verify, delete in follow-up. Wrong cookie name — login proxy sets rtp_access_token, not auth_token.
		// const token = cookies.get('auth_token');
		const token = cookies.get('rtp_access_token');

		// Forward query parameters
		const queryParams = url.searchParams.toString();
		const endpoint = queryParams
			? `${API_URL}/api/products?${queryParams}`
			: `${API_URL}/api/products`;

		const response = await fetch(endpoint, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				...(token ? { Authorization: `Bearer ${token}` } : {})
			}
		});

		const data = await response.json();

		if (!response.ok) {
			return json(data, { status: response.status });
		}

		return json(data);
	} catch (err) {
		console.error('[API Proxy] Failed to fetch products:', err);
		return error(500, 'Failed to fetch products');
	}
};

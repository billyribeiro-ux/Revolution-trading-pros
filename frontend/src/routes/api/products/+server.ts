/**
 * Products API Proxy
 * Forwards requests to backend /api/products endpoint
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { logger } from '$lib/utils/logger';

const API_URL = import.meta.env.VITE_API_URL || 'https://revolution-trading-pros-api.fly.dev';

export const GET: RequestHandler = async ({ url, cookies }) => {
	try {
		const token = cookies.get('auth_token');

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
		logger.error('[API Proxy] Failed to fetch products:', err);
		return error(500, 'Failed to fetch products');
	}
};

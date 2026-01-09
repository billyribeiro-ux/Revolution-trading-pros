/**
 * Products API Proxy
 * ICT 7 FIX: Routes requests to Rust API backend with graceful fallback
 * Supports query params: product_type, per_page, page, etc.
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

// Production fallback - Rust API on Fly.io
const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';
const BACKEND_URL = env.BACKEND_URL || env.VITE_API_URL || PROD_BACKEND;

export const GET: RequestHandler = async ({ url, cookies, fetch }) => {
	try {
		const token = cookies.get('auth_token');
		
		// Forward all query params to backend
		const queryString = url.search;

		const response = await fetch(`${BACKEND_URL}/api/products${queryString}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				'Accept': 'application/json',
				...(token ? { 'Authorization': `Bearer ${token}` } : {})
			}
		});

		if (!response.ok) {
			// Return mock data for development if backend unavailable
			if (response.status === 404 || response.status === 500) {
				return json({
					products: [],
					data: [],
					total: 0,
					meta: {
						current_page: 1,
						per_page: 100,
						total: 0,
						last_page: 1
					}
				});
			}

			const error = await response.json().catch(() => ({ message: 'Request failed' }));
			return json(error, { status: response.status });
		}

		const data = await response.json();
		return json(data);
	} catch (error) {
		console.debug('[API] Products not available');
		// Return empty products on error - graceful degradation
		return json({
			products: [],
			data: [],
			total: 0,
			meta: {
				current_page: 1,
				per_page: 100,
				total: 0,
				last_page: 1
			}
		});
	}
};

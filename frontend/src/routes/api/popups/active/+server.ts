/**
 * Active Popups API
 * Returns popups that should be displayed on the current page
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

// Production fallback - Rust API on Fly.io
const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';
const BACKEND_URL = env.BACKEND_URL || env.VITE_API_URL || PROD_BACKEND;

export const GET: RequestHandler = async ({ url, fetch }) => {
	const page = url.searchParams.get('page') || '/';

	try {
		const response = await fetch(`${BACKEND_URL}/api/popups/active?page=${encodeURIComponent(page)}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				'Accept': 'application/json'
			}
		});

		if (!response.ok) {
			// Backend may not have popups configured yet
			return json({ popups: [] });
		}

		const data = await response.json();
		return json(data);
	} catch (error) {
		console.debug('[API] Popups not available:', error);
		// Return empty popups on error - graceful degradation
		return json({ popups: [] });
	}
};

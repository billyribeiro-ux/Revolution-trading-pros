/**
 * Coupons API Proxy
 * Routes requests to Rust API backend to avoid CORS issues
 *
 * @version 2.1.0 - January 2026 - Cloudflare compatible
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { logger } from '$lib/utils/logger';

// Production fallback - Rust API on Fly.io
const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';

// Empty data for graceful degradation
const EMPTY_DATA = {
	coupons: [],
	total: 0
};

/**
 * Get backend URL - always use production backend for reliability
 * Cloudflare platform.env is typed differently, so we use the hardcoded fallback
 */
function getBackendUrl(): string {
	return PROD_BACKEND;
}

export const GET: RequestHandler = async ({ request }) => {
	try {
		const backendUrl = getBackendUrl();
		const authHeader = request.headers.get('Authorization') || '';

		// If no auth, return empty data silently
		if (!authHeader) {
			return json(EMPTY_DATA);
		}

		const response = await fetch(`${backendUrl}/api/admin/coupons`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: authHeader
			}
		});

		if (!response.ok) {
			return json(EMPTY_DATA);
		}

		const text = await response.text();
		if (!text) {
			return json(EMPTY_DATA);
		}

		try {
			const data = JSON.parse(text);
			return json(data);
		} catch {
			return json(EMPTY_DATA);
		}
	} catch {
		return json(EMPTY_DATA);
	}
};

export const POST: RequestHandler = async ({ request }) => {
	try {
		const backendUrl = getBackendUrl();
		const authHeader = request.headers.get('Authorization') || '';
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
		logger.error('[API] Create coupon error:', error);
		return json({ message: 'Failed to create coupon' }, { status: 500 });
	}
};

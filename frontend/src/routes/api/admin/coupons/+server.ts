/**
 * Coupons API Proxy
 * Routes requests to Rust API backend to avoid CORS issues
 *
 * @version 2.1.0 - January 2026 - Cloudflare compatible
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

// Production fallback - Rust API on Fly.io
import { env } from '$env/dynamic/private';
const PROD_BACKEND =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

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

export const GET: RequestHandler = async ({ request, cookies }) => {
	try {
		const backendUrl = getBackendUrl();
		// FIX-2026-04-26: prefer canonical rtp_access_token cookie, fall back to header.
		// Old: const authHeader = request.headers.get('Authorization') || '';
		const cookieToken = cookies.get('rtp_access_token');
		const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
		const token = cookieToken || headerToken;
		// If no auth, return empty data silently (preserve original behavior)
		if (!token) {
			return json(EMPTY_DATA);
		}
		const authHeader = `Bearer ${token}`;

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

export const POST: RequestHandler = async ({ request, cookies }) => {
	try {
		const backendUrl = getBackendUrl();
		// FIX-2026-04-26: prefer canonical rtp_access_token cookie, fall back to header.
		// Old: const authHeader = request.headers.get('Authorization') || '';
		const cookieToken = cookies.get('rtp_access_token');
		const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
		const token = cookieToken || headerToken;
		if (!token) {
			return json({ message: 'Unauthorized' }, { status: 401 });
		}
		const authHeader = `Bearer ${token}`;
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
		console.error('[API] Create coupon error:', error);
		return json({ message: 'Failed to create coupon' }, { status: 500 });
	}
};

/**
 * Tags API Endpoint
 * ICT 7 Principal Engineer Grade
 *
 * Proxies to backend for blog post tags.
 *
 * @version 2.0.0 - January 2026
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

import { env } from '$env/dynamic/private';
const PROD_BACKEND =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

export const GET: RequestHandler = async ({ request, cookies }) => {
	const backendUrl = PROD_BACKEND;
	// FIX-2026-04-26: prefer canonical rtp_access_token cookie, fall back to header.
	// Old: const authHeader = request.headers.get('Authorization') || '';
	const cookieToken = cookies.get('rtp_access_token');
	const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
	const token = cookieToken || headerToken;
	if (!token) error(401, 'Unauthorized');
	const authHeader = `Bearer ${token}`;

	try {
		const response = await fetch(`${backendUrl}/api/admin/tags`, {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: authHeader
			}
		});

		if (response.ok) {
			const data = await response.json();
			return json({ success: true, data: data.data || data });
		}

		console.warn(`Backend tags returned ${response.status}`);
	} catch (err) {
		console.warn('Backend tags not available:', err);
	}

	return json({
		success: false,
		data: [],
		error: 'Failed to fetch tags from backend'
	});
};

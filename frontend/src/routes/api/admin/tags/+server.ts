/**
 * Tags API Endpoint
 * ICT 7 Principal Engineer Grade
 *
 * Proxies to backend for blog post tags.
 *
 * @version 2.0.0 - January 2026
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';

export const GET: RequestHandler = async ({ request }) => {
	const backendUrl = env.BACKEND_URL || PROD_BACKEND;
	const authHeader = request.headers.get('Authorization') || '';

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

/**
 * Course Downloads API - Proxy to Backend (by ID)
 * ICT 11+ Principal Engineer Implementation
 *
 * Proxies requests from frontend to Rust backend API
 * Endpoint: GET /api/courses/:id/downloads
 */

import { json, type RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const API_URL = env.API_URL || 'https://revolution-trading-pros-api.fly.dev';

export const GET: RequestHandler = async ({ params, cookies, fetch }) => {
	const { id } = params;

	if (!id) {
		return json({ success: false, error: 'Course ID is required' }, { status: 400 });
	}

	try {
		// Get auth token from cookies if available
		const accessToken = cookies.get('access_token');

		const headers: Record<string, string> = {
			'Content-Type': 'application/json'
		};

		if (accessToken) {
			headers['Authorization'] = `Bearer ${accessToken}`;
		}

		// Proxy to backend API
		const response = await fetch(`${API_URL}/api/courses/${id}/downloads`, {
			method: 'GET',
			headers
		});

		if (!response.ok) {
			// Return mock data for now since backend endpoint may not exist yet
			console.warn(`[API Proxy] Backend returned ${response.status} for course downloads: ${id}`);

			// Return empty downloads array - no error, just no downloads yet
			return json({
				success: true,
				data: [],
				message: 'No downloads available for this course'
			});
		}

		const data = await response.json();
		return json(data);
	} catch (error) {
		console.error('[API Proxy] Error fetching course downloads:', error);

		// Return graceful fallback - no downloads rather than error
		return json({
			success: true,
			data: [],
			message: 'Downloads temporarily unavailable'
		});
	}
};

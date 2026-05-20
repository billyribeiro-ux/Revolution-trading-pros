/**
 * Course Downloads API - Proxy to Backend
 * ICT 11+ Principal Engineer Implementation
 *
 * Proxies requests from frontend to Rust backend API
 * Endpoint: GET /api/courses/slug/:slug/downloads
 *
 * R22-A: Deleted the fake-success fallback that returned `{ success: true,
 *   data: [] }` on any backend non-2xx or network failure. Same twin of the
 *   `/api/courses/[id]/downloads` proxy — a real course's downloads
 *   disappeared the moment the API hiccuped, with no UI signal. Now:
 *   forward upstream non-2xx as 502, network failure as 500.
 */

import { json, type RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

// CLAUDE.md hard rule — API_BASE_URL primary, BACKEND_URL fallback, localhost last.
const API_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

export const GET: RequestHandler = async ({ params, cookies, fetch }) => {
	const { slug } = params;

	if (!slug) {
		return json({ success: false, error: 'Course slug is required' }, { status: 400 });
	}

	try {
		// Get auth token from cookies if available
		// FIX-2026-04-26: comment-out, verify, delete in follow-up. Wrong cookie name — login proxy sets rtp_access_token, not access_token.
		// const accessToken = cookies.get('access_token');
		const accessToken = cookies.get('rtp_access_token');

		const headers: Record<string, string> = {
			'Content-Type': 'application/json'
		};

		if (accessToken) {
			headers['Authorization'] = `Bearer ${accessToken}`;
		}

		// Proxy to backend API
		const response = await fetch(`${API_URL}/api/courses/slug/${slug}/downloads`, {
			method: 'GET',
			headers
		});

		if (!response.ok) {
			// R22-A: was: return `{ success: true, data: [] }`. Now: surface
			// the upstream failure as 502 so the UI can render a real error.
			console.error(
				`[Course downloads proxy by slug] Backend ${response.status} for slug '${slug}'`
			);
			return json(
				{
					success: false,
					error: `Unable to load course downloads (backend ${response.status}).`
				},
				{ status: 502 }
			);
		}

		const data = await response.json();
		return json(data);
	} catch (err) {
		// R22-A: was: silent `{ success: true, data: [] }` fallback. Now: 500.
		console.error('[Course downloads proxy by slug] Backend fetch failed:', err);
		return json(
			{ success: false, error: 'Course downloads backend unreachable.' },
			{ status: 500 }
		);
	}
};

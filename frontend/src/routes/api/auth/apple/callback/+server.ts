/**
 * Apple Sign-In Callback Proxy - ICT Level 7 Principal Engineer Grade
 *
 * Proxies the Apple Sign-In callback to the backend.
 * Apple uses form_post response mode, so this is a POST handler.
 *
 * @version 1.0.0 - January 2026
 */

import { redirect, json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

const API_URL = 'https://revolution-trading-pros-api.fly.dev';

/**
 * Apple Sign-In uses form_post response mode, sending a POST request
 * with form data containing the authorization code and ID token.
 */
export const POST: RequestHandler = async ({ request }) => {
	try {
		// Get the form data from Apple's POST
		const formData = await request.formData();

		// Forward to backend
		const backendResponse = await fetch(`${API_URL}/api/auth/apple/callback`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/x-www-form-urlencoded'
			},
			body: new URLSearchParams(
				Object.fromEntries(
					Array.from(formData.entries()).map(([key, value]) => [key, String(value)])
				)
			)
		});

		// If backend returns a redirect, follow it
		if (backendResponse.redirected) {
			throw redirect(302, backendResponse.url);
		}

		// If backend returns 3xx status, get the Location header
		if (backendResponse.status >= 300 && backendResponse.status < 400) {
			const location = backendResponse.headers.get('Location');
			if (location) {
				throw redirect(302, location);
			}
		}

		// If not a redirect, return the response as-is
		const data = await backendResponse.json();
		return json(data, { status: backendResponse.status });
	} catch (error) {
		// Handle redirect errors (they're actually success cases)
		if (error && typeof error === 'object' && 'status' in error && 'location' in error) {
			throw error;
		}

		console.error('[Apple Callback] Error:', error);
		// Redirect to login with error on failure
		throw redirect(302, '/login?error=apple_callback_failed');
	}
};

/**
 * Also support GET for fallback (some Apple error cases)
 */
export const GET: RequestHandler = async ({ url }) => {
	const error = url.searchParams.get('error');
	if (error) {
		throw redirect(302, `/login?error=${encodeURIComponent(error)}`);
	}
	throw redirect(302, '/login');
};

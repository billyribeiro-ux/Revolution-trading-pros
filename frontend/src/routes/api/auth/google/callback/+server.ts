/**
 * Google OAuth Callback Proxy - ICT Level 7 Principal Engineer Grade
 *
 * Proxies the Google OAuth callback to the backend.
 * This is needed because Google's callback URL points to frontend,
 * but the token exchange happens on backend.
 *
 * @version 1.0.0 - January 2026
 */

import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';

const API_URL = API_BASE_URL || BACKEND_URL || 'http://localhost:8080';

export const GET: RequestHandler = async ({ url }) => {
	// Forward all query parameters to backend callback handler
	const queryString = url.search;
	redirect(302, `${API_URL}/api/auth/google/callback${queryString}`);
};

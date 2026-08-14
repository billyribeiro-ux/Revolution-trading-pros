/**
 * Apple Sign-In Initiation - ICT Level 7 Principal Engineer Grade
 *
 * Redirects user to backend Apple Sign-In flow.
 * The backend handles all OAuth logic (state, nonce, token validation).
 *
 * @version 1.0.0 - January 2026
 */

import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';

const API_URL = API_BASE_URL || BACKEND_URL || 'http://localhost:8080';

export const GET: RequestHandler = async () => {
	// Redirect to backend OAuth initiation endpoint
	// Backend will:
	// 1. Generate state token for CSRF protection
	// 2. Generate nonce for ID token validation
	// 3. Store state in database
	// 4. Redirect to Apple authorization URL
	redirect(302, `${API_URL}/api/auth/apple`);
};

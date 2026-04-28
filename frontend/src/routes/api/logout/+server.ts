import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

// FIX-2026-04-26: env.VITE_API_URL → canonical pattern
// const PROD_API_ROOT = 'http://localhost:8080';
// const API_ROOT = env.VITE_API_URL || env.BACKEND_URL || PROD_API_ROOT;
const API_ROOT =
	env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';
const API_URL = `${API_ROOT}/api`;

/**
 * Logout Proxy Endpoint - ICT11+ Principal Engineer Grade
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Proxies logout requests to Rust API backend with proper cookie handling.
 *
 * SECURITY:
 * - Forwards Authorization header for token validation
 * - Forwards X-Session-ID for session revocation
 * - ALWAYS clears httpOnly cookies regardless of backend response
 *
 * @version 2.0.0 - FIX: Always clear cookies on logout
 * @author Revolution Trading Pros
 */
export const POST: RequestHandler = async ({ request, cookies }) => {
	// Get tokens from cookies for backend call
	const token = cookies.get('rtp_access_token');
	const sessionId = request.headers.get('X-Session-ID') || '';

	// Call backend to invalidate session (best effort)
	try {
		await fetch(`${API_URL}/logout`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...(token && { Authorization: `Bearer ${token}` }),
				...(sessionId && { 'X-Session-ID': sessionId })
			}
		});
	} catch (error) {
		// Backend failure shouldn't prevent logout
		console.error('[Logout Proxy] Backend logout failed:', error);
	}

	// ALWAYS clear cookies regardless of backend response
	cookies.delete('rtp_access_token', { path: '/' });
	cookies.delete('rtp_refresh_token', { path: '/' });

	return json({ success: true, message: 'Logged out successfully' });
};

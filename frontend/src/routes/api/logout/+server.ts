import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { env } from '$env/dynamic/private';

const API_URL = env.VITE_API_URL || 'http://localhost:8000/api';

/**
 * Logout Proxy Endpoint - ICT11+ Principal Engineer Grade
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Proxies logout requests to Laravel backend with proper cookie handling.
 *
 * SECURITY:
 * - Forwards Authorization header for token validation
 * - Forwards X-Session-ID for session revocation
 * - Includes credentials for httpOnly cookie cleanup
 * - Propagates Set-Cookie headers for client-side cleanup
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
export const POST: RequestHandler = async ({ request, cookies }) => {
	try {
		// Extract auth headers from client request
		const authHeader = request.headers.get('Authorization') || '';
		const sessionId = request.headers.get('X-Session-ID') || '';
		const cookieHeader = request.headers.get('cookie') || '';

		// Forward request to Laravel backend
		const response = await fetch(`${API_URL}/logout`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				'Accept': 'application/json',
				'Authorization': authHeader,
				'X-Session-ID': sessionId,
				'Cookie': cookieHeader
			},
			credentials: 'include'
		});

		// Parse response (may be empty on success)
		let data: Record<string, unknown>;
		try {
			data = await response.json();
		} catch {
			// Empty response body is valid for logout
			data = { message: 'Logged out successfully' };
		}

		// Forward Set-Cookie headers to clear httpOnly cookies
		const setCookieHeader = response.headers.get('set-cookie');
		const headers: Record<string, string> = {};

		if (setCookieHeader) {
			headers['Set-Cookie'] = setCookieHeader;
		}

		return json(data, {
			status: response.status,
			headers
		});
	} catch (error) {
		// ICT11+ Pattern: Log error but don't expose internals
		console.error('[API Proxy] Logout error:', error);

		// Return success anyway - client state should be cleared regardless
		// This prevents logout failures from blocking the user
		return json(
			{ message: 'Logged out successfully' },
			{ status: 200 }
		);
	}
};

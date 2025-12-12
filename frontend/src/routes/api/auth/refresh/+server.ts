import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

const API_URL = process.env.VITE_API_URL || 'http://localhost:8000/api';

/**
 * Proxy auth refresh requests to Laravel backend
 * This allows the frontend to use relative URLs while the backend handles auth
 */
export const POST: RequestHandler = async ({ request, cookies }) => {
	try {
		const sessionId = request.headers.get('X-Session-ID') || '';
		
		// Forward all cookies to backend
		const cookieHeader = request.headers.get('cookie') || '';
		
		const response = await fetch(`${API_URL}/auth/refresh`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				'Accept': 'application/json',
				'X-Session-ID': sessionId,
				'Cookie': cookieHeader
			},
			credentials: 'include'
		});

		const data = await response.json();

		// Forward Set-Cookie headers from backend
		const setCookieHeader = response.headers.get('set-cookie');
		
		return json(data, {
			status: response.status,
			headers: setCookieHeader ? { 'Set-Cookie': setCookieHeader } : {}
		});
	} catch (error) {
		console.error('[API Proxy] Auth refresh error:', error);
		return json(
			{ error: 'Token refresh failed', message: 'Unable to refresh authentication token' },
			{ status: 500 }
		);
	}
};

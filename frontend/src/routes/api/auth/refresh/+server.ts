import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { env } from '$env/dynamic/private';

// Production fallback - NEVER use localhost in production
const PROD_API = 'https://revolution-backend.fly.dev/api';
const API_URL = env.VITE_API_URL || env.BACKEND_URL ? `${env.BACKEND_URL}/api` : PROD_API;

/**
 * Proxy auth refresh requests to Laravel backend
 * ICT11+ Principal Engineer: Forwards refresh_token in body as backend expects
 */
export const POST: RequestHandler = async ({ request, cookies }) => {
	try {
		const sessionId = request.headers.get('X-Session-ID') || '';
		
		// Forward all cookies to backend
		const cookieHeader = request.headers.get('cookie') || '';
		
		// Parse request body to get refresh_token
		let body: { refresh_token?: string } = {};
		try {
			body = await request.json();
		} catch {
			// Empty body is acceptable for cookie-based refresh
		}
		
		const response = await fetch(`${API_URL}/auth/refresh`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				'Accept': 'application/json',
				'X-Session-ID': sessionId,
				'Cookie': cookieHeader
			},
			credentials: 'include',
			body: JSON.stringify(body)
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

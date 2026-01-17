/**
 * Same-origin proxy for analytics event tracking
 * ICT 11+ CORB Fix: Routes client-side analytics calls through SvelteKit server
 */
import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const PROD_API_ROOT = 'https://revolution-trading-pros-api.fly.dev';
const API_ROOT = env.VITE_API_URL || env.BACKEND_URL || PROD_API_ROOT;

export const POST: RequestHandler = async ({ request, fetch, cookies }) => {
	try {
		const body = await request.json();
		const token = cookies.get('rtp_access_token');

		const headers: Record<string, string> = {
			Accept: 'application/json',
			'Content-Type': 'application/json'
		};
		if (token) headers.Authorization = `Bearer ${token}`;

		const response = await fetch(`${API_ROOT}/api/analytics/track`, {
			method: 'POST',
			headers,
			body: JSON.stringify(body)
		});

		if (!response.ok) {
			return json({ success: false }, { status: response.status });
		}

		const data = await response.json();
		return json(data, { status: 200 });
	} catch (error) {
		console.error('[Analytics Track Proxy] Error:', error);
		return json({ success: false }, { status: 200 });
	}
};
